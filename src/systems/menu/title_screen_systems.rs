use bevy_app::{AppBuilder, Plugin, EventWriter};
use bevy_ecs::prelude::*;
use bevy_transform::{hierarchy::{BuildChildren, DespawnRecursiveExt}};
use gdnative::{core_types::{Rect2, Point2, Size2, Transform2D}, godot_warn};

use crate::{menus::{title_screen::{TitleScreen, TitleScreenState}, menu_data::MenuData, menu_state::MenuState}, systems::{backgrounds::events::BackgroundGroupEvent, visual_server::{canvas_item::{CanvasItemBundle, ClipRect}, text::common::Text}, input::Input, audio_server::audio::Audio}, core::{enumerations::{MainMenuOption, CombatMode}, configuration::Configuration}, drawing::print_data::PrintData};

use super::{setup_layers::HudLayer, components::MenuSoundManager};

struct MenuOptionText {
    index: usize,
    print_data: PrintData
}

struct MenuContainer;

#[derive(Default)]
struct TitleScreenTag;

fn show_title_screen(
    mut commands: Commands,
    mut background_group_event: EventWriter<BackgroundGroupEvent>,
    hud_layer_query: Query<Entity, With<HudLayer>>,
    title_screen: Res<TitleScreen>,
    configuration: Res<Configuration>,
    menu_data: Res<MenuData>,
) {
    let background_group = &title_screen.non_combat_screen.background_group;
    let mut title_screen_state = TitleScreenState::default();
    let hud_entity = hud_layer_query.single().expect("HudLayer not found");
    let mut menu_offset = 0;
    let height = title_screen.spacing.y * (title_screen.visiblemenuitems as f32 - 1.0) + title_screen.marginytop as f32 + title_screen.marginybottom as f32;
    let clip_rect = ClipRect::global(Rect2::new(Point2::new(0.0, 1.0 + title_screen.menuposition.y - title_screen.spacing.y), Size2::new(configuration.screen_size.width, height)));
    let screen_entity = commands.spawn_bundle(CanvasItemBundle::default())
        .insert(TitleScreenTag::default())
        .id();
    let menu_container_entity = commands.spawn_bundle(CanvasItemBundle {
        clip_rect,
        ..Default::default()
    })
        .insert(MenuContainer)
        .id();

    commands.entity(screen_entity).push_children(&[menu_container_entity]);

    let mut menus = Vec::<MainMenuOption>::new();

    for menu_item in MainMenuOption::iter() {
        let text_option = title_screen.menutext.get(menu_item);
        if text_option.is_none() {
            continue;
        }
        menus.push(menu_item.clone());
        let text = text_option.unwrap();
        let print_data = title_screen.mainfont;
        let mut location = title_screen.menuposition;
        location.x += title_screen.spacing.x * menu_offset as f32;
        location.y += title_screen.spacing.y * menu_offset as f32;
        menu_offset += 1;

        let menu_text_entity_option = menu_data.font_map.insert_font(
            &mut commands,
            print_data,
            location,
            ClipRect::default(),
            text
        );

        if let Some(menu_text_entity) = menu_text_entity_option {
            commands.entity(menu_text_entity).insert(MenuOptionText {
                index: menu_offset - 1,
                print_data: print_data.clone()
            });
            commands.entity(menu_container_entity).push_children(&[menu_text_entity]);
            title_screen_state.menuitemcount += 1;
        }
    }

    title_screen_state.menus = menus;

    background_group_event.send(BackgroundGroupEvent {
        layer: screen_entity.clone(),
        background_group: background_group.clone(),
    });

    commands.entity(hud_entity).push_children(&[screen_entity]);
    commands.entity(screen_entity).insert(title_screen_state);
}

fn update_active_menu_item(
    title_screen: Res<TitleScreen>,
    menu_data: Res<MenuData>,
    input: Res<Input>,
    menu_sound_manager: Res<MenuSoundManager>,
    mut menu_state: ResMut<State<MenuState>>,
    mut combat_mode: ResMut<State<CombatMode>>,
    mut audio: ResMut<Audio>,
    mut menu_container_query: Query<&mut Transform2D, With<MenuContainer>>,
    mut state_query: Query<&mut TitleScreenState>,
    mut menu_query: Query<(&mut MenuOptionText, &mut Text)>,
) {
    let mainfont = title_screen.mainfont;
    let activefont = title_screen.activefont;
    let mut state = state_query.single_mut().unwrap();
    let mut changed = false;

    if input.just_pressed("P1_D") || input.just_pressed("P2_D") {
        if state.currentmenuitem == state.menuitemcount - 1 {
            state.currentmenuitem = 0;
            state.verticalmenudrawoffset = 0.0;
        } else {
            state.currentmenuitem += 1;

            let menuoffset = state.verticalmenudrawoffset as f32 / title_screen.spacing.y;
            if (state.currentmenuitem as f32) >= menuoffset + title_screen.visiblemenuitems as f32 {
                state.verticalmenudrawoffset += title_screen.spacing.y;
            }
        }

        if let Some(soundid) = title_screen.soundcursormove {
            if let Some(sound) = menu_sound_manager.0.get_sound(soundid) {
                audio.play(sound.stream.clone());
            }
        }

        changed = true;
    }

    if input.just_pressed("P1_U") || input.just_pressed("P2_U") {
        if state.currentmenuitem == 0 {
            state.currentmenuitem = state.menuitemcount - 1;
            if state.menuitemcount > title_screen.visiblemenuitems as usize {
                state.verticalmenudrawoffset = title_screen.spacing.y * (state.menuitemcount as i32 - title_screen.visiblemenuitems) as f32;
            } else {
                state.verticalmenudrawoffset = 0.0;
            }
        } else {
            state.currentmenuitem -= 1;
            let menuoffset = state.verticalmenudrawoffset / title_screen.spacing.y;
            if (state.currentmenuitem as f32) < menuoffset {
                state.verticalmenudrawoffset -= title_screen.spacing.y;
            }
        }

        if let Some(soundid) = title_screen.soundcursormove {
            if let Some(sound) = menu_sound_manager.0.get_sound(soundid) {
                audio.play(sound.stream.clone());
            }
        }

        changed = true;
    }

    if input.just_pressed("P1_s") || input.just_pressed("P2_s") {
        let current_menu = state.menus[state.currentmenuitem];

        match current_menu {
            MainMenuOption::Versus => {
                menu_state.set(MenuState::Select).ok();
                combat_mode.set(CombatMode::Versus).ok();
            },
            MainMenuOption::Training => {
                menu_state.set(MenuState::Select).ok();
                combat_mode.set(CombatMode::Training).ok();
            },
            _ => {
                godot_warn!("Menu not implemented");
            }
        }

        if let Some(soundid) = title_screen.soundselect {
            if let Some(sound) = menu_sound_manager.0.get_sound(soundid) {
                audio.play(sound.stream.clone());
            }
        }
    }

    if !changed {
        return;
    }

    let mut transform = menu_container_query.single_mut().unwrap();
    *transform = Transform2D::translation(0.0, -state.verticalmenudrawoffset);

    for (mut menu_option_text, text) in menu_query.iter_mut() {
        if menu_option_text.index == state.currentmenuitem && menu_option_text.print_data != activefont {
            menu_option_text.print_data = activefont.clone();
            menu_data.font_map.update_font(activefont, text);
        } else if menu_option_text.index != state.currentmenuitem && menu_option_text.print_data != mainfont {
            menu_option_text.print_data = mainfont.clone();
            menu_data.font_map.update_font(mainfont, text);
        }
    }
}

fn hide_title_screen(
    mut commands: Commands,
    query: Query<Entity, With<TitleScreenTag>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Default)]
pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(MenuState::Title)
                    .with_system(show_title_screen.system())
            )
            .add_system_set(
                SystemSet::on_update(MenuState::Title)
                    .with_system(update_active_menu_item.system())
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::Title)
                    .with_system(hide_title_screen.system())
            );
    }
}
