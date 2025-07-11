pub mod input;
pub mod level;
pub mod player;
pub mod projectile;

use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;
use bevy_egui::egui::CollapsingHeader;
use bevy_egui::{egui, EguiContexts};
use bevy_ggrs::{prelude::*, LocalPlayers, RollbackFrameCount};
use rapier2d::dynamics::IntegrationParameters;

use core::core_systems;
use core::event::RollbackEventAppExt;
use core::physics::collider::PhysicsCollider;
use core::physics::*;
use core::utilities::ggrs::SpawnWithRollbackCommandsExt;
use core::utilities::hash::transform_hasher;
use core::utilities::maths::RotationAngle;
use core::utilities::sound::StopSoundInBackgroundAppExt;
use core::AddCoreAppExt;

use crate::game::input::input_system;
use crate::game::level::LevelRectBundle;
use crate::game::player::*;
use crate::game::projectile::bullet::*;
use crate::game::projectile::grenade::*;
use crate::menu::menu_main::goto_main_menu;
use crate::{GameArgs, GameAssets, GameConfig, State};

pub trait AddGameAppExt {
    fn add_game(&mut self, fps: usize) -> &mut Self;
}

impl AddGameAppExt for App {
    fn add_game(&mut self, fps: usize) -> &mut Self {
        self.add_core::<GameConfig, _>(fps, input_system)
            .stop_sounds_in_background()
            //
            .rollback_events::<DamageEvent>()
            //
            .checksum_component::<Transform>(transform_hasher)
            .checksum_component_with_hash::<Game>()
            .checksum_component_with_hash::<Stats>()
            .checksum_component_with_hash::<Bullet>()
            .checksum_component_with_hash::<Health>()
            .checksum_component_with_hash::<Player>()
            .checksum_component_with_hash::<Grenade>()
            //
            .rollback_component_with_copy::<Game>()
            .rollback_component_with_copy::<Stats>()
            .rollback_component_with_copy::<Bullet>()
            .rollback_component_with_copy::<Health>()
            .rollback_component_with_copy::<Player>()
            .rollback_component_with_copy::<Grenade>()
            .rollback_component_with_clone::<Transform>()
            //
            .add_systems(OnEnter(State::Game), setup)
            .add_systems(
                Update,
                (update, physics_debug_systems()).run_if(in_state(State::Game)),
            )
            .add_systems(OnExit(State::Game), cleanup)
            //
            .add_systems(
                GgrsSchedule,
                ((
                    core_systems(),
                    player_system,
                    bullet_system,
                    grenade_system,
                    grenade_fuse_system,
                )
                    .run_if(in_state(State::Game)))
                .chain(),
            )
    }
}

#[derive(Hash, Copy, Clone, Default, Component)]
pub struct Game {}

fn setup(
    mut commands: Commands,
    //
    game_args: Res<GameArgs>,
    game_assets: Res<GameAssets>,
) {
    commands.spawn((
        Game {},
        Camera2d {},
        Transform::from_scale(Vec3::splat(0.5)),
        AudioPlayer::new(game_assets.background_music.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(0.5),
            ..default()
        },
    ));

    // Level
    {
        commands.spawn_with_rollback(LevelRectBundle::new(
            PhysicsCollider::Rectangle { width: 160.0, height: 10.0 },
            RotationAngle::Degrees(0.0),
            Vec3::new(0.0, 50.0, 0.0),
        ));
        commands.spawn_with_rollback(LevelRectBundle::new(
            PhysicsCollider::Rectangle { width: 160.0, height: 10.0 },
            RotationAngle::Degrees(0.0),
            Vec3::new(0.0, -50.0, 0.0),
        ));
        commands.spawn_with_rollback(LevelRectBundle::new(
            PhysicsCollider::Rectangle { width: 10.0, height: 90.0 },
            RotationAngle::Degrees(0.0),
            Vec3::new(80.0, 0.0, 0.0),
        ));
        commands.spawn_with_rollback(LevelRectBundle::new(
            PhysicsCollider::Rectangle { width: 10.0, height: 90.0 },
            RotationAngle::Degrees(0.0),
            Vec3::new(-80.0, 0.0, 0.0),
        ));
    }

    for handle in 0..game_args.num_players {
        commands.spawn_with_rollback(PlayerBundle::new(
            handle,
            &game_args,
            &game_assets,
        ));
    }
}

fn update(
    mut contexts: EguiContexts,
    //
    frame: Res<RollbackFrameCount>,
    checksum: Res<Checksum>,
    mut game_args: ResMut<GameArgs>,
    mut next_state: ResMut<NextState<State>>,
) {
    egui::Window::new("Debugger").show(contexts.ctx_mut(), |ui| {
        CollapsingHeader::new("Input")
            .default_open(true)
            .show(ui, |ui| {
                ui.checkbox(&mut game_args.randomize_input, "Randomize input");
            });
        CollapsingHeader::new("Checksum")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("Frame {}", frame.0));
                ui.label(format!("Checksum {}", checksum.0));
            });

        if ui.button("Back to main menu").clicked() {
            goto_main_menu(&mut next_state);
        }
    });
}

fn cleanup(
    mut commands: Commands,
    //
    query: Query<Entity, With<Game>>,
) {
    commands.remove_resource::<Physics>();
    commands.remove_resource::<LocalPlayers>();
    commands.remove_resource::<Session<GameConfig>>();

    // https://github.com/gschup/bevy_ggrs/issues/93
    commands.insert_resource(Time::new_with(GgrsTime));

    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn goto_game(
    commands: &mut Commands,
    next_state: &mut NextState<State>,
    //
    args: &GameArgs,
    session: Session<GameConfig>,
    local_players: LocalPlayers,
) {
    let fps = args.fps as f32;

    commands.insert_resource(session);
    commands.insert_resource(local_players);
    commands.insert_resource(Scaler::default());
    commands.insert_resource(Physics {
        integration_parameters: IntegrationParameters {
            dt: 1.0 / fps,
            min_ccd_dt: 1.0 / fps / 100.0,
            ..default()
        },
        ..default()
    });
    next_state.set(State::Game);
}
