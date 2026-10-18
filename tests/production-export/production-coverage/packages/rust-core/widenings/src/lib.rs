#![no_std]
#![forbid(unsafe_code)]
use core::cmp::Ordering;
use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;
pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);
pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }
pub fn work() -> u64 { WORK.load(Memory::Relaxed) }

pub fn nat_add(a: u64, b: u64) -> R<u64> { a.checked_add(b).ok_or(Overflow) }
pub fn nat_sub(a: u64, b: u64) -> u64 { a.saturating_sub(b) }
pub fn nat_mul(a: u64, b: u64) -> R<u64> { a.checked_mul(b).ok_or(Overflow) }
pub fn nat_quot(a: u64, b: u64, z: u64) -> u64 { a.checked_div(b).unwrap_or(z) }
pub fn nat_rem(a: u64, b: u64, z: u64) -> u64 { a.checked_rem(b).unwrap_or(z) }
pub fn nat_eq(a: u64, b: u64) -> bool { a == b }
pub fn nat_le(a: u64, b: u64) -> bool { a <= b }
pub fn nat_lt(a: u64, b: u64) -> bool { a < b }
pub fn nat_succ(a: u64) -> R<u64> { a.checked_add(1).ok_or(Overflow) }
pub fn int_add(a: i64, b: i64) -> R<i64> { a.checked_add(b).ok_or(Overflow) }
pub fn int_sub(a: i64, b: i64) -> R<i64> { a.checked_sub(b).ok_or(Overflow) }
pub fn int_mul(a: i64, b: i64) -> R<i64> { a.checked_mul(b).ok_or(Overflow) }
pub fn int_neg(a: i64) -> R<i64> { a.checked_neg().ok_or(Overflow) }
pub fn int_quot(a: i64, b: i64, z: i64) -> R<i64> { if b == 0 { Ok(z) } else { a.checked_div(b).ok_or(Overflow) } }
pub fn int_rem(a: i64, b: i64, z: i64) -> i64 { if b == 0 { z } else { a.wrapping_rem(b) } }
pub fn bool_not(a: bool) -> bool { !a }
pub fn bool_and(a: bool, b: bool) -> bool { a && b }
pub fn bool_or(a: bool, b: bool) -> bool { a || b }

pub trait Same { fn same(&self, other: &Self) -> bool; }
pub trait Key { fn key(&self, other: &Self) -> Ordering; }
macro_rules! scalar {
    ($($t:ty),*) => { $(
        impl Same for $t { fn same(&self, other: &Self) -> bool { self == other } }
        impl Key for $t { fn key(&self, other: &Self) -> Ordering { self.cmp(other) } }
    )* };
}
scalar!(bool, u8, u16, u32, u64, i8, i16, i32, i64);
impl Same for Ordering { fn same(&self, other: &Self) -> bool { self == other } }
impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering {
        match self.0.key(&other.0) { Ordering::Equal => self.1.key(&other.1), decided => decided }
    }
}
pub fn equal<T: Same>(a: T, b: T) -> bool { a.same(&b) }
pub fn compare<T: Key>(a: T, b: T) -> Ordering { a.key(&b) }

macro_rules! fixed {
    ($m:ident, $t:ident) => {
        pub mod $m {
            pub fn checked_add(a: $t, b: $t) -> Option<$t> { a.checked_add(b) }
            pub fn checked_sub(a: $t, b: $t) -> Option<$t> { a.checked_sub(b) }
            pub fn checked_mul(a: $t, b: $t) -> Option<$t> { a.checked_mul(b) }
            pub fn checked_quot(a: $t, b: $t) -> Option<$t> { a.checked_div(b) }
            pub fn bit_and(a: $t, b: $t) -> $t { a & b }
            pub fn bit_or(a: $t, b: $t) -> $t { a | b }
            pub fn bit_xor(a: $t, b: $t) -> $t { a ^ b }
            pub fn bit_not(a: $t) -> $t { !a }
            pub fn shift_left(a: $t, amount: u32) -> Option<$t> { if amount < $t::BITS { Some(a.wrapping_shl(amount)) } else { None } }
            pub fn shift_right(a: $t, amount: u32) -> Option<$t> { if amount < $t::BITS { Some(a.wrapping_shr(amount)) } else { None } }
            pub fn convert(a: i128) -> Option<$t> { $t::try_from(a).ok() }
        }
    };
}
fixed!(fixed_u8, u8); fixed!(fixed_u16, u16); fixed!(fixed_u32, u32); fixed!(fixed_u64, u64);
fixed!(fixed_i8, i8); fixed!(fixed_i16, i16); fixed!(fixed_i32, i32); fixed!(fixed_i64, i64);
pub fn checked_neg_i8(a: i8) -> Option<i8> { a.checked_neg() }
pub fn checked_neg_i16(a: i16) -> Option<i16> { a.checked_neg() }
pub fn checked_neg_i32(a: i32) -> Option<i32> { a.checked_neg() }
pub fn checked_neg_i64(a: i64) -> Option<i64> { a.checked_neg() }

pub fn f0(v0: u8, v1: i8, v2: u16, v3: i16) -> R<(i64, u64)> {
    Ok(({
        let a329 = {
            let a107 = {
                let v4: u8 = v0;
                let a52 = {
                    let a24 = {
                        let a10 = {
                            let a3 = {
                                let a1 = v4;
                                let a2 = 128u8;
                                fixed_u8::bit_and(a1, a2)
                            };
                            let a4 = 128u8;
                            if equal(a3, a4) {
                                128i64
                            } else {
                                0i64
                            }
                        };
                        let a11 = {
                            let a9 = {
                                let a7 = {
                                    let a5 = v4;
                                    let a6 = 64u8;
                                    fixed_u8::bit_and(a5, a6)
                                };
                                let a8 = 64u8;
                                if equal(a7, a8) {
                                    64i64
                                } else {
                                    0i64
                                }
                            };
                            int_neg(a9)?
                        };
                        int_sub(a10, a11)?
                    };
                    let a25 = {
                        let a23 = {
                            let a21 = {
                                let a14 = {
                                    let a12 = v4;
                                    let a13 = 32u8;
                                    fixed_u8::bit_and(a12, a13)
                                };
                                let a15 = 32u8;
                                if equal(a14, a15) {
                                    32i64
                                } else {
                                    0i64
                                }
                            };
                            let a22 = {
                                let a20 = {
                                    let a18 = {
                                        let a16 = v4;
                                        let a17 = 16u8;
                                        fixed_u8::bit_and(a16, a17)
                                    };
                                    let a19 = 16u8;
                                    if equal(a18, a19) {
                                        16i64
                                    } else {
                                        0i64
                                    }
                                };
                                int_neg(a20)?
                            };
                            int_sub(a21, a22)?
                        };
                        int_neg(a23)?
                    };
                    int_sub(a24, a25)?
                };
                let a53 = {
                    let a51 = {
                        let a49 = {
                            let a35 = {
                                let a28 = {
                                    let a26 = v4;
                                    let a27 = 8u8;
                                    fixed_u8::bit_and(a26, a27)
                                };
                                let a29 = 8u8;
                                if equal(a28, a29) {
                                    8i64
                                } else {
                                    0i64
                                }
                            };
                            let a36 = {
                                let a34 = {
                                    let a32 = {
                                        let a30 = v4;
                                        let a31 = 4u8;
                                        fixed_u8::bit_and(a30, a31)
                                    };
                                    let a33 = 4u8;
                                    if equal(a32, a33) {
                                        4i64
                                    } else {
                                        0i64
                                    }
                                };
                                int_neg(a34)?
                            };
                            int_sub(a35, a36)?
                        };
                        let a50 = {
                            let a48 = {
                                let a46 = {
                                    let a39 = {
                                        let a37 = v4;
                                        let a38 = 2u8;
                                        fixed_u8::bit_and(a37, a38)
                                    };
                                    let a40 = 2u8;
                                    if equal(a39, a40) {
                                        2i64
                                    } else {
                                        0i64
                                    }
                                };
                                let a47 = {
                                    let a45 = {
                                        let a43 = {
                                            let a41 = v4;
                                            let a42 = 1u8;
                                            fixed_u8::bit_and(a41, a42)
                                        };
                                        let a44 = 1u8;
                                        if equal(a43, a44) {
                                            1i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    int_neg(a45)?
                                };
                                int_sub(a46, a47)?
                            };
                            int_neg(a48)?
                        };
                        int_sub(a49, a50)?
                    };
                    int_neg(a51)?
                };
                int_sub(a52, a53)?
            };
            let a108 = {
                let v5: i8 = v1;
                let a105 = {
                    let a77 = {
                        let a63 = {
                            let a56 = {
                                let a54 = v5;
                                let a55 = i8::MIN;
                                fixed_i8::bit_and(a54, a55)
                            };
                            let a57 = i8::MIN;
                            if equal(a56, a57) {
                                -128i64
                            } else {
                                0i64
                            }
                        };
                        let a64 = {
                            let a62 = {
                                let a60 = {
                                    let a58 = v5;
                                    let a59 = 64i8;
                                    fixed_i8::bit_and(a58, a59)
                                };
                                let a61 = 64i8;
                                if equal(a60, a61) {
                                    64i64
                                } else {
                                    0i64
                                }
                            };
                            int_neg(a62)?
                        };
                        int_sub(a63, a64)?
                    };
                    let a78 = {
                        let a76 = {
                            let a74 = {
                                let a67 = {
                                    let a65 = v5;
                                    let a66 = 32i8;
                                    fixed_i8::bit_and(a65, a66)
                                };
                                let a68 = 32i8;
                                if equal(a67, a68) {
                                    32i64
                                } else {
                                    0i64
                                }
                            };
                            let a75 = {
                                let a73 = {
                                    let a71 = {
                                        let a69 = v5;
                                        let a70 = 16i8;
                                        fixed_i8::bit_and(a69, a70)
                                    };
                                    let a72 = 16i8;
                                    if equal(a71, a72) {
                                        16i64
                                    } else {
                                        0i64
                                    }
                                };
                                int_neg(a73)?
                            };
                            int_sub(a74, a75)?
                        };
                        int_neg(a76)?
                    };
                    int_sub(a77, a78)?
                };
                let a106 = {
                    let a104 = {
                        let a102 = {
                            let a88 = {
                                let a81 = {
                                    let a79 = v5;
                                    let a80 = 8i8;
                                    fixed_i8::bit_and(a79, a80)
                                };
                                let a82 = 8i8;
                                if equal(a81, a82) {
                                    8i64
                                } else {
                                    0i64
                                }
                            };
                            let a89 = {
                                let a87 = {
                                    let a85 = {
                                        let a83 = v5;
                                        let a84 = 4i8;
                                        fixed_i8::bit_and(a83, a84)
                                    };
                                    let a86 = 4i8;
                                    if equal(a85, a86) {
                                        4i64
                                    } else {
                                        0i64
                                    }
                                };
                                int_neg(a87)?
                            };
                            int_sub(a88, a89)?
                        };
                        let a103 = {
                            let a101 = {
                                let a99 = {
                                    let a92 = {
                                        let a90 = v5;
                                        let a91 = 2i8;
                                        fixed_i8::bit_and(a90, a91)
                                    };
                                    let a93 = 2i8;
                                    if equal(a92, a93) {
                                        2i64
                                    } else {
                                        0i64
                                    }
                                };
                                let a100 = {
                                    let a98 = {
                                        let a96 = {
                                            let a94 = v5;
                                            let a95 = 1i8;
                                            fixed_i8::bit_and(a94, a95)
                                        };
                                        let a97 = 1i8;
                                        if equal(a96, a97) {
                                            1i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    int_neg(a98)?
                                };
                                int_sub(a99, a100)?
                            };
                            int_neg(a101)?
                        };
                        int_sub(a102, a103)?
                    };
                    int_neg(a104)?
                };
                int_sub(a105, a106)?
            };
            int_sub(a107, a108)?
        };
        let a330 = {
            let a327 = {
                let v6: u16 = v2;
                let a216 = {
                    let a160 = {
                        let a132 = {
                            let a118 = {
                                let a111 = {
                                    let a109 = v6;
                                    let a110 = 32768u16;
                                    fixed_u16::bit_and(a109, a110)
                                };
                                let a112 = 32768u16;
                                if equal(a111, a112) {
                                    32768i64
                                } else {
                                    0i64
                                }
                            };
                            let a119 = {
                                let a117 = {
                                    let a115 = {
                                        let a113 = v6;
                                        let a114 = 16384u16;
                                        fixed_u16::bit_and(a113, a114)
                                    };
                                    let a116 = 16384u16;
                                    if equal(a115, a116) {
                                        16384i64
                                    } else {
                                        0i64
                                    }
                                };
                                int_neg(a117)?
                            };
                            int_sub(a118, a119)?
                        };
                        let a133 = {
                            let a131 = {
                                let a129 = {
                                    let a122 = {
                                        let a120 = v6;
                                        let a121 = 8192u16;
                                        fixed_u16::bit_and(a120, a121)
                                    };
                                    let a123 = 8192u16;
                                    if equal(a122, a123) {
                                        8192i64
                                    } else {
                                        0i64
                                    }
                                };
                                let a130 = {
                                    let a128 = {
                                        let a126 = {
                                            let a124 = v6;
                                            let a125 = 4096u16;
                                            fixed_u16::bit_and(a124, a125)
                                        };
                                        let a127 = 4096u16;
                                        if equal(a126, a127) {
                                            4096i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    int_neg(a128)?
                                };
                                int_sub(a129, a130)?
                            };
                            int_neg(a131)?
                        };
                        int_sub(a132, a133)?
                    };
                    let a161 = {
                        let a159 = {
                            let a157 = {
                                let a143 = {
                                    let a136 = {
                                        let a134 = v6;
                                        let a135 = 2048u16;
                                        fixed_u16::bit_and(a134, a135)
                                    };
                                    let a137 = 2048u16;
                                    if equal(a136, a137) {
                                        2048i64
                                    } else {
                                        0i64
                                    }
                                };
                                let a144 = {
                                    let a142 = {
                                        let a140 = {
                                            let a138 = v6;
                                            let a139 = 1024u16;
                                            fixed_u16::bit_and(a138, a139)
                                        };
                                        let a141 = 1024u16;
                                        if equal(a140, a141) {
                                            1024i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    int_neg(a142)?
                                };
                                int_sub(a143, a144)?
                            };
                            let a158 = {
                                let a156 = {
                                    let a154 = {
                                        let a147 = {
                                            let a145 = v6;
                                            let a146 = 512u16;
                                            fixed_u16::bit_and(a145, a146)
                                        };
                                        let a148 = 512u16;
                                        if equal(a147, a148) {
                                            512i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    let a155 = {
                                        let a153 = {
                                            let a151 = {
                                                let a149 = v6;
                                                let a150 = 256u16;
                                                fixed_u16::bit_and(a149, a150)
                                            };
                                            let a152 = 256u16;
                                            if equal(a151, a152) {
                                                256i64
                                            } else {
                                                0i64
                                            }
                                        };
                                        int_neg(a153)?
                                    };
                                    int_sub(a154, a155)?
                                };
                                int_neg(a156)?
                            };
                            int_sub(a157, a158)?
                        };
                        int_neg(a159)?
                    };
                    int_sub(a160, a161)?
                };
                let a217 = {
                    let a215 = {
                        let a213 = {
                            let a185 = {
                                let a171 = {
                                    let a164 = {
                                        let a162 = v6;
                                        let a163 = 128u16;
                                        fixed_u16::bit_and(a162, a163)
                                    };
                                    let a165 = 128u16;
                                    if equal(a164, a165) {
                                        128i64
                                    } else {
                                        0i64
                                    }
                                };
                                let a172 = {
                                    let a170 = {
                                        let a168 = {
                                            let a166 = v6;
                                            let a167 = 64u16;
                                            fixed_u16::bit_and(a166, a167)
                                        };
                                        let a169 = 64u16;
                                        if equal(a168, a169) {
                                            64i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    int_neg(a170)?
                                };
                                int_sub(a171, a172)?
                            };
                            let a186 = {
                                let a184 = {
                                    let a182 = {
                                        let a175 = {
                                            let a173 = v6;
                                            let a174 = 32u16;
                                            fixed_u16::bit_and(a173, a174)
                                        };
                                        let a176 = 32u16;
                                        if equal(a175, a176) {
                                            32i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    let a183 = {
                                        let a181 = {
                                            let a179 = {
                                                let a177 = v6;
                                                let a178 = 16u16;
                                                fixed_u16::bit_and(a177, a178)
                                            };
                                            let a180 = 16u16;
                                            if equal(a179, a180) {
                                                16i64
                                            } else {
                                                0i64
                                            }
                                        };
                                        int_neg(a181)?
                                    };
                                    int_sub(a182, a183)?
                                };
                                int_neg(a184)?
                            };
                            int_sub(a185, a186)?
                        };
                        let a214 = {
                            let a212 = {
                                let a210 = {
                                    let a196 = {
                                        let a189 = {
                                            let a187 = v6;
                                            let a188 = 8u16;
                                            fixed_u16::bit_and(a187, a188)
                                        };
                                        let a190 = 8u16;
                                        if equal(a189, a190) {
                                            8i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    let a197 = {
                                        let a195 = {
                                            let a193 = {
                                                let a191 = v6;
                                                let a192 = 4u16;
                                                fixed_u16::bit_and(a191, a192)
                                            };
                                            let a194 = 4u16;
                                            if equal(a193, a194) {
                                                4i64
                                            } else {
                                                0i64
                                            }
                                        };
                                        int_neg(a195)?
                                    };
                                    int_sub(a196, a197)?
                                };
                                let a211 = {
                                    let a209 = {
                                        let a207 = {
                                            let a200 = {
                                                let a198 = v6;
                                                let a199 = 2u16;
                                                fixed_u16::bit_and(a198, a199)
                                            };
                                            let a201 = 2u16;
                                            if equal(a200, a201) {
                                                2i64
                                            } else {
                                                0i64
                                            }
                                        };
                                        let a208 = {
                                            let a206 = {
                                                let a204 = {
                                                    let a202 = v6;
                                                    let a203 = 1u16;
                                                    fixed_u16::bit_and(a202, a203)
                                                };
                                                let a205 = 1u16;
                                                if equal(a204, a205) {
                                                    1i64
                                                } else {
                                                    0i64
                                                }
                                            };
                                            int_neg(a206)?
                                        };
                                        int_sub(a207, a208)?
                                    };
                                    int_neg(a209)?
                                };
                                int_sub(a210, a211)?
                            };
                            int_neg(a212)?
                        };
                        int_sub(a213, a214)?
                    };
                    int_neg(a215)?
                };
                int_sub(a216, a217)?
            };
            let a328 = {
                let v7: i16 = v3;
                let a325 = {
                    let a269 = {
                        let a241 = {
                            let a227 = {
                                let a220 = {
                                    let a218 = v7;
                                    let a219 = i16::MIN;
                                    fixed_i16::bit_and(a218, a219)
                                };
                                let a221 = i16::MIN;
                                if equal(a220, a221) {
                                    -32768i64
                                } else {
                                    0i64
                                }
                            };
                            let a228 = {
                                let a226 = {
                                    let a224 = {
                                        let a222 = v7;
                                        let a223 = 16384i16;
                                        fixed_i16::bit_and(a222, a223)
                                    };
                                    let a225 = 16384i16;
                                    if equal(a224, a225) {
                                        16384i64
                                    } else {
                                        0i64
                                    }
                                };
                                int_neg(a226)?
                            };
                            int_sub(a227, a228)?
                        };
                        let a242 = {
                            let a240 = {
                                let a238 = {
                                    let a231 = {
                                        let a229 = v7;
                                        let a230 = 8192i16;
                                        fixed_i16::bit_and(a229, a230)
                                    };
                                    let a232 = 8192i16;
                                    if equal(a231, a232) {
                                        8192i64
                                    } else {
                                        0i64
                                    }
                                };
                                let a239 = {
                                    let a237 = {
                                        let a235 = {
                                            let a233 = v7;
                                            let a234 = 4096i16;
                                            fixed_i16::bit_and(a233, a234)
                                        };
                                        let a236 = 4096i16;
                                        if equal(a235, a236) {
                                            4096i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    int_neg(a237)?
                                };
                                int_sub(a238, a239)?
                            };
                            int_neg(a240)?
                        };
                        int_sub(a241, a242)?
                    };
                    let a270 = {
                        let a268 = {
                            let a266 = {
                                let a252 = {
                                    let a245 = {
                                        let a243 = v7;
                                        let a244 = 2048i16;
                                        fixed_i16::bit_and(a243, a244)
                                    };
                                    let a246 = 2048i16;
                                    if equal(a245, a246) {
                                        2048i64
                                    } else {
                                        0i64
                                    }
                                };
                                let a253 = {
                                    let a251 = {
                                        let a249 = {
                                            let a247 = v7;
                                            let a248 = 1024i16;
                                            fixed_i16::bit_and(a247, a248)
                                        };
                                        let a250 = 1024i16;
                                        if equal(a249, a250) {
                                            1024i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    int_neg(a251)?
                                };
                                int_sub(a252, a253)?
                            };
                            let a267 = {
                                let a265 = {
                                    let a263 = {
                                        let a256 = {
                                            let a254 = v7;
                                            let a255 = 512i16;
                                            fixed_i16::bit_and(a254, a255)
                                        };
                                        let a257 = 512i16;
                                        if equal(a256, a257) {
                                            512i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    let a264 = {
                                        let a262 = {
                                            let a260 = {
                                                let a258 = v7;
                                                let a259 = 256i16;
                                                fixed_i16::bit_and(a258, a259)
                                            };
                                            let a261 = 256i16;
                                            if equal(a260, a261) {
                                                256i64
                                            } else {
                                                0i64
                                            }
                                        };
                                        int_neg(a262)?
                                    };
                                    int_sub(a263, a264)?
                                };
                                int_neg(a265)?
                            };
                            int_sub(a266, a267)?
                        };
                        int_neg(a268)?
                    };
                    int_sub(a269, a270)?
                };
                let a326 = {
                    let a324 = {
                        let a322 = {
                            let a294 = {
                                let a280 = {
                                    let a273 = {
                                        let a271 = v7;
                                        let a272 = 128i16;
                                        fixed_i16::bit_and(a271, a272)
                                    };
                                    let a274 = 128i16;
                                    if equal(a273, a274) {
                                        128i64
                                    } else {
                                        0i64
                                    }
                                };
                                let a281 = {
                                    let a279 = {
                                        let a277 = {
                                            let a275 = v7;
                                            let a276 = 64i16;
                                            fixed_i16::bit_and(a275, a276)
                                        };
                                        let a278 = 64i16;
                                        if equal(a277, a278) {
                                            64i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    int_neg(a279)?
                                };
                                int_sub(a280, a281)?
                            };
                            let a295 = {
                                let a293 = {
                                    let a291 = {
                                        let a284 = {
                                            let a282 = v7;
                                            let a283 = 32i16;
                                            fixed_i16::bit_and(a282, a283)
                                        };
                                        let a285 = 32i16;
                                        if equal(a284, a285) {
                                            32i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    let a292 = {
                                        let a290 = {
                                            let a288 = {
                                                let a286 = v7;
                                                let a287 = 16i16;
                                                fixed_i16::bit_and(a286, a287)
                                            };
                                            let a289 = 16i16;
                                            if equal(a288, a289) {
                                                16i64
                                            } else {
                                                0i64
                                            }
                                        };
                                        int_neg(a290)?
                                    };
                                    int_sub(a291, a292)?
                                };
                                int_neg(a293)?
                            };
                            int_sub(a294, a295)?
                        };
                        let a323 = {
                            let a321 = {
                                let a319 = {
                                    let a305 = {
                                        let a298 = {
                                            let a296 = v7;
                                            let a297 = 8i16;
                                            fixed_i16::bit_and(a296, a297)
                                        };
                                        let a299 = 8i16;
                                        if equal(a298, a299) {
                                            8i64
                                        } else {
                                            0i64
                                        }
                                    };
                                    let a306 = {
                                        let a304 = {
                                            let a302 = {
                                                let a300 = v7;
                                                let a301 = 4i16;
                                                fixed_i16::bit_and(a300, a301)
                                            };
                                            let a303 = 4i16;
                                            if equal(a302, a303) {
                                                4i64
                                            } else {
                                                0i64
                                            }
                                        };
                                        int_neg(a304)?
                                    };
                                    int_sub(a305, a306)?
                                };
                                let a320 = {
                                    let a318 = {
                                        let a316 = {
                                            let a309 = {
                                                let a307 = v7;
                                                let a308 = 2i16;
                                                fixed_i16::bit_and(a307, a308)
                                            };
                                            let a310 = 2i16;
                                            if equal(a309, a310) {
                                                2i64
                                            } else {
                                                0i64
                                            }
                                        };
                                        let a317 = {
                                            let a315 = {
                                                let a313 = {
                                                    let a311 = v7;
                                                    let a312 = 1i16;
                                                    fixed_i16::bit_and(a311, a312)
                                                };
                                                let a314 = 1i16;
                                                if equal(a313, a314) {
                                                    1i64
                                                } else {
                                                    0i64
                                                }
                                            };
                                            int_neg(a315)?
                                        };
                                        int_sub(a316, a317)?
                                    };
                                    int_neg(a318)?
                                };
                                int_sub(a319, a320)?
                            };
                            int_neg(a321)?
                        };
                        int_sub(a322, a323)?
                    };
                    int_neg(a324)?
                };
                int_sub(a325, a326)?
            };
            int_sub(a327, a328)?
        };
        int_sub(a329, a330)?
    }, {
        let a471 = {
            let v8: u8 = v0;
            let a375 = {
                let a351 = {
                    let a339 = {
                        let a333 = {
                            let a331 = v8;
                            let a332 = 128u8;
                            fixed_u8::bit_and(a331, a332)
                        };
                        let a334 = 128u8;
                        if equal(a333, a334) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a340 = {
                        let a337 = {
                            let a335 = v8;
                            let a336 = 64u8;
                            fixed_u8::bit_and(a335, a336)
                        };
                        let a338 = 64u8;
                        if equal(a337, a338) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a339, a340)?
                };
                let a352 = {
                    let a349 = {
                        let a343 = {
                            let a341 = v8;
                            let a342 = 32u8;
                            fixed_u8::bit_and(a341, a342)
                        };
                        let a344 = 32u8;
                        if equal(a343, a344) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a350 = {
                        let a347 = {
                            let a345 = v8;
                            let a346 = 16u8;
                            fixed_u8::bit_and(a345, a346)
                        };
                        let a348 = 16u8;
                        if equal(a347, a348) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a349, a350)?
                };
                nat_add(a351, a352)?
            };
            let a376 = {
                let a373 = {
                    let a361 = {
                        let a355 = {
                            let a353 = v8;
                            let a354 = 8u8;
                            fixed_u8::bit_and(a353, a354)
                        };
                        let a356 = 8u8;
                        if equal(a355, a356) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a362 = {
                        let a359 = {
                            let a357 = v8;
                            let a358 = 4u8;
                            fixed_u8::bit_and(a357, a358)
                        };
                        let a360 = 4u8;
                        if equal(a359, a360) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a361, a362)?
                };
                let a374 = {
                    let a371 = {
                        let a365 = {
                            let a363 = v8;
                            let a364 = 2u8;
                            fixed_u8::bit_and(a363, a364)
                        };
                        let a366 = 2u8;
                        if equal(a365, a366) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a372 = {
                        let a369 = {
                            let a367 = v8;
                            let a368 = 1u8;
                            fixed_u8::bit_and(a367, a368)
                        };
                        let a370 = 1u8;
                        if equal(a369, a370) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a371, a372)?
                };
                nat_add(a373, a374)?
            };
            nat_add(a375, a376)?
        };
        let a472 = {
            let v9: u16 = v2;
            let a469 = {
                let a421 = {
                    let a397 = {
                        let a385 = {
                            let a379 = {
                                let a377 = v9;
                                let a378 = 32768u16;
                                fixed_u16::bit_and(a377, a378)
                            };
                            let a380 = 32768u16;
                            if equal(a379, a380) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a386 = {
                            let a383 = {
                                let a381 = v9;
                                let a382 = 16384u16;
                                fixed_u16::bit_and(a381, a382)
                            };
                            let a384 = 16384u16;
                            if equal(a383, a384) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a385, a386)?
                    };
                    let a398 = {
                        let a395 = {
                            let a389 = {
                                let a387 = v9;
                                let a388 = 8192u16;
                                fixed_u16::bit_and(a387, a388)
                            };
                            let a390 = 8192u16;
                            if equal(a389, a390) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a396 = {
                            let a393 = {
                                let a391 = v9;
                                let a392 = 4096u16;
                                fixed_u16::bit_and(a391, a392)
                            };
                            let a394 = 4096u16;
                            if equal(a393, a394) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a395, a396)?
                    };
                    nat_add(a397, a398)?
                };
                let a422 = {
                    let a419 = {
                        let a407 = {
                            let a401 = {
                                let a399 = v9;
                                let a400 = 2048u16;
                                fixed_u16::bit_and(a399, a400)
                            };
                            let a402 = 2048u16;
                            if equal(a401, a402) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a408 = {
                            let a405 = {
                                let a403 = v9;
                                let a404 = 1024u16;
                                fixed_u16::bit_and(a403, a404)
                            };
                            let a406 = 1024u16;
                            if equal(a405, a406) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a407, a408)?
                    };
                    let a420 = {
                        let a417 = {
                            let a411 = {
                                let a409 = v9;
                                let a410 = 512u16;
                                fixed_u16::bit_and(a409, a410)
                            };
                            let a412 = 512u16;
                            if equal(a411, a412) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a418 = {
                            let a415 = {
                                let a413 = v9;
                                let a414 = 256u16;
                                fixed_u16::bit_and(a413, a414)
                            };
                            let a416 = 256u16;
                            if equal(a415, a416) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a417, a418)?
                    };
                    nat_add(a419, a420)?
                };
                nat_add(a421, a422)?
            };
            let a470 = {
                let a467 = {
                    let a443 = {
                        let a431 = {
                            let a425 = {
                                let a423 = v9;
                                let a424 = 128u16;
                                fixed_u16::bit_and(a423, a424)
                            };
                            let a426 = 128u16;
                            if equal(a425, a426) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a432 = {
                            let a429 = {
                                let a427 = v9;
                                let a428 = 64u16;
                                fixed_u16::bit_and(a427, a428)
                            };
                            let a430 = 64u16;
                            if equal(a429, a430) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a431, a432)?
                    };
                    let a444 = {
                        let a441 = {
                            let a435 = {
                                let a433 = v9;
                                let a434 = 32u16;
                                fixed_u16::bit_and(a433, a434)
                            };
                            let a436 = 32u16;
                            if equal(a435, a436) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a442 = {
                            let a439 = {
                                let a437 = v9;
                                let a438 = 16u16;
                                fixed_u16::bit_and(a437, a438)
                            };
                            let a440 = 16u16;
                            if equal(a439, a440) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a441, a442)?
                    };
                    nat_add(a443, a444)?
                };
                let a468 = {
                    let a465 = {
                        let a453 = {
                            let a447 = {
                                let a445 = v9;
                                let a446 = 8u16;
                                fixed_u16::bit_and(a445, a446)
                            };
                            let a448 = 8u16;
                            if equal(a447, a448) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a454 = {
                            let a451 = {
                                let a449 = v9;
                                let a450 = 4u16;
                                fixed_u16::bit_and(a449, a450)
                            };
                            let a452 = 4u16;
                            if equal(a451, a452) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a453, a454)?
                    };
                    let a466 = {
                        let a463 = {
                            let a457 = {
                                let a455 = v9;
                                let a456 = 2u16;
                                fixed_u16::bit_and(a455, a456)
                            };
                            let a458 = 2u16;
                            if equal(a457, a458) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a464 = {
                            let a461 = {
                                let a459 = v9;
                                let a460 = 1u16;
                                fixed_u16::bit_and(a459, a460)
                            };
                            let a462 = 1u16;
                            if equal(a461, a462) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a463, a464)?
                    };
                    nat_add(a465, a466)?
                };
                nat_add(a467, a468)?
            };
            nat_add(a469, a470)?
        };
        nat_add(a471, a472)?
    }))
}

pub fn run(p0: u8, p1: i8, p2: u16, p3: i16) -> R<(i64, u64)> {
    f0(p0, p1, p2, p3)
}
