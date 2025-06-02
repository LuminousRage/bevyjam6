//! This file is all about rendering the card
//! For card definition, see `card_library.rs`

use bevy::{color::palettes::css::BLACK, prelude::*, text::TextBounds};

use crate::{asset_tracking::LoadResource, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CardAssets>();
    app.load_resource::<CardAssets>();
}

enum CardCategory {
    Idk,
}
struct Card {
    name: &'static str,
    description: &'static str,
    image: Handle<Image>,
    // category: CardCategory,
    insanity: u32,
}

pub fn card(card_assets: &CardAssets) -> impl Bundle {
    (
        Name::new("Card"),
        Sprite {
            image: card_assets.frame.clone(),
            // image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
            ..default()
        },
        Transform::from_scale(Vec2::splat(0.2).extend(1.0)),
        children![
            card_image(card_assets),
            card_name(card_assets, "Asad's wife"),
            card_desc(
                card_assets,
                "She's apparently a star or a comet or something"
            )
        ],
    )
}

fn card_image(card_assets: &CardAssets) -> impl Bundle {
    (
        Name::new("Card Image"),
        Sprite {
            image: card_assets.image.clone(),
            ..default()
        },
        Transform::from_translation(-Vec3::Z),
    )
}

fn card_name(card_assets: &CardAssets, name: &'static str) -> impl Bundle {
    (
        Name::new("Card Name"),
        Text2d::new(name),
        TextFont {
            font_size: 120.0,
            font: card_assets.font.clone(),
            ..default()
        },
        Transform::from_xyz(0., -125., 1.0),
        TextColor(Color::Srgba(BLACK)),
    )
}

fn card_desc(card_assets: &CardAssets, desc: &'static str) -> impl Bundle {
    let box_size = Vec2::new(750.0, 420.0);
    (
        // Debugging background for the description box
        // Sprite::from_color(Color::srgb(0.25, 0.25, 0.55), box_size),
        Transform::from_xyz(0., -420., 1.0),
        children![(
            Name::new("Card Desc"),
            Text2d::new(desc),
            TextFont {
                font_size: 70.0,
                font: card_assets.font.clone(),
                ..default()
            },
            TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
            // Wrap text in the rectangle
            TextBounds::from(box_size),
            // Ensure the text is drawn on top of the box
            // Transform::from_xyz(0., -250., 1.0),
            TextColor(Color::Srgba(BLACK)),
        )],
    )
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct CardAssets {
    #[dependency]
    frame: Handle<Image>,
    #[dependency]
    image: Handle<Image>,
    #[dependency]
    font: Handle<Font>,
}

impl FromWorld for CardAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            frame: assets.load("images/card_frame.png"),
            image: assets.load("images/card_image.png"),
            font: assets.load("fonts/Amarante-Regular.ttf"),
        }
    }
}

pub fn spawn_card(mut commands: Commands, card_assets: Res<CardAssets>) {
    commands.spawn((
        Name::new("Card"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![card(&card_assets),],
    ));
}
