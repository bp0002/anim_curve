
use std::ops::Add;

use super::ErrorCurve;

/// 关键帧曲线数值类型
pub use crate::types::KeyFrameCurveValue;
pub use crate::types::KeyFrameDataType;

/// 构建帧数据结构
#[derive(Debug)]
pub struct CurveFrameValue<T: FrameDataValue> {
    /// 帧数据值
    value: T,
    args: [T; 2]
}

impl<T: FrameDataValue> CurveFrameValue<T> {
    pub fn new(value: T, args: [T; 2]) -> Self {
        CurveFrameValue {
            value,
            args
        }
    }
    pub fn value(&self) -> &T {
        &self.value
    }
    pub fn intangent(&self) -> &T {
        &self.args[0]
    }
    pub fn outtangent(&self) -> &T {
        &self.args[1]
    }
}

pub trait FrameValueScale {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self;
}

/// 动画数据类型Idx分配器 - 使用 usize, 便于用Vec存储类型
pub struct KeyFrameDataTypeAllocator {
    counter: KeyFrameDataType,
}

impl KeyFrameDataTypeAllocator {
    pub fn default() -> Self {
        Self {
            counter: 0
        }
    }
    pub fn alloc(
        &mut self,
    ) -> Result<KeyFrameDataType, ErrorCurve> {
        if self.counter == KeyFrameDataType::MAX {
            Err(ErrorCurve::KeyFrameDataTypeCannotAllocMore)
        } else {
            let id = self.counter;
            self.counter += 1;
            Ok(id)
        }
    }
}

pub trait FrameDataValue: Clone {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self;
    fn append(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self;
    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: KeyFrameCurveValue, frame_delta: KeyFrameCurveValue) -> Self;
    fn size() -> usize;
}

impl<T: Clone + FrameValueScale + Add<Output = Self>> FrameDataValue for T {
    fn interpolate(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        self.scale(1.0 - amount) + rhs.scale(amount)
    }
    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: KeyFrameCurveValue, frame_delta: KeyFrameCurveValue) -> Self {
        let _1 = 1 as KeyFrameCurveValue;
        let _2 = 2 as KeyFrameCurveValue;
        let _3 = 3 as KeyFrameCurveValue;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

        return (((value1.scale(part1)) + (value2.scale(part2))) + (tangent1.scale(part3 * frame_delta))) + (tangent2.scale(part4 * frame_delta));
    }
    fn append(&self, rhs: &Self, amount: KeyFrameCurveValue) -> Self {
        self.clone() + rhs.scale(amount)
    }
    fn size() -> usize {
        8
    }
}

/// f32
impl FrameValueScale for f32 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        self * rhs as Self
    }
}

/// f64
impl FrameValueScale for f64 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        self * rhs as Self
    }
}

/// u8
impl FrameValueScale for u8 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as Self
    }
}

/// u16
impl FrameValueScale for u16 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as Self
    }
}

/// u32
impl FrameValueScale for u32 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as Self
    }
}

/// u64
impl FrameValueScale for u64 {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as Self
    }
}

/// usize
impl FrameValueScale for usize {
    fn scale(&self, rhs: KeyFrameCurveValue) -> Self {
        (*self as KeyFrameCurveValue * rhs) as Self
    }
}
