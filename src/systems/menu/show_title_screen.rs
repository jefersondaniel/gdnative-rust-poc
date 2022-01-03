use bevy_app::EventWriter;
use bevy_ecs::prelude::*;
use bevy_transform::{hierarchy::BuildChildren};
use gdnative::core_types::{Vector2, Rect2, Point2, Size2};

use crate::{menus::{title_screen::{TitleScreen, TitleScreenState}, menu_data::MenuData}, systems::{backgrounds::events::BackgroundGroupEvent, visual_server::canvas_item::CanvasItemBundle}, core::{enumerations::MainMenuOption, configuration::Configuration}};

use super::{setup_layers::HudLayer, components::Screen};

pub fn show_title_screen(
    mut commands: Commands,
    mut background_group_event: EventWriter<BackgroundGroupEvent>,
    hud_layer_query: Query<Entity, With<HudLayer>>,
    title_screen: Res<TitleScreen>,
    configuration: Res<Configuration>,
    menu_data: Res<MenuData>,
) {
    let background_group = &title_screen.non_combat_screen.background_group;
    let title_screen_state = TitleScreenState::default();
    let hud_entity = hud_layer_query.single().expect("HudLayer not found");
    let screen_entity = commands.spawn_bundle(CanvasItemBundle::default())
        .insert(Screen::default())
        .with_children(|screen_builder| {
            let mut menu_offset = 0;
            let height = title_screen.spacing.y * (title_screen.visiblemenuitems as f32 - 1.0) + title_screen.marginytop as f32 + title_screen.marginybottom as f32;
            let scissorrect = Rect2::new(Point2::new(0.0, title_screen.menuposition.y - title_screen.spacing.y), Size2::new(configuration.screen_size.width, height));

            for menu_item in MainMenuOption::iter() {
                let text_option = title_screen.menutext.get(menu_item);
                if text_option.is_none() {
                    continue;
                }
                let text = text_option.unwrap();
                let print_data = title_screen.mainfont;
                let mut location = title_screen.menuposition;
                location.x += title_screen.spacing.x * menu_offset as f32;
                location.y += title_screen.spacing.y * menu_offset as f32;
                location.y -= title_screen_state.verticalmenudrawoffset as f32;
                menu_offset += 1;

                menu_data.font_map.insert_font(
                    screen_builder,
                    print_data,
                    location,
                    text
                );
            };
        })
        .id();

    background_group_event.send(BackgroundGroupEvent {
        layer: screen_entity.clone(),
        background_group: background_group.clone(),
    });

    commands.entity(hud_entity).push_children(&[screen_entity]);
    commands.entity(screen_entity).insert(title_screen_state);
}
