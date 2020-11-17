// use crate::XYZ;
use crate::XYZW;
use core::{
    marker::Sized,
    ops::{Add, Div, Mul, Sub}
};

pub trait NumConsts : Sized {
    const ZERO: Self;
    const ONE: Self;
}

pub trait Num:
    NumConsts
    + Copy
    + Clone
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
{
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

pub trait Float: Num {
    fn sqrt(self) -> Self;
}

impl NumConsts for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Num for f32 {
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        f32::min(self, other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        f32::max(self, other)
    }
}

impl Float for f32 {
    #[inline(always)]
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

impl NumConsts for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Num for f64 {
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        f64::min(self, other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        f64::max(self, other)
    }
}

impl Float for f64 {
    #[inline(always)]
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
}

impl<T: Num> Add for XYZW<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        XYZW {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: Num> Div for XYZW<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        XYZW {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl<T: Num> Div<T> for XYZW<T> {
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        XYZW {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl<T: Num> Mul for XYZW<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
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
    fn mul(self, other: T) -> Self::Output {
        XYZW {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl<T: Num> Sub for XYZW<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        XYZW {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}
