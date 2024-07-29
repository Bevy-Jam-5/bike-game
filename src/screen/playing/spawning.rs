use bevy::{prelude::*, ui::Val::*};
use blenvy::{BlueprintInfo, BlueprintInstanceReady};
use ui_palette::LABEL_TEXT;

use super::{LoadingText, PlayState};
use crate::game::assets::FontHandles;
use crate::ui::prelude::*;
use crate::{game::spawn::level::SpawnLevel, util::single_mut};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PlayState::Spawning), enter_spawning);
    app.add_systems(
        Update,
        update_spawning_text.run_if(in_state(PlayState::Spawning)),
    );
}

fn enter_spawning(mut commands: Commands, fonts: Res<FontHandles>) {
    commands.trigger(SpawnLevel);

    commands
        .ui_root()
        .insert(StateScoped(PlayState::Spawning))
        .with_children(|children| {
            children.label("Spawning Level...", fonts.rubik_regular.clone_weak());
            children.label("", fonts.rubik_regular.clone_weak());
            children.label("This takes a while.", fonts.rubik_regular.clone_weak());
            children.label(
                "No worries, nothing crashed :)",
                fonts.rubik_regular.clone_weak(),
            );
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
                                font: fonts.rubik_regular.clone_weak(),
                                font_size: 24.0,
                                color: LABEL_TEXT,
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
    // Doing a +1 here because sometimes, we need to activate a hack
    // for Blenvy when it doesn't insert `BlueprintInstanceReady` on `World`
    // In the worst case, this can take 10 seconds to kick in,
    // during which the game has already spawned everything.
    // This would make the player very confused as to why nothing is happening even though the game is ready.
    // So let's pretend that there is one more blueprint than there actually is ;)
    let total = q_blueprints.iter().count() + 1;
    let ready = q_blueprints.iter().filter(|&ready| ready).count();
    let label = format!("{ready}/{total}");
    text.sections[0].value = label;
}
