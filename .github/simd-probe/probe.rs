//! The SIMD probe of decision #109 (D4), run on the pinned Rust by
//! `.github/simd-probe/run.sh` and recorded in VERIFICATION.md ("The SIMD probe
//! of #109 on the pinned Rust").
//!
//! For each of the four targets #109 lists, `inside` uses that target's
//! integer SIMD intrinsics inside a `#[target_feature]` function, and
//! `outside` (compiled only with `--cfg probe_outside`) calls `inside` from a
//! function without the attribute. The crate forbids unsafe code, as the
//! safe-Rust rule (RP-09) requires of an exported root.
#![no_std]
#![forbid(unsafe_code)]

#[cfg(target_arch = "x86_64")]
pub mod probe {
    use core::arch::x86_64::{
        __m256i, _mm256_extract_epi32, _mm256_madd_epi16, _mm256_maddubs_epi16, _mm256_set1_epi16,
        _mm256_set1_epi8, _mm256_setr_epi8,
    };

    /// Unsigned bytes times signed bytes, summed in pairs to 16 bits and then
    /// in pairs to 32 bits.
    #[target_feature(enable = "avx2")]
    pub fn dot(a: __m256i, b: __m256i) -> __m256i {
        _mm256_madd_epi16(_mm256_maddubs_epi16(a, b), _mm256_set1_epi16(1))
    }

    #[target_feature(enable = "avx2")]
    pub fn inside(x: i8) -> i32 {
        let a = _mm256_setr_epi8(
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        );
        _mm256_extract_epi32::<0>(dot(a, _mm256_set1_epi8(x)))
    }

    #[cfg(probe_outside)]
    pub fn outside(x: i8) -> i32 {
        inside(x)
    }
}

#[cfg(target_arch = "aarch64")]
pub mod probe {
    use core::arch::aarch64::{vaddvq_s32, vdup_n_s8, vmull_s8, vpaddlq_s16};

    #[target_feature(enable = "neon")]
    pub fn inside(x: i8) -> i32 {
        vaddvq_s32(vpaddlq_s16(vmull_s8(vdup_n_s8(x), vdup_n_s8(3))))
    }

    #[cfg(probe_outside)]
    pub fn outside(x: i8) -> i32 {
        inside(x)
    }
}

#[cfg(target_arch = "wasm32")]
pub mod probe {
    use core::arch::wasm32::{
        i16x8_extmul_low_i8x16, i32x4_dot_i16x8, i32x4_extract_lane, i8x16_splat,
    };

    #[target_feature(enable = "simd128")]
    pub fn inside(x: i8) -> i32 {
        let products = i16x8_extmul_low_i8x16(i8x16_splat(x), i8x16_splat(3));
        i32x4_extract_lane::<0>(i32x4_dot_i16x8(
            products,
            i16x8_extmul_low_i8x16(i8x16_splat(1), i8x16_splat(1)),
        ))
    }

    #[cfg(probe_outside)]
    pub fn outside(x: i8) -> i32 {
        inside(x)
    }
}

#[cfg(target_arch = "arm")]
pub mod probe {
    use core::arch::arm::__qadd8;

    #[target_feature(enable = "dsp")]
    pub fn inside(x: i32) -> i32 {
        __qadd8(x, x)
    }

    #[cfg(probe_outside)]
    pub fn outside(x: i32) -> i32 {
        inside(x)
    }
}
