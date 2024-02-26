pub mod player;

use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_ggrs::{prelude::*, LocalPlayers};

use crate::core::anim::{sprite_sheet_animation_system, SpriteSheetAnimation};
use crate::core::ggrs::AddGgrsCoreAppExt;
use crate::core::levels::load_levels_system;
use crate::core::physics::{player_controller_system, PlayerController};
use crate::game::conf::{GameArgs, GameAssets, GameConfig, State};
use crate::game::game::player::input::input_system;
use crate::game::game::player::{player_level_follow_system, player_system, Player};

pub trait AddGameAppExt {
    fn add_game(&mut self, fps: usize) -> &mut Self;
}

impl AddGameAppExt for App {
    fn add_game(&mut self, fps: usize) -> &mut Self {
        self.add_ggrs::<GameConfig, _>(fps, input_system)
            .checksum_component_with_hash::<Player>()
            .rollback_component_with_clone::<Player>()
            //
            .add_systems(OnEnter(State::Game), setup)
            .add_systems(
                GgrsSchedule,
                ((
                    player_system,
                    player_level_follow_system,
                    //
                    load_levels_system,
                    player_controller_system,
                    sprite_sheet_animation_system,
                )
                    .run_if(in_state(State::Game)))
                .chain(),
            )
            .add_systems(OnExit(State::Game), cleanup)
    }
}

#[derive(Copy, Clone, Component)]
pub struct Game {}

fn setup(mut commands: Commands, args: Res<GameArgs>, texture_assets: Res<GameAssets>) {
    let game = Game {};

    commands.spawn((game, Camera2dBundle::default()));
    commands.spawn((
        game,
        LdtkWorldBundle {
            ldtk_handle: texture_assets.tileset_project.clone(),
            ..default()
        },
    ));

    for handle in 0..args.num_players {
        let transform = Transform::from_translation(Vec3::new((handle * 32) as f32, 1.0, 5.0));
        commands
            .spawn((
                game,
                Player { handle },
                PlayerController::default(),
                SpriteSheetBundle {
                    transform,
                    texture_atlas: texture_assets.player.clone(),
                    ..default()
                },
                SpriteSheetAnimation {
                    timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
                    start: 0,
                    finish: 3,
                },
            ))
            .add_rollback();
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Game>>) {
    commands.remove_resource::<LocalPlayers>();
    commands.remove_resource::<Session<GameConfig>>();

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn goto_game(commands: &mut Commands, next_state: &mut NextState<State>, session: Session<GameConfig>, local_players: LocalPlayers) {
    commands.insert_resource(session);
    commands.insert_resource(local_players);
    next_state.set(State::Game);
}
