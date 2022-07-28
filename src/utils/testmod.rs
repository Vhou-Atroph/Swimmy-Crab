//! Test module. Prints "swim swim" on startup and nothing else.

use bevy::prelude::*;

fn swimswim() {
    println!("swim swim")
}

pub struct TestMod;
impl Plugin for TestMod {
    fn build(&self,app:&mut App) {
        app.add_startup_system(swimswim);
    }
}