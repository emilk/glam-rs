// use crate::XYZ;
use core::marker::Sized;
use core::ops::{Add, Div, Mul};
use crate::XYZW;

pub trait Num:
    Sized + Copy + Clone + Add<Output = Self> + Div<Output = Self> + Mul<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;
}

pub trait Float: Num {
    fn sqrt(self) -> Self;
}

impl Num for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Float for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

impl Num for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Float for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
}

impl<T: Num> Add for XYZW<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        XYZW {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: Num> Mul for XYZW<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        XYZW {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl<T: Num> Mul<T> for XYZW<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        XYZW {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

pub trait Float4 {
    type S: Sized;
    const ZERO: Self;
    const ONE: Self;
    const UNIT_X: Self;
    const UNIT_Y: Self;
    const UNIT_Z: Self;
    const UNIT_W: Self;

    fn new(x: Self::S, y: Self::S, z: Self::S, w: Self::S) -> Self;
    fn splat(s: Self::S) -> Self;

    fn div(self, other: Self) -> Self;

    fn dot(self, other: Self) -> Self::S;
    fn length(self) -> Self::S;
    fn length_recip(self) -> Self::S;
    fn normalize(self) -> Self;
}

mod scalar {
    use super::{Float, Float4, Num};
    use crate::XYZW;
    impl<T: Float> Float4 for XYZW<T> {
        type S = T;
        const ZERO: Self = Self {
            x: <T as Num>::ZERO,
            y: <T as Num>::ZERO,
            z: <T as Num>::ZERO,
            w: <T as Num>::ZERO,
        };
        const ONE: Self = Self {
            x: <T as Num>::ONE,
            y: <T as Num>::ONE,
            z: <T as Num>::ONE,
            w: <T as Num>::ONE,
        };
        const UNIT_X: Self = Self {
            x: <T as Num>::ONE,
            y: <T as Num>::ZERO,
            z: <T as Num>::ZERO,
            w: <T as Num>::ZERO,
        };
        const UNIT_Y: Self = Self {
            x: <T as Num>::ZERO,
            y: <T as Num>::ONE,
            z: <T as Num>::ZERO,
            w: <T as Num>::ZERO,
        };
        const UNIT_Z: Self = Self {
            x: <T as Num>::ZERO,
            y: <T as Num>::ZERO,
            z: <T as Num>::ONE,
            w: <T as Num>::ZERO,
        };
        const UNIT_W: Self = Self {
            x: <T as Num>::ZERO,
            y: <T as Num>::ZERO,
            z: <T as Num>::ZERO,
            w: <T as Num>::ONE,
        };

        fn new(x: T, y: T, z: T, w: T) -> Self {
            Self { x, y, z, w }
        }

        fn splat(s: T) -> Self {
            Self {
                x: s,
                y: s,
                z: s,
                w: s,
            }
        }

        fn div(self, other: Self) -> Self {
            Self {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z,
                w: self.w / other.w,
            }
        }

        fn dot(self, other: Self) -> Self::S {
            (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
        }

        fn length(self) -> Self::S {
            self.dot(self).sqrt()
        }

        fn length_recip(self) -> Self::S {
            <T as Num>::ONE / self.length()
        }

        fn normalize(self) -> Self {
            self * self.length_recip()
        }
    }
}

#[cfg(target_feature = "sse2")]
mod sse2 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    use super::Float4;

    #[inline]
    unsafe fn dot_as_m128(lhs: __m128, rhs: __m128) -> __m128 {
        let x2_y2_z2_w2 = _mm_mul_ps(lhs, rhs);
        let z2_w2_0_0 = _mm_shuffle_ps(x2_y2_z2_w2, x2_y2_z2_w2, 0b00_00_11_10);
        let x2z2_y2w2_0_0 = _mm_add_ps(x2_y2_z2_w2, z2_w2_0_0);
        let y2w2_0_0_0 = _mm_shuffle_ps(x2z2_y2w2_0_0, x2z2_y2w2_0_0, 0b00_00_00_01);
        _mm_add_ps(x2z2_y2w2_0_0, y2w2_0_0_0)
    }

    /// Returns Vec4 dot in all lanes of Vec4
    // #[inline]
    // pub(crate) fn dot_as_float4(lhs: __m128, rhs: __m128) -> __m128 {
    //     unsafe {
    //         let dot_in_x = dot_as_m128(lhs, rhs);
    //         _mm_shuffle_ps(dot_in_x, dot_in_x, 0b00_00_00_00)
    //     }
    // }

    impl Float4 for __m128 {
        type S = f32;
        const ZERO: __m128 = const_m128!([0.0; 4]);
        const ONE: __m128 = const_m128!([1.0; 4]);
        const UNIT_X: __m128 = const_m128!([1.0, 0.0, 0.0, 0.0]);
        const UNIT_Y: __m128 = const_m128!([0.0, 1.0, 0.0, 0.0]);
        const UNIT_Z: __m128 = const_m128!([0.0, 0.0, 1.0, 0.0]);
        const UNIT_W: __m128 = const_m128!([0.0, 0.0, 0.0, 1.0]);

        fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            unsafe { _mm_set_ps(w, z, y, x) }
        }

        fn splat(s: f32) -> Self {
            unsafe { _mm_set_ps1(s) }
        }

        fn div(self, other: Self) -> Self {
            unsafe { _mm_div_ps(self, other) }
        }

        fn dot(self, other: Self) -> f32 {
            unsafe { _mm_cvtss_f32(dot_as_m128(self, other)) }
        }

        fn length(self) -> f32 {
            unsafe {
                let dot = dot_as_m128(self, self);
                _mm_cvtss_f32(_mm_sqrt_ps(dot))
            }
        }

        fn length_recip(self) -> f32 {
            unsafe {
                let dot = dot_as_m128(self, self);
                // _mm_rsqrt_ps is lower precision
                _mm_cvtss_f32(_mm_div_ps(Self::ONE, _mm_sqrt_ps(dot)))
            }
        }

        fn normalize(self) -> Self {
            unsafe {
                let dot = dot_as_m128(self, self);
                _mm_div_ps(self, _mm_sqrt_ps(dot))
            }
        }
    }
}
