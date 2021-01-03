use bevy::prelude::*;
use bevy_mod_picking::*; 
use crate::pieces::*; 

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            .add_system(select_square.system());
    }
}

struct Square {
    pub x: u8, 
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y) % 2 == 1
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

fn create_board(commands: &mut Commands, mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Add meshes and materials 
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. })); 

    // Spawn 64 squares 
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(), 
                // Alternating pattern 
                material: if (i + j) % 2 == 1 {
                    materials.add(Color::rgb(1., 0.9, 0.9).into())
                } else {
                    materials.add(Color::rgb(0., 0.1, 0.1).into())
                }, 
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)), 
                ..Default::default()
            })
            .with(PickableMesh::default()) 
            .with(Square {
                x: i, 
                y: j,
            });
        }
    }
}

fn color_squares(pick_state: Res<PickState>, selected_square: Res<SelectedSquare>, 
    mut materials: ResMut<Assets<StandardMaterial>>, 
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>, 
) {
    // Get entity under cursor 
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    }; 

    for (entity, square, material_handle) in query.iter() {
        // Get material 
        let material = materials.get_mut(material_handle).unwrap(); 

        // Change material color 
        material.albedo = if Some(entity) == top_entity {
            Color::rgb(0.8, 0.3, 0.3)
        } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1., 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        }; 
    }
}

fn select_square(pick_state: Res<PickState>, 
    mouse_button_inputs: Res<Input<MouseButton>>, 
    mut selected_square: ResMut<SelectedSquare>, 
    mut selected_piece: ResMut<SelectedPiece>, 
    squares_query: Query<&Square>, 
    mut pieces_query: Query<(Entity, &mut Piece)>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get square under cursor and set it as selected 
    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        // Get actual square to ensure it exists and is a square 
        if let Ok(square) = squares_query.get(*square_entity) {
            // Mark as selected 
            selected_square.entity = Some(*square_entity); 
            if let Some(selected_piece_entity) = selected_piece.entity {
                let pieces_vec = pieces_query.iter_mut().map(|(_, piece)| *piece).collect();
                // Move selected piece to selected square 
                if let Ok((_piece_entity, mut piece)) = pieces_query.get_mut(selected_piece_entity) {
                    if piece.is_move_valid((square.x, square.y), pieces_vec) {
                        piece.x = square.x; 
                        piece.y = square.y;     
                    }
                }
                selected_square.entity = None; 
                selected_piece.entity = None;
            } else {
                // Select piece in currently selected square  
                for (piece_entity, piece) in pieces_query.iter_mut() {
                    if piece.x == square.x && piece.y == square.y {
                        selected_piece.entity = Some(piece_entity); 
                        break; 
                    }
                }
            }
        }
    } else {
        // Player clicked outside board, deselect everything 
        selected_square.entity = None; 
        selected_piece.entity = None; 
    }; 
}