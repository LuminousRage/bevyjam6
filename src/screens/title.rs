//! The title screen that appears after the splash screen.

use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::audio::music;
use crate::theme::prelude::*;
use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), (open_main_menu, spawn_title_screen));
    app.add_systems(OnExit(Screen::Title), close_menu);

    app.register_type::<TitleAssets>();
    app.load_resource::<TitleAssets>();
    app.add_systems(
        OnEnter(Screen::Title),
        start_title_music.run_if(resource_exists::<TitleAssets>),
    );
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn spawn_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        widget::ui_root("Menu Background Screen"),
        StateScoped(Screen::Title),
        children![(
            Name::new("Background image"),
            Node {
                margin: UiRect::all(Val::Auto),
                width: Val::Percent(100.0),
                ..default()
            },
            ImageNode::new(asset_server.load_with_settings(
                // This should be an embedded asset for instant loading, but that is
                // currently [broken on Windows Wasm builds](https://github.com/bevyengine/bevy/issues/14246).
                "images/background.png",
                |settings: &mut ImageLoaderSettings| {
                    // Make an exception for the splash image in case
                    // `ImagePlugin::default_nearest()` is used for pixel art.
                    settings.sampler = ImageSampler::linear();
                },
            )),
        )],
    ));
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct TitleAssets {
    #[dependency]
    pub music: Handle<AudioSource>,
    #[dependency]
    pub allura: Handle<Font>,
    #[dependency]
    pub crimson: Handle<Font>,
}

impl FromWorld for TitleAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Elevator_Music_V1.ogg"),
            allura: assets.load("fonts/Allura/Allura-Regular.ttf"),
            crimson: assets.load("fonts/Crimson_Text/CrimsonText-Regular.ttf"),
        }
    }
}

fn start_title_music(mut commands: Commands, credits_music: Res<TitleAssets>) {
    commands.spawn((
        Name::new("Credits Music"),
        StateScoped(Screen::Title),
        music(credits_music.music.clone()),
    ));
}
