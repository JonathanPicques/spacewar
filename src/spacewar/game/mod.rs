pub mod player;

use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelIid};
use bevy_egui::egui::CollapsingHeader;
use bevy_egui::{egui, EguiContexts};
use bevy_ggrs::{prelude::*, LocalPlayers};

use crate::core::anim::{sprite_sheet_animator_system, SpriteSheetAnimator};
use crate::core::levels::{load_levels_system, LoadedLevels};
use crate::core::physics::*;
use crate::core::utilities::ggrs::SpawnWithRollbackCommandsExt;
use crate::core::utilities::maths::*;
use crate::core::AddCoreAppExt;
use crate::spacewar::conf::{GameArgs, GameAssets, GameConfig, State};
use crate::spacewar::game::player::input::input_system;
use crate::spacewar::game::player::{player_level_follow_system, player_system, Player};
use crate::spacewar::menu::menu_main::goto_main_menu;

pub trait AddGameAppExt {
    fn add_game(&mut self, fps: usize) -> &mut Self;
}

impl AddGameAppExt for App {
    fn add_game(&mut self, fps: usize) -> &mut Self {
        self.add_core::<GameConfig, _>(fps, input_system)
            .checksum_component_with_hash::<Player>()
            .rollback_component_with_clone::<Player>()
            //
            .add_systems(OnEnter(State::Game), setup)
            .add_systems(
                Update,
                (update, physics_debug_system).run_if(in_state(State::Game)),
            )
            .add_systems(OnExit(State::Game), cleanup)
            //
            .add_systems(
                GgrsSchedule,
                ((
                    player_system,
                    player_level_follow_system,
                    //
                    load_levels_system,
                    sprite_sheet_animator_system,
                    //
                    physics_create_handles_system,
                    physics_system,
                    physics_sync_system,
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
    game_assets: Res<GameAssets>,
) {
    commands.insert_resource(Physics::default());
    commands.insert_resource(LoadedLevels::new(LevelIid::new(
        "a2a50ff0-66b0-11ec-9cd7-c721746049b9",
    )));

    commands.spawn((
        Game {},
        Camera2dBundle {
            transform: Transform::from_scale(Vec3::splat(0.5)),
            ..default()
        },
    ));
    commands.spawn((
        Game {},
        LdtkWorldBundle {
            ldtk_handle: game_assets.tileset_project.clone(),
            ..default()
        },
    ));
    commands.spawn_with_rollback((
        Game {},
        Transform::default()
            .with_rotation(0.0.to_bevy(Angle::Degrees))
            .with_translation(Vec3::new(0.0, -30.0, 0.0)),
        PhysicsBody::Fixed,
        PhysicsCollider { width: 35.0, height: 1.0 },
    ));
    commands.spawn_with_rollback((
        Game {},
        Transform::default()
            .with_rotation(0.0.to_bevy(Angle::Degrees))
            .with_translation(Vec3::new(150.0, 10.0, 0.0)),
        PhysicsBody::Fixed,
        PhysicsCollider { width: 5.0, height: 5.0 },
    ));
    commands.spawn_with_rollback((
        Game {},
        Transform::default()
            .with_rotation(20.0.to_bevy(Angle::Degrees))
            .with_translation(Vec3::new(150.0, -30.0, 0.0)),
        PhysicsBody::Fixed,
        PhysicsCollider { width: 5.0, height: 5.0 },
    ));
    commands.spawn_with_rollback((
        Game {},
        Transform::default()
            .with_rotation((-20.0).to_bevy(Angle::Degrees))
            .with_translation(Vec3::new(-100.0, -30.0, 0.0)),
        PhysicsBody::Fixed,
        PhysicsCollider { width: 5.0, height: 5.0 },
    ));
    commands.spawn_with_rollback((
        Game {},
        Transform::default()
            .with_rotation((20.0).to_bevy(Angle::Degrees))
            .with_translation(Vec3::new(-200.0, -35.0, 0.0)),
        PhysicsBody::Fixed,
        PhysicsCollider { width: 5.0, height: 5.0 },
    ));

    for handle in 0..args.num_players {
        commands.spawn_with_rollback((
            Game {},
            Player { handle, ..default() },
            //
            PhysicsBody::KinematicPositionBased,
            PhysicsCollider { width: 0.8, height: 1.8 },
            PhysicsCharacterController::default(),
            //
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    anchor: Anchor::Custom(Vec2::new(0.0, -0.25)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new((handle * 32) as f32, 1.0, 5.0)),
                texture_atlas: game_assets.player.clone(),
                ..default()
            },
            SpriteSheetAnimator {
                timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
                animation: game_assets.player_idle_anim.clone(),
            },
        ));
    }
}

fn update(
    mut contexts: EguiContexts,
    //
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
                ui.label(format!("{}", checksum.0));
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
    commands.remove_resource::<LoadedLevels>();
    commands.remove_resource::<LocalPlayers>();
    commands.remove_resource::<Session<GameConfig>>();

    // https://github.com/gschup/bevy_ggrs/issues/93
    commands.insert_resource(Time::new_with(GgrsTime));

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn goto_game(
    commands: &mut Commands,
    next_state: &mut NextState<State>,
    //
    session: Session<GameConfig>,
    local_players: LocalPlayers,
) {
    commands.insert_resource(session);
    commands.insert_resource(local_players);
    next_state.set(State::Game);
}
