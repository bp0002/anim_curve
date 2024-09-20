//!  实现 hermite 曲线相关函数
use crate::types::KeyFrameCurveValue;

pub fn hermite(value1: KeyFrameCurveValue, tangent1: KeyFrameCurveValue, value2: KeyFrameCurveValue, tangent2: KeyFrameCurveValue, amount: KeyFrameCurveValue) -> KeyFrameCurveValue {
    let _1 = 1.;
    let _2 = 2.;
    let _3 = 3.;

    let squared = amount * amount;
    let cubed = amount * squared;
    let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
    let part2 = (-_2 * cubed) + (_3 * squared);
    let part3 = (cubed - (_2 * squared)) + amount;
    let part4 = cubed - squared;

    return (((value1 * part1) + (value2 * part2)) + (tangent1 * part3)) + (tangent2 * part4);
}