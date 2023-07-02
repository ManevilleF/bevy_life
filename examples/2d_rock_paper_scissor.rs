use bevy::prelude::*;
use bevy_life::{CellState, CellularAutomatonPlugin, MooreCell2d, SimulationBatch};
use rand::Rng;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

impl RockPaperScissor {
    pub const fn beaten_by(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissor,
            Self::Scissor => Self::Rock,
        }
    }
}

impl CellState for RockPaperScissor {
    fn new_cell_state<'a>(&self, neighbor_cells: impl Iterator<Item = &'a Self>) -> Self {
        let beaten_by = self.beaten_by();
        let count = neighbor_cells
            .filter(|state| *state == &beaten_by)
            .count();
        if count > 2 {
            beaten_by
        } else {
            *self
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rock Paper Scissor".to_string(),
                resolution: [1200.0, 800.0].into(),
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugin(CellularAutomatonPlugin::<MooreCell2d, RockPaperScissor>::default())
        .insert_resource(SimulationBatch)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_map)
        .add_system(color_sprites)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_map(mut commands: Commands) {
    spawn_map(&mut commands);
}

fn spawn_map(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = (300, 200);
    let sprite_size = 4.;
    let color = Color::rgba(0., 0., 0., 0.);

    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            -(size_x as f32 * sprite_size) / 2.,
            -(size_y as f32 * sprite_size) / 2.,
            0.,
        )))
        .with_children(|builder| {
            for y in 0..=size_y {
                for x in 0..=size_x {
                    let state = match rng.gen_range(0.0..=1.0) {
                        x if x < 0.33 => RockPaperScissor::Rock,
                        x if x < 0.66 => RockPaperScissor::Paper,
                        _ => RockPaperScissor::Scissor,
                    };
                    builder.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(sprite_size)),
                                color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                sprite_size * x as f32,
                                sprite_size * y as f32,
                                0.,
                            ),
                            ..Default::default()
                        },
                        MooreCell2d::new(IVec2::new(x, y)),
                        state,
                    ));
                }
            }
        });
    println!("map generated");
}

pub fn color_sprites(
    mut query: Query<(&RockPaperScissor, &mut Sprite), Changed<RockPaperScissor>>,
) {
    for (state, mut sprite) in query.iter_mut() {
        match state {
            RockPaperScissor::Rock => sprite.color = Color::BLUE,
            RockPaperScissor::Paper => sprite.color = Color::BEIGE,
            RockPaperScissor::Scissor => sprite.color = Color::RED,
        }
    }
}
