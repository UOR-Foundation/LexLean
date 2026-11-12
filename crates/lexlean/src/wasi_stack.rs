/// The stack the `lexlean` executable is linked with on WASI (SPEC.md §8.3).
/// WASI has no threads, so the check pipeline runs on this one stack, and
/// `Limits::compile_stack_plan` hosts its depth on it as it does on a compile
/// thread elsewhere: 16 MiB for the pipeline and 64 KiB for each of 1,024
/// levels. `build.rs` links the executable with it, and the plan reads it, from
/// this one file.
pub const WASI_STACK_BYTES: usize = 80 * 1024 * 1024;
