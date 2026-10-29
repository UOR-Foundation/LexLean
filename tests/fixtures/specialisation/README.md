# specialisation

The fixture of `GL-19` and `VR-24` (SPEC.md §13.12, §18.8).

- `request.toml`: a specialisation request for `Std.Commutative.comm` and
  `Std.Associative.assoc` at `Nat`, `Int`, `UInt32` and `BitVec 32`.
- `project/SpecHost.lean`: the host module the emitter writes for it. Regenerate
  with `cargo xtask emit-wrappers tests/fixtures/specialisation/request.toml
  tests/fixtures/specialisation/project/SpecHost.lean`.
- `project/`: a project whose two entries (`test.spec::nat-comm`,
  `test.spec::int-comm`) denote wrappers. The cases copy it, run `lake build
  SpecHost`, lock, and verify; they carry no lock and no build output.
