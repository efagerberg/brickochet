use bevy::prelude::*;

use crate::gameplay::brick::key_frames;
use test_case::test_case;

struct SampleKeyFramesCase<T> {
    key_frames: Vec<key_frames::KeyFrame<T>>,
    t: f32,
    expected: Result<T, key_frames::SampleKeyFramesError>,
}

#[test_case(
    SampleKeyFramesCase::<f32> {
        key_frames: vec![],
        t: 0.0,
        expected: Err(key_frames::SampleKeyFramesError::NotStarted),
    };
    "empty key_frames"
)]
#[test_case(
    SampleKeyFramesCase {
        key_frames: vec![
            key_frames::KeyFrame { t: 0.0, value: 0.0, interp: key_frames::Interpolation::Constant },
            key_frames::KeyFrame { t: 1.0, value: 10.0, interp: key_frames::Interpolation::Constant },
        ],
        t: -0.5,
        expected: Err(key_frames::SampleKeyFramesError::NotStarted),
    };
    "before first key frame"
)]
#[test_case(
    SampleKeyFramesCase {
        key_frames: vec![
            key_frames::KeyFrame { t: 0.0, value: 0.0, interp: key_frames::Interpolation::Constant },
            key_frames::KeyFrame { t: 1.0, value: 10.0, interp: key_frames::Interpolation::Constant },
        ],
        t: 0.5,
        expected: Ok(0.0),
    };
    "between constant key frames"
)]
#[test_case(
    SampleKeyFramesCase {
        key_frames: vec![
            key_frames::KeyFrame { t: 0.0, value: 0.0, interp: key_frames::Interpolation::Linear },
            key_frames::KeyFrame { t: 1.0, value: 10.0, interp: key_frames::Interpolation::Linear },
        ],
        t: 0.5,
        expected: Ok(5.0),
    };
    "between linear key frames"
)]
#[test_case(
    SampleKeyFramesCase {
        key_frames: vec![
            key_frames::KeyFrame::<Vec2> { t: 0.0, value: Vec2::new(0.0, 0.0), interp: key_frames::Interpolation::Linear },
            key_frames::KeyFrame::<Vec2> { t: 1.0, value: Vec2::new(10.0, 10.0), interp: key_frames::Interpolation::Linear },
        ],
        t: 0.5,
        expected: Ok(Vec2::new(5.0, 5.0)),
    };
    "between linear Vec2 key frames"
)]
#[test_case(
    SampleKeyFramesCase {
        key_frames: vec![
            key_frames::KeyFrame::<Vec3> { t: 0.0, value: Vec3::new(0.0, 0.0, 0.0), interp: key_frames::Interpolation::Linear },
            key_frames::KeyFrame::<Vec3> { t: 1.0, value: Vec3::new(10.0, 10.0, 5.0), interp: key_frames::Interpolation::Linear },
        ],
        t: 0.5,
        expected: Ok(Vec3::new(5.0, 5.0, 2.5)),
    };
    "between linear Vec3 key frames"
)]
#[test_case(
    SampleKeyFramesCase {
        key_frames: vec![
            key_frames::KeyFrame { t: 0.0, value: 0.0, interp: key_frames::Interpolation::Constant },
            key_frames::KeyFrame { t: 1.0, value: 10.0, interp: key_frames::Interpolation::Constant },
        ],
        t: 1.5,
        expected: Err(key_frames::SampleKeyFramesError::Finished),
    };
    "after last key frame"
)]
fn test_sample_key_frames<T>(case: SampleKeyFramesCase<T>) where T: key_frames::Lerp + Copy + PartialEq + std::fmt::Debug {
    let result = key_frames::sample_key_frames(&case.key_frames, case.t);
    assert_eq!(result, case.expected);
}
