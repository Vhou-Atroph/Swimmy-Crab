//! Module for a friendly swimming blue crab! <https://en.wikipedia.org/wiki/Portunus_pelagicus>
use bevy::{math::*,prelude::*};

// Constants:
const BAILEY_SCALE: Vec3 = const_vec3!([2.0,2.0,0.0]);

/// Full plguin for Bailey
pub struct BaileyPlugin;
impl Plugin for BaileyPlugin {
    fn build(&self,app:&mut App) {
        app.add_startup_system(make_bailey);
    }
}

#[derive(Component)]
pub struct Bailey;
#[derive(Component)]
pub struct BaileyState(bool);

fn make_bailey(mut commands:Commands,
    asset_server:Res<AssetServer>) {
    commands.spawn()
        .insert(Bailey)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-200.0,0.0,0.5),
                scale: BAILEY_SCALE,
                ..default()
            },
            texture: asset_server.load("sprites/bailey.png"),
            ..default()
        })
        .insert(crate::char::events::Collider)
    ;
}