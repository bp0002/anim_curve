#![feature(test)]
extern crate test;

#[cfg(test)]
mod test_frame {

    use anim_curve::*;
    use test::Bencher;
    
    #[test]
    fn test_step() {
    
        let frame_count = 60;
        let mut curves = vec![];

        // for i in 0..1_000_000 {
            let mut key_frames = FrameCurve::curve_frame_values(60);
            FrameCurve::curve_frame_values_frame(&mut key_frames, 0, 0.0);
            FrameCurve::curve_frame_values_frame(&mut key_frames, 15 as FrameIndex, 1.0);
            FrameCurve::curve_frame_values_frame(&mut key_frames, 30 as FrameIndex, 3.0);
            FrameCurve::curve_frame_values_frame(&mut key_frames, 60 as FrameIndex, 4.0);

            curves.push(
                key_frames
            );
            
            for i in 0..frame_count {
                let v = curves[0].interple(i as KeyFrameCurveValue / 60., &AnimationAmountCalc::from_steps(1, EStepMode::JumpStart));
                println!("{:?}", v);
            }
        // }
        // b.iter(move || {
        //     let mut v = 0.;
        //     for i in 0..1_000_000 {
        //         v = v + curves.get(i).unwrap().interple(10.0, &AnimationAmountCalc::default());
        //     }
        // });
    }   

    #[bench]
    fn test_minmaxcurve_peformance(b: &mut Bencher) {
    
            let frame_count = 60 as FrameIndex;
            let mut curves = vec![];

        for _ in 0..1_000_000 {

            // MinMaxCurve
            let mut key_frames = FrameCurve::curve_minmax_curve(0.0, 1.0, 60);
            FrameCurve::curve_minmax_curve_frame(&mut key_frames, 0, 0.0, 2.0, 2.0);
            FrameCurve::curve_minmax_curve_frame(&mut key_frames, (frame_count/2) as FrameIndex, 0.5, 0.0, 0.0);
            FrameCurve::curve_minmax_curve_frame(&mut key_frames, frame_count as FrameIndex, 1.0, 2.0, 2.0);

            curves.push(
                key_frames
            );
        }
        b.iter(move || {
            let mut v = 0.;
            for i in 0..1_000_000 {
                v = v + curves.get(i).unwrap().interple(10.0, &AnimationAmountCalc::default());
            }
        });
    }
    
    #[bench]
    fn test_linear_peformance(b: &mut Bencher) {
    
            let frame_count = 60;
            let mut curves = vec![];

        for _ in 0..1_000_000 {
            let mut key_frames = FrameCurve::curve_frame_values(60);
            FrameCurve::curve_frame_values_frame(&mut key_frames, 0, 0.0f32);
            FrameCurve::curve_frame_values_frame(&mut key_frames, frame_count as FrameIndex, 1.0f32);

            curves.push(
                key_frames
            );
        }
        b.iter(move || {
            let mut v = 0.;
            for i in 0..1_000_000 {
                v = v + curves.get(i).unwrap().interple(10.0, &AnimationAmountCalc::default());
            }
        });
    }   
    #[bench]
    fn test_easing_peformance(b: &mut Bencher) {
    
            let frame_count = 60;
            let mut curves = vec![];

        for _ in 0..1_000_000 {
            let key_frames = FrameCurve::curve_easing(0.0, 1.0, frame_count as FrameIndex, frame_count, EEasingMode::None);

            curves.push(
                key_frames
            );
        }

        let mut vs = Vec::with_capacity(30000);
        let amount = 0.5;
        b.iter(|| {
            // for i in 0..1_000_000 {
            let mut v = 0.0;
            for i in 0..1_000_000 {
                let frames = curves.get(i).unwrap();

                v += frames.interple(amount, &AnimationAmountCalc::default());

            }
            vs.push(v);
        });
        println!("============= v.len() = {}", vs.len());
    }
    
    #[bench]
    fn test_steps_peformance(b: &mut Bencher) {
    
            let frame_count = 60;
            let mut curves = vec![];

        for _ in 0..1_000_000 {
            let mut key_frames = FrameCurve::curve_frame_values(60);
            FrameCurve::curve_frame_values_frame(&mut key_frames, 0, 0.0f32);
            FrameCurve::curve_frame_values_frame(&mut key_frames, 30, 0.2f32);
            FrameCurve::curve_frame_values_frame(&mut key_frames, frame_count as FrameIndex, 1.0f32);

            curves.push(
                key_frames
            );
        }
        b.iter(move || {
            let mut v = 0.;
            for i in 0..1_000_000 {
                v = v + curves.get(i).unwrap().interple(10.0, &AnimationAmountCalc::default());
            }
        });
    }   
}