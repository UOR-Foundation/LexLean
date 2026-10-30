module
public import Init.Data.BitVec.Basic
public import Init.Prelude
public import MathlibBlake3G.Cancellation
public import MathlibBlake3G.Rotation
set_option autoImplicit false
namespace MathlibBlake3G.Mixing

public theorem mixing_inverse (llv0 : BitVec 32) (llv1 : BitVec 32) (llv2 : BitVec 32) (llv3 : BitVec 32) (llv4 : BitVec 32) (llv5 : BitVec 32) (llv6 : BitVec 32) (llv7 : BitVec 32) (llv8 : BitVec 32) (llv9 : BitVec 32) (llv10 : BitVec 32) (llv11 : BitVec 32) (llv12 : BitVec 32) (llv13 : BitVec 32) (llv14 : BitVec 32) (llv15 : BitVec 32) (llv16 : BitVec 32) (llv17 : BitVec 32) (llv18 : BitVec 32) (llv19 : BitVec 32) (llv20 : BitVec 32) (llv21 : BitVec 32) : (Eq llv6 (HAdd.hAdd (HAdd.hAdd llv0 llv1) llv4)) → (Eq llv9 (BitVec.rotateRight (HXor.hXor llv3 llv6) 16)) → (Eq llv8 (HAdd.hAdd llv2 llv9)) → (Eq llv7 (BitVec.rotateRight (HXor.hXor llv1 llv8) 12)) → (Eq llv10 (HAdd.hAdd (HAdd.hAdd llv6 llv7) llv5)) → (Eq llv13 (BitVec.rotateRight (HXor.hXor llv9 llv10) 8)) → (Eq llv12 (HAdd.hAdd llv8 llv13)) → (Eq llv11 (BitVec.rotateRight (HXor.hXor llv7 llv12) 7)) → (Eq llv14 (HXor.hXor (BitVec.rotateLeft llv11 7) llv12)) → (Eq llv15 (HSub.hSub llv12 llv13)) → (Eq llv16 (HXor.hXor (BitVec.rotateLeft llv13 8) llv10)) → (Eq llv17 (HSub.hSub (HSub.hSub llv10 llv14) llv5)) → (Eq llv19 (HXor.hXor (BitVec.rotateLeft llv14 12) llv15)) → (Eq llv20 (HSub.hSub llv15 llv16)) → (Eq llv21 (HXor.hXor (BitVec.rotateLeft llv16 16) llv17)) → (Eq llv18 (HSub.hSub (HSub.hSub llv17 llv19) llv4)) → And (And (And (Eq llv18 llv0) (Eq llv19 llv1)) (Eq llv20 llv2)) (Eq llv21 llv3) := by
  intro llh0 llh1 llh2 llh3 llh4 llh5 llh6 llh7 llh8 llh9 llh10 llh11 llh12 llh13 llh14 llh15
  have llh16 : Eq llv14 llv7 := by
    rw [llh8, llh7]
    exact MathlibBlake3G.Rotation.unmix llv7 llv12 (7 : Nat)
  have llh17 : Eq llv15 llv8 := by
    rw [llh9, llh6]
    exact MathlibBlake3G.Cancellation.add_sub_cancel llv8 llv13
  have llh18 : Eq llv16 llv9 := by
    rw [llh10, llh5]
    exact MathlibBlake3G.Rotation.unmix llv9 llv10 (8 : Nat)
  have llh19 : Eq llv17 llv6 := by
    rw [llh11, llh16, llh4]
    exact MathlibBlake3G.Cancellation.add_sub_sub_cancel llv6 llv7 llv5
  have llh20 : Eq llv19 llv1 := by
    rw [llh12, llh16, llh17, llh3]
    exact MathlibBlake3G.Rotation.unmix llv1 llv8 (12 : Nat)
  have llh21 : Eq llv20 llv2 := by
    rw [llh13, llh17, llh18, llh2]
    exact MathlibBlake3G.Cancellation.add_sub_cancel llv2 llv9
  have llh22 : Eq llv21 llv3 := by
    rw [llh14, llh18, llh19, llh1]
    exact MathlibBlake3G.Rotation.unmix llv3 llv6 (16 : Nat)
  have llh23 : Eq llv18 llv0 := by
    rw [llh15, llh19, llh20, llh0]
    exact MathlibBlake3G.Cancellation.add_sub_sub_cancel llv0 llv1 llv4
  constructor
  constructor
  constructor
  exact llh23
  exact llh20
  exact llh21
  exact llh22

end MathlibBlake3G.Mixing
