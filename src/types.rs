
/// 关键帧曲线数值类型,动画进度
#[cfg(feature = "amount_f32")]
pub type KeyFrameCurveValue = f32;
#[cfg(feature = "amount_f64")]
pub type KeyFrameCurveValue = f64;

/// 关键帧序号
#[cfg(feature = "frameidx_u16")]
pub type FrameIndex = u16;
#[cfg(feature = "frameidx_u32")]
pub type FrameIndex = u32;
/// 每秒帧数, FPS
pub type FramePerSecond = FrameIndex;

pub type InOutTangent<T> = Vec<T>;
pub type CubicBezier<T> = Vec<T>;

pub type KeyFrameDataType = usize;
