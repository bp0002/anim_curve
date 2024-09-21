
use crate::{EAmountMode, AmountParam, steps::EStepMode, curve::{KeyFrameCurveValue, FrameIndex}, easing::EEasingMode};

/// 动画进度计算器
pub struct AnimationAmountCalc {
    // 进度曲线
    mode: EAmountMode,
    // 进度计算参数
    param: AmountParam,
    // 进度曲线函数,创建时赋值,不在运行时对mode进行匹配,获得更好性能
    call: fn(KeyFrameCurveValue, &AmountParam) -> KeyFrameCurveValue,
}

impl Default for AnimationAmountCalc {
    fn default() -> Self {
        Self {
            mode: EAmountMode::None,
            param: AmountParam::default(),
            call: EAmountMode::get_transform_amount_call(EAmountMode::None),
        }
    }
}

impl AnimationAmountCalc {
    pub fn mode(&self) -> EAmountMode {
        self.mode
    }
    /// 创建一个步进模式的进度计算器
    pub fn from_steps(step: FrameIndex, mode: EStepMode) -> Self {
        if step < 1 {
            AnimationAmountCalc::default()
        } else {
            let mode = EAmountMode::Steps(mode);
            Self {
                mode,
                param: AmountParam(step as KeyFrameCurveValue, 0., 0., 0.),
                call: EAmountMode::get_transform_amount_call(mode),
            }
        }
    }
    /// 创建一个缓动模式的进度计算器
    pub fn from_easing(mode: EEasingMode) -> Self {
        let mode = EAmountMode::Easing(mode);
        Self {
            mode,
            param: AmountParam::default(),
            call: EAmountMode::get_transform_amount_call(mode),
        }
    }
    /// 创建一个三次贝塞尔曲线模式的进度计算器
    pub fn from_cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        let mode = EAmountMode::CubicBezier;
        Self {
            mode,
            param: AmountParam(x1 as KeyFrameCurveValue, y1 as KeyFrameCurveValue, x2 as KeyFrameCurveValue, y2 as KeyFrameCurveValue),
            call: EAmountMode::get_transform_amount_call(mode),
        }
    }
    /// 计算进度
    pub fn calc(&self, amount: KeyFrameCurveValue) -> KeyFrameCurveValue {
        let call = &self.call;
        call(amount, &self.param)
    }
}