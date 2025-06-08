//! The credits menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};

use crate::{menus::Menu, screens::title::TitleAssets, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_credits_menu(mut commands: Commands, title_assets: Res<TitleAssets>) {
    commands.spawn((
        widget::ui_root("Credits Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Credits),
        BackgroundColor(Color::BLACK.with_alpha(0.7)),
        children![
            widget::title("Brought to you by", &title_assets, 40.0),
            widget::label("Tifereth (Programming, all nighter puller), 4321louis (Programming, people skills user)", &title_assets),
            widget::label("Cassie (Chief Animator, Chief Artist), Varshna (Chief Artist, Chief Background Designer)", &title_assets),
            widget::label("acid (Story, Ancestor, Vision haver), Hethan (Music, not a Bevy enjoyer)", &title_assets),
            widget::title("External assets used:", &title_assets, 40.0),
            widget::label("Fonts: Allura, Crimson", &title_assets),
            widget::title("We would love your feedback!", &title_assets, 40.0),
            widget::label("As always, positive feedback goes to giro308 (he didn't even participate in this jam)",&title_assets),
            widget::label("Negative feedback goes to 4321louis (he did nothing wrong this jam)",&title_assets),
            widget::button("Back", go_back_on_click, &title_assets),
        ],

    ));
}

fn go_back_on_click(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
