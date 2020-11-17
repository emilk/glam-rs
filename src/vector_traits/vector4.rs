use crate::XYZW;

pub trait Vector4Consts {
    const ZERO: Self;
    const ONE: Self;
    const UNIT_X: Self;
    const UNIT_Y: Self;
    const UNIT_Z: Self;
    const UNIT_W: Self;
}

pub trait Vector4 {
    type S: Sized;

    fn new(x: Self::S, y: Self::S, z: Self::S, w: Self::S) -> Self;
    fn splat(s: Self::S) -> Self;

    fn from_slice_unaligned(slice: &[Self::S]) -> Self;
    fn write_to_slice_unaligned(self, slice: &mut [Self::S]);

    fn deref(&self) -> &XYZW<Self::S>;
    fn deref_mut(&mut self) -> &mut XYZW<Self::S>;

    fn add(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;

    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

pub trait Float4: Vector4 {
    fn dot(self, other: Self) -> Self::S;
    fn length(self) -> Self::S;
    fn length_recip(self) -> Self::S;
    fn normalize(self) -> Self;
}

mod scalar {
    use crate::scalar_traits::{Float, Num, NumConsts};
    use crate::vector_traits::{Float4, Vector4, Vector4Consts};
    use crate::XYZW;
    impl<T: Float> Vector4Consts for XYZW<T> {
        const ZERO: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ZERO,
            z: <T as NumConsts>::ZERO,
            w: <T as NumConsts>::ZERO,
        };
        const ONE: Self = Self {
            x: <T as NumConsts>::ONE,
            y: <T as NumConsts>::ONE,
            z: <T as NumConsts>::ONE,
            w: <T as NumConsts>::ONE,
        };
        const UNIT_X: Self = Self {
            x: <T as NumConsts>::ONE,
            y: <T as NumConsts>::ZERO,
            z: <T as NumConsts>::ZERO,
            w: <T as NumConsts>::ZERO,
        };
        const UNIT_Y: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ONE,
            z: <T as NumConsts>::ZERO,
            w: <T as NumConsts>::ZERO,
        };
        const UNIT_Z: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ZERO,
            z: <T as NumConsts>::ONE,
            w: <T as NumConsts>::ZERO,
        };
        const UNIT_W: Self = Self {
            x: <T as NumConsts>::ZERO,
            y: <T as NumConsts>::ZERO,
            z: <T as NumConsts>::ZERO,
            w: <T as NumConsts>::ONE,
        };
    }

    impl<T: Num> Vector4 for XYZW<T> {
        type S = T;

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

        fn from_slice_unaligned(slice: &[Self::S]) -> Self {
            Self {
                x: slice[0],
                y: slice[1],
                z: slice[2],
                w: slice[3],
            }
        }

        fn write_to_slice_unaligned(self, slice: &mut [Self::S]) {
            slice[0] = self.x;
            slice[1] = self.y;
            slice[2] = self.z;
            slice[3] = self.w;
        }

        fn deref(&self) -> &XYZW<Self::S> {
            self
        }

        fn deref_mut(&mut self) -> &mut XYZW<Self::S> {
            self
        }

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
                w: self.w + other.w,
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

        fn mul(self, other: Self) -> Self {
            Self {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
                w: self.w * other.w,
            }
        }

        fn sub(self, other: Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
                w: self.w - other.w,
            }
        }

        fn min(self, other: Self) -> Self {
            Self {
                x: self.x.min(other.x),
                y: self.y.min(other.y),
                z: self.z.min(other.z),
                w: self.w.min(other.w),
            }
        }

        fn max(self, other: Self) -> Self {
            Self {
                x: self.x.max(other.x),
                y: self.y.max(other.y),
                z: self.z.max(other.z),
                w: self.w.max(other.w),
            }
        }
    }

    impl<T: Float> Float4 for XYZW<T> {
        fn dot(self, other: Self) -> Self::S {
            (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
        }

        fn length(self) -> Self::S {
            self.dot(self).sqrt()
        }

        fn length_recip(self) -> Self::S {
            <T as NumConsts>::ONE / self.length()
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

    use crate::{const_m128, XYZW};
    use super::{Float4, Vector4, Vector4Consts};

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

    impl Vector4Consts for __m128 {
        const ZERO: __m128 = const_m128!([0.0; 4]);
        const ONE: __m128 = const_m128!([1.0; 4]);
        const UNIT_X: __m128 = const_m128!([1.0, 0.0, 0.0, 0.0]);
        const UNIT_Y: __m128 = const_m128!([0.0, 1.0, 0.0, 0.0]);
        const UNIT_Z: __m128 = const_m128!([0.0, 0.0, 1.0, 0.0]);
        const UNIT_W: __m128 = const_m128!([0.0, 0.0, 0.0, 1.0]);
    }

    impl Vector4 for __m128 {
        type S = f32;

        fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            unsafe { _mm_set_ps(w, z, y, x) }
        }

        fn splat(s: f32) -> Self {
            unsafe { _mm_set_ps1(s) }
        }

        fn from_slice_unaligned(slice: &[Self::S]) -> Self {
            assert!(slice.len() >= 4);
            unsafe { _mm_loadu_ps(slice.as_ptr()) }
        }

        fn write_to_slice_unaligned(self, slice: &mut [Self::S]) {
            unsafe {
                assert!(slice.len() >= 4);
                _mm_storeu_ps(slice.as_mut_ptr(), self);
            }
        }

        fn deref(&self) -> &XYZW<Self::S> {
            unsafe { &*(self as *const Self as *const XYZW<Self::S>) }
        }

        fn deref_mut(&mut self) -> &mut XYZW<Self::S> {
            unsafe { &mut *(self as *mut Self as *mut XYZW<Self::S>) }
        }

        fn add(self, other: Self) -> Self {
            unsafe { _mm_add_ps(self, other) }
        }

        fn div(self, other: Self) -> Self {
            unsafe { _mm_div_ps(self, other) }
        }

        fn mul(self, other: Self) -> Self {
            unsafe { _mm_mul_ps(self, other) }
        }

        fn sub(self, other: Self) -> Self {
            unsafe { _mm_sub_ps(self, other) }
        }

        fn min(self, other: Self) -> Self {
            unsafe {
                _mm_min_ps(self, other)
            }
        }

        fn max(self, other: Self) -> Self {
            unsafe {
                _mm_max_ps(self, other)
            }
        }
    }

    impl Float4 for __m128 {
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
