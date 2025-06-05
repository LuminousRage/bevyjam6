use avian2d::prelude::{CollisionLayers, PhysicsLayer};

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Default, // Layer 0 - the default layer that objects are assigned to
    Player, // Layer 1
    Enemy,  // Layer 2
    Ground, // Layer 3
}

pub fn player_hit_boxes() -> CollisionLayers {
    CollisionLayers::new(GameLayer::Player, GameLayer::Enemy)
}
pub fn enemy_hit_boxes() -> CollisionLayers {
    CollisionLayers::new(GameLayer::Enemy, GameLayer::Player)
}
pub fn enemy_hurt_boxes() -> CollisionLayers {
    CollisionLayers::new(GameLayer::Enemy, GameLayer::Player)
}
pub fn player_hurt_boxes() -> CollisionLayers {
    CollisionLayers::new(GameLayer::Player, GameLayer::Enemy)
}
