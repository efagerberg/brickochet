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