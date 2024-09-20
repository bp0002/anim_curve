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

pub mod frame;
pub mod frame_curve;
pub mod curves;
