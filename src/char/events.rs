//! Module for events and interactions between chars
use bevy::{prelude::*,time::FixedTimestep};

//Constants
pub const TIME_STEP: f32 = 1.0 / 300.0;
pub const BAILEY_SPEED: f32 = 1000.0;

pub struct Events;
impl Plugin for Events {
    fn build(&self,app:&mut App) {
        app.add_system_set(SystemSet::new()
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            .with_system(swim)
            .with_system(cam)
            );
    }
}

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
struct CollisionEvent;

/// Check for collision against dangerous non-debug brick objects
fn collision_check() {

}

/// Checks for collisions against the top and bottom walls (debug bricks)
fn wall_collis() {

}

/// Moves a character (specifically, Bailey), when a mouse button is clicked
fn swim(buttons:Res<Input<MouseButton>>,
    mut query:Query<&mut Transform, With<crate::char::bailey::Bailey>>) {
    
    let mut bailey_trans=query.single_mut();
    let mut direction=-0.095;
    let mut dirx=0.0;

    if buttons.any_just_pressed([MouseButton::Left,MouseButton::Right]) { //checks to see if either left or right mouse buttons were pressed
        direction+=10.0;
        dirx+=7.5;
    }
    let bailey_posy=bailey_trans.translation.y+direction*BAILEY_SPEED*TIME_STEP;
    let bailey_posx=bailey_trans.translation.x+dirx*BAILEY_SPEED*TIME_STEP;
    bailey_trans.translation.y=bailey_posy;
    bailey_trans.translation.x=bailey_posx;
}

pub fn cam(buttons:Res<Input<MouseButton>>,
    mut query:Query<&mut Transform, With<Camera2d>>) { //would love to figure out how to combine moving things at the same time so i wouldnt have to make multiple functions like this- just throwing them in the same function seems to crash bevy

    let mut cam_trans=query.single_mut();
    let mut dirx=0.0;

    if buttons.any_just_pressed([MouseButton::Left,MouseButton::Right]) { //checks to see if either left or right mouse buttons were pressed
        dirx+=7.5;
    }
    let cam_posx=cam_trans.translation.x+dirx*BAILEY_SPEED*TIME_STEP;
    cam_trans.translation.x=cam_posx;
}