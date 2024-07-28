use bevy::{prelude::*, ui::Val::*};
use blenvy::{BlueprintInfo, BlueprintInstanceReady};
use ui_palette::LABEL_TEXT;

use super::{LoadingText, PlayState};
use crate::ui::prelude::*;
use crate::{
    game::spawn::{level::SpawnLevel},
    util::single_mut,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PlayState::Spawning), enter_spawning);
    app.add_systems(
        Update,
        update_spawning_text.run_if(in_state(PlayState::Spawning)),
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
