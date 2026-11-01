//! Measure the certified components against their Rust oracles (#277) and
//! write `target/bootstrap-perf/{results.json,table.md}`:
//!
//! `cargo run --release -p repo-conformance --bin bootstrap_perf [package…]`
//!
//! With no package named, every subject is measured.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use repo_conformance::cases::bootstrap_perf::{measure, Heap, STACK};

/// The system allocator, counting the bytes it holds and their peak.
struct Counting;

static HELD: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

// SAFETY: every method forwards to `System` with the caller's arguments, so
// it upholds `GlobalAlloc`'s contract exactly as `System` does; the counters
// are only read for the measurement.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller's contract for `alloc` is `System.alloc`'s.
        let block = unsafe { System.alloc(layout) };
        if !block.is_null() {
            let held = HELD.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
            PEAK.fetch_max(held, Ordering::Relaxed);
        }
        block
    }

    unsafe fn dealloc(&self, block: *mut u8, layout: Layout) {
        // SAFETY: `block` came from `alloc` or `realloc` above, i.e. from
        // `System`, with this layout.
        unsafe { System.dealloc(block, layout) };
        HELD.fetch_sub(layout.size(), Ordering::Relaxed);
    }

    unsafe fn realloc(&self, block: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        // SAFETY: as `dealloc`, and `size` is the caller's, as `realloc`
        // requires.
        let moved = unsafe { System.realloc(block, layout, size) };
        if !moved.is_null() {
            if size >= layout.size() {
                let grown = size - layout.size();
                let held = HELD.fetch_add(grown, Ordering::Relaxed) + grown;
                PEAK.fetch_max(held, Ordering::Relaxed);
            } else {
                HELD.fetch_sub(layout.size() - size, Ordering::Relaxed);
            }
        }
        moved
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

struct Counted;

impl Heap for Counted {
    fn mark(&self) -> usize {
        let held = HELD.load(Ordering::Relaxed);
        PEAK.store(held, Ordering::Relaxed);
        held
    }

    fn peak_since(&self, mark: usize) -> usize {
        PEAK.load(Ordering::Relaxed).saturating_sub(mark)
    }
}

fn main() {
    let named: Vec<String> = std::env::args().skip(1).collect();
    let table = std::thread::Builder::new()
        .stack_size(STACK)
        .spawn(move || {
            measure(&Counted, &|package| {
                named.is_empty() || named.iter().any(|name| name == package)
            })
        })
        .expect("the measuring thread starts")
        .join()
        .unwrap_or_else(|failure| std::panic::resume_unwind(failure));
    println!("{table}");
}
