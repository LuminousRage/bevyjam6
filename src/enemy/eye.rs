use std::time::Duration;

use bevy::prelude::*;

use crate::asset_tracking::LoadResource;

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
}

impl FromWorld for EyeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            eye: assets.load("images/boss_eye_main.png"),
            ring: assets.load("images/boss_eye_ring.png"),
            wings: assets.load("images/boss_eye_wings.png"),
        }
    }
}

pub fn the_eye(
    eye_assets: &EyeAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::new(1500, 1006), 5, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // let player_animation = PlayerAnimation::new();

    (
        Name::new("The Eye"),
        Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
        Visibility::default(),
        children![
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
            (Name::new("Eye"), Sprite::from_image(eye_assets.eye.clone()),),
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
            if self.reverse {
                self.frame -= 1;

                if self.frame == 0 {
                    self.reverse = false;
                }
            }

            if !self.reverse {
                self.frame += 1;

                if self.frame == Self::NUM_FRAMES - 1 {
                    self.reverse = true;
                }
            }
        }
    }
}

fn animation_updater(mut query: Query<&mut EyeAnimation>, time: Res<Time>) {
    for mut animation in &mut query {
        animation.update_frame(time.delta());
    }
}

fn update_eye_animation(
    mut query: Query<(&mut Sprite, &mut Transform, &mut EyeAnimation)>,
    time: Res<Time>,
) {
    for (mut sprite, mut transform, mut animation) in &mut query {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = animation.frame;
        } else {
            transform
                .rotation
                .smooth_nudge(&animation.target, 0.4, time.delta_secs());

            if (transform.rotation - animation.target).length() < 0.1 {
                animation.update_target();
            }
        };
    }
}
