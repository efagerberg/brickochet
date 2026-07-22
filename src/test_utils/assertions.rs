use assert_float_eq;
use bevy::prelude::*;

pub fn assert_messages<T>(app: &App, expected: &[T])
where
    T: Message + std::fmt::Debug + PartialEq + Copy,
{
    let messages = app.world().resource::<Messages<T>>();
    let mut cursor = messages.get_cursor();

    let actual: Vec<T> = cursor.read(messages).copied().collect();

    assert_eq!(actual, expected);
}

pub fn assert_message_count<T>(app: &App, expected_count: usize)
where
    T: Message,
{
    let messages = app.world().resource::<Messages<T>>();
    let mut cursor = messages.get_cursor();

    let actual_count = cursor.read(messages).count();

    assert_eq!(actual_count, expected_count);
}

pub fn assert_floats_eq(
    actual: impl IntoIterator<Item = f32>,
    expected: impl IntoIterator<Item = f32>,
) {
    let actual: Vec<f32> = actual.into_iter().collect();
    let expected: Vec<f32> = expected.into_iter().collect();

    assert_eq!(
        actual.len(),
        expected.len(),
        "length mismatch: {} vs {}",
        actual.len(),
        expected.len()
    );

    for ((x, y)) in actual.into_iter().zip(expected) {
        assert_float_eq::assert_float_relative_eq!(x, y);
    }
}
pub fn assert_vec3_eq(actual: Vec3, expected: Vec3) {
    assert_floats_eq(actual.to_array(), expected.to_array());
}

pub fn assert_vec2_eq(actual: Vec2, expected: Vec2) {
    assert_floats_eq(actual.to_array(), expected.to_array());
}
