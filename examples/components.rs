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

struct UiInfoboxState {
    visibility: bool,
    animal_state: UiInfoboxAnimalsState,
    animal_cats_state: UiInfoboxCatsContentState,
}
impl Default for UiInfoboxState {
    fn default() -> Self {
        Self {
            visibility: true,
            animal_state: UiInfoboxAnimalsState::Cats,
            animal_cats_state: UiInfoboxCatsContentState::Kittens,
        }
    }
}

#[derive(Component)]
struct UiInfoboxRoot;
#[derive(Component)]
struct UiInfoboxAnimals;
#[derive(Component)]
struct UiInfoboxCat;

struct FrameCounter(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(UiInfoboxState::default())
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
    commands.spawn_bundle(UiCameraBundle::default());
}

fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut ui_infobox_state: ResMut<UiInfoboxState>,
    mut frame_counter: ResMut<FrameCounter>,
) {
    frame_counter.0 += 1;

    if mouse_button_input.just_pressed(MouseButton::Middle) {
        info!("Toggle visibility");
        ui_infobox_state.visibility = !ui_infobox_state.visibility;
        frame_counter.0 = 0;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("Will show dogs");
        ui_infobox_state.animal_state = UiInfoboxAnimalsState::Dogs;
        frame_counter.0 = 0;
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        info!("Will show cats");
        if ui_infobox_state.animal_state == UiInfoboxAnimalsState::Cats {
            if ui_infobox_state.animal_cats_state == UiInfoboxCatsContentState::Kittens {
                info!("Will show cats facts");
                ui_infobox_state.animal_cats_state = UiInfoboxCatsContentState::Facts;
            } else {
                info!("Will show cat kittens");
                ui_infobox_state.animal_cats_state = UiInfoboxCatsContentState::Kittens;
            }
        } else {
            ui_infobox_state.animal_state = UiInfoboxAnimalsState::Cats;
        }
        frame_counter.0 = 0;
    }
}

fn spawn_ui_infobox(
    mut commands: Commands,
    ui_root: Query<Entity, With<UiInfoboxRoot>>,
    ui_infobox_state: Res<UiInfoboxState>,
    asset_server: Res<AssetServer>,
    frame_counter: Res<FrameCounter>,
) {
    if ui_infobox_state.is_changed() {
        // Reset ui
        if let Ok(e) = ui_root.get_single() {
            commands.entity(e).despawn_recursive();
        }
        if ui_infobox_state.visibility {
            info!("Frame: {:?} infobox root", frame_counter.0);
            //Spawn root node
            commands
                .spawn_bundle(NodeBundle {
                    color: Color::RED.into(),
                    style: Style {
                        size: Size::new(Val::Px(600.0), Val::Px(400.0)),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::FlexStart,
                        flex_direction: FlexDirection::ColumnReverse,
                        position: Rect {
                            left: Val::Px(20.0),
                            bottom: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(Interaction::None)
                .insert(UiInfoboxRoot)
                // Add children some of which can be stateful
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Animals".to_owned(),
                            TextStyle {
                                font_size: 32.0,
                                color: Color::BLUE,
                                font: asset_server.load("font.ttf").clone(),
                            },
                            Default::default(),
                        ),
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
                            ..default()
                        },
                        ..default()
                    });
                    parent
                        .spawn_bundle(NodeBundle::default())
                        .insert(UiInfoboxAnimals);
                });
        }
    }
}

fn spawn_ui_infobox_cats(
    mut commands: Commands,
    query: Query<Entity, Added<UiInfoboxAnimals>>,
    ui_infobox_state: Res<UiInfoboxState>,
    asset_server: Res<AssetServer>,
) {
    for e in query.iter() {
        if ui_infobox_state.animal_state == UiInfoboxAnimalsState::Cats {
            commands
                .entity(e)
                .insert_bundle(NodeBundle {
                    color: Color::ORANGE.into(),
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::FlexStart,
                        size: Size::new(Val::Percent(100.0), Val::Px(300.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    //Should spawn cat stuff
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Cat stuff",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::BLUE,
                                font: asset_server.load("font.ttf").clone(),
                            },
                            Default::default(),
                        ),
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
                            ..default()
                        },
                        ..default()
                    });
                    parent
                        .spawn_bundle(NodeBundle::default())
                        .insert(UiInfoboxCat);
                });
        }
    }
}

fn spawn_ui_infobox_cats_facts(
    mut commands: Commands,
    query: Query<Entity, Added<UiInfoboxCat>>,
    ui_infobox_state: Res<UiInfoboxState>,
    asset_server: Res<AssetServer>,
) {
    for e in query.iter() {
        if ui_infobox_state.animal_cats_state == UiInfoboxCatsContentState::Facts {
            commands
                .entity(e)
                .insert_bundle(NodeBundle {
                    color: Color::GREEN.into(),
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    //Should spawn cat facts
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Cats can jump 5 times their own height.",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::BLUE,
                                font: asset_server.load("font.ttf").clone(),
                            },
                            Default::default(),
                        ),
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
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
    query: Query<Entity, Added<UiInfoboxCat>>,
    ui_infobox_state: Res<UiInfoboxState>,
    asset_server: Res<AssetServer>,
    frame_counter: Res<FrameCounter>,
) {
    for e in query.iter() {
        if ui_infobox_state.animal_cats_state == UiInfoboxCatsContentState::Kittens {
            info!("Frame: {:?} infobox kittens", frame_counter.0);
            commands
                .entity(e)
                .insert_bundle(NodeBundle {
                    color: Color::GREEN.into(),
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
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
    query: Query<Entity, Added<UiInfoboxAnimals>>,
    ui_infobox_state: Res<UiInfoboxState>,
    asset_server: Res<AssetServer>,
) {
    for e in query.iter() {
        if ui_infobox_state.animal_state == UiInfoboxAnimalsState::Dogs {
            commands
                .entity(e)
                .insert_bundle(NodeBundle {
                    color: Color::ORANGE.into(),
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::FlexStart,
                        size: Size::new(Val::Percent(100.0), Val::Px(300.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    //Should spawn dog stuff
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Dog stuff",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::YELLOW,
                                font: asset_server.load("font.ttf").clone(),
                            },
                            Default::default(),
                        ),
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
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
