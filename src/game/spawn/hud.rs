//! Spawn the HUD by triggering other observers.

use bevy::prelude::*;

use crate::{
    game::{assets::FontHandles, game_end::GAME_END_MONEY, quest::advance_quest::ActiveQuest},
    screen::Screen,
    util::single_mut,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_hud);
    app.add_systems(
        Update,
        update_active_quests.run_if(
            resource_exists_and_changed::<ActiveQuest>.or_else(resource_removed::<ActiveQuest>()),
        ),
    );
}

#[derive(Event, Debug)]
pub struct SpawnHud;

#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct TimeText;

#[derive(Component)]
pub struct ActiveQuestUiContainer;

#[derive(Component)]
pub struct ActiveQuestText;

fn spawn_hud(_trigger: Trigger<SpawnHud>, mut commands: Commands, fonts: Res<FontHandles>) {
    use Val::*;

    let text_style = TextStyle {
        font: fonts.rubik_regular.clone_weak(),
        font_size: 24.0,
        color: Color::WHITE,
    };

    commands
        .spawn((
            Name::new("HUD"),
            NodeBundle {
                background_color: Color::srgba(0.0, 0.0, 0.1, 0.7).into(),
                style: Style {
                    width: Px(240.0),
                    padding: UiRect::all(Px(10.0)),
                    top: Px(0.0),
                    left: Px(0.0),
                    row_gap: Px(10.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                border_radius: BorderRadius::bottom_right(Px(10.0)),
                ..default()
            },
            StateScoped(Screen::Playing),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("Money text"),
                MoneyText,
                TextBundle::from_sections([
                    TextSection::new("Money: ", text_style.clone()),
                    TextSection::new(format!("$0 out of ${}", GAME_END_MONEY), text_style.clone()),
                ]),
            ));
            children.spawn((
                Name::new("Time text"),
                TimeText,
                TextBundle::from_sections([
                    TextSection::new("Time: ", text_style.clone()),
                    TextSection::new("00:00", text_style.clone()),
                ]),
            ));
        });

    commands
        .spawn((
            Name::new("Active quests"),
            ActiveQuestUiContainer,
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Px(10.0)),
                    position_type: PositionType::Absolute,
                    top: Px(0.0),
                    right: Px(0.0),
                    row_gap: Px(10.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::End,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.1, 0.7).into(),
                visibility: Visibility::Hidden,
                border_radius: BorderRadius::bottom_left(Px(10.0)),
                ..default()
            },
            StateScoped(Screen::Playing),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("Active quest header"),
                TextBundle::from_section(
                    "Active Quest",
                    TextStyle {
                        font: fonts.rubik_bold.clone_weak(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                ),
            ));
            children.spawn((
                Name::new("Quest divider"),
                NodeBundle {
                    style: Style {
                        width: Px(175.0),
                        height: Px(5.0),
                        display: Display::Block,
                        ..default()
                    },
                    border_radius: BorderRadius::all(Percent(100.0)),
                    background_color: Color::srgb(0.05, 0.95, 1.0).into(),
                    ..default()
                },
            ));
            children.spawn((
                Name::new("Active quest text"),
                ActiveQuestText,
                TextBundle::from_section("", text_style),
            ));
        });
}

fn update_active_quests(
    mut q_active_quest_container: Query<&mut Visibility, With<ActiveQuestUiContainer>>,
    mut q_active_quest_text: Query<&mut Text, With<ActiveQuestText>>,
    active_quest: Option<Res<ActiveQuest>>,
) {
    let mut container_visibility = single_mut!(q_active_quest_container);
    for mut text in &mut q_active_quest_text {
        if let Some(stage) = active_quest.as_ref().and_then(|quest| quest.history.last()) {
            text.sections[0].value = stage.place.description();
            *container_visibility = Visibility::Inherited;
        } else {
            text.sections[0].value = String::new();
            *container_visibility = Visibility::Hidden;
        }
    }
}
