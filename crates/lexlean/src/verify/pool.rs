//! The scheduler of the independent proof processes of one verification stage
//! (SPEC.md §22.11).
//!
//! The processes of a frontier of module elaboration, of the replays, and of
//! the audit members do not depend on each other, so up to `width` of them run
//! at once. What the caller sees does not depend on how many did: results are
//! handed to it one at a time, in the order of the items, whatever order the
//! processes finished in. That is the whole of the contract, and it is why a
//! verification at width 4 publishes the bytes a verification at width 1
//! does — nothing downstream of this function can observe a completion order.
//!
//! A failure is the first one in item order, too. Workers claim items from a
//! start order the caller's hints may weight (longest, and largest expected
//! memory footprint, first), but the results they produce are consumed in
//! item order. When item `f` falls in item order every earlier item was
//! claimed and is finished before this returns; a later item that is already
//! running is finished and its result dropped; no item after a known failure
//! is started. The failure reported is therefore the failure width 1 would
//! have stopped at.
//!
//! A memory budget (§22.11) is a second gate on claiming: an item is started
//! only when the summed expected peaks of the running items plus its own stay
//! within the budget. It is a schedule, not a kill: a process that outgrows
//! its hint runs to completion and is measured by the surrounding stage.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Condvar, Mutex};

use crate::code;
use crate::diagnostic::Diagnostic;
use crate::error::LexLeanError;
use crate::verify::operational::{Hint, Hints};
use crate::verify::telemetry;

/// Release the expected memory of one claimed item when dropped: subtracts
/// `expected` and wakes every waiter, in one lock window so no two workers
/// interleave moot notifications between them. (A second, nested lock of a
/// single-word `Mutex<u64>` inside the claim window deadlocks a stage's
/// release path, which is how the two-lock form of this block once froze
/// the gate's own concurrency test.)
struct ReleaseOnDrop<'a> {
    in_flight: &'a Mutex<u64>,
    freed: &'a Condvar,
    expected: u64,
}

impl Drop for ReleaseOnDrop<'_> {
    fn drop(&mut self) {
        let mut in_flight = self.in_flight.lock().expect("budget lock");
        *in_flight = in_flight.saturating_sub(self.expected);
        drop(in_flight);
        self.freed.notify_all();
    }
}

fn internal(message: &str) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(
        code!("LLI9001"),
        format!("phase verify: {message}"),
    ))
}

/// The start order of the items of one stage: longest observed wall time
/// first, largest expected memory footprint first, then the caller's item
/// order to keep the schedule deterministic. Hints label items by name; an
/// item no hint knows starts last among equals.
pub fn start_order(
    items: &[usize],
    hints: Option<&Hints>,
    label: &dyn Fn(usize) -> String,
) -> Vec<usize> {
    let mut ordered: Vec<usize> = items.to_vec();
    ordered.sort_by_key(|&item| {
        let hint = hints.and_then(|hints| hints.get(&label(item))).copied();
        let (wall_ms, rss) = hint.map_or((0, 0), |hint: Hint| {
            (hint.wall_ms, hint.peak_rss_kib.unwrap_or(0))
        });
        (std::cmp::Reverse(wall_ms), std::cmp::Reverse(rss))
    });
    ordered
}

/// The claim order with one twist: items after the failure boundary `stop_at`
/// (a position in item order) move to the tail, so a worker whose claimed
/// item loses to the failure check does not keep an early slot of a worker
/// holding the budget of an item before the failure.
pub fn claim_order(
    ordered: &[usize],
    positions: &BTreeMap<usize, usize>,
    stop_at: usize,
) -> Vec<usize> {
    let (before, after): (Vec<usize>, Vec<usize>) = ordered
        .iter()
        .copied()
        .partition(|item| positions[item] <= stop_at);
    before.into_iter().chain(after).collect()
}

/// Run `job` on each of `items`, at most `width` at a time, and hand each
/// result to `consume` in the order of `items`.
///
/// `is_failure` says whether a result is one `consume` will refuse; at width
/// above 1 it lets the workers stop early, and `consume` remains the judge: it must
/// return an error for every result `is_failure` accepts, and the first error
/// it returns in item order is the one returned here.
///
/// # Errors
///
/// The first error `consume` returns, or `LLI9001` when a worker panicked or
/// stopped without a failure having been consumed.
pub fn run_ordered<R, C>(
    width: usize,
    items: &[usize],
    job: &(impl Fn(usize) -> R + Sync),
    is_failure: &(impl Fn(&R) -> bool + Sync),
    consume: C,
) -> Result<(), LexLeanError>
where
    R: Send,
    C: FnMut(usize, R) -> Result<(), LexLeanError>,
{
    run_ordered_with(
        width,
        items,
        None,
        None,
        &|_| String::new(),
        job,
        is_failure,
        consume,
    )
}

/// [`run_ordered`] with the full resource profile (§22.11): `hints` pick the
/// start order under `label`, and `memory_kib` stops a worker from starting
/// an item whose expected peak (plus those of the items already running)
/// would pass the budget. A stage without hints or a budget schedules in
/// item order exactly as [`run_ordered`] does.
#[allow(clippy::too_many_arguments)]
pub fn run_ordered_with<R, C>(
    width: usize,
    items: &[usize],
    hints: Option<&Hints>,
    memory_kib: Option<u64>,
    label: &dyn Fn(usize) -> String,
    job: &(impl Fn(usize) -> R + Sync),
    is_failure: &(impl Fn(&R) -> bool + Sync),
    mut consume: C,
) -> Result<(), LexLeanError>
where
    R: Send,
    C: FnMut(usize, R) -> Result<(), LexLeanError>,
{
    if items.is_empty() {
        return Ok(());
    }
    // Width 1 is the sequence the verifier ran before there was a width: item
    // `i + 1` starts only after item `i` has been consumed, on the calling
    // thread, so a failure only `consume` can see (a missing `.olean`, a
    // staging error) stops the stage before another process is started.
    if width == 1 {
        for (index, &item) in items.iter().enumerate() {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| job(item)))
                .map_err(|_| internal("verification process worker panicked"))?;
            telemetry::completed(item, index + 1);
            let failed = is_failure(&result);
            consume(item, result)?;
            if failed {
                return Err(internal(
                    "verification stage consumed a failed process without refusing it",
                ));
            }
        }
        return Ok(());
    }
    let workers = width.clamp(1, items.len());
    // The order workers claim items in: hint-weighted, so that a stage's
    // heaviest processes start earliest. A hint only reorders claims: the
    // results are consumed in `items` order below, and a failure is still
    // the least-position one in that order.
    let order = start_order(items, hints, label);
    // Positions are within the *caller\'s* `items` array — they decide both
    // what may still start after a failure (stop_after is a position in that
    // array) and what order results are consumed in. The start order is only
    // a permutation of the same items: it changes nothing about positions.
    let position_of: BTreeMap<usize, usize> = items
        .iter()
        .copied()
        .enumerate()
        .map(|(position, item)| (item, position))
        .collect();
    let claim_index = AtomicUsize::new(0);
    /// Where the claimant and consumer meet the memory budget.
    struct MemoryGate {
        budget: u64,
        in_flight: Mutex<u64>,
        freed: Condvar,
    }
    let gate = memory_kib.map(|budget| MemoryGate {
        budget,
        in_flight: Mutex::new(0),
        freed: Condvar::new(),
    });
    let expected_by_item: BTreeMap<usize, u64> = order
        .iter()
        .map(|&item| {
            (
                item,
                if gate.is_none() {
                    0
                } else {
                    hints
                        .and_then(|hints| hints.get(&label(item)))
                        .and_then(|hint| hint.peak_rss_kib)
                        .unwrap_or(0)
                },
            )
        })
        .collect();
    // The least position (in item order) known to have failed. Items after
    // it are not claimed; items before it still are, because the failure to
    // report is the least one in item order and they may hold it.
    let stop_after = AtomicUsize::new(usize::MAX);
    let (sender, receiver) = mpsc::channel::<(usize, R)>();
    let context = telemetry::current();
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|_| {
                let sender = sender.clone();
                let context = context.clone();
                let (claim_index, stop_after) = (&claim_index, &stop_after);
                let gate = gate.as_ref();
                let (order, position_of, expected_by_item) =
                    (&order, &position_of, &expected_by_item);
                scope.spawn(move || {
                    let _installed = telemetry::install(context);
                    loop {
                        let sequence = claim_index.fetch_add(1, Ordering::SeqCst);
                        let stop_at = stop_after.load(Ordering::SeqCst);
                        let effective = claim_order(order, position_of, stop_at);
                        if sequence >= effective.len() {
                            break;
                        }
                        let item = effective[sequence];
                        let position = position_of[&item];
                        if position > stop_at {
                            break;
                        }
                        let expected = expected_by_item[&item];
                        if let Some(gate) = gate {
                            // Claim in one lock window; the expected peak is
                            // credited, and its release is a `Drop` guard so a
                            // second lock of the same single-word Mutex in one
                            // thread's window cannot appear (that nested form
                            // once froze the gate's own concurrency test).
                            let release = {
                                let mut in_flight = gate.in_flight.lock().expect("budget lock");
                                // An item whose expected peak alone passes the
                                // budget runs alone rather than waiting forever:
                                // the hint is wrong, not a fence (§22.11: a
                                // hint is a schedule input, never a kill).
                                while *in_flight > 0 && *in_flight + expected > gate.budget {
                                    in_flight = gate.freed.wait(in_flight).expect("budget lock");
                                }
                                *in_flight += expected;
                                ReleaseOnDrop {
                                    in_flight: &gate.in_flight,
                                    freed: &gate.freed,
                                    expected,
                                }
                            };
                            let result = job(item);
                            // The claim's memory entry is released here, one
                            // lock window at the end of the job.
                            drop(release);
                            if is_failure(&result) {
                                stop_after.fetch_min(position, Ordering::SeqCst);
                            }
                            if sender.send((position, result)).is_err() {
                                return;
                            }
                            continue;
                        }
                        let result = job(item);
                        if is_failure(&result) {
                            stop_after.fetch_min(position, Ordering::SeqCst);
                        }
                        if sender.send((position, result)).is_err() {
                            break;
                        }
                    }
                })
            })
            .collect();
        drop(sender);

        let mut arrived: BTreeMap<usize, R> = BTreeMap::new();
        let mut consumed = 0usize;
        let mut outcome: Result<(), LexLeanError> = Ok(());
        let mut sequence = 0usize;
        for (position, result) in receiver {
            sequence += 1;
            telemetry::completed(items[position], sequence);
            arrived.insert(position, result);
            while outcome.is_ok() {
                let Some(ready) = arrived.remove(&consumed) else {
                    break;
                };
                if let Err(error) = consume(items[consumed], ready) {
                    stop_after.fetch_min(consumed, Ordering::SeqCst);
                    outcome = Err(error);
                }
                consumed += 1;
            }
        }
        // Every worker has dropped its sender, so each has finished (or
        // panicked) and nothing started here outlives this call.
        let panicked = handles
            .into_iter()
            .map(std::thread::ScopedJoinHandle::join)
            .any(|joined| joined.is_err());
        if outcome.is_ok() && panicked {
            return Err(internal("verification process worker panicked"));
        }
        if outcome.is_ok() && consumed != items.len() {
            return Err(internal(
                "verification stage stopped before every process was consumed",
            ));
        }
        outcome
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{mpsc, Mutex};
    use std::time::{Duration, Instant};

    use super::{run_ordered, run_ordered_with, start_order};
    use crate::diagnostic::Diagnostic;
    use crate::error::LexLeanError;
    use crate::verify::operational::Hint;

    fn refusal(text: &str) -> LexLeanError {
        LexLeanError::from_diagnostic(Diagnostic::new(crate::code!("LLI9001"), text))
    }

    /// Items 0..n finish in reverse order: item `i` waits for item `i + 1`.
    /// The results are still consumed in item order.
    #[test]
    fn results_are_consumed_in_item_order_whatever_order_they_finish_in() {
        let items: Vec<usize> = (0..4).collect();
        let senders: Vec<_> = (0..items.len()).map(|_| mpsc::channel::<()>()).collect();
        let (txs, rxs): (Vec<_>, Vec<_>) = senders.into_iter().unzip();
        let rxs: Vec<Mutex<mpsc::Receiver<()>>> = rxs.into_iter().map(Mutex::new).collect();
        let finished = Mutex::new(Vec::new());
        let mut consumed = Vec::new();
        run_ordered(
            4,
            &items,
            &|item: usize| {
                if item + 1 < 4 {
                    rxs[item]
                        .lock()
                        .expect("receiver lock")
                        .recv_timeout(Duration::from_secs(5))
                        .expect("the next item finishes first");
                }
                finished.lock().expect("finished lock").push(item);
                if item > 0 {
                    txs[item - 1].send(()).expect("wake the previous item");
                }
                item * 10
            },
            &|_: &usize| false,
            |item, value| {
                assert_eq!(value, item * 10);
                consumed.push(item);
                Ok(())
            },
        )
        .expect("all four succeed");
        assert_eq!(
            *finished.lock().expect("finished lock"),
            vec![3, 2, 1, 0],
            "the jobs really did finish in reverse order"
        );
        assert_eq!(
            consumed,
            vec![0, 1, 2, 3],
            "and were consumed in item order"
        );
    }

    /// Two items fail; the one with the least position is the one reported,
    /// though the later one finishes first.
    #[test]
    fn the_first_failure_in_item_order_is_the_one_reported() {
        let items: Vec<usize> = (0..4).collect();
        let (tx, rx) = mpsc::channel::<()>();
        let rx = Mutex::new(rx);
        let outcome = run_ordered(
            4,
            &items,
            &|item: usize| {
                match item {
                    1 => {
                        rx.lock()
                            .expect("receiver lock")
                            .recv_timeout(Duration::from_secs(5))
                            .expect("item 3 fails first");
                    }
                    3 => tx.send(()).expect("wake item 1"),
                    _ => {}
                }
                item
            },
            &|value: &usize| *value == 1 || *value == 3,
            |item, value| {
                if value == 1 || value == 3 {
                    Err(refusal(&format!("item {item} failed")))
                } else {
                    Ok(())
                }
            },
        );
        let error = outcome.expect_err("two items fail");
        assert!(
            error.to_string().contains("item 1 failed"),
            "the first failure in item order is reported, not the first to finish: {error}"
        );
    }

    /// At width 1 an item after a failure is never started, and at any width
    /// the number of items started after the failed one stays below `width`.
    #[test]
    fn nothing_is_started_after_a_known_failure_at_width_one() {
        let items: Vec<usize> = (0..6).collect();
        let started = AtomicUsize::new(0);
        let outcome = run_ordered(
            1,
            &items,
            &|item: usize| {
                started.fetch_add(1, Ordering::SeqCst);
                item
            },
            &|value: &usize| *value == 2,
            |_, value| {
                if value == 2 {
                    Err(refusal("item 2 failed"))
                } else {
                    Ok(())
                }
            },
        );
        assert!(outcome.is_err());
        assert_eq!(started.load(Ordering::SeqCst), 3, "items 0, 1 and 2 only");
    }

    /// At width 1 the stage is strictly sequential: a job starts only after
    /// the previous result has been consumed.
    #[test]
    fn width_one_alternates_job_and_consume() {
        let items: Vec<usize> = (0..4).collect();
        let log = Mutex::new(Vec::new());
        let mut order = Vec::new();
        run_ordered(
            1,
            &items,
            &|item: usize| {
                log.lock().expect("lock").push(format!("job {item}"));
                item
            },
            &|_: &usize| false,
            |item, _| {
                log.lock().expect("lock").push(format!("consume {item}"));
                order.push(item);
                Ok(())
            },
        )
        .expect("four jobs");
        assert_eq!(
            *log.lock().expect("lock"),
            [
                "job 0",
                "consume 0",
                "job 1",
                "consume 1",
                "job 2",
                "consume 2",
                "job 3",
                "consume 3"
            ]
        );
        assert_eq!(order, [0, 1, 2, 3]);
        // A failure only `consume` can see ends the stage before the next job.
        let started = AtomicUsize::new(0);
        let outcome = run_ordered(
            1,
            &items,
            &|item: usize| {
                started.fetch_add(1, Ordering::SeqCst);
                item
            },
            &|_: &usize| false,
            |item, _| {
                if item == 0 {
                    Err(refusal("only consume sees this"))
                } else {
                    Ok(())
                }
            },
        );
        assert!(outcome.is_err());
        assert_eq!(
            started.load(Ordering::SeqCst),
            1,
            "no process after the refusal"
        );
    }

    /// Of two failures known out of item order, the earlier position bounds the
    /// items that may start: with failures at 1 and then at 3 (both already
    /// running), none of the eight items after them is started, and the failure
    /// reported is item 1's.
    #[test]
    fn a_later_failure_does_not_reopen_the_stage() {
        let items: Vec<usize> = (0..12).collect();
        let started = Mutex::new(Vec::new());
        let (first_failed, wait_for_first) = mpsc::channel::<()>();
        let wait_for_first = Mutex::new(wait_for_first);
        let third_finished = std::sync::atomic::AtomicBool::new(false);
        let outcome = run_ordered(
            4,
            &items,
            &|item: usize| {
                started.lock().expect("lock").push(item);
                match item {
                    // Item 1 fails first, item 3 after it; items 0 and 2 stay
                    // busy until both have, so no worker is free to claim more
                    // before the failures are known.
                    1 => {
                        // Not before item 3 is running: it fails after this.
                        let give_up = Instant::now() + Duration::from_secs(10);
                        while started.lock().expect("lock").len() < 4 && Instant::now() < give_up {
                            std::thread::sleep(Duration::from_millis(5));
                        }
                        first_failed.send(()).expect("announce");
                    }
                    3 => {
                        wait_for_first
                            .lock()
                            .expect("lock")
                            .recv_timeout(Duration::from_secs(10))
                            .expect("item 1 failed first");
                        std::thread::sleep(Duration::from_millis(50));
                        third_finished.store(true, Ordering::SeqCst);
                    }
                    0 | 2 => {
                        let give_up = Instant::now() + Duration::from_secs(10);
                        while !third_finished.load(Ordering::SeqCst) && Instant::now() < give_up {
                            std::thread::sleep(Duration::from_millis(5));
                        }
                        std::thread::sleep(Duration::from_millis(50));
                    }
                    _ => {}
                }
                item
            },
            &|item: &usize| *item == 1 || *item == 3,
            |item, value| {
                if value == 1 || value == 3 {
                    Err(refusal(&format!("item {item} failed")))
                } else {
                    Ok(())
                }
            },
        );
        let error = outcome.expect_err("two items fail");
        assert!(error.to_string().contains("item 1 failed"), "{error}");
        let mut seen = started.lock().expect("lock").clone();
        seen.sort_unstable();
        assert_eq!(
            seen,
            [0, 1, 2, 3],
            "nothing after the failure at item 1 started"
        );
    }

    #[test]
    fn a_panicking_worker_is_an_internal_error_not_a_hang() {
        let items: Vec<usize> = (0..3).collect();
        let outcome = run_ordered(
            2,
            &items,
            &|item: usize| {
                assert!(item != 1, "a job panics");
                item
            },
            &|_: &usize| false,
            |_, _| Ok(()),
        );
        let error = outcome.expect_err("the panic is reported");
        assert!(error.to_string().contains("LLI9001"), "{error}");
    }

    #[test]
    fn no_items_is_nothing_to_do() {
        run_ordered(3, &[], &|item: usize| item, &|_: &usize| false, |_, _| {
            Ok(())
        })
        .expect("empty");
    }

    /// The heaviest items start first, while results still arrive and are
    /// consumed in item order.
    #[test]
    fn hints_order_the_start_and_never_the_consumption() {
        let items: Vec<usize> = (0..4).collect();
        let hints: BTreeMap<String, Hint> = BTreeMap::from([
            (
                "i3".to_owned(),
                Hint {
                    peak_rss_kib: Some(50),
                    wall_ms: 900,
                },
            ),
            (
                "i1".to_owned(),
                Hint {
                    peak_rss_kib: Some(10),
                    wall_ms: 400,
                },
            ),
        ]);
        let label = |item: usize| format!("i{item}");
        let ordered = start_order(&items, Some(&hints), &label);
        assert_eq!(ordered, vec![3, 1, 0, 2], "longest and largest first");
        let starts = Mutex::new(Vec::new());
        let mut consumed = Vec::new();
        run_ordered_with(
            4,
            &items,
            Some(&hints),
            None,
            &label,
            &|item: usize| {
                starts.lock().expect("lock").push(item);
                std::thread::sleep(Duration::from_millis(20));
                item
            },
            &|_: &usize| false,
            |item, _| {
                consumed.push(item);
                Ok(())
            },
        )
        .expect("all items");
        let firsts: Vec<usize> = starts.lock().expect("lock").clone();
        assert_eq!(
            &firsts[..3],
            &[3, 1, 0],
            "the hinted heaviest was started first"
        );
        assert_eq!(consumed, [0, 1, 2, 3], "consumed in item order");
    }

    /// Under a memory budget the hinted big process never runs together with
    /// any other; the stage still finishes every item. A hint whose expected
    /// peak alone exceeds the budget is climaxed, not stuck: the item runs
    /// solo (a wrong hint, not a fence).
    #[test]
    fn the_memory_budget_gates_concurrent_claims() {
        let items: Vec<usize> = (0..3).collect();
        let hints: BTreeMap<String, Hint> = BTreeMap::from([
            (
                "big".to_owned(),
                Hint {
                    peak_rss_kib: Some(800),
                    wall_ms: 900,
                },
            ),
            (
                "small1".to_owned(),
                Hint {
                    peak_rss_kib: Some(300),
                    wall_ms: 100,
                },
            ),
            (
                "small2".to_owned(),
                Hint {
                    peak_rss_kib: Some(300),
                    wall_ms: 90,
                },
            ),
        ]);
        let label = |item: usize| match item {
            0 => "big".to_owned(),
            1 => "small1".to_owned(),
            _ => "small2".to_owned(),
        };
        let runs = Mutex::new(std::collections::BTreeMap::<usize, (u128, u128)>::new());
        let clock = Instant::now();
        run_ordered_with(
            3,
            &items,
            Some(&hints),
            Some(899),
            &label,
            &|item: usize| {
                let start = clock.elapsed().as_millis();
                std::thread::sleep(Duration::from_millis(if item == 0 { 150 } else { 60 }));
                let end = clock.elapsed().as_millis();
                runs.lock().expect("runs lock").insert(item, (start, end));
                item
            },
            &|_: &usize| false,
            |_, _| Ok(()),
        )
        .expect("all items");
        let spans = runs.into_inner().expect("runs lock");
        let relax = 25u128; // wall-clock slop between thread wakeups
        let (a_start, a_end) = spans[&0];
        for other in [1, 2] {
            let (b_start, b_end) = spans[&other];
            let overlaps =
                a_start < b_end.saturating_sub(relax) && b_start < a_end.saturating_sub(relax);
            assert!(
                !overlaps,
                "big [{a_start},{a_end}] overlapped {other} [{b_start},{b_end}] past the budget"
            );
        }
        // An item whose expected peak alone exceeds the budget must run
        // (alone) rather than wait forever — a wrong hint, not a fence.
        let over: Vec<usize> = vec![0];
        let hints_over: BTreeMap<String, Hint> = BTreeMap::from([(
            "alone".to_owned(),
            Hint {
                peak_rss_kib: Some(1_000_000),
                wall_ms: 1,
            },
        )]);
        run_ordered_with(
            2,
            &over,
            Some(&hints_over),
            Some(1),
            &|_| "alone".to_owned(),
            &|item: usize| item,
            &|_: &usize| false,
            |_, _| Ok(()),
        )
        .expect("the over-budget hint does not fence the stage");
    }

    /// A failure still resolves to the least position in item order when
    /// hints reordered the claims.
    #[test]
    fn a_reordered_failure_is_still_the_first_in_item_order() {
        let items: Vec<usize> = (0..3).collect();
        let hints: BTreeMap<String, Hint> = BTreeMap::from([(
            "i2".to_owned(),
            Hint {
                peak_rss_kib: None,
                wall_ms: 900,
            },
        )]);
        let outcome = run_ordered_with(
            2,
            &items,
            Some(&hints),
            None,
            &|item: usize| format!("i{item}"),
            &|item: usize| item,
            &|item: &usize| *item == 1 || *item == 2,
            |item, _| {
                if item == 1 || item == 2 {
                    Err(refusal(&format!("item {item} failed")))
                } else {
                    Ok(())
                }
            },
        );
        let error = outcome.expect_err("two fail");
        assert!(error.to_string().contains("item 1 failed"), "{error}");
    }
}
