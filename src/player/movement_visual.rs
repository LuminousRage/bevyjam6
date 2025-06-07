use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    animation::{Animation, reversible_animation},
    physics::creature::Grounded,
    player::{
        character::{Player, PlayerAssets, PlayerLayoutAssets, PlayerSpriteMode, player_sprite},
        movement::{Dashing, MovementAction},
    },
};
const IDLE_FRAME_NUM: usize = 10;
const RUN_FRAME_NUM: usize = 32;
const JUMP_FRAME_NUM: usize = 34;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_player_transform,
            (movement_visual, update_player_sprite_animation).chain(),
        ),
    );
}

fn update_player_transform(mut player: Single<(&Player, &mut Transform)>) {
    let (p, transform) = &mut *player;

    transform.scale.x = p.face_direction.x * transform.scale.x.abs();
}

fn update_player_sprite_animation(
    mut player: Single<(&mut Sprite, &mut PlayerSpriteMode), With<Player>>,
    animation: Res<Animation>,
) {
    let (sprite, player_mode) = &mut *player;

    let Some(texture_atlas) = &mut sprite.texture_atlas else {
        return;
    };

    if !animation.0.just_finished() {
        return;
    }

    match &mut **player_mode {
        PlayerSpriteMode::Idle(reverse) => {
            reversible_animation(reverse, &mut texture_atlas.index, IDLE_FRAME_NUM);
        }
        PlayerSpriteMode::Run => {}
        PlayerSpriteMode::Jump => {}
        PlayerSpriteMode::Dash => {}
    }

    dbg!(texture_atlas.index);
}

fn movement_visual(
    time: Res<Time>,
    mut commands: Commands,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Single<(
        Entity,
        &mut Player,
        &mut Sprite,
        &mut LinearVelocity,
        Has<Grounded>,
        Has<Dashing>,
    )>,
    player_assets: Res<PlayerAssets>,
    player_layout: Res<PlayerLayoutAssets>,
) {
    let (entity, player, sprite, lv, is_grounded, is_dashing) = &mut *controllers;
}
