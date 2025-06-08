use avian2d::prelude::*;
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    sprite::Anchor,
};

use crate::{
    asset_tracking::LoadResource,
    collision_layers::GameLayer,
    enemy::{
        boss::boss,
        eye::{EyeAssets, the_eye},
        slime::{SlimeAssets, slime},
    },
    player::{
        character::{PlayerAssets, PlayerLayoutAssets, player},
        weapon::{WeaponAssets, weapon},
    },
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    pub background: Handle<Image>,
    #[dependency]
    pub fog: Handle<Image>,
    #[dependency]
    pub light: Handle<Image>,
    #[dependency]
    pub platform_long: Handle<Image>,
    #[dependency]
    pub platform_medium: Handle<Image>,
    #[dependency]
    pub platform_short: Handle<Image>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            background: assets.load_with_settings(
                "images/background.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            fog: assets.load_with_settings(
                "images/fog.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            platform_long: assets.load_with_settings(
                "images/platform_long.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            platform_medium: assets.load_with_settings(
                "images/platform_medium.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            platform_short: assets.load_with_settings(
                "images/platform_short.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            light: assets.load_with_settings(
                "images/light.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    player_layout_assets: Res<PlayerLayoutAssets>,
    weapon_assets: Res<WeaponAssets>,
    slime_assets: Res<SlimeAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level_assets: Res<LevelAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    eye_assets: Res<EyeAssets>,
) {
    commands.spawn((
        Name::new("Background"),
        Transform::from_scale(Vec2::splat(1.3).extend(-5.)),
        Sprite::from_image(level_assets.background.clone()),
    ));
    commands.spawn((
        Name::new("Foreground Fog"),
        Transform {
            translation: Vec3::new(0., 0., 8.),
            scale: Vec3::new(1.3, 1.0, 1.0),
            ..default()
        },
        Sprite {
            image: level_assets.fog.clone(),
            anchor: Anchor::Custom(vec2(0., 1.)),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Foreground Light"),
        Transform {
            translation: Vec3::new(180., 400., 8.),
            scale: Vec3::new(1.3, 1.3, 1.0),
            ..default()
        },
        Sprite {
            image: level_assets.light.clone(),
            // anchor: Anchor::Custom(vec2(0., 1.)),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Background Solid"),
        Transform::from_scale(Vec2::splat(1.3).extend(-8.)),
        Sprite::from_color(Color::Srgba(Srgba::new(0., 4., 73., 0.)), Vec2::INFINITY),
    ));

    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            player(&player_assets, &player_layout_assets),
            weapon(&weapon_assets)
        ],
    ));

    let red_slime = || slime(&slime_assets, Vec3::new(200.0, 2000.0, 0.0), true);
    let black_slime = || slime(&slime_assets, Vec3::new(-200.0, 2000.0, 0.0), false);

    commands.spawn(red_slime());
    commands.spawn(black_slime());
    commands.spawn(boss(
        &eye_assets,
        &mut texture_atlas_layouts,
        Vec3::new(-200.0, 100.0, 0.0),
    ));
    // commands.spawn(slime());

    commands.spawn((
        Name::new("Platform"),
        Sprite {
            image: level_assets.platform_long.clone(),
            anchor: Anchor::Custom(Vec2::new(0.0, 0.35)),
            custom_size: Some(Vec2::new(1500.0, 500.0)),
            image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
            ..default()
        },
        Transform::from_xyz(0.0, -600.0, 5.0),
        RigidBody::Static,
        Collider::rectangle(1500.0, 100.0),
        CollisionLayers::new(GameLayer::Ground, LayerMask::ALL),
    ));

    commands.spawn(platform_medium(
        "Platform Left",
        Transform::from_xyz(-820., -205., 5.),
        &level_assets,
    ));
    commands.spawn(platform_medium(
        "Platform Right",
        Transform::from_xyz(820., -205., 5.),
        &level_assets,
    ));

    commands.spawn(platform_small(
        "Platform Small",
        Transform::from_xyz(0., 100., 5.),
        &level_assets,
    ));
}

fn platform_small(
    name: &'static str,
    transform: Transform,
    level_assets: &LevelAssets,
) -> impl Bundle {
    (
        Name::new(name),
        Sprite {
            image: level_assets.platform_short.clone(),
            anchor: Anchor::Custom(Vec2::new(0.0, 0.50)),
            custom_size: Some(Vec2::new(300.0, 500.0)),
            image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
            ..default()
        },
        transform,
        RigidBody::Static,
        Collider::triangle(
            Vec2::new(-150.0, 0.0),
            Vec2::new(150., 0.0),
            Vec2::new(0.0, -120.0),
        ),
        CollisionLayers::new(GameLayer::Ground, LayerMask::ALL),
    )
}

fn platform_medium(
    name: &'static str,
    transform: Transform,
    level_assets: &LevelAssets,
) -> impl Bundle {
    (
        Name::new(name),
        Sprite {
            image: level_assets.platform_medium.clone(),
            anchor: Anchor::Custom(Vec2::new(0.0, 0.51)),
            custom_size: Some(Vec2::new(500.0, 500.0)),
            image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
            ..default()
        },
        transform,
        RigidBody::Static,
        // 500., 50.
        Collider::triangle(
            Vec2::new(-250.0, 0.0),
            Vec2::new(250., 0.0),
            Vec2::new(0.0, -130.0),
        ),
        CollisionLayers::new(GameLayer::Ground, LayerMask::ALL),
    )
}
