use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_ggrs::prelude::*;

pub trait SpawnWithRollbackCommandsExt<'w, 's> {
    fn spawn_with_rollback<'a, T>(&'a mut self, bundle: T) -> EntityCommands<'w, 's, 'a>
    where
        T: Bundle;
}

impl<'w, 's> SpawnWithRollbackCommandsExt<'w, 's> for Commands<'w, 's> {
    fn spawn_with_rollback<'a, T>(&'a mut self, bundle: T) -> EntityCommands<'w, 's, 'a>
    where
        T: Bundle,
    {
        let mut e = self.spawn(bundle);
        e.add_rollback();
        e
    }
}
