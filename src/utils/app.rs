use bevy::{prelude::*,window::PresentMode};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub fn apprun() {
    App::new()
    .insert_resource(WindowDescriptor {
        title: "Swimmy Crab".to_string(),
        width: 550.,
        height: 650.,
        resizable: false,
        present_mode: PresentMode::Fifo,
        ..default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.3, 0.5)))
    .add_plugins(DefaultPlugins)
    .add_startup_system(twod_space)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(crate::utils::testmod::TestMod)
    .add_plugin(crate::char::bailey::BaileyPlugin)
    .add_plugin(crate::char::events::Events)
    .add_plugin(crate::char::dangers::DebugBrick) //only add this if you want to test death
    .run();
}

fn twod_space(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::new_with_far(100.));
}