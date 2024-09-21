//! 实现 Easing 缓动函数

#[cfg(feature = "amount_f32")]
use std::f32::consts::PI;
#[cfg(feature = "amount_f64")]
use std::f64::consts::PI;

use simba::scalar::ComplexField;

use crate::{easing::EEasingMode, types::KeyFrameCurveValue};

/// https://easings.net/# - 缓动函数实现
pub fn easing_call(x: KeyFrameCurveValue, mode: &EEasingMode) -> KeyFrameCurveValue {
    match mode {
        EEasingMode::None           => linear_in        (x),
        EEasingMode::BackIn         => back_in          (x),
        EEasingMode::BackOut        => back_out         (x),
        EEasingMode::BackInOut      => back_in_out      (x),
        EEasingMode::CircleIn       => circle_in        (x),
        EEasingMode::CircleOut      => circle_out       (x),
        EEasingMode::CircleInOut    => circle_in_out    (x),
        EEasingMode::CubicIn        => cubic_in         (x),
        EEasingMode::CubicOut       => cubic_out        (x),
        EEasingMode::CubicInOut     => cubic_in_out     (x),
        EEasingMode::SineIn         => sine_in          (x),
        EEasingMode::SineOut        => sine_out         (x),
        EEasingMode::SineInOut      => sine_in_out      (x),
        EEasingMode::QuadIn         => quad_in          (x),
        EEasingMode::QuadOut        => quad_out         (x),
        EEasingMode::QuadInOut      => quad_in_out      (x),
        EEasingMode::QuartIn        => quart_in         (x),
        EEasingMode::QuartOut       => quart_out        (x),
        EEasingMode::QuartInOut     => quart_in_out     (x),
        EEasingMode::QuintIn        => quint_in         (x),
        EEasingMode::QuintOut       => quint_out        (x),
        EEasingMode::QuintInOut     => quint_in_out     (x),
        EEasingMode::ExpoIn         => expo_in          (x),
        EEasingMode::ExpoOut        => expo_out         (x),
        EEasingMode::ExpoInOut      => expo_in_out      (x),
        EEasingMode::ElasticIn      => elastic_in       (x),
        EEasingMode::ElasticOut     => elastic_out      (x),
        EEasingMode::ElasticInOut   => elastic_in_out   (x),
        EEasingMode::BounceIn       => bounce_in        (x),
        EEasingMode::BounceOut      => bounce_out       (x),
        EEasingMode::BounceInOut    => bounce_in_out    (x),
    }
}

pub fn get_easing_call(mode: EEasingMode) -> fn(KeyFrameCurveValue) -> KeyFrameCurveValue {
    match mode {
        EEasingMode::None           => linear_in        ,
        EEasingMode::BackIn         => back_in          ,
        EEasingMode::BackOut        => back_out         ,
        EEasingMode::BackInOut      => back_in_out      ,
        EEasingMode::CircleIn       => circle_in        ,
        EEasingMode::CircleOut      => circle_out       ,
        EEasingMode::CircleInOut    => circle_in_out    ,
        EEasingMode::CubicIn        => cubic_in         ,
        EEasingMode::CubicOut       => cubic_out        ,
        EEasingMode::CubicInOut     => cubic_in_out     ,
        EEasingMode::SineIn         => sine_in          ,
        EEasingMode::SineOut        => sine_out         ,
        EEasingMode::SineInOut      => sine_in_out      ,
        EEasingMode::QuadIn         => quad_in          ,
        EEasingMode::QuadOut        => quad_out         ,
        EEasingMode::QuadInOut      => quad_in_out      ,
        EEasingMode::QuartIn        => quart_in         ,
        EEasingMode::QuartOut       => quart_out        ,
        EEasingMode::QuartInOut     => quart_in_out     ,
        EEasingMode::QuintIn        => quint_in         ,
        EEasingMode::QuintOut       => quint_out        ,
        EEasingMode::QuintInOut     => quint_in_out     ,
        EEasingMode::ExpoIn         => expo_in          ,
        EEasingMode::ExpoOut        => expo_out         ,
        EEasingMode::ExpoInOut      => expo_in_out      ,
        EEasingMode::ElasticIn      => elastic_in       ,
        EEasingMode::ElasticOut     => elastic_out      ,
        EEasingMode::ElasticInOut   => elastic_in_out   ,
        EEasingMode::BounceIn       => bounce_in        ,
        EEasingMode::BounceOut      => bounce_out       ,
        EEasingMode::BounceInOut    => bounce_in_out    ,
    }
}

pub fn back_in(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let xx = x * x;
    let c1 = 1.70158;
    let c3 = c1 + 1.;

    return c3 * x * xx - c1 * xx;
}
pub fn back_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let c1 = 1.70158;
    let c3 = c1 + 1.;

    let temp = x - 1.;

    return 1. + c3 * temp.powi(3) + c1 * temp.powi(2);
}
pub fn back_in_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let c1 = 1.70158;
    let c2 = c1 * 1.525;

    let _1 = 1.;
    let _2 = 2.;
    let _2x = x * 2.;

    if x < 0.5 {
        KeyFrameCurveValue::powi(_2x, 2) * ((c2 + _1) * _2x - c2) / _2
    }
    else {
        (KeyFrameCurveValue::powi(_2x - _2, 2) * ((c2 + _1) * (_2x - _2) + c2) + _2) / _2
    }
}

pub fn bounce_in(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    1. - bounce_out(1. - x)
}
pub fn bounce_out(mut x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let n1 = 7.5625;
    let d1 = 2.75;

    if x < 1. / d1 {
        return n1 * x * x;
    } else if x < 2.0 / d1 {
        x -= 1.5 / d1;

        return n1 * x * x + 0.75;
    } else if x < 2.5 / d1 {
        x -= 2.25 / d1;

        return n1 * x * x + 0.9375;
    } else {
        x -= 2.625 / d1;

        return n1 * x * x + 0.984375;
    }
}
pub fn bounce_in_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _0_5 = 0.5;
    let _1 = 1.;

    if x < _0_5 {
        (_1 - bounce_out(_1 - (x + x))) * _0_5
    }
    else {
        (_1 + bounce_out((x + x) - _1)) * _0_5
    }
}

pub fn circle_in(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _0 = 0.;
    let _1 = 1.;

    _1 - KeyFrameCurveValue::sqrt(_1) - KeyFrameCurveValue::powi(x, 2)
}
pub fn circle_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _1 = 1.;

    // _1 - circle_in(_1 - x)
    KeyFrameCurveValue::sqrt(_1) - KeyFrameCurveValue::powi(_1 - x, 2)
}
pub fn circle_in_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _0_5 = 0.5;
    let _2 = 2.;
    let _1 = 1.;

    if x < _0_5 {
        circle_in(x * _2) / _2
    }
    else {
        (KeyFrameCurveValue::sqrt(_1) - KeyFrameCurveValue::powi(_2 - x - x, 2) + _1) / _2
    }
}

pub fn cubic_in(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    KeyFrameCurveValue::powi(x, 3)
}
pub fn cubic_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _1 = 1.;

    _1 - cubic_in(_1 - x)
}
pub fn cubic_in_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _0_5 = 0.5;
    let _1 = 1.;

    if x < _0_5 {
        let _16 = 16.;
        let xx = x * x;
        _16 * xx * xx * x
    }
    else {
        let _2 = 2.;
        _1 - KeyFrameCurveValue::powi(- _2 * x + _2, 3) / _2
    }
}

pub fn quad_in(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    x * x
}
pub fn quad_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _1 = 1.;

    _1 - quad_in(_1 - x)
}
pub fn quad_in_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _0_5 = 0.5;
    let _1 = 1.;
    let _2 = 2.;

    if x < _0_5 {
        quad_in(x) * _2
    }
    else {
        _1 - quad_in(_2 * (_1 - x)) * _0_5
    }
}

pub fn quart_in(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    KeyFrameCurveValue::powi(x, 4)
}
pub fn quart_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _1 = 1.;

    _1 - KeyFrameCurveValue::powi(_1 - x, 4)
}
pub fn quart_in_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _0_5 = 0.5;
    let _1 = 1.;
    let _8 = 8.;

    if x < _0_5 {
        _8 * KeyFrameCurveValue::powi(x, 4)
    }
    else {
        let t = -x -x + _1 + _1; // 2-2x
        _1 - KeyFrameCurveValue::powi(t, 4) * _0_5
    }
}

pub fn quint_in(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    KeyFrameCurveValue::powi(x, 5)
}
pub fn quint_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _1 = 1.;
    
    _1 - quint_in(_1 - x)
}
pub fn quint_in_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _0_5 = 0.5;
    let _16 = 16.;
    let _1 = 1.;

    if x < _0_5 {
        _16 * quint_in(x)
    }
    else {
        _1 - quint_in(_1 + _1 - x - x) * _0_5
    }
}

pub fn sine_in(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _1 = 1.;
    let _2 = 2.;

    _1 - (x * PI / _2).cos()
}
pub fn sine_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _2 = 2.;

    (x * PI / _2).sin()
}
pub fn sine_in_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _1 = 1.;
    let _2 = 2.;

    -((x * PI).cos() - _1) / _2
}

pub fn expo_in(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    if x == 0. {
        0.
    }
    else {
        let _2 = 2.;
        let _10 = 10.;
        _2.powf(_10 * x - _10)
    }
}
pub fn expo_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _1 = 1.;
    if x == _1 {
        1.
    }
    else {
        let _2 = 2.;
        let _10 = 10.;
        _1 - _2.powf(-_10 * x)
    }
}
pub fn expo_in_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    
    let _1 = 1.;
    let _0_5 = 0.5;

    if x == 0. {
        0.
    }
    else if x == _1 {
        1.
    }
    else if x < _0_5 {
        let _2 = 2.;
        let _10 = 10.;
        let _20 = 20.;

        _2.powf(_20 * x - _10) / _2
    }
    else {
        let _2 = 2.;
        let _10 = 10.;
        let _20 = 20.;

        _1 - _2.powf(-_20 * x + _10) / _2
    }
}

pub fn elastic_in(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _0 = 0.;
    let _0_5 = 0.5;
    let _1 = 1.;
    let _2 = 2.;
    let _3 = 3.;
    let _10 = 10.;
    let _20 = 20.;
    let _t = 10.75;

    let c4 = _2 * PI / _3;
    
    if x == _0 {
        _0
    }
    else if x == _1 {
        _1
    }
    else {
        -_2.powf(_10 * x - _10) * ((_10 * x - _t) * c4).sin()
    }
}
pub fn elastic_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _0 = 0.;
    let _0_5 = 0.5;
    let _1 = 1.;
    let _2 = 2.;
    let _3 = 3.;
    let _10 = 10.;
    let _20 = 20.;

    let _t = 0.75;

    let c4 = _2 * PI /_3;
    
    if x == _0 {
        _0
    }
    else if x == _1 {
        _1
    }
    else {
        _2.powf(-_10 * x) * ((_10 * x - _t) * c4).sin() + _1
    }
}
pub fn elastic_in_out(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _0 = 0.;
    let _0_5 = 0.5;
    let _1 = 1.;
    let _2 = 2.;
    let _3 = 3;
    let _10 = 10.;
    let _20 = 20.;

    let _4_5 = 4.5;
    let _t = 11.125;

    let c5 = _2 * PI / _4_5;
    
    if x == _0 {
        _0
    }
    else if x == _1 {
        _1
    }
    else if x < _0_5 {
        -_2.powf(_20 * x - _10) * ((_20 * x - _t) * c5).sin() / _2
    }
    else {
        _2.powf(-_20 * x + _10) * ((_20 * x - _t) * c5).sin() / _2 + _1
    }
}

pub fn linear_in(x: KeyFrameCurveValue) -> KeyFrameCurveValue {
    x
}