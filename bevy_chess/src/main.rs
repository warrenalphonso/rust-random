use bevy::prelude::*; 

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
        .run();
}