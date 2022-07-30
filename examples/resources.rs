//! This example illustrates a bevy ecs-ui design pattern

use bevy::prelude::*;

#[derive(PartialEq, Eq)]
enum UiInfoboxAnimalsState {
    Dogs,
    Cats,
}

#[derive(PartialEq, Eq)]
enum UiInfoboxCatsContentState {
    Kittens,
    Facts,
}

struct UiInfoboxVisibility(bool);

#[derive(Default)]
struct UiInfobox {
    root: Option<Entity>,
    animals: Option<Entity>,
    cat_content: Option<Entity>,
}

struct FrameCounter(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(UiInfobox::default())
        .insert_resource(UiInfoboxVisibility(true))
        .insert_resource(UiInfoboxAnimalsState::Cats)
        .insert_resource(UiInfoboxCatsContentState::Facts)
        .insert_resource(FrameCounter(0))
        .add_startup_system(setup)
        .add_system(mouse_click_system)
        .add_system(spawn_ui_infobox.after(mouse_click_system))
        .add_system(spawn_ui_infobox_cats.after(spawn_ui_infobox))
        .add_system(spawn_ui_infobox_dogs.after(spawn_ui_infobox))
        .add_system(spawn_ui_infobox_cats_facts.after(spawn_ui_infobox_cats))
        .add_system(spawn_ui_infobox_cats_kittens.after(spawn_ui_infobox_cats))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut visibility: ResMut<UiInfoboxVisibility>,
    mut ui_animals_state: ResMut<UiInfoboxAnimalsState>,
    mut ui_cats_content_state: ResMut<UiInfoboxCatsContentState>,
    mut frame_counter: ResMut<FrameCounter>,
) {
    frame_counter.0 += 1;

    if mouse_button_input.just_pressed(MouseButton::Middle) {
        info!("Toggle visibility");
        visibility.0 = !visibility.0;
        frame_counter.0 = 0;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("Will show dogs");
        *ui_animals_state = UiInfoboxAnimalsState::Dogs;
        frame_counter.0 = 0;
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        info!("Will show cats");
        if *ui_animals_state == UiInfoboxAnimalsState::Cats {
            if *ui_cats_content_state == UiInfoboxCatsContentState::Kittens {
                info!("Will show cats facts");
                *ui_cats_content_state = UiInfoboxCatsContentState::Facts;
            } else {
                info!("Will show cat kittens");
                *ui_cats_content_state = UiInfoboxCatsContentState::Kittens;
            }
        } else {
            *ui_animals_state = UiInfoboxAnimalsState::Cats;
        }
        frame_counter.0 = 0;
    }
}

fn spawn_ui_infobox(
    mut commands: Commands,
    visibility: Res<UiInfoboxVisibility>,
    mut ui_infobox: ResMut<UiInfobox>,
    mut ui_animals_state: ResMut<UiInfoboxAnimalsState>,
    asset_server: Res<AssetServer>,
    frame_counter: Res<FrameCounter>,
) {
    if visibility.is_changed() {
        if !visibility.0 {
            // Ui should be hidden
            if let Some(e) = ui_infobox.root {
                commands.entity(e).despawn_recursive();
                ui_infobox.root = None;
            }
        } else {
            //Spawn root node
            info!("Frame: {:?} infobox root", frame_counter.0);

            ui_infobox.root = Some(
                commands
                    .spawn_bundle(NodeBundle {
                        color: Color::RED.into(),
                        style: Style {
                            size: Size::new(Val::Px(600.0), Val::Px(400.0)),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::FlexStart,
                            flex_direction: FlexDirection::ColumnReverse,
                            position: UiRect {
                                left: Val::Px(20.0),
                                bottom: Val::Px(10.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Interaction::None)
                    // Add children some of which can be stateful
                    .with_children(|parent| {
                        ui_infobox.root = Some(parent.parent_entity());
                        parent.spawn_bundle(TextBundle {
                            text: Text::from_section(
                                "Animals".to_owned(),
                                TextStyle {
                                    font_size: 32.0,
                                    color: Color::BLUE,
                                    font: asset_server.load("font.ttf").clone(),
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            ..default()
                        });
                        ui_animals_state.set_changed();
                        ui_infobox.animals = Some(
                            parent
                                .spawn_bundle(NodeBundle {
                                    color: Color::ORANGE.into(),
                                    style: Style {
                                        flex_direction: FlexDirection::ColumnReverse,
                                        align_items: AlignItems::FlexStart,
                                        size: Size::new(Val::Percent(100.0), Val::Px(300.0)),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .id(),
                        );
                    })
                    .id(),
            );
        }
    }
}

fn spawn_ui_infobox_cats(
    mut commands: Commands,
    ui_animals_state: ResMut<UiInfoboxAnimalsState>,
    mut ui_infobox: ResMut<UiInfobox>,
    mut ui_cats_content_state: ResMut<UiInfoboxCatsContentState>,
    asset_server: Res<AssetServer>,
    visibility: Res<UiInfoboxVisibility>,
) {
    if ui_animals_state.is_changed()
        && *ui_animals_state == UiInfoboxAnimalsState::Cats
        && visibility.0
    {
        if let Some(e) = ui_infobox.animals {
            //Remove descendants if needed
            commands.entity(e).despawn_descendants();

            commands.entity(e).with_children(|parent| {
                //Should spawn cat stuff
                parent.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        "Cat stuff",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::BLUE,
                            font: asset_server.load("font.ttf").clone(),
                        },
                    ),
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                });

                ui_cats_content_state.set_changed();
                ui_infobox.cat_content = Some(
                    parent
                        .spawn_bundle(NodeBundle {
                            color: Color::GREEN.into(),
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .id(),
                );
            });
        }
    }
}

fn spawn_ui_infobox_cats_facts(
    mut commands: Commands,
    ui_infobox: Res<UiInfobox>,
    ui_cats_content_state: Res<UiInfoboxCatsContentState>,
    asset_server: Res<AssetServer>,
    visibility: Res<UiInfoboxVisibility>,
) {
    if ui_cats_content_state.is_changed()
        && *ui_cats_content_state == UiInfoboxCatsContentState::Facts
        && visibility.0
    {
        if let Some(e) = ui_infobox.cat_content {
            //Remove descendants if needed
            commands.entity(e).despawn_descendants();

            commands.entity(e).with_children(|parent| {
                //Should spawn cat facts
                parent.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        "Cats can jump 5 times their own height.",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::BLUE,
                            font: asset_server.load("font.ttf").clone(),
                        },
                    ),
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                });
            });
        }
    }
}

fn spawn_ui_infobox_cats_kittens(
    mut commands: Commands,
    ui_infobox: Res<UiInfobox>,
    ui_cats_content_state: Res<UiInfoboxCatsContentState>,
    asset_server: Res<AssetServer>,
    frame_counter: Res<FrameCounter>,
    visibility: Res<UiInfoboxVisibility>,
) {
    if ui_cats_content_state.is_changed()
        && *ui_cats_content_state == UiInfoboxCatsContentState::Kittens
        && visibility.0
    {
        if let Some(e) = ui_infobox.cat_content {
            //Remove descendants if needed
            commands.entity(e).despawn_descendants();
            info!("Frame: {:?} infobox kittens", frame_counter.0);

            commands.entity(e).with_children(|parent| {
                //Should spawn kittens
                parent.spawn_bundle(ImageBundle {
                    image: asset_server.load("kittens.png").into(),
                    ..default()
                });
            });
        }
    }
}

///  Dog stuff
fn spawn_ui_infobox_dogs(
    mut commands: Commands,
    ui_animals_state: ResMut<UiInfoboxAnimalsState>,
    ui_infobox: Res<UiInfobox>,
    asset_server: Res<AssetServer>,
    visibility: Res<UiInfoboxVisibility>,
) {
    if ui_animals_state.is_changed()
        && *ui_animals_state == UiInfoboxAnimalsState::Dogs
        && visibility.0
    {
        if let Some(e) = ui_infobox.animals {
            //Remove descendants if needed
            commands.entity(e).despawn_descendants();

            commands.entity(e).with_children(|parent| {
                //Should spawn dog stuff
                parent.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        "Dog stuff",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::YELLOW,
                            font: asset_server.load("font.ttf").clone(),
                        },
                    ),
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                });

                parent.spawn_bundle(ImageBundle {
                    image: asset_server.load("dog.png").into(),
                    ..default()
                });
            });
        }
    }
}
