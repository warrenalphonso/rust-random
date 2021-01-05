use crate::{board::*, pieces::*};
use bevy::prelude::*;

// Component to mark the Text entity 
struct NextMoveText; 

// Initialize UiCamera and text 
fn init_next_move_text(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());

    commands
        .spawn(CameraUiBundle::default())
        // root node 
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.), 
                    top: Val::Px(10.), 
                    ..Default::default()
                }, 
                ..Default::default()
            }, 
            material, 
            ..Default::default()
        })
        .with_children(|parent| {
            parent 
                .spawn(TextBundle {
                    text: Text {
                        value: "Next move: White".to_string(), 
                        font, 
                        style: TextStyle {
                            font_size: 40., 
                            color: Color::rgb(0.8, 0.8, 0.8),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(NextMoveText);
        });
}

// Update text with correct turn 
fn next_move_text_update(
    turn: ChangedRes<PlayerTurn>, // ChangedRes means Bevy only runs this when PlayerTurn changes, not every frame
    mut query: Query<(&mut Text, &NextMoveText)>,
) {
    for (mut text, _tag) in query.iter_mut() {
        text.value = format!(
            "Next move: {}", 
            match turn.0 {
                PieceColor::White => "White", 
                PieceColor::Black => "Black",
            }
        );
    }
}

pub struct UIPlugin; 
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(init_next_move_text.system())
            .add_system(next_move_text_update.system());
    }
}