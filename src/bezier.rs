//!  实现 bezier 曲线相关函数

use crate::types::KeyFrameCurveValue;

pub fn cubic_bezier(x1: KeyFrameCurveValue, y1: KeyFrameCurveValue, x2: KeyFrameCurveValue, y2: KeyFrameCurveValue, t: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _x1 = x1;
    let _x2 = x2;
    let _y1 = y1;
    let _y2 = y2;

    let _0  = 0.0;
    let _1  = 1.0;
    let _2  = 2.0;
    let _3  = 3.0;
    let _6  = 6.0;

    let f0  = _1 - _3 * _x2 + _3 * _x1;
    let f1  = _3 * _x2 - _6 * _x1;
    let f2  = _3 * _x1;

    let mut refined_t = t;

    for _ in 0..5 {
        let refined_t2 = refined_t * refined_t;
        let refined_t3 = refined_t2 * refined_t;

        let x = f0 * refined_t3 + f1 * refined_t2 + f2 * refined_t;
        let slop = _1 / (_3 * f0 * refined_t2 + _2 * f1 * refined_t + f2);

        refined_t -= (x - t) * slop;

        refined_t = KeyFrameCurveValue::min(_1, KeyFrameCurveValue::max(_0, refined_t));
    };

    _3 * KeyFrameCurveValue::powi(_1 - refined_t, 2) * refined_t * _y1 + _3 * (_1 - refined_t) * KeyFrameCurveValue::powi(refined_t, 2) * _y2 + KeyFrameCurveValue::powi(refined_t, 3)
}
