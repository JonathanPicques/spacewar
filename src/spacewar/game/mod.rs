pub mod input;
pub mod player;
pub mod projectile;

use bevy::prelude::*;
use bevy_egui::egui::CollapsingHeader;
use bevy_egui::{egui, EguiContexts};
use bevy_ggrs::{prelude::*, LocalPlayers, RollbackFrameCount};
use rapier2d::dynamics::IntegrationParameters;
use rapier2d::geometry::InteractionGroups;

use crate::core::core_systems;
use crate::core::physics::body::{PhysicsBody, PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::core::physics::collider::{PhysicsCollider, PhysicsColliderOptions};
use crate::core::physics::*;
use crate::core::utilities::ggrs::SpawnWithRollbackCommandsExt;
use crate::core::utilities::hash::transform_hasher;
use crate::core::utilities::maths::*;
use crate::core::AddCoreAppExt;
use crate::spacewar::game::input::input_system;
use crate::spacewar::game::player::{player_system, Player, PlayerBundle, PlayerStats};
use crate::spacewar::game::projectile::Projectile;
use crate::spacewar::menu::menu_main::goto_main_menu;
use crate::spacewar::{GameArgs, GameAssets, GameConfig, Layer, State};

pub trait AddGameAppExt {
    fn add_game(&mut self, fps: usize) -> &mut Self;
}

impl AddGameAppExt for App {
    fn add_game(&mut self, fps: usize) -> &mut Self {
        self.add_core::<GameConfig, _>(fps, input_system)
            //
            .checksum_component::<Transform>(transform_hasher)
            .checksum_component_with_hash::<Game>()
            .checksum_component_with_hash::<Player>()
            .checksum_component_with_hash::<Projectile>()
            .checksum_component_with_hash::<PlayerStats>()
            //
            .rollback_component_with_copy::<Game>()
            .rollback_component_with_clone::<Player>()
            .rollback_component_with_clone::<Transform>()
            .rollback_component_with_clone::<Projectile>()
            .rollback_component_with_clone::<PlayerStats>()
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
                ((core_systems(), player_system).run_if(in_state(State::Game))).chain(),
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
        Camera2dBundle {
            transform: Transform::from_scale(Vec3::splat(0.5)),
            ..default()
        },
    ));

    commands.spawn_with_rollback((
        Game {},
        Transform::default()
            .with_rotation(0.0.to_bevy(Angle::Degrees))
            .with_translation(Vec3::new(0.0, -30.0, 0.0)),
        //
        PhysicsBody::Fixed,
        PhysicsCollider::Rectangle { width: 35.0, height: 1.0 },
        PhysicsColliderOptions::from_collision_groups(InteractionGroups {
            filter: Layer::Wall.into(),
            memberships: Layer::Wall.into(),
        }),
    ));
    commands.spawn_with_rollback((
        Game {},
        Transform::default()
            .with_rotation(0.0.to_bevy(Angle::Degrees))
            .with_translation(Vec3::new(150.0, 10.0, 0.0)),
        //
        PhysicsBody::Fixed,
        PhysicsCollider::Rectangle { width: 5.0, height: 5.0 },
        PhysicsColliderOptions::from_collision_groups(InteractionGroups {
            filter: Layer::Wall.into(),
            memberships: Layer::Wall.into(),
        }),
    ));
    commands.spawn_with_rollback((
        Game {},
        Transform::default()
            .with_rotation(20.0.to_bevy(Angle::Degrees))
            .with_translation(Vec3::new(150.0, -30.0, 0.0)),
        //
        PhysicsBody::Fixed,
        PhysicsCollider::Rectangle { width: 5.0, height: 5.0 },
        PhysicsColliderOptions::from_collision_groups(InteractionGroups {
            filter: Layer::Wall.into(),
            memberships: Layer::Wall.into(),
        }),
    ));
    commands.spawn_with_rollback((
        Game {},
        Transform::default()
            .with_rotation((-20.0).to_bevy(Angle::Degrees))
            .with_translation(Vec3::new(-100.0, -30.0, 0.0)),
        //
        PhysicsBody::Fixed,
        PhysicsCollider::Rectangle { width: 5.0, height: 5.0 },
        PhysicsColliderOptions::from_collision_groups(InteractionGroups {
            filter: Layer::Wall.into(),
            memberships: Layer::Wall.into(),
        }),
    ));
    commands.spawn_with_rollback((
        Game {},
        Transform::default()
            .with_rotation((20.0).to_bevy(Angle::Degrees))
            .with_translation(Vec3::new(-200.0, -35.0, 0.0)),
        //
        PhysicsBody::Fixed,
        PhysicsCollider::Rectangle { width: 5.0, height: 5.0 },
        PhysicsColliderOptions::from_collision_groups(InteractionGroups {
            filter: Layer::Wall.into(),
            memberships: Layer::Wall.into(),
        }),
    ));

    commands.spawn_with_rollback((
        Game {},
        Transform::default()
            .with_rotation((20.0).to_bevy(Angle::Degrees))
            .with_translation(Vec3::new(0.0, 55.0, 0.0)),
        //
        PhysicsBody::Dynamic,
        PhysicsBodyOptions::from_gravity_scale(0.0),
        PhysicsBodyVelocity {
            linear_velocity: Some(Vec2::new(0.0, 0.0)),
            angular_velocity: Some(20.0_f32.to_radians()),
        },
        //
        PhysicsCollider::Rectangle { width: 1.0, height: 1.0 },
        PhysicsColliderOptions {
            restitution: 1.0,
            collision_groups: InteractionGroups {
                filter: Layer::Wall.into(),
                memberships: Layer::Wall.into(),
            },
            ..default()
        },
    ));

    for handle in 0..game_args.num_players {
        commands.spawn_with_rollback(PlayerBundle::new(handle, &game_assets));
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
        commands.entity(e).despawn_recursive();
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
