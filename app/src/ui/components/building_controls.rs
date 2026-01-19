use bevy::prelude::*;
use crate::game::components::StagedBuilding;

/// UI component for controlling building stages
#[derive(Component)]
pub struct BuildingControlsUI;

/// Marker component for upgrade button
#[derive(Component)]
pub struct UpgradeButton;

/// Marker component for downgrade button
#[derive(Component)]
pub struct DowngradeButton;

/// Marker component for stage display text
#[derive(Component)]
pub struct StageDisplayText;

/// System to spawn the building controls UI
pub fn spawn_building_controls(mut commands: Commands) {
    commands
        .spawn((
            BuildingControlsUI,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Town Hall Controls",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Stage display
            parent.spawn((
                StageDisplayText,
                TextBundle::from_section(
                    "Stage: 4",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                        ..default()
                    },
                ),
            ));

            // Upgrade button
            parent
                .spawn((
                    UpgradeButton,
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                        ..default()
                    },
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Upgrade Stage",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            // Downgrade button
            parent
                .spawn((
                    DowngradeButton,
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(0.6, 0.2, 0.2)),
                        ..default()
                    },
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Downgrade Stage",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
}

/// System to handle upgrade button clicks
pub fn handle_upgrade_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<UpgradeButton>)>,
    mut building_query: Query<&mut StagedBuilding>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            for mut building in building_query.iter_mut() {
                building.upgrade();
                println!("Upgraded to stage {}", building.current_stage.as_u8());
            }
        }
    }
}

/// System to handle downgrade button clicks
pub fn handle_downgrade_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<DowngradeButton>)>,
    mut building_query: Query<&mut StagedBuilding>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            for mut building in building_query.iter_mut() {
                let current = building.current_stage.as_u8();
                if current > 0 {
                    building.set_stage(current - 1);
                    println!("Downgraded to stage {}", building.current_stage.as_u8());
                }
            }
        }
    }
}

/// System to update stage display text
pub fn update_stage_display(
    building_query: Query<&StagedBuilding, Changed<StagedBuilding>>,
    mut text_query: Query<&mut Text, With<StageDisplayText>>,
) {
    for building in building_query.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Stage: {}", building.current_stage.as_u8());
        }
    }
}

/// System to add button hover effects
pub fn button_hover_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, Or<(With<UpgradeButton>, With<DowngradeButton>)>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.8, 0.8, 0.8));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.7, 0.7, 0.7));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
        }
    }
}
