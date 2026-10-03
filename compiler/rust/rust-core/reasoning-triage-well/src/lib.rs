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

#[derive(Clone)]
pub enum Adt0 {
    C0(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Adt0, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

#[derive(Clone)]
pub enum Fn0 {
    F33,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn0::F33 => f33(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Result<u64, (bool, bool)>> {
    let v1: (Adt1, bool) = f38(Fn0::F33, 11u64, f30(v0.clone()))?;
    let (_, h1) = v1.clone();
    if h1 {
        Ok(f36(v0.clone(), {
            let (h2, _) = v1.clone();
            h2
        }))
    } else {
        Ok(Err::<u64, (bool, bool)>((false, false)))
    }
}

pub fn f1(v0: Adt0) -> bool {
    let a4 = 380u64;
    let a5 = match v0.clone() {
        Adt0::C0(h3, _, _, _, _, _) => {
            h3
        }
    };
    nat_lt(a4, a5)
}

pub fn f2(v0: Adt0) -> bool {
    let a7 = 90u64;
    let a8 = match v0.clone() {
        Adt0::C0(_, h6, _, _, _, _) => {
            h6
        }
    };
    nat_lt(a7, a8)
}

pub fn f3(v0: Adt0) -> bool {
    let a10 = 20u64;
    let a11 = match v0.clone() {
        Adt0::C0(_, _, h9, _, _, _) => {
            h9
        }
    };
    nat_lt(a10, a11)
}

pub fn f4(v0: Adt0) -> bool {
    let a13 = 12u64;
    let a14 = match v0.clone() {
        Adt0::C0(_, _, _, h12, _, _) => {
            h12
        }
    };
    nat_lt(a13, a14)
}

pub fn f5(v0: Adt0) -> bool {
    let a16 = match v0.clone() {
        Adt0::C0(_, _, _, _, h15, _) => {
            h15
        }
    };
    let a17 = 90u64;
    nat_lt(a16, a17)
}

pub fn f6(v0: Adt1) -> bool {
    let a22 = {
        let a19 = match v0.clone() {
            Adt1::C0(_, h18, _, _, _, _, _, _, _) => {
                h18
            }
        };
        let a20 = 0u64;
        nat_eq(a19, a20)
    };
    let a23 = f1(match v0.clone() {
        Adt1::C0(h21, _, _, _, _, _, _, _, _) => {
            h21
        }
    });
    bool_and(a22, a23)
}

pub fn f7(v0: Adt1) -> bool {
    let a28 = {
        let a25 = match v0.clone() {
            Adt1::C0(_, _, h24, _, _, _, _, _, _) => {
                h24
            }
        };
        let a26 = 0u64;
        nat_eq(a25, a26)
    };
    let a29 = f2(match v0.clone() {
        Adt1::C0(h27, _, _, _, _, _, _, _, _) => {
            h27
        }
    });
    bool_and(a28, a29)
}

pub fn f8(v0: Adt1) -> bool {
    let a34 = {
        let a31 = match v0.clone() {
            Adt1::C0(_, _, _, h30, _, _, _, _, _) => {
                h30
            }
        };
        let a32 = 0u64;
        nat_eq(a31, a32)
    };
    let a35 = f3(match v0.clone() {
        Adt1::C0(h33, _, _, _, _, _, _, _, _) => {
            h33
        }
    });
    bool_and(a34, a35)
}

pub fn f9(v0: Adt1) -> bool {
    let a40 = {
        let a37 = match v0.clone() {
            Adt1::C0(_, _, _, _, h36, _, _, _, _) => {
                h36
            }
        };
        let a38 = 0u64;
        nat_eq(a37, a38)
    };
    let a41 = f4(match v0.clone() {
        Adt1::C0(h39, _, _, _, _, _, _, _, _) => {
            h39
        }
    });
    bool_and(a40, a41)
}

pub fn f10(v0: Adt1) -> R<bool> {
    let a57 = {
        let a43 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h42, _, _, _) => {
                h42
            }
        };
        let a44 = 0u64;
        nat_eq(a43, a44)
    };
    let a58 = {
        let a55 = 2u64;
        let a56 = {
            let a53 = {
                let a50 = {
                    let a47 = match v0.clone() {
                        Adt1::C0(_, h45, _, _, _, _, _, _, _) => {
                            h45
                        }
                    };
                    let a48 = match v0.clone() {
                        Adt1::C0(_, _, h46, _, _, _, _, _, _) => {
                            h46
                        }
                    };
                    nat_add(a47, a48)?
                };
                let a51 = match v0.clone() {
                    Adt1::C0(_, _, _, h49, _, _, _, _, _) => {
                        h49
                    }
                };
                nat_add(a50, a51)?
            };
            let a54 = match v0.clone() {
                Adt1::C0(_, _, _, _, h52, _, _, _, _) => {
                    h52
                }
            };
            nat_add(a53, a54)?
        };
        nat_le(a55, a56)
    };
    Ok(bool_and(a57, a58))
}

pub fn f11(v0: Adt1) -> bool {
    let a72 = {
        let a60 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h59, _, _) => {
                h59
            }
        };
        let a61 = 0u64;
        nat_eq(a60, a61)
    };
    let a73 = {
        let a70 = {
            let a63 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h62, _, _, _) => {
                    h62
                }
            };
            let a64 = 1u64;
            nat_eq(a63, a64)
        };
        let a71 = {
            let a68 = {
                let m67 = match v0.clone() {
                    Adt1::C0(h66, _, _, _, _, _, _, _, _) => {
                        h66
                    }
                };
                match m67 {
                    Adt0::C0(_, _, _, _, _, h65) => {
                        h65
                    }
                }
            };
            let a69 = 1u64;
            nat_eq(a68, a69)
        };
        bool_and(a70, a71)
    };
    bool_and(a72, a73)
}

pub fn f12(v0: Adt1) -> bool {
    let a83 = {
        let a75 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h74, _) => {
                h74
            }
        };
        let a76 = 0u64;
        nat_eq(a75, a76)
    };
    let a84 = {
        let a81 = {
            let a78 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h77, _, _) => {
                    h77
                }
            };
            let a79 = 1u64;
            nat_eq(a78, a79)
        };
        let a82 = f5(match v0.clone() {
            Adt1::C0(h80, _, _, _, _, _, _, _, _) => {
                h80
            }
        });
        bool_and(a81, a82)
    };
    bool_and(a83, a84)
}

pub fn f13(v0: Adt1) -> bool {
    let a116 = {
        let a86 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h85) => {
                h85
            }
        };
        let a87 = 3u64;
        nat_lt(a86, a87)
    };
    let a117 = {
        let a114 = {
            let a94 = {
                let a89 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h88) => {
                        h88
                    }
                };
                let a90 = 0u64;
                nat_eq(a89, a90)
            };
            let a95 = {
                let a92 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h91, _, _, _) => {
                        h91
                    }
                };
                let a93 = 1u64;
                nat_eq(a92, a93)
            };
            bool_and(a94, a95)
        };
        let a115 = {
            let a112 = {
                let a102 = {
                    let a97 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, h96) => {
                            h96
                        }
                    };
                    let a98 = 1u64;
                    nat_eq(a97, a98)
                };
                let a103 = {
                    let a100 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h99, _, _) => {
                            h99
                        }
                    };
                    let a101 = 1u64;
                    nat_eq(a100, a101)
                };
                bool_and(a102, a103)
            };
            let a113 = {
                let a110 = {
                    let a105 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, h104) => {
                            h104
                        }
                    };
                    let a106 = 2u64;
                    nat_eq(a105, a106)
                };
                let a111 = {
                    let a108 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h107, _) => {
                            h107
                        }
                    };
                    let a109 = 1u64;
                    nat_eq(a108, a109)
                };
                bool_and(a110, a111)
            };
            bool_or(a112, a113)
        };
        bool_or(a114, a115)
    };
    bool_and(a116, a117)
}

pub fn f14(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h118, _, _, _, _, _, _, _, _) => {
            h118
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, h119, _, _, _, _, _, _) => {
            h119
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h120, _, _, _, _, _) => {
            h120
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h121, _, _, _, _) => {
            h121
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h122, _, _, _) => {
            h122
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h123, _, _) => {
            h123
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h124, _) => {
            h124
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h125) => {
            h125
        }
    })
}

pub fn f15(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h126, _, _, _, _, _, _, _, _) => {
            h126
        }
    }, match v0.clone() {
        Adt1::C0(_, h127, _, _, _, _, _, _, _) => {
            h127
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, h128, _, _, _, _, _) => {
            h128
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h129, _, _, _, _) => {
            h129
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h130, _, _, _) => {
            h130
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h131, _, _) => {
            h131
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h132, _) => {
            h132
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h133) => {
            h133
        }
    })
}

pub fn f16(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h134, _, _, _, _, _, _, _, _) => {
            h134
        }
    }, match v0.clone() {
        Adt1::C0(_, h135, _, _, _, _, _, _, _) => {
            h135
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h136, _, _, _, _, _, _) => {
            h136
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, h137, _, _, _, _) => {
            h137
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h138, _, _, _) => {
            h138
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h139, _, _) => {
            h139
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h140, _) => {
            h140
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h141) => {
            h141
        }
    })
}

pub fn f17(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h142, _, _, _, _, _, _, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt1::C0(_, h143, _, _, _, _, _, _, _) => {
            h143
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h144, _, _, _, _, _, _) => {
            h144
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h145, _, _, _, _, _) => {
            h145
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h146, _, _, _) => {
            h146
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h147, _, _) => {
            h147
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h148, _) => {
            h148
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h149) => {
            h149
        }
    })
}

pub fn f18(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h150, _, _, _, _, _, _, _, _) => {
            h150
        }
    }, match v0.clone() {
        Adt1::C0(_, h151, _, _, _, _, _, _, _) => {
            h151
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h152, _, _, _, _, _, _) => {
            h152
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h153, _, _, _, _, _) => {
            h153
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h154, _, _, _, _) => {
            h154
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h155, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h156, _) => {
            h156
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h157) => {
            h157
        }
    })
}

pub fn f19(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h158, _, _, _, _, _, _, _, _) => {
            h158
        }
    }, match v0.clone() {
        Adt1::C0(_, h159, _, _, _, _, _, _, _) => {
            h159
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h160, _, _, _, _, _, _) => {
            h160
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h161, _, _, _, _, _) => {
            h161
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h162, _, _, _, _) => {
            h162
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h163, _, _, _) => {
            h163
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h164, _) => {
            h164
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h165) => {
            h165
        }
    })
}

pub fn f20(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h166, _, _, _, _, _, _, _, _) => {
            h166
        }
    }, match v0.clone() {
        Adt1::C0(_, h167, _, _, _, _, _, _, _) => {
            h167
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h168, _, _, _, _, _, _) => {
            h168
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h169, _, _, _, _, _) => {
            h169
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h170, _, _, _, _) => {
            h170
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h171, _, _, _) => {
            h171
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h172, _, _) => {
            h172
        }
    }, 1u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h173) => {
            h173
        }
    })
}

pub fn f21(v0: Adt1) -> R<Adt1> {
    Ok(Adt1::C0(match v0.clone() {
        Adt1::C0(h174, _, _, _, _, _, _, _, _) => {
            h174
        }
    }, match v0.clone() {
        Adt1::C0(_, h175, _, _, _, _, _, _, _) => {
            h175
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h176, _, _, _, _, _, _) => {
            h176
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h177, _, _, _, _, _) => {
            h177
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h178, _, _, _, _) => {
            h178
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h179, _, _, _) => {
            h179
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h180, _, _) => {
            h180
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h181, _) => {
            h181
        }
    }, {
        let a183 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h182) => {
                h182
            }
        };
        let a184 = 1u64;
        nat_add(a183, a184)?
    }))
}

pub fn f22(v0: Adt1) -> Option<Adt1> {
    if f6(v0.clone()) {
        Some(f14(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f23(v0: Adt1) -> Option<Adt1> {
    if f7(v0.clone()) {
        Some(f15(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f24(v0: Adt1) -> Option<Adt1> {
    if f8(v0.clone()) {
        Some(f16(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f25(v0: Adt1) -> Option<Adt1> {
    if f9(v0.clone()) {
        Some(f17(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f26(v0: Adt1) -> R<Option<Adt1>> {
    if f10(v0.clone())? {
        Ok(Some(f18(v0.clone())))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f27(v0: Adt1) -> Option<Adt1> {
    if f11(v0.clone()) {
        Some(f19(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f28(v0: Adt1) -> Option<Adt1> {
    if f12(v0.clone()) {
        Some(f20(v0.clone()))
    } else {
        None::<Adt1>
    }
}

pub fn f29(v0: Adt1) -> R<Option<Adt1>> {
    if f13(v0.clone()) {
        Ok(Some(f21(v0.clone())?))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f30(v0: Adt0) -> Adt1 {
    Adt1::C0(v0.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f31(v0: Adt1) -> R<Option<Adt2>> {
    if f6(v0.clone()) {
        Ok(Some(Adt2::C0))
    } else {
        if f7(v0.clone()) {
            Ok(Some(Adt2::C1))
        } else {
            if f8(v0.clone()) {
                Ok(Some(Adt2::C2))
            } else {
                if f9(v0.clone()) {
                    Ok(Some(Adt2::C3))
                } else {
                    if f10(v0.clone())? {
                        Ok(Some(Adt2::C4))
                    } else {
                        if f11(v0.clone()) {
                            Ok(Some(Adt2::C5))
                        } else {
                            if f12(v0.clone()) {
                                Ok(Some(Adt2::C6))
                            } else {
                                if f13(v0.clone()) {
                                    Ok(Some(Adt2::C7))
                                } else {
                                    Ok(None::<Adt2>)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f32(v0: Adt1, v1: Adt2) -> R<Option<Adt1>> {
    let m185 = v1.clone();
    match m185 {
        Adt2::C0 => {
            Ok(f22(v0.clone()))
        }
        Adt2::C1 => {
            Ok(f23(v0.clone()))
        }
        Adt2::C2 => {
            Ok(f24(v0.clone()))
        }
        Adt2::C3 => {
            Ok(f25(v0.clone()))
        }
        Adt2::C4 => {
            f26(v0.clone())
        }
        Adt2::C5 => {
            Ok(f27(v0.clone()))
        }
        Adt2::C6 => {
            Ok(f28(v0.clone()))
        }
        Adt2::C7 => {
            f29(v0.clone())
        }
    }
}

pub fn f33(v0: Adt1) -> R<Option<Adt1>> {
    let m186 = f31(v0.clone())?;
    match m186 {
        None => {
            Ok(None::<Adt1>)
        }
        Some(v1) => {
            f32(v0.clone(), v1.clone())
        }
    }
}

pub fn f34(v0: Adt1) -> Option<u64> {
    Some(match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h187) => {
            h187
        }
    })
}

pub fn f35(v0: Adt0, v1: Adt1) -> Option<u64> {
    let m188 = f34(v1.clone());
    match m188 {
        None => {
            None::<u64>
        }
        Some(v2) => {
            let v3: bool = f37(v0.clone(), v2);
            if v3 {
                Some(v2)
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f36(v0: Adt0, v1: Adt1) -> Result<u64, (bool, bool)> {
    let m189 = f35(v0.clone(), v1.clone());
    match m189 {
        None => {
            let m190 = f34(v1.clone());
            match m190 {
                None => {
                    Err::<u64, (bool, bool)>((false, true))
                }
                Some(_) => {
                    Err::<u64, (bool, bool)>((true, false))
                }
            }
        }
        Some(v2) => {
            Ok::<u64, (bool, bool)>(v2)
        }
    }
}

pub fn f37(v0: Adt0, v1: u64) -> bool {
    let a207 = {
        let a191 = v1;
        let a192 = 3u64;
        nat_le(a191, a192)
    };
    let a208 = {
        let a205 = {
            let a193 = v1;
            let a194 = 3u64;
            nat_eq(a193, a194)
        };
        let a206 = {
            let a204 = {
                let a202 = f5(v0.clone());
                let a203 = {
                    let a200 = {
                        let a196 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, h195) => {
                                h195
                            }
                        };
                        let a197 = 1u64;
                        nat_eq(a196, a197)
                    };
                    let a201 = {
                        let a198 = f1(v0.clone());
                        let a199 = f2(v0.clone());
                        bool_and(a198, a199)
                    };
                    bool_and(a200, a201)
                };
                bool_and(a202, a203)
            };
            bool_not(a204)
        };
        bool_or(a205, a206)
    };
    bool_and(a207, a208)
}

pub fn f38(v0: Fn0, v1: u64, v2: Adt1) -> R<(Adt1, bool)> {
    let m209 = v1;
    if m209 == 0 {
        Ok((v2.clone(), false))
    } else {
        let v3 = m209 - 1;
        let m211 = {
            let c210: Fn0 = v0.clone();
            c210.apply(v2.clone())?
        };
        match m211 {
            None => {
                Ok((v2.clone(), true))
            }
            Some(v4) => {
                f38(v0.clone(), v3, v4.clone())
            }
        }
    }
}

pub fn run(p0: Adt0) -> R<Result<u64, (bool, bool)>> {
    f0(p0)
}
