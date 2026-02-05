use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: u8,
    pub current: u8,
}

#[derive(Component)]
pub struct HealthColors {
    pub max: LinearRgba,
    pub min: LinearRgba,
}

#[derive(Clone)]
pub enum Affects {
    SelfOnly,
    Others(Vec<Entity>),
    #[allow(dead_code)]
    SelfAndOthers(Vec<Entity>),
}

#[derive(Component)]
pub struct ChangeOnCollision {
    pub delta: i16,
    pub affected: Affects,
}

impl ChangeOnCollision {
    pub fn affected_entities(&self, entity: Entity) -> Box<dyn Iterator<Item = Entity> + '_> {
        match &self.affected {
            Affects::SelfOnly => Box::new(std::iter::once(entity)),
            Affects::Others(others) => Box::new(others.iter().copied()),
            Affects::SelfAndOthers(others) => {
                Box::new(std::iter::once(entity).chain(others.iter().copied()))
            }
        }
    }
}
