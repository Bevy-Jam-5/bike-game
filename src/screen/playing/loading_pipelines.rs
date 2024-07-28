use bevy::{prelude::*, ui::Val::*};
use bevy_pipelines_ready::PipelinesReady;
use ui_palette::LABEL_TEXT;

use super::{LoadingText, PlayState};
use crate::util::single_mut;
use crate::{third_party::pipelines_ready::EXPECTED_PIPELINES, ui::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(PlayState::LoadingPipelines),
        enter_loading_pipelines,
    );
    app.add_systems(
        Update,
        update_loading_pipelines_text.run_if(in_state(PlayState::LoadingPipelines)),
    );
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
