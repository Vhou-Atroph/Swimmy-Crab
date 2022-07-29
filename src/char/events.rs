//! Module for events and interactions between chars
use bevy::prelude::*;

pub struct Events;
impl Plugin for Events {
    fn build(&self,app:&mut App) {
        app.add_system(swim);
    }
}

#[derive(Component)]
pub struct Collider;

/// Check for collision between objects
fn collision_check() {

}

/// Moves a character (specifically, Bailey)
fn swim(buttons: Res<Input<MouseButton>>,) {
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) { //checks to see if either left or right mouse buttons were pressed
        println!("mouse button clicked: {:?}",buttons)
    }
}