//! The credits menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};

use crate::{
    health::Health,
    menus::Menu,
    player::character::Player,
    screens::{Screen, title::TitleAssets},
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Results), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Results).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_credits_menu(
    mut commands: Commands,
    title_assets: Res<TitleAssets>,
    player_health: Single<&Health, With<Player>>,
) {
    let health = *player_health;
    commands.spawn((
        widget::ui_root("Credits Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Results),
        BackgroundColor(Color::BLACK.with_alpha(0.7)),
        children![
            widget::title("Congratulations!", &title_assets, 40.0),
            widget::label(
                "Your final score is: (the higher the better)",
                &title_assets
            ),
            widget::label(health.current.to_string(), &title_assets),
            widget::title("Thank you for playing!", &title_assets, 40.0),
            widget::button("Credits", go_credit_on_click, &title_assets),
        ],
    ));
}

fn go_credit_on_click(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Menu>>) {
    next_screen.set(Menu::Credits);
}

fn go_back(mut next_screen: ResMut<NextState<Menu>>) {
    next_screen.set(Menu::Credits);
}
