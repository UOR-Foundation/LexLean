//! Target machine contracts (SPEC.md §17.15, §17.22, `GN-09`, #119): the
//! `LexLeanStd.Target` module against the instructions it models.
//!
//! Each of the nine instruction roots is lowered and run by the calculus
//! evaluator on a seeded corpus whose lanes sit at the saturation and wrap
//! edges, and the same corpus is run by the instruction itself, compiled by
//! the pinned rustc under `#[target_feature]`. Where the host can execute the
//! target the two must agree on every case; where it cannot, the harness is
//! built for the target and the case says so (C9: rustc and the hardware are
//! checked by differentials, never assumed).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;
use std::process::Command;

use lexlean::calculus::{interp, rust, Value};
use lexlean::production::lower::Lowered;
use serde_json::Value as Json;

use super::stdlib::lower_in;
use crate::support::{self, P};

/// The module.
const MODULE: &str = "LexLeanStd.Target";

/// The effects every instruction root declares and its closure realizes: the
/// result list is allocated, the lane arithmetic is checked, and the lanes
/// are walked by recursion.
const EFFECTS: [&str; 3] = ["allocation", "overflow", "recursion"];

/// The evaluator's fuel.
const FUEL: u64 = 5_000_000;
/// Seeded cases per instruction, beside the edges.
const SEEDED: usize = 300;

/// An inclusive range of lane values.
type Range = (i32, i32);
const U8: Range = (0, 255);
const I8: Range = (-128, 127);
const I16: Range = (-32768, 32767);

/// One modeled instruction: its root, the operand shapes it reads, and the
/// target it belongs to. The index in [`INSTRUCTIONS`] is the code the
/// generated harness dispatches on.
struct Instruction {
    root: &'static str,
    target: usize,
    first: (usize, Range),
    /// Lanes and range of the second operand; none for a unary instruction.
    second: Option<(usize, Range)>,
}

const INSTRUCTIONS: [Instruction; 9] = [
    Instruction {
        root: "avx2MaddubsEpi16",
        target: 0,
        first: (32, U8),
        second: Some((32, I8)),
    },
    Instruction {
        root: "avx2MaddEpi16",
        target: 0,
        first: (16, I16),
        second: Some((16, I16)),
    },
    Instruction {
        root: "avx2AddsEpi16",
        target: 0,
        first: (16, I16),
        second: Some((16, I16)),
    },
    Instruction {
        root: "neonMullS8",
        target: 1,
        first: (8, I8),
        second: Some((8, I8)),
    },
    Instruction {
        root: "neonPaddlqS16",
        target: 1,
        first: (8, I16),
        second: None,
    },
    Instruction {
        root: "neonQaddqS16",
        target: 1,
        first: (8, I16),
        second: Some((8, I16)),
    },
    Instruction {
        root: "wasmExtmulLowI8x16",
        target: 2,
        first: (16, I8),
        second: Some((16, I8)),
    },
    Instruction {
        root: "wasmDotI16x8",
        target: 2,
        first: (8, I16),
        second: Some((8, I16)),
    },
    Instruction {
        root: "wasmAddSatI16x8",
        target: 2,
        first: (8, I16),
        second: Some((8, I16)),
    },
];

/// A D4 target.
struct Target {
    triple: &'static str,
    feature: &'static str,
}

const TARGETS: [Target; 3] = [
    Target {
        triple: "x86_64-unknown-linux-gnu",
        feature: "avx2",
    },
    Target {
        triple: "aarch64-unknown-linux-gnu",
        feature: "neon",
    },
    Target {
        triple: "wasm32-unknown-unknown",
        feature: "simd128",
    },
];

/// A deterministic generator (xorshift64*), so the corpus is the same on
/// every host.
struct Seeded(u64);

impl Seeded {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
    /// A lane: half the time an edge of the range or zero, else uniform.
    fn lane(&mut self, (low, high): Range) -> i32 {
        if self.below(2) == 0 {
            let edges = [
                low,
                high,
                0_i32.max(low),
                (-1_i32).max(low),
                1_i32.min(high),
                low + 1,
                high - 1,
            ];
            edges[usize::try_from(self.below(7)).expect("an index")]
        } else {
            let span = u64::try_from(i64::from(high) - i64::from(low) + 1).expect("a span");
            low + i32::try_from(self.below(span)).expect("a lane")
        }
    }
}

/// One case: the instruction and its operands.
type Case = (usize, Vec<i32>, Vec<i32>);

/// The corpus of an instruction: every lane at one edge and at the other, the
/// edges alternating, then seeded cases.
fn corpus(code: usize) -> Vec<Case> {
    let instruction = &INSTRUCTIONS[code];
    let mut seeded = Seeded(0x9e37_79b9_7f4a_7c15 ^ u64::try_from(code).expect("a code") << 32);
    let (first_lanes, first_range) = instruction.first;
    let second = instruction.second;
    let mut cases = Vec::new();
    let patterns: [fn(Range, usize) -> i32; 5] = [
        |(low, _), _| low,
        |(_, high), _| high,
        |(low, high), index| if index % 2 == 0 { low } else { high },
        |(low, high), index| if index % 2 == 0 { high } else { low },
        |_, _| 0,
    ];
    for first_pattern in patterns {
        for second_pattern in patterns {
            cases.push((
                code,
                (0..first_lanes)
                    .map(|index| first_pattern(first_range, index))
                    .collect(),
                second.map_or_else(Vec::new, |(lanes, range)| {
                    (0..lanes)
                        .map(|index| second_pattern(range, index))
                        .collect()
                }),
            ));
        }
    }
    for _ in 0..SEEDED {
        cases.push((
            code,
            (0..first_lanes).map(|_| seeded.lane(first_range)).collect(),
            second.map_or_else(Vec::new, |(lanes, range)| {
                (0..lanes).map(|_| seeded.lane(range)).collect()
            }),
        ));
    }
    cases
}

fn list(lanes: &[i32]) -> Value {
    Value::List {
        items: lanes
            .iter()
            .map(|lane| Value::Int {
                value: lane.to_string(),
            })
            .collect(),
    }
}

/// The lanes of an evaluator result. They are wider than a lane of the
/// instruction, so that a model that forgets to wrap or saturate disagrees
/// with the instruction instead of failing to parse.
fn lanes_of(observed: &Json) -> Vec<i64> {
    assert_eq!(observed["kind"], "list", "a list of lanes: {observed}");
    observed["items"]
        .as_array()
        .expect("items")
        .iter()
        .map(|item| {
            item["value"]
                .as_str()
                .expect("an integer")
                .parse()
                .expect("a lane")
        })
        .collect()
}

/// What the evaluator makes of every case of a root.
fn evaluate(lowered: &Lowered, cases: &[Case]) -> Vec<Vec<i64>> {
    cases
        .iter()
        .map(|(_, first, second)| {
            let mut arguments = vec![list(first)];
            if !second.is_empty() {
                arguments.push(list(second));
            }
            let outcome = interp::run(&lowered.program, FUEL, lowered.entry(), &arguments);
            let observed = rust::observable(&outcome)
                .unwrap_or_else(|| panic!("{first:?} {second:?}: {outcome:?}"));
            lanes_of(&observed)
        })
        .collect()
}

/// The Rust source of the harness: the cases, the instruction of every
/// modeled target of the architecture it is compiled for, and an entry that
/// prints the lanes of each case (a native program) or exports them (a
/// WebAssembly module).
fn harness(cases: &[Case]) -> String {
    let mut source = String::from(HARNESS_HEAD);
    source.push_str("static CASES: &[(u32, &[i32], &[i32])] = &[\n");
    for (code, first, second) in cases {
        let _ = writeln!(source, "    ({code}, &{first:?}, &{second:?}),");
    }
    source.push_str("];\n");
    source.push_str(HARNESS_TAIL);
    source
}

const HARNESS_HEAD: &str = r#"#![allow(dead_code, unused_unsafe, unexpected_cfgs)]
use core::mem::transmute;

fn i8s<const N: usize>(v: &[i32]) -> [i8; N] {
    let mut o = [0i8; N];
    for (d, s) in o.iter_mut().zip(v) {
        *d = *s as i8;
    }
    o
}
fn u8s<const N: usize>(v: &[i32]) -> [u8; N] {
    let mut o = [0u8; N];
    for (d, s) in o.iter_mut().zip(v) {
        *d = *s as u8;
    }
    o
}
fn i16s<const N: usize>(v: &[i32]) -> [i16; N] {
    let mut o = [0i16; N];
    for (d, s) in o.iter_mut().zip(v) {
        *d = *s as i16;
    }
    o
}
fn widen<T: Copy + Into<i32>>(r: &[T]) -> Vec<i32> {
    r.iter().map(|x| (*x).into()).collect()
}

#[cfg(target_arch = "x86_64")]
mod arch {
    use super::*;
    use core::arch::x86_64::*;

    #[target_feature(enable = "avx2")]
    fn maddubs(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 16] =
            unsafe { transmute(_mm256_maddubs_epi16(transmute(u8s::<32>(a)), transmute(i8s::<32>(b)))) };
        widen(&r)
    }
    #[target_feature(enable = "avx2")]
    fn madd(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i32; 8] =
            unsafe { transmute(_mm256_madd_epi16(transmute(i16s::<16>(a)), transmute(i16s::<16>(b)))) };
        widen(&r)
    }
    #[target_feature(enable = "avx2")]
    fn adds(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 16] =
            unsafe { transmute(_mm256_adds_epi16(transmute(i16s::<16>(a)), transmute(i16s::<16>(b)))) };
        widen(&r)
    }
    pub fn compute(op: u32, a: &[i32], b: &[i32]) -> Vec<i32> {
        assert!(is_x86_feature_detected!("avx2"), "the host has no AVX2");
        unsafe {
            match op {
                0 => maddubs(a, b),
                1 => madd(a, b),
                2 => adds(a, b),
                _ => panic!("no such instruction"),
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
mod arch {
    use super::*;
    use core::arch::aarch64::*;

    #[target_feature(enable = "neon")]
    fn mull(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 8] = unsafe { transmute(vmull_s8(transmute(i8s::<8>(a)), transmute(i8s::<8>(b)))) };
        widen(&r)
    }
    #[target_feature(enable = "neon")]
    fn paddl(a: &[i32], _b: &[i32]) -> Vec<i32> {
        let r: [i32; 4] = unsafe { transmute(vpaddlq_s16(transmute(i16s::<8>(a)))) };
        widen(&r)
    }
    #[target_feature(enable = "neon")]
    fn qadd(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 8] = unsafe { transmute(vqaddq_s16(transmute(i16s::<8>(a)), transmute(i16s::<8>(b)))) };
        widen(&r)
    }
    pub fn compute(op: u32, a: &[i32], b: &[i32]) -> Vec<i32> {
        unsafe {
            match op {
                3 => mull(a, b),
                4 => paddl(a, b),
                5 => qadd(a, b),
                _ => panic!("no such instruction"),
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod arch {
    use super::*;
    use core::arch::wasm32::*;

    #[target_feature(enable = "simd128")]
    fn extmul(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 8] =
            unsafe { transmute(i16x8_extmul_low_i8x16(transmute(i8s::<16>(a)), transmute(i8s::<16>(b)))) };
        widen(&r)
    }
    #[target_feature(enable = "simd128")]
    fn dot(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i32; 4] = unsafe { transmute(i32x4_dot_i16x8(transmute(i16s::<8>(a)), transmute(i16s::<8>(b)))) };
        widen(&r)
    }
    #[target_feature(enable = "simd128")]
    fn add_sat(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 8] = unsafe { transmute(i16x8_add_sat(transmute(i16s::<8>(a)), transmute(i16s::<8>(b)))) };
        widen(&r)
    }
    pub fn compute(op: u32, a: &[i32], b: &[i32]) -> Vec<i32> {
        match op {
            6 => extmul(a, b),
            7 => dot(a, b),
            8 => add_sat(a, b),
            _ => panic!("no such instruction"),
        }
    }
}

"#;

const HARNESS_TAIL: &str = r#"
#[cfg(not(any(target_arch = "wasm32", lib_build)))]
fn main() {
    for (index, (op, a, b)) in CASES.iter().enumerate() {
        let lanes: Vec<String> = arch::compute(*op, a, b).iter().map(|v| v.to_string()).collect();
        println!("{index} {}", lanes.join(","));
    }
}

#[cfg(target_arch = "wasm32")]
mod exports {
    static mut OUT: [i32; 64] = [0; 64];
    #[no_mangle]
    pub extern "C" fn count() -> u32 {
        super::CASES.len() as u32
    }
    #[no_mangle]
    pub extern "C" fn out_ptr() -> *const i32 {
        core::ptr::addr_of!(OUT) as *const i32
    }
    #[no_mangle]
    pub extern "C" fn run(index: u32) -> u32 {
        let (op, a, b) = super::CASES[index as usize];
        let lanes = super::arch::compute(op, a, b);
        unsafe {
            let out = &mut *core::ptr::addr_of_mut!(OUT);
            for (d, s) in out.iter_mut().zip(&lanes) {
                *d = *s;
            }
        }
        lanes.len() as u32
    }
}
"#;

/// The driver `node` runs over the module: the lanes of every case, in the
/// native harness's format.
const NODE_DRIVER: &str = r"
const fs = require('fs');
const wasm = new WebAssembly.Instance(new WebAssembly.Module(fs.readFileSync(process.argv[2])), {}).exports;
const lines = [];
for (let index = 0; index < wasm.count(); index++) {
  const length = wasm.run(index);
  lines.push(index + ' ' + Array.from(new Int32Array(wasm.memory.buffer, wasm.out_ptr(), length)).join(','));
}
console.log(lines.join('\n'));
";

/// How a target's instructions were checked.
enum Ran {
    /// The instruction ran and its lanes are these, one entry per case.
    Executed(Vec<Vec<i32>>),
    /// The harness was built for the target by the pinned rustc and not run.
    BuiltOnly,
    /// The standard library of the target is not installed here, so the
    /// harness was neither built nor run.
    NotBuilt,
}

/// Whether the pinned toolchain has the standard library of `triple`,
/// looked up from the repository root so that `rustup` selects the pinned
/// toolchain.
fn std_installed(triple: &str) -> bool {
    let output = Command::new("rustc")
        .current_dir(support::repo_root().as_std_path())
        .args(["--print", "sysroot"])
        .output()
        .expect("rustc runs");
    let sysroot = String::from_utf8(output.stdout).expect("utf8");
    Path::new(sysroot.trim())
        .join("lib/rustlib")
        .join(triple)
        .is_dir()
}

/// Run `rustc` from the repository root, so that `rustup` selects the
/// toolchain `rust-toolchain.toml` pins.
fn rustc(arguments: &[&str]) {
    let output = Command::new("rustc")
        .current_dir(support::repo_root().as_std_path())
        .args(arguments)
        .output()
        .expect("rustc runs");
    assert!(
        output.status.success(),
        "rustc {arguments:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn parse_lines(stdout: &str, cases: usize) -> Vec<Vec<i32>> {
    let rows: Vec<Vec<i32>> = stdout
        .lines()
        .map(|line| {
            let (_, lanes) = line.split_once(' ').expect("an index and lanes");
            lanes
                .split(',')
                .filter(|lane| !lane.is_empty())
                .map(|lane| lane.parse().expect("a lane"))
                .collect()
        })
        .collect();
    assert_eq!(rows.len(), cases, "one line per case");
    rows
}

/// The instructions of one target, run if the host can run them.
fn run_target(directory: &Path, target: usize, cases: &[Case]) -> Ran {
    let triple = TARGETS[target].triple;
    let source = directory.join(format!("harness_{target}.rs"));
    std::fs::write(&source, harness(cases)).expect("the harness");
    let source = source.to_str().expect("a path");
    let output = directory.join(format!("harness_{target}"));
    let output = output.to_str().expect("a path");
    // A target the host executes natively is compiled for the host (no
    // `--target`, so it needs no extra standard library and runs on every
    // operating system of that architecture); any other target is compiled
    // with `--target`.
    let args = |extra: &[&'static str], cross: bool| -> Vec<String> {
        let mut args: Vec<String> = ["--edition=2021", "-C", "opt-level=1"]
            .into_iter()
            .map(str::to_owned)
            .collect();
        if cross {
            args.extend(["--target".to_owned(), triple.to_owned()]);
        }
        args.extend(extra.iter().map(|argument| (*argument).to_owned()));
        args
    };
    let compile = |mut arguments: Vec<String>, output: &str| {
        arguments.extend(["-o".to_owned(), output.to_owned(), source.to_owned()]);
        let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();
        rustc(&borrowed);
    };
    match target {
        0 if cfg!(target_arch = "x86_64") && host_has_avx2() => {
            compile(args(&[], false), output);
            let ran = Command::new(output).output().expect("the harness runs");
            assert!(
                ran.status.success(),
                "{}",
                String::from_utf8_lossy(&ran.stderr)
            );
            Ran::Executed(parse_lines(
                &String::from_utf8(ran.stdout).expect("utf8"),
                cases.len(),
            ))
        }
        1 if cfg!(target_arch = "aarch64") => {
            compile(args(&[], false), output);
            let ran = Command::new(output).output().expect("the harness runs");
            assert!(
                ran.status.success(),
                "{}",
                String::from_utf8_lossy(&ran.stderr)
            );
            Ran::Executed(parse_lines(
                &String::from_utf8(ran.stdout).expect("utf8"),
                cases.len(),
            ))
        }
        2 if node_available() && std_installed(triple) => {
            let module = directory.join("harness_2.wasm");
            let module = module.to_str().expect("a path");
            compile(args(&["--crate-type=cdylib"], true), module);
            let driver = directory.join("driver.cjs");
            std::fs::write(&driver, NODE_DRIVER).expect("the driver");
            let ran = Command::new("node")
                .arg(&driver)
                .arg(module)
                .output()
                .expect("node runs");
            assert!(
                ran.status.success(),
                "{}",
                String::from_utf8_lossy(&ran.stderr)
            );
            Ran::Executed(parse_lines(
                &String::from_utf8(ran.stdout).expect("utf8"),
                cases.len(),
            ))
        }
        _ => {
            // The host cannot execute this target: build the harness for it,
            // so that the intrinsics at least compile under the attribute.
            // The architecture of the host builds natively; any other target
            // is built only where its standard library is installed.
            let native = (target == 0 && cfg!(target_arch = "x86_64"))
                || (target == 1 && cfg!(target_arch = "aarch64"));
            if !native && !std_installed(triple) {
                return Ran::NotBuilt;
            }
            let library = directory.join(format!("libharness_{target}.rlib"));
            compile(
                args(&["--crate-type=lib", "--cfg", "lib_build"], !native),
                library.to_str().expect("a path"),
            );
            Ran::BuiltOnly
        }
    }
}

fn host_has_avx2() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        std::arch::is_x86_feature_detected!("avx2")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

fn node_available() -> bool {
    Command::new("node")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

/// The roots' lowered programs, after checking the eligibility report of the
/// module declares exactly the nine instruction roots and their effects.
fn lowered_roots(project: &P) -> BTreeMap<&'static str, Lowered> {
    project.check_ok();
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanStd/Target.eligibility.json")
                .as_std_path(),
        )
        .expect("the eligibility report"),
    )
    .expect("the report is JSON");
    let reported = report["roots"].as_array().expect("roots");
    assert_eq!(
        reported
            .iter()
            .map(|row| row["root"].as_str().expect("root").to_owned())
            .collect::<Vec<_>>(),
        INSTRUCTIONS
            .iter()
            .map(|instruction| format!("{MODULE}.{}", instruction.root))
            .collect::<Vec<_>>(),
        "the nine roots are the module's production roots, in order"
    );
    for (row, instruction) in reported.iter().zip(&INSTRUCTIONS) {
        let declared: Vec<&str> = row["declared_effects"]
            .as_array()
            .expect("declared effects")
            .iter()
            .map(|effect| effect.as_str().expect("an effect"))
            .collect();
        assert_eq!(
            declared, EFFECTS,
            "{}: the declared effects",
            instruction.root
        );
        let targets = row["targets"].as_array().expect("targets");
        assert_eq!(targets.len(), 1, "{}: one target", instruction.root);
        assert_eq!(targets[0]["target"], "rust-std");
        assert_eq!(targets[0]["status"], "eligible");
        let realized: Vec<&str> = targets[0]["effects"]
            .as_array()
            .expect("effects")
            .iter()
            .map(|effect| effect["effect"].as_str().expect("an effect"))
            .collect();
        assert_eq!(
            realized, EFFECTS,
            "{}: the declaration admits exactly the realized effects",
            instruction.root
        );
    }
    let checked = support::checked_project(project);
    INSTRUCTIONS
        .iter()
        .map(|instruction| {
            (
                instruction.root,
                lower_in(&checked, project, MODULE, instruction.root),
            )
        })
        .collect()
}

/// Run the case for `GN-09`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub(super) fn run(id: &str) {
    assert_eq!(id, "GN-09");
    // D4: three targets, and the DSP target waits for stable, safe intrinsics.
    assert_eq!(
        TARGETS.map(|target| (target.triple, target.feature)),
        [
            ("x86_64-unknown-linux-gnu", "avx2"),
            ("aarch64-unknown-linux-gnu", "neon"),
            ("wasm32-unknown-unknown", "simd128"),
        ]
    );
    let project = P::stdlib();
    let lowered = lowered_roots(&project);

    // The evaluator on the corpus of every instruction.
    let cases: Vec<Vec<Case>> = (0..INSTRUCTIONS.len()).map(corpus).collect();
    let modeled: Vec<Vec<Vec<i64>>> = INSTRUCTIONS
        .iter()
        .enumerate()
        .map(|(code, instruction)| evaluate(&lowered[instruction.root], &cases[code]))
        .collect();

    // The instruction itself, target by target.
    let directory = tempfile::Builder::new()
        .prefix("lexlean-target-machine-")
        .tempdir()
        .expect("tempdir");
    let mut oracles: BTreeMap<usize, Vec<Vec<i64>>> = BTreeMap::new();
    for (target, description) in TARGETS.iter().enumerate() {
        let codes: Vec<usize> = (0..INSTRUCTIONS.len())
            .filter(|code| INSTRUCTIONS[*code].target == target)
            .collect();
        let target_cases: Vec<Case> = codes
            .iter()
            .flat_map(|code| cases[*code].iter().cloned())
            .collect();
        match run_target(directory.path(), target, &target_cases) {
            Ran::Executed(observed) => {
                let mut position = 0;
                for code in codes {
                    let rows: Vec<Vec<i64>> = observed[position..position + cases[code].len()]
                        .iter()
                        .map(|row| row.iter().map(|lane| i64::from(*lane)).collect())
                        .collect();
                    position += rows.len();
                    let disagreements: Vec<String> = rows
                        .iter()
                        .zip(&modeled[code])
                        .zip(&cases[code])
                        .filter(|((instruction, model), _)| instruction != model)
                        .map(|((instruction, model), (_, first, second))| {
                            format!("{first:?} {second:?}: instruction {instruction:?}, model {model:?}")
                        })
                        .collect();
                    assert!(
                        disagreements.is_empty(),
                        "{} on {} (+{}): the model and the instruction disagree on {} of {} cases: {:?}",
                        INSTRUCTIONS[code].root,
                        description.triple,
                        description.feature,
                        disagreements.len(),
                        rows.len(),
                        &disagreements[..disagreements.len().min(3)]
                    );
                    oracles.insert(code, rows);
                }
                eprintln!(
                    "GN-09: {} (+{}): executed {} cases",
                    description.triple,
                    description.feature,
                    target_cases.len()
                );
            }
            Ran::NotBuilt => {
                assert!(
                    !support::normative_host(),
                    "{}: the normative host builds or runs every target",
                    description.triple
                );
                eprintln!(
                    "GN-09: {} (+{}): not built on this host",
                    description.triple, description.feature
                );
            }
            Ran::BuiltOnly => {
                assert!(
                    !(target == 0 || target == 2) || !support::normative_host(),
                    "{}: the normative host runs this target (AVX2, and Node.js for SIMD128)",
                    description.triple
                );
                eprintln!(
                    "GN-09: {} (+{}): built, not executed on this host",
                    description.triple, description.feature
                );
            }
        }
    }

    // The gate is armed: a saturation bound one past the lane's maximum, and
    // the model disagrees with an instruction that ran.
    let (code, oracle) = oracles
        .iter()
        .find(|(code, _)| {
            INSTRUCTIONS[**code].root.contains("Adds")
                || INSTRUCTIONS[**code].root.contains("AddSat")
        })
        .map(|(code, oracle)| (*code, oracle.clone()))
        .expect("a saturating instruction ran");
    let planted = P::stdlib();
    let text = planted.read_raw("src/Target.lex.tex");
    let site = "(prim less_than (32767 : Int) value : Bool) then (32767 : Int)";
    assert_eq!(text.matches(site).count(), 1, "the plant site exists once");
    planted.write_raw(
        "src/Target.lex.tex",
        &text.replacen(
            site,
            "(prim less_than (32768 : Int) value : Bool) then (32768 : Int)",
            1,
        ),
    );
    let checked = support::checked_project(&planted);
    let root = lower_in(&checked, &planted, MODULE, INSTRUCTIONS[code].root);
    assert_ne!(
        evaluate(&root, &cases[code]),
        oracle,
        "a planted wrong saturation bound is detected by the differential"
    );

    // Under the pinned Lean the module's theorems and the certificates of its
    // roots are kernel-checked, and a false theorem is refused.
    if !support::lean_backed("GN-09") {
        return;
    }
    let falsified = P::stdlib();
    let text = falsified.read_raw("src/Target.lex.tex");
    let start = text
        .find("name: maddubs_saturates_high,")
        .expect("the theorem");
    let site = "(32767 : Int)";
    let offset = start
        + text[start..]
            .find(site)
            .expect("the statement names the saturated lane");
    let mut changed = text.clone();
    changed.replace_range(offset..offset + site.len(), "(32766 : Int)");
    falsified.write_raw("src/Target.lex.tex", &changed);
    {
        let _environment = support::env_shared();
        falsified.verify_fails_with("LLV7002");
    }
    let verified = support::verify_ok(&project);
    let attestation: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .root
                .join(".lexlean/verified")
                .join(verified.attestation_id.to_hex())
                .join("attestation.json")
                .as_std_path(),
        )
        .expect("attestation"),
    )
    .expect("the attestation is JSON");
    let rows = attestation["declarations"]
        .as_array()
        .expect("declarations");
    for theorem in THEOREMS {
        let full = format!("{MODULE}.{theorem}");
        let row = rows
            .iter()
            .find(|row| row["name"].as_str() == Some(full.as_str()))
            .unwrap_or_else(|| panic!("`{full}` is audited"));
        assert!(
            row["observed"].as_array().is_some_and(|observed| observed
                .iter()
                .all(|axiom| matches!(axiom.as_str(), Some("propext" | "Quot.sound")))),
            "`{full}` observes no axiom but propext and Quot.sound: {row}"
        );
    }
}

/// The theorems of the module, which the kernel checks.
const THEOREMS: [&str; 18] = [
    "maddubs_saturates_high",
    "maddubs_saturates_low",
    "maddubs_exact",
    "madd_wraps",
    "madd_exact",
    "adds_saturates",
    "mull_widens",
    "mull_takes_eight",
    "paddl_widens",
    "qadd_saturates",
    "extmul_takes_low_eight",
    "dot_wraps",
    "add_sat_saturates",
    "price_reads_whole_instructions",
    "price_bytes",
    "price_empty",
    "worst_case_is_largest",
    "worst_case_bounded_by_operations",
];
