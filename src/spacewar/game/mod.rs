pub mod player;

use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelIid};
use bevy_egui::{egui, EguiContexts};
use bevy_ggrs::{prelude::*, LocalPlayers};

use crate::core::anim::{sprite_sheet_animation_system, SpriteSheetAnimation};
use crate::core::ggrs::AddGgrsCoreAppExt;
use crate::core::levels::{load_levels_system, LoadedLevels};
use crate::core::physics::{player_controller_system, PlayerController};
use crate::spacewar::conf::{GameArgs, GameAssets, GameConfig, State};
use crate::spacewar::game::player::input::input_system;
use crate::spacewar::game::player::{player_level_follow_system, player_system, Player};
use crate::spacewar::menu::menu_main::goto_main_menu;

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
            .add_systems(Update, update.run_if(in_state(State::Game)))
            .add_systems(OnExit(State::Game), cleanup)
            //
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
    }
}

#[derive(Copy, Clone, Component)]
pub struct Game {}

fn setup(
    mut commands: Commands,
    //
    args: Res<GameArgs>,
    texture_assets: Res<GameAssets>,
) {
    commands.insert_resource(LoadedLevels::new(LevelIid::new(
        "a2a50ff0-66b0-11ec-9cd7-c721746049b9",
    )));

    commands.spawn((
        Game {},
        Camera2dBundle {
            global_transform: GlobalTransform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
    ));
    commands.spawn((
        Game {},
        LdtkWorldBundle {
            ldtk_handle: texture_assets.tileset_project.clone(),
            ..default()
        },
    ));

    for handle in 0..args.num_players {
        let transform = Transform::from_translation(Vec3::new((handle * 32) as f32, 1.0, 5.0));
        commands
            .spawn((
                Game {},
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

fn update(
    mut contexts: EguiContexts,
    //
    mut next_state: ResMut<NextState<State>>,
) {
    egui::Window::new("Menu").show(contexts.ctx_mut(), |ui| {
        if ui.button("Back").clicked() {
            goto_main_menu(&mut next_state);
        }
    });
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Game>>) {
    commands.remove_resource::<LoadedLevels>();
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
