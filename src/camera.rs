use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    //Commands are a queue of instructions that build on the World (the game's data)
    commands.spawn(Camera2d::default()); //.spawn is just like Instantiate(). It spawns an Entity in the World.
}
