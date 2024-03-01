use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_ggrs::prelude::*;

pub trait SpawnWithRollbackCommandsExt {
    fn spawn_with_rollback<T>(&mut self, bundle: T) -> EntityCommands<'_>
    where
        T: Bundle;
}

impl<'w, 's> SpawnWithRollbackCommandsExt for Commands<'w, 's> {
    fn spawn_with_rollback<T>(&mut self, bundle: T) -> EntityCommands<'_>
    where
        T: Bundle,
    {
        let mut e = self.spawn(bundle);
        e.add_rollback();
        e
    }
}
