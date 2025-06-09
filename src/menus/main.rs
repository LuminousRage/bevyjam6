//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{
    GAME_NAME,
    asset_tracking::ResourceHandles,
    menus::Menu,
    screens::{Screen, title::TitleAssets},
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands, title_assets: Res<TitleAssets>) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::title(GAME_NAME, &title_assets, 120.),
            widget::button("Start", enter_loading_or_story_screen, &title_assets),
            widget::button("Settings", open_settings_menu, &title_assets),
            widget::button("Credits", open_credits_menu, &title_assets),
            widget::button("Exit", exit_app, &title_assets),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::title(GAME_NAME, &title_assets, 120.),
            widget::button("Start", enter_loading_or_story_screen, &title_assets),
            widget::button("Settings", open_settings_menu, &title_assets),
            widget::button("Credits", open_credits_menu, &title_assets),
        ],
    ));
}

fn enter_loading_or_story_screen(
    _: Trigger<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Story);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
