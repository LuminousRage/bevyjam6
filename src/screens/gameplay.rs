//! The screen state for the main gameplay.

use avian2d::prelude::{Physics, PhysicsTime};
use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};

use crate::{
    Pause,
    audio::music,
    level::arena::{LevelAssets, spawn_level},
    menus::Menu,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    // Entry point of the game. Add the system you are testing currently to swap it out.
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
    app.add_systems(OnEnter(Screen::Gameplay), start_game_play_music);
    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>, mut time: ResMut<Time<Physics>>) {
    next_pause.set(Pause(false));
    time.unpause();
}

fn pause(mut next_pause: ResMut<NextState<Pause>>, mut time: ResMut<Time<Physics>>) {
    next_pause.set(Pause(true));
    time.pause();
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn start_game_play_music(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Music"),
        StateScoped(Screen::Gameplay),
        music(level_assets.music.clone()),
    ));
}
