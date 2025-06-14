use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;

use crate::{
    PausableSystems,
    asset_tracking::LoadResource,
    collision_layers::player_hurt_boxes,
    health::{Health, health_bar, hurtbox_prefab},
    physics::creature::Grounded,
    player::movement::movement::{PlayerMovementBundle, PlayerMovementState},
};

use super::configs::{CHARACTER_GRAVITY_SCALE, CHARACTER_HEALTH};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAssets>();
    app.register_type::<PlayerLayoutAssets>();

    app.load_resource::<PlayerAssets>();
    app.add_systems(Update, player_fall_recovery.in_set(PausableSystems));
    app.add_systems(Update, reset_player_gravity_scale.in_set(PausableSystems));
    app.add_systems(Startup, init_player_layout);
}

#[derive(Component)]
pub struct Player {
    pub face_direction: Vec2,
    pub attack_direction: Vec2,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub player_idle: Handle<Image>,
    #[dependency]
    pub player_run: Handle<Image>,
    #[dependency]
    pub player_dash: Handle<Image>,
    #[dependency]
    pub player_jump: Handle<Image>,
    #[dependency]
    pub player_step_sounds: Vec<Handle<AudioSource>>,
    #[dependency]
    pub player_dash_sounds: Vec<Handle<AudioSource>>,
}

fn init_player_layout(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let run_layout = TextureAtlasLayout::from_grid(UVec2::new(390, 560), 8, 4, None, None);
    let jump_layout = TextureAtlasLayout::from_grid(UVec2::new(390, 580), 8, 5, None, None);
    let idle_layout = TextureAtlasLayout::from_grid(UVec2::new(390, 560), 8, 2, None, None);

    commands.insert_resource(PlayerLayoutAssets {
        player_idle: texture_atlas_layouts.add(idle_layout),
        player_run: texture_atlas_layouts.add(run_layout),
        player_jump: texture_atlas_layouts.add(jump_layout),
    });
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerLayoutAssets {
    #[dependency]
    pub player_idle: Handle<TextureAtlasLayout>,
    #[dependency]
    pub player_run: Handle<TextureAtlasLayout>,
    #[dependency]
    pub player_jump: Handle<TextureAtlasLayout>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            player_idle: assets.load("images/player/player_idle.png"),
            player_run: assets.load("images/player/player_run.png"),
            player_dash: assets.load("images/player/player_dash.png"),
            player_jump: assets.load("images/player/player_jump.png"),
            player_step_sounds: (1..=8)
                .map(|i| assets.load(&format!("audio/sound_effects/player/footsteps_{}.ogg", i)))
                .collect(),
            player_dash_sounds: (1..=8)
                .map(|i| assets.load(&format!("audio/sound_effects/player/dash_{}.ogg", i)))
                .collect(),
        }
    }
}

fn reset_player_gravity_scale(
    mut player: Single<(&mut GravityScale, Has<Grounded>), With<Player>>,
) {
    let (gs, is_grounded) = &mut *player;

    if *is_grounded {
        gs.0 = CHARACTER_GRAVITY_SCALE;
    }
}

fn player_fall_recovery(
    mut player: Single<(&mut Transform, &mut LinearVelocity, &mut GravityScale), With<Player>>,
) {
    let (transform, lv, gs) = &mut *player;

    if transform.translation.y < -1500.0 {
        lv.y = 0.0;
        gs.0 = 0.5;
        // TODO: add a period of invulnerability
        transform.translation.y = 300.0;
        transform.translation.x = 0.0;
    }
}

pub fn player(
    player_assets: &PlayerAssets,
    player_layout_assets: &PlayerLayoutAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> impl Bundle {
    let movement_state = PlayerMovementState::Idle(false);
    let (image, texture_atlas) =
        player_sprite(movement_state.clone(), player_assets, player_layout_assets);
    (
        Name::new("Player"),
        Transform::from_xyz(0.0, 0.0, 2.0),
        Player {
            face_direction: Vec2::X,
            attack_direction: Vec2::X,
        },
        Sprite {
            image,
            // this should fit on y. x is the variable part
            custom_size: Some(Vec2::new(300., 275.0)),
            image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
            texture_atlas,
            ..default()
        },
        PlayerMovementBundle::new(Collider::capsule(15.0, 170.0), Vector::ONE),
        Health::new(CHARACTER_HEALTH),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(CHARACTER_GRAVITY_SCALE),
        children![
            hurtbox_prefab(
                Collider::capsule(30.0, 135.0),
                player_hurt_boxes(),
                0.5,
                Transform::default()
            ),
            // health_bar(Transform::from_xyz(0., 120., 1.), meshes, materials)
        ],
    )
}

pub fn player_sprite(
    mode: PlayerMovementState,
    player_assets: &PlayerAssets,
    player_layout: &PlayerLayoutAssets,
) -> (Handle<Image>, Option<TextureAtlas>) {
    match mode {
        PlayerMovementState::Idle(_) => (
            player_assets.player_idle.clone(),
            Some(TextureAtlas {
                layout: player_layout.player_idle.clone(),
                index: 0,
            }),
        ),
        PlayerMovementState::Run => (
            player_assets.player_run.clone(),
            Some(TextureAtlas {
                layout: player_layout.player_run.clone(),
                index: 0,
            }),
        ),
        PlayerMovementState::Dash(_) => (player_assets.player_dash.clone(), None),
        PlayerMovementState::Jump(_) => (
            player_assets.player_jump.clone(),
            Some(TextureAtlas {
                layout: player_layout.player_jump.clone(),
                index: 0,
            }),
        ),
    }
}
