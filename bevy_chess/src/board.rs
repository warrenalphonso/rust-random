use bevy::{app::AppExit, prelude::*};
use bevy_mod_picking::*; 
use crate::pieces::*; 

pub struct PlayerTurn(pub PieceColor);
impl PlayerTurn {
    fn change(&mut self) {
        self.0 = match self.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}
impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColor::White)
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<PlayerTurn>()
            .init_resource::<SquareMaterials>()
            .add_event::<ResetSelectedEvent>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            .add_system(select_square.system())
            .add_system(move_piece.system())
            .add_system(select_piece.system())
            .add_system(despawn_taken_pieces.system())
            .add_system(reset_selected.system());
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

struct SquareMaterials {
    highlight_color: Handle<StandardMaterial>, 
    selected_color: Handle<StandardMaterial>,
    black_color: Handle<StandardMaterial>,
    white_color: Handle<StandardMaterial>,
}
impl FromResources for SquareMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap(); 
        SquareMaterials {
            highlight_color: materials.add(Color::rgb(0.8, 0.3, 0.3).into()),
            selected_color: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
            black_color: materials.add(Color::rgb(0., 0.1, 0.1).into()),
            white_color: materials.add(Color::rgb(1., 0.9, 0.9).into()),
        }
    }
}

fn create_board(
    commands: &mut Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    materials: Res<SquareMaterials>, 
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
                    materials.white_color.clone()
                } else {
                    materials.black_color.clone()
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

fn color_squares(
    pick_state: Res<PickState>, 
    selected_square: Res<SelectedSquare>, 
    materials: Res<SquareMaterials>,
    mut query: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>, 
) {
    // Get entity under cursor 
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    }; 

    for (entity, square, mut material) in query.iter_mut() {
        // Change material color 
        *material = if Some(entity) == top_entity {
            materials.highlight_color.clone()
        } else if Some(entity) == selected_square.entity {
            materials.selected_color.clone()
        } else if square.is_white() {
            materials.white_color.clone()
        } else {
            materials.black_color.clone()
        }; 
    }
}

fn select_square(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>, 
) {
    // Ensure left button is pressed 
    if !mouse_button_inputs.just_pressed(MouseButton::Left) { return; }

    // Get square under cursor and set as selected 
    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        selected_square.entity = Some(*square_entity); 
    } else {
        // Player clicked outside board so deselect everything 
        selected_square.entity = None; 
        selected_piece.entity = None; 
    }
}

// Selects piece in selected square 
fn select_piece(
    selected_square: ChangedRes<SelectedSquare>, 
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>, 
    squares_query: Query<&Square>, 
    pieces_query: Query<(Entity, &Piece)>,
) {
    let square_entity = if let Some(entity) = selected_square.entity {
        entity 
    } else {
        return;
    };

    let square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    if selected_piece.entity.is_none() {
        // Select piece in currently selected square 
        for (piece_entity, piece) in pieces_query.iter() {
            if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
                selected_piece.entity = Some(piece_entity); 
                break; 
            }
        }
    }
}

fn move_piece(
    commands: &mut Commands, 
    selected_square: ChangedRes<SelectedSquare>, 
    selected_piece: Res<SelectedPiece>, 
    mut turn: ResMut<PlayerTurn>, 
    squares_query: Query<&Square>, 
    mut pieces_query: Query<(Entity, &mut Piece)>,
    mut reset_selected_event: ResMut<Events<ResetSelectedEvent>>, // ??
) {
    let square_entity = if let Some(entity) = selected_square.entity {
        entity 
    } else {
        return; 
    };

    let square = if let Ok(square) = squares_query.get(square_entity) {
        square 
    } else { 
        return; 
    };

    if let Some(selected_piece_entity) = selected_piece.entity {
        let pieces_vec = pieces_query.iter_mut().map(|(_, piece)| *piece).collect(); 
        let pieces_entity_vec = pieces_query
            .iter_mut()
            .map(|(entity, piece)| (entity, *piece))
            .collect::<Vec<(Entity, Piece)>>(); 
        // Move selected piece to selected square 
        let mut piece = if let Ok((_piece_entity, piece)) = pieces_query.get_mut(selected_piece_entity) {
            piece
        } else {
            return;
        }; 
    
        if piece.is_move_valid((square.x, square.y), pieces_vec) {
            // Check if piece of opposite color exists and despawn it 
            for (other_entity, other_piece) in pieces_entity_vec {
                if other_piece.x == square.x 
                    && other_piece.y == square.y 
                    && other_piece.color != piece.color
                {
                    // Mark as Taken 
                    commands.insert_one(other_entity, Taken);
                }
            }

            // Move piece 
            piece.x = square.x;
            piece.y = square.y;

            // Change turn 
            turn.change();
        }

        reset_selected_event.send(ResetSelectedEvent);
    }
}

struct ResetSelectedEvent; 

fn reset_selected(
    mut event_reader: Local<EventReader<ResetSelectedEvent>>, 
    events: Res<Events<ResetSelectedEvent>>, 
    mut selected_square: ResMut<SelectedSquare>, 
    mut selected_piece: ResMut<SelectedPiece>,
) {
    for _event in event_reader.iter(&events) {
        selected_square.entity = None; 
        selected_piece.entity = None; 
    }
}

struct Taken; 

fn despawn_taken_pieces(
    commands: &mut Commands,
    mut app_exit_events: ResMut<Events<AppExit>>, 
    query: Query<(Entity, &Piece, &Taken)>,
) {
    for (entity, piece, _taken) in query.iter() {
        // If King was Taken, exit 
        if piece.piece_type == PieceType::King {
            println!("{} won!", match piece.color {
                PieceColor::White => "Black", 
                PieceColor::Black => "White",
            });
            app_exit_events.send(AppExit);
        }

        // Despawn piece and children 
        commands.despawn_recursive(entity);
    }
}
