module
public import Blake3GSpecs
public import Init.Prelude
set_option autoImplicit false
namespace MathlibBlake3G.Cancellation

public theorem add_sub_cancel (llv0 : BitVec 32) (llv1 : BitVec 32) : Eq (HSub.hSub (HAdd.hAdd llv0 llv1) llv1) llv0 := by
  exact LexLeanSpec.BitVec32.BitVec.add_sub_cancel llv0 llv1

public theorem add_sub_cancel_group (llv0 : BitVec 32) (llv1 : BitVec 32) : Eq (HSub.hSub (HAdd.hAdd llv0 llv1) llv1) llv0 := by
  exact LexLeanSpec.BitVec32.add_sub_cancel_right llv0 llv1

public theorem add_sub_cancel_u32 (llv0 : UInt32) (llv1 : UInt32) : Eq (HSub.hSub (HAdd.hAdd llv0 llv1) llv1) llv0 := by
  exact LexLeanSpec.U32.UInt32.add_sub_cancel llv0 llv1

public theorem add_sub_sub_cancel (llv0 : BitVec 32) (llv1 : BitVec 32) (llv2 : BitVec 32) : Eq (HSub.hSub (HSub.hSub (HAdd.hAdd (HAdd.hAdd llv0 llv1) llv2) llv1) llv2) llv0 := by
  rw [LexLeanSpec.BitVec32.add_right_comm llv0 llv1 llv2, LexLeanSpec.BitVec32.add_sub_cancel_right (HAdd.hAdd llv0 llv2) llv1, LexLeanSpec.BitVec32.add_sub_cancel_right llv0 llv2]

public theorem xor_cancel (llv0 : BitVec 32) (llv1 : BitVec 32) : Eq (HXor.hXor (HXor.hXor llv0 llv1) llv1) llv0 := by
  rw [LexLeanSpec.BitVec32.BitVec.xor_assoc llv0 llv1 llv1, LexLeanSpec.BitVec32.BitVec.xor_self, LexLeanSpec.BitVec32.BitVec.xor_zero]

public theorem xor_cancel_u32 (llv0 : UInt32) (llv1 : UInt32) : Eq (HXor.hXor (HXor.hXor llv0 llv1) llv1) llv0 := by
  rw [LexLeanSpec.U32.UInt32.xor_assoc llv0 llv1 llv1, LexLeanSpec.U32.UInt32.xor_self, LexLeanSpec.U32.UInt32.xor_zero]

end MathlibBlake3G.Cancellation
