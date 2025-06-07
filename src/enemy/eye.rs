use std::time::Duration;

use avian2d::prelude::Collider;
use bevy::prelude::*;

use crate::{
    animation::reversible_animation,
    asset_tracking::LoadResource,
    collision_layers::{enemy_hit_boxes, enemy_hurt_boxes},
    health::{hitbox_prefab, hurtbox_prefab},
    player::character::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<EyeAssets>();
    app.load_resource::<EyeAssets>();
    app.add_systems(
        Update,
        (
            animation_updater,
            update_eye_animation.run_if(resource_exists::<EyeAssets>),
        )
            .chain(),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct EyeAssets {
    #[dependency]
    eye: Handle<Image>,
    #[dependency]
    ring: Handle<Image>,
    #[dependency]
    wings: Handle<Image>,
    #[dependency]
    red: Handle<Image>,
    #[dependency]
    white: Handle<Image>,
    #[dependency]
    pupil: Handle<Image>,
}

impl FromWorld for EyeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            eye: assets.load("images/eye/boss_eye_main.png"),
            ring: assets.load("images/eye/boss_eye_ring.png"),
            wings: assets.load("images/eye/boss_eye_wings.png"),
            white: assets.load("images/eye/boss_eye_white.png"),
            red: assets.load("images/eye/boss_eye_red.png"),
            pupil: assets.load("images/eye/boss_eye_pupil.png"),
        }
    }
}

pub fn the_eye(
    eye_assets: &EyeAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    scale: Vec2,
    translation: Vec3,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::new(1500, 1006), 5, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    (
        Visibility::default(),
        Transform::from_scale(scale.extend(1.0)).with_translation(translation),
        children![
            hurtbox_prefab(
                Collider::circle(100.),
                enemy_hurt_boxes(),
                0.0,
                Transform::default()
            ),
            hitbox_prefab(
                Collider::circle(100.),
                enemy_hit_boxes(),
                0.0,
                10.0,
                Transform::default()
            ),
            (
                Name::new("Wings"),
                Sprite {
                    image: eye_assets.wings.clone(),

                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout,
                        index: 0,
                    }),
                    ..default()
                },
                EyeAnimation::new()
            ),
            (
                Name::new("Ring"),
                Sprite::from_image(eye_assets.ring.clone()),
                EyeAnimation::new()
            ),
            (
                Name::new("Eye White"),
                Sprite::from_image(eye_assets.white.clone())
            ),
            (
                Name::new("Pupil"),
                Sprite::from_image(eye_assets.pupil.clone()),
                EyeAnimation::new()
            ),
            (Name::new("Eye"), Sprite::from_image(eye_assets.eye.clone())),
        ],
    )
}

// The whole code below can do some refactoring

#[derive(Component)]
pub struct EyeAnimation {
    timer: Timer,
    frame: usize,
    reverse: bool,
    pub target: Quat,
}

impl EyeAnimation {
    const NUM_FRAMES: usize = 30;
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
            frame: 0,
            reverse: false,
            target: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2 / 3.0),
        }
    }

    fn update_target(&mut self) {
        self.target = self.target.inverse();
    }

    fn update_frame(&mut self, time: Duration) {
        self.timer.tick(time);
        if self.timer.just_finished() {
            reversible_animation(&mut self.reverse, &mut self.frame, Self::NUM_FRAMES);
        }
    }
}

fn animation_updater(mut query: Query<&mut EyeAnimation>, time: Res<Time>) {
    for mut animation in &mut query {
        animation.update_frame(time.delta());
    }
}

fn update_eye_animation(
    mut query: Query<
        (
            &mut Sprite,
            &mut Transform,
            &mut GlobalTransform,
            &mut EyeAnimation,
            &Name,
        ),
        Without<Player>,
    >,
    player: Single<&Transform, With<Player>>,
    time: Res<Time>,
) {
    for (mut sprite, mut transform, mut global_transform, mut animation, name) in &mut query {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = animation.frame;
        } else {
            //Marker? I hardly know 'er
            match name.as_str() {
                "Pupil" => {
                    let dir =
                        -global_transform.translation().truncate() + player.translation.truncate();
                    let target = (&dir.normalize_or_zero() * 50.0).extend(1.0);
                    transform
                        .translation
                        .smooth_nudge(&target, 1.2, time.delta_secs());
                }
                "Ring" => {
                    transform
                        .rotation
                        .smooth_nudge(&animation.target, 0.4, time.delta_secs());

                    if (transform.rotation - animation.target).length() < 0.1 {
                        animation.update_target();
                    }
                }
                _ => {}
            }
        };
    }
}
