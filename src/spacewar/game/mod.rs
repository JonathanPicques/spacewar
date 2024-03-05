pub mod input;
pub mod player;

use bevy::prelude::*;
use bevy_egui::egui::CollapsingHeader;
use bevy_egui::{egui, EguiContexts};
use bevy_ggrs::{prelude::*, LocalPlayers};

use crate::core::anim::sprite_sheet_animator_system;
use crate::core::clock::{ttl_system, TimeToLive};
use crate::core::frame::Frame;
use crate::core::physics::*;
use crate::core::utilities::ggrs::SpawnWithRollbackCommandsExt;
use crate::core::utilities::hash::transform_hasher;
use crate::core::utilities::maths::*;
use crate::core::AddCoreAppExt;
use crate::spacewar::game::input::input_system;
use crate::spacewar::game::player::{player_system, Player};
use crate::spacewar::menu::menu_main::goto_main_menu;
use crate::spacewar::{GameArgs, GameConfig, State};

pub trait AddGameAppExt {
    fn add_game(&mut self, fps: usize) -> &mut Self;
}

impl AddGameAppExt for App {
    fn add_game(&mut self, fps: usize) -> &mut Self {
        self.add_core::<GameConfig, _>(fps, input_system)
            .checksum_component::<Transform>(transform_hasher)
            .checksum_component_with_hash::<Player>()
            .rollback_component_with_clone::<Player>()
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
                    player_system,
                    //
                    ttl_system,
                    sprite_sheet_animator_system,
                    //
                    physics_systems(),
                )
                    .run_if(in_state(State::Game)))
                .chain(),
            )
    }
}

#[derive(Copy, Clone, Component)]
pub struct Game {}

fn setup(mut commands: Commands) {
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
            .with_translation(Vec3::new(0.0, 0.0, 0.0)),
        TimeToLive::from_secs_f32(1.0),
        //
        PhysicsBody::Dynamic,
        PhysicsCollider::Rectangle { width: 5.0, height: 5.0 },
    ));
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
    commands.insert_resource(Frame::default());
    commands.insert_resource(Physics::default());
    commands.insert_resource(session);
    commands.insert_resource(local_players);
    next_state.set(State::Game);
}
