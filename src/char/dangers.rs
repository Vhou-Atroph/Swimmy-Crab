use bevy::{prelude::*};



//Debug Brick below
pub struct DebugBrick;
impl Plugin for DebugBrick {
    fn build(&self,app:&mut App) {
        app.add_startup_system(evil_brick);
    }
}

#[derive(Component)]
pub struct DbgBrick;

fn evil_brick(mut commands:Commands,
    asset_server:Res<AssetServer>) {
    commands.spawn()
        .insert(DbgBrick)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(200.0,0.0,0.5),
                ..default()
            },
            texture: asset_server.load("sprites/evil_brick.png"),
            ..default()
        })
        .insert(crate::char::events::Collider);
}