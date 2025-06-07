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
            update_animation_atlas.run_if(resource_exists::<EyeAssets>),
        )
            .chain(),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct EyeAssets {
    #[dependency]
    eye: Handle<Image>,
}

impl FromWorld for EyeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            eye: assets.load("images/boss_eye.png"),
        }
    }
}

pub fn the_eye(
    eye_assets: &EyeAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::new(1500, 1006), 10, 5, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // let player_animation = PlayerAnimation::new();

    (
        Name::new("The EYe"),
        Sprite {
            image: eye_assets.eye.clone(),

            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0, // index: player_animation.get_atlas_index(),
            }),
            ..default()
        },
        Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
        EyeAnimation::new(),
    )
}

#[derive(Component)]
pub struct EyeAnimation {
    timer: Timer,
    frame: usize,
}

impl EyeAnimation {
    const NUM_FRAMES: usize = 50;
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            frame: 0,
        }
    }

    fn update_frame(&mut self, time: Duration) {
        self.timer.tick(time);
        if self.timer.just_finished() {
            self.frame = (self.frame + 1) % Self::NUM_FRAMES;
        }
    }
}

fn animation_updater(mut query: Query<&mut EyeAnimation>, time: Res<Time>) {
    for mut animation in &mut query {
        animation.update_frame(time.delta());
    }
}

fn update_animation_atlas(mut query: Query<(&mut Sprite, &EyeAnimation)>) {
    for (mut sprite, animation) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        atlas.index = animation.frame;
    }
}
