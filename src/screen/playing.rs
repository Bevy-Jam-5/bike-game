//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};
use bevy_pipelines_ready::PipelinesReady;
use blenvy::{BlueprintInfo, BlueprintInstanceReady, GameWorldTag};
use ui_palette::LABEL_TEXT;

use super::{PlayState, Screen};
use crate::{
    game::{
        assets::SoundtrackKey,
        audio::soundtrack::PlaySoundtrack,
        spawn::{hud::SpawnHud, level::SpawnLevel},
    },
    util::single_mut,
};
use crate::{third_party::pipelines_ready::EXPECTED_PIPELINES, ui::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PlayState::Spawning), enter_spawning);
    app.add_systems(
        Update,
        update_spawning_text.run_if(in_state(PlayState::Spawning)),
    );
    app.add_systems(
        OnEnter(PlayState::LoadingPipelines),
        enter_loading_pipelines,
    );
    app.add_systems(
        Update,
        update_loading_pipelines_text.run_if(in_state(PlayState::LoadingPipelines)),
    );
    app.add_systems(OnEnter(PlayState::Active), enter_active);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_spawning(mut commands: Commands) {
    commands.trigger(SpawnLevel);
    commands
        .ui_root()
        .insert(StateScoped(PlayState::Spawning))
        .with_children(|children| {
            children.label("Spawning Level...");
            children.label("");
            children.label("This takes a while.");
            children.label("No worries, nothing crashed :)");
            children
                .spawn((
                    Name::new("Loading Indicator"),
                    NodeBundle {
                        style: Style {
                            width: Px(500.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|children| {
                    children.spawn((
                        Name::new("Loading Indicator Text"),
                        LoadingText,
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font_size: 24.0,
                                color: LABEL_TEXT,
                                ..default()
                            },
                        ),
                    ));
                });
        });
}

#[derive(Debug, Component)]
struct LoadingText;

fn update_spawning_text(
    mut q_text: Query<&mut Text, With<LoadingText>>,
    q_blueprints: Query<Has<BlueprintInstanceReady>, With<BlueprintInfo>>,
) {
    let mut text = single_mut!(q_text);
    let total = q_blueprints.iter().count();
    let ready = q_blueprints.iter().filter(|&ready| ready).count();
    let label = format!("{ready}/{total}");
    text.sections[0].value = label;
}

fn enter_loading_pipelines(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(PlayState::LoadingPipelines))
        .with_children(|children| {
            children.label("Loading graphics pipelines...");
            children
                .spawn((
                    Name::new("Loading Indicator"),
                    NodeBundle {
                        style: Style {
                            width: Px(500.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|children| {
                    children.spawn((
                        Name::new("Loading Indicator Text"),
                        LoadingText,
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font_size: 24.0,
                                color: LABEL_TEXT,
                                ..default()
                            },
                        ),
                    ));
                });
        });
}

fn update_loading_pipelines_text(
    mut q_text: Query<&mut Text, With<LoadingText>>,
    ready: Res<PipelinesReady>,
) {
    let mut text = single_mut!(q_text);
    let ready = ready.get();
    let total = EXPECTED_PIPELINES;
    let label = format!("{ready}/{total}");
    text.sections[0].value = label;
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn enter_active(mut commands: Commands, mut q_world: Query<&mut Visibility, With<GameWorldTag>>) {
    commands.trigger(SpawnHud);
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
    let mut visibility = single_mut!(q_world);
    *visibility = Visibility::Inherited;
}
