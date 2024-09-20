# anim_curve

Rust value animation curve

* 动画曲线包含
  * 关键帧帧数据
  * 曲线进度计算器

* 曲线进度数据类型可选 `f32` 或 `f64`, 默认为 `f32`
* 曲线类型
  * 关键帧插值
    * 线性插值
    * CubicSplice 插值
  * CubicBezier 插值曲线
  * 类Unity的 MinMaxCurve Hermit插值曲线

## Example

* 创建线性缓动曲线
```Rust
    let curve = FrameCurve::curve_easing(0.0, 1.0, frame_count as FrameIndex, frame_count, EEasingMode::None);
```

* 创建关键帧线性插值曲线
```Rust
    let mut curve = FrameCurve::curve_frame_values(60);
    FrameCurve::curve_frame_values_frame(&mut curve, 0, 0.0);
    FrameCurve::curve_frame_values_frame(&mut curve, 15 as FrameIndex, 1.0);
    FrameCurve::curve_frame_values_frame(&mut curve, 30 as FrameIndex, 3.0);
    FrameCurve::curve_frame_values_frame(&mut curve, 60 as FrameIndex, 4.0);

    // 创建步进进度计算器
    let amountcalc = AnimationAmountCalc::from_steps(1, EStepMode::JumpStart);
    let val = curve.interple(i as KeyFrameCurveValue / 60., &amountcalc);
```

* 创建MinMaxCurve曲线
```Rust
    let mut curve = FrameCurve::curve_minmax_curve(0.0, 1.0, 60);
    FrameCurve::curve_minmax_curve_frame(&mut curve, 0, 0.0, 2.0, 2.0);
    FrameCurve::curve_minmax_curve_frame(&mut curve, (frame_count/2) as FrameIndex, 0.5, 0.0, 0.0);
    FrameCurve::curve_minmax_curve_frame(&mut curve, frame_count as FrameIndex, 1.0, 2.0, 2.0);
```
