//!
//! 关键帧数据结构
pub use crate::types::FrameIndex;
pub use crate::types::FramePerSecond;
pub use crate::types::InOutTangent;
pub use crate::types::CubicBezier;

#[derive(Debug)]
pub enum ErrorCurve {
    KeyFrameDataTypeCannotAllocMore,
}

mod frame;
mod frame_curve;
mod curves;

pub use frame::*;
pub use frame_curve::*;
pub use curves::*;
