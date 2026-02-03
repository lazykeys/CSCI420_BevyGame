
/*
*   I just followed a tutorial on how to make snake to understand
*   basic syntax and the entity component system architecture. This
*   obviously isn't our project haha.
*
*   Also, I labelled anything that I found was similar to Unity's
*   specific functions or other systems, to help us better grasp
*   Bevy. - Jagger
*/

use bevy::prelude::*;
//use bevy::window::PrimaryWindow;

//main() is where our app-building code goes, which usually involves hooking up Systems to each other (I explain what a System is below).
fn main() {
    App::new() //this tells Bevy that we're making an application.

        //plugins are like Unity's packages. Default plugins include important
        //ones like the one that makes the window appear.
        .add_plugins(DefaultPlugins)
        
        //Systems perform logic on specific components. Think of how Unity's
        //Physics system performs logic on all Rigidbody components.
        //In here, we state that we want to add the setup_camera System at Startup,
        //which exactly like calling a function in Unity's Start() function.
        .add_systems(Startup, (setup_camera, spawn_snake))
        .add_systems(Update, snake_movement) // <- Update is called every frame, just like in Unity
        //.add_systems(PostUpdate, (position_translation, size_scaling))

        .run(); //this tells Bevy to finally run the project after completing all of the above function calls.
}

//=====Systems=====//

//this function is a System that spawns a camera in the World, so we can see what's going on!
fn setup_camera(mut commands: Commands) { //Commands are a queue of instructions that build on the World (the game's data)
    commands.spawn(Camera2d::default()); //.spawn is just like Instantiate(). It spawns an Entity in the World.
}

//=====Snake Components & Systems=====//

//this tag tells Bevy that the SnakeHead struct below is a Component. Components are exactly how they are in Unity.
#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::linear_rgb(0.7, 0.7, 0.7);

//this System spawns the snake's head
fn spawn_snake(mut commands: Commands)
{
    commands
        .spawn(Sprite::from_color(SNAKE_HEAD_COLOR, vec2(10.0, 10.0)))
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3})
        .insert(Size::tile(0.8));
}

//this System controls the snake's movement. The Query type in the parameters searches through all entities
//and returns the given types. To move the snake, we need a reference to a Transform that has the SnakeHead Component.
fn snake_movement(

    //a Res, or Resource, is a single global instance of some data type that works
    //independently of entities. They are basically Singletons.
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>
) {
    let up: bool = keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW);
    let down: bool = keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS);
    let left: bool = keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA);
    let right: bool = keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD);

    let speed = 5.0;

    for mut transform in head_positions.iter_mut() {

        if up {
            transform.translation.y += speed;
        }

        if down {
            transform.translation.y -= speed;
        }

        if left {
            transform.translation.x -= speed;
        }

        if right {
            transform.translation.x += speed;
        }
    }
}

//=====Grid Components & Systems=====//

const GRID_WIDTH: u32 = 10;
const GRID_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32
}

impl Size {
    pub fn tile(x: f32) -> Self {
        Self {
            width: x,
            height: x
        }
    }
}

// //applies sprite scaling to all Transforms with a Size component
// fn size_scaling(
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     mut size_positions: Query<(&Size, &mut Transform)>
// ) {
//     let Ok(window) = window_query.single() else { return; }; //panics if there is more than one window
    
//     for (sprite_size, mut transform) in size_positions.iter_mut() {
//         transform.scale = Vec3::new(
//             sprite_size.width / GRID_WIDTH as f32 * window.width(),
//             sprite_size.height / GRID_HEIGHT as f32 * window.height(),
//             1.0
//         );
//     }
// }

// fn position_translation(
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     mut transform_positions: Query<(&Position, &mut Transform)>
// ) {
//     fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
//         let tile_size = bound_window / bound_game;
//         pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
//     }

//     let Ok(window) = window_query.single() else { return; }; //panics if there is more than one window

//     for (pos, mut transform) in transform_positions.iter_mut() {
//         transform.translation = Vec3::new(
//             convert(pos.x as f32, window.width(), GRID_WIDTH as f32),
//             convert(pos.y as f32, window.height(), GRID_HEIGHT as f32),
//             0.0
//         );
//     }
// }

