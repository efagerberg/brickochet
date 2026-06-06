use bevy::prelude::*;
use serde;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub enum Interpolation {
    Constant, // hold value until next keyframe (step)
    Linear,   // linear between keys
}

#[derive(Clone, serde::Deserialize)]
pub struct KeyFrame<T> {
    pub t: f32,
    pub value: T,
    pub interp: Interpolation,
}

pub enum NextKeyFrameError {
    NotStarted,
    Finished,
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
) -> Result<V, NextKeyFrameError> {
    if key_frames.is_empty() {
        return Err(NextKeyFrameError::NotStarted);
    }
    if t < key_frames[0].t {
        return Err(NextKeyFrameError::NotStarted);
    }
    if t >= key_frames.last().unwrap().t {
        return Err(NextKeyFrameError::Finished);
    }

    let right_idx = key_frames.iter().position(|kf| kf.t > t).unwrap();
    let left_idx = right_idx - 1;
    let left = &key_frames[left_idx];
    let right = &key_frames[right_idx];

    match left.interp {
        Interpolation::Constant => Ok(left.value),
        Interpolation::Linear => {
            let span = right.t - left.t;
            let u = if span <= 0.0 {
                0.0
            } else {
                (t - left.t) / span
            };
            Ok(V::lerp(left.value, right.value, u))
        }
    }
}
