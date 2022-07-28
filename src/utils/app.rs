use bevy::prelude::*;

pub fn apprun() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(crate::utils::testmod::TestMod)
    .run();
}