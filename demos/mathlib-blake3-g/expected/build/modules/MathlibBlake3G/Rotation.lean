module
public import Blake3GHost
public import Init.Data.BitVec.Basic
public import Init.Prelude
public import MathlibBlake3G.Cancellation
set_option autoImplicit false
namespace MathlibBlake3G.Rotation

public theorem rotate_left_right (llv0 : BitVec 32) : And (And (And (Eq (BitVec.rotateLeft (BitVec.rotateRight llv0 7) 7) llv0) (Eq (BitVec.rotateLeft (BitVec.rotateRight llv0 8) 8) llv0)) (Eq (BitVec.rotateLeft (BitVec.rotateRight llv0 12) 12) llv0)) (Eq (BitVec.rotateLeft (BitVec.rotateRight llv0 16) 16) llv0) := by
  constructor
  constructor
  constructor
  exact Blake3GHost.rotateLeft_rotateRight llv0 7
  exact Blake3GHost.rotateLeft_rotateRight llv0 8
  exact Blake3GHost.rotateLeft_rotateRight llv0 12
  exact Blake3GHost.rotateLeft_rotateRight llv0 16

public theorem unmix (llv0 : BitVec 32) (llv1 : BitVec 32) (llv2 : Nat) : Eq (HXor.hXor (BitVec.rotateLeft (BitVec.rotateRight (HXor.hXor llv0 llv1) llv2) llv2) llv1) llv0 := by
  rw [Blake3GHost.rotateLeft_rotateRight (HXor.hXor llv0 llv1) llv2]
  exact MathlibBlake3G.Cancellation.xor_cancel llv0 llv1

end MathlibBlake3G.Rotation
