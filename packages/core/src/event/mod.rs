pub mod events;

use bevy::prelude::*;
use bevy_ggrs::RollbackApp;

use crate::event::events::{RollbackEvent, RollbackEvents};

pub trait RollbackEventAppExt {
    fn rollback_events<E>(&mut self) -> &mut Self
    where
        E: RollbackEvent;
}

impl RollbackEventAppExt for App {
    fn rollback_events<E>(&mut self) -> &mut Self
    where
        E: RollbackEvent,
    {
        self.insert_resource(RollbackEvents::<E>::default())
            .checksum_resource_with_hash::<RollbackEvents<E>>()
            .rollback_resource_with_clone::<RollbackEvents<E>>()
    }
}
