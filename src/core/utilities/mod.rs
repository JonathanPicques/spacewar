use std::marker::PhantomData;

use bevy::prelude::*;
use clap::Parser;

pub struct ArgsPlugin<T>
where
    T: Parser + Resource,
{
    phantom: PhantomData<T>,
}

impl<T> Plugin for ArgsPlugin<T>
where
    T: Parser + Resource,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(T::parse());
    }
}

impl<T> Default for ArgsPlugin<T>
where
    T: Parser + Resource,
{
    fn default() -> Self {
        Self { phantom: Default::default() }
    }
}
