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
