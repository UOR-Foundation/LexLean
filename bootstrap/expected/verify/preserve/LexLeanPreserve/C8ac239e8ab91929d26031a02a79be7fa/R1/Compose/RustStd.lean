import LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1
import LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd
import LexLeanPreservation.Compose
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
set_option linter.unusedVariables false
namespace LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.Compose.RustStd

theorem __wt_0 : ∀ (__s : (LexLeanBoot.AxiomOutput.Rejection)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd.program _root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd.krate _root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd.flags (_root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.__enc_0 __s) (.adt 0) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_bytes (__s).message) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encOption _root_.LexLeanPreservation.Rust.wt_bytes) (__s).declaration) _root_.LexLeanPreservation.Rust.wtl_nil))

theorem __wt_2 : ∀ (__s : (LexLeanBoot.AxiomOutput.Record)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd.program _root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd.krate _root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd.flags (_root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.__enc_2 __s) (.adt 2) :=
  fun __s => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_bytes (__s).declaration) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_bytes) (__s).axioms) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encOption _root_.LexLeanPreservation.Rust.wt_bytes) (__s).expected) _root_.LexLeanPreservation.Rust.wtl_nil)))

theorem __wt_1 : ∀ (__v : (LexLeanBoot.AxiomOutput.Scan)), _root_.LexLeanPreservation.Rust.WT _root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd.program _root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd.krate _root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd.flags (_root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.__enc_1 __v) (.adt 1)
  | LexLeanBoot.AxiomOutput.Scan.between __x0 __x1 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList __wt_2) __x0) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_bytes) __x1) _root_.LexLeanPreservation.Rust.wtl_nil))
  | LexLeanBoot.AxiomOutput.Scan.inside __x0 __x1 __x2 __x3 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList __wt_2) __x0) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_bytes) __x1) (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_bytes __x2) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encOption _root_.LexLeanPreservation.Rust.wt_bytes) __x3) _root_.LexLeanPreservation.Rust.wtl_nil))))
  | LexLeanBoot.AxiomOutput.Scan.stopped __x0 => _root_.LexLeanPreservation.Rust.wt_adt rfl (_root_.LexLeanPreservation.Rust.wtl_cons (__wt_0 __x0) _root_.LexLeanPreservation.Rust.wtl_nil)

/-- The rendering of the root, invoked on the encoded arguments, realizes
the encoded source result. -/
theorem root (stdout : ByteArray) (expected : (List ByteArray)) (__e_hrep : _root_.LexLeanPreservation.Rust.RepresentableL [(_root_.LexLeanTarget.TargetSyntax.Value.bytes stdout), ((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.bytes) expected)]) : ∃ __e_ro, _root_.LexLeanPreservation.Rust.RealizesFn Bool.true (_root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.denote stdout expected) __e_ro ∧ _root_.LexLeanPreservation.Rust.RCI _root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd.krate (_root_.LexLeanPreservation.Rust.fnIdent 0) [(_root_.LexLeanTarget.TargetSyntax.Value.bytes stdout), ((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.bytes) expected)] __e_ro :=
  _root_.LexLeanPreservation.Rust.compose (fun __e_n => _root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.RustStd.root __e_n 0) rfl (_root_.LexLeanPreservation.Rust.wtl_cons (_root_.LexLeanPreservation.Rust.wt_bytes stdout) (_root_.LexLeanPreservation.Rust.wtl_cons ((_root_.LexLeanPreservation.Rust.wt_encList _root_.LexLeanPreservation.Rust.wt_bytes) expected) _root_.LexLeanPreservation.Rust.wtl_nil)) rfl rfl (_root_.LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.root stdout expected) (_root_.LexLeanPreservation.Rust.rel_ne_stuck _ _)

end LexLeanPreserve.C8ac239e8ab91929d26031a02a79be7fa.R1.Compose.RustStd
