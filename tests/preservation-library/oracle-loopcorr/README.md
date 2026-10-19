# Loop correspondence oracle

One-time, never a release input (SPEC.md §17.10). The Lean sources of the tail-recursion
elimination proof that `FunLoop` and `funSem_loop` (`SP-20`) rest on, checked over the
committed machine modules and the library (`lean/asm.sh` concatenates them in order and runs
the pinned Lean with `LEAN_PATH` at a built library). The commit after the one that adds this
directory exports them to kernel terms and removes it.
