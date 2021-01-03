use bevy::prelude::*; 
use bevy_mod_picking::*; 

mod board; 
mod pieces; 
use board::*;
use pieces::*; 

fn main() {
    App::build()
        // Set antialiasing to use 4 samples (?!) 
        .add_resource(Msaa { samples: 4 }) // add greater realism to a digital image by smoothing jagged edges on curved lines and diagonals
        // Set WindowDescriptor Resource to change title and size of window 
        .add_resource(WindowDescriptor {
            title: "Chess!".to_string(), 
            width: 800., 
            height: 800.,
            ..Default::default() 
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(DebugPickingPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(create_board.system())
        .add_startup_system(create_pieces.system()) 
        .run();
}

fn setup(commands: &mut Commands) {
    commands
        // Camera 
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(), 
                Vec3::new(-7., 20., 4.),
            )), 
            ..Default::default()
        })
        .with(PickSource::default())
        // Light 
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4., 8., 4.)), 
            ..Default::default()
        }); 
}
