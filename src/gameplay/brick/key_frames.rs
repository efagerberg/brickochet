use bevy::prelude::*;
use serde;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub enum Interpolation {
    Constant, // hold value until next key frame (step)
    Linear,   // linear between keys
}

#[derive(Clone, serde::Deserialize)]
pub struct KeyFrame<T> {
    pub t: f32,
    pub value: T,
    pub interp: Interpolation,
}

#[derive(Debug, PartialEq)]
pub enum SampleKeyFramesError {
    NotStarted,
    NoKeyFrames,
}

#[derive(Debug, PartialEq)]
pub enum SampleKeyFrameValue<V> {
    InProgress(V),
    Complete(V), // final keyframe reached/exceeded — sample and then remove
}

// generic sampler for payloads that implement a simple lerp operation
pub trait Lerp: Copy {
    fn lerp(a: Self, b: Self, u: f32) -> Self;
}
impl Lerp for f32 {
    fn lerp(a: Self, b: Self, u: f32) -> Self {
        a + (b - a) * u
    }
}
impl Lerp for bevy::prelude::Vec2 {
    fn lerp(a: Self, b: Self, u: f32) -> Self {
        a + (b - a) * u
    }
}
impl Lerp for bevy::prelude::Vec3 {
    fn lerp(a: Self, b: Self, u: f32) -> Self {
        a + (b - a) * u
    }
}

pub fn sample_key_frames<V: Lerp + Copy>(
    key_frames: &[KeyFrame<V>],
    t: f32,
) -> Result<SampleKeyFrameValue<V>, SampleKeyFramesError> {
    if key_frames.is_empty() {
        return Err(SampleKeyFramesError::NoKeyFrames);
    }

    if t < key_frames[0].t {
        return Err(SampleKeyFramesError::NotStarted);
    }

    let clamped_t = t.min(1.0);
    let complete = clamped_t == 1.0;

    let left: &KeyFrame<V>;
    let right: &KeyFrame<V>;
    let value: V;
    if let Some(right_idx) = key_frames.iter().position(|kf| kf.t > clamped_t) {
        let left_idx = right_idx - 1;
        left = &key_frames[left_idx];
        right = &key_frames[right_idx];
        value = match left.interp {
            Interpolation::Constant => left.value,
            Interpolation::Linear => {
                let span = right.t - left.t;
                let u = (clamped_t - left.t) / span;
                V::lerp(left.value, right.value, u)
            }
        };
    } else {
        value = key_frames.last().unwrap().value;
    }

    Ok(if complete {
        SampleKeyFrameValue::Complete(value)
    } else {
        SampleKeyFrameValue::InProgress(value)
    })
}
