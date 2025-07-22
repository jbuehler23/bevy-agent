//! Game templates and code generation utilities

use crate::error::{BevyAIError, Result};
use handlebars::Handlebars;
use serde_json::{json, Value};
use std::collections::HashMap;

/// Template manager for Bevy AI
pub struct TemplateManager {
    handlebars: Handlebars<'static>,
}

/// Template context for code generation
#[derive(Debug, Clone)]
pub struct TemplateContext {
    pub project_name: String,
    pub description: String,
    pub features: Vec<String>,
    pub dependencies: Vec<String>,
    pub bevy_version: String,
    pub custom_variables: HashMap<String, Value>,
}

/// Game template definition
#[derive(Debug, Clone)]
pub struct GameTemplate {
    pub name: String,
    pub description: String,
    pub category: GameCategory,
    pub main_template: String,
    pub additional_files: HashMap<String, String>,
    pub dependencies: Vec<String>,
    pub features: Vec<String>,
}

/// Game categories for templates
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameCategory {
    Platformer,
    Shooter,
    Puzzle,
    Strategy,
    Rpg,
    Racing,
    Simulation,
    Arcade,
    Educational,
    Experimental,
}

impl TemplateManager {
    /// Create a new template manager
    pub fn new() -> Result<Self> {
        let mut handlebars = Handlebars::new();
        
        // Register built-in templates
        Self::register_builtin_templates(&mut handlebars)?;
        
        Ok(Self { handlebars })
    }
    
    /// Register built-in templates
    fn register_builtin_templates(handlebars: &mut Handlebars) -> Result<()> {
        // Basic game template
        handlebars.register_template_string("basic_game", BASIC_GAME_TEMPLATE)?;
        
        // 2D Platformer template
        handlebars.register_template_string("platformer_2d", PLATFORMER_2D_TEMPLATE)?;
        
        // 3D FPS template
        handlebars.register_template_string("fps_3d", FPS_3D_TEMPLATE)?;
        
        // Puzzle game template
        handlebars.register_template_string("puzzle_game", PUZZLE_GAME_TEMPLATE)?;
        
        // Strategy game template
        handlebars.register_template_string("strategy_game", STRATEGY_GAME_TEMPLATE)?;
        
        Ok(())
    }
    
    /// Generate code from template
    pub fn generate(&self, template_name: &str, context: &TemplateContext) -> Result<String> {
        let template_context = self.create_handlebars_context(context)?;
        
        let result = self.handlebars
            .render(template_name, &template_context)
            .map_err(|e| BevyAIError::Template(e))?;
        
        Ok(result)
    }
    
    /// Create handlebars context from template context
    fn create_handlebars_context(&self, context: &TemplateContext) -> Result<Value> {
        let mut handlebars_context = json!({
            "project_name": context.project_name,
            "description": context.description,
            "features": context.features,
            "dependencies": context.dependencies,
            "bevy_version": context.bevy_version,
        });
        
        // Add custom variables
        if let Some(object) = handlebars_context.as_object_mut() {
            for (key, value) in &context.custom_variables {
                object.insert(key.clone(), value.clone());
            }
        }
        
        Ok(handlebars_context)
    }
    
    /// Get available templates
    pub fn available_templates(&self) -> Vec<&str> {
        self.handlebars.get_templates().keys().map(|s| s.as_str()).collect()
    }
    
    /// Register custom template
    pub fn register_template(&mut self, name: &str, template: &str) -> Result<()> {
        self.handlebars
            .register_template_string(name, template)
            .map_err(|e| BevyAIError::TemplateCreation(e))?;
        Ok(())
    }
    
    /// Get built-in game templates
    pub fn builtin_templates() -> Vec<GameTemplate> {
        vec![
            GameTemplate {
                name: "basic_game".to_string(),
                description: "A basic Bevy game with camera, lighting, and a simple scene".to_string(),
                category: GameCategory::Educational,
                main_template: BASIC_GAME_TEMPLATE.to_string(),
                additional_files: HashMap::new(),
                dependencies: vec!["bevy".to_string()],
                features: vec!["3D rendering".to_string(), "Basic input".to_string()],
            },
            GameTemplate {
                name: "platformer_2d".to_string(),
                description: "A 2D platformer with player movement, physics, and collectibles".to_string(),
                category: GameCategory::Platformer,
                main_template: PLATFORMER_2D_TEMPLATE.to_string(),
                additional_files: HashMap::new(),
                dependencies: vec!["bevy".to_string()],
                features: vec!["2D sprites".to_string(), "Physics".to_string(), "Player movement".to_string()],
            },
            GameTemplate {
                name: "fps_3d".to_string(),
                description: "A 3D first-person shooter with player controller and basic enemies".to_string(),
                category: GameCategory::Shooter,
                main_template: FPS_3D_TEMPLATE.to_string(),
                additional_files: HashMap::new(),
                dependencies: vec!["bevy".to_string()],
                features: vec!["3D rendering".to_string(), "FPS controls".to_string(), "Shooting mechanics".to_string()],
            },
            GameTemplate {
                name: "puzzle_game".to_string(),
                description: "A puzzle game with grid-based mechanics and level progression".to_string(),
                category: GameCategory::Puzzle,
                main_template: PUZZLE_GAME_TEMPLATE.to_string(),
                additional_files: HashMap::new(),
                dependencies: vec!["bevy".to_string()],
                features: vec!["Grid system".to_string(), "Puzzle mechanics".to_string(), "Level management".to_string()],
            },
            GameTemplate {
                name: "strategy_game".to_string(),
                description: "A real-time strategy game with unit management and resource collection".to_string(),
                category: GameCategory::Strategy,
                main_template: STRATEGY_GAME_TEMPLATE.to_string(),
                additional_files: HashMap::new(),
                dependencies: vec!["bevy".to_string()],
                features: vec!["Unit management".to_string(), "Resource system".to_string(), "RTS mechanics".to_string()],
            },
        ]
    }
}

impl Default for TemplateManager {
    fn default() -> Self {
        Self::new().expect("Failed to create template manager")
    }
}

impl TemplateContext {
    /// Create a new template context
    pub fn new(project_name: String, description: String) -> Self {
        Self {
            project_name,
            description,
            features: Vec::new(),
            dependencies: vec!["bevy".to_string()],
            bevy_version: "0.12".to_string(),
            custom_variables: HashMap::new(),
        }
    }
    
    /// Add a feature
    pub fn with_feature(mut self, feature: String) -> Self {
        self.features.push(feature);
        self
    }
    
    /// Add a dependency
    pub fn with_dependency(mut self, dependency: String) -> Self {
        self.dependencies.push(dependency);
        self
    }
    
    /// Set Bevy version
    pub fn with_bevy_version(mut self, version: String) -> Self {
        self.bevy_version = version;
        self
    }
    
    /// Add custom variable
    pub fn with_variable<T: Into<Value>>(mut self, key: String, value: T) -> Self {
        self.custom_variables.insert(key, value.into());
        self
    }
}

impl std::fmt::Display for GameCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameCategory::Platformer => write!(f, "Platformer"),
            GameCategory::Shooter => write!(f, "Shooter"),
            GameCategory::Puzzle => write!(f, "Puzzle"),
            GameCategory::Strategy => write!(f, "Strategy"),
            GameCategory::Rpg => write!(f, "RPG"),
            GameCategory::Racing => write!(f, "Racing"),
            GameCategory::Simulation => write!(f, "Simulation"),
            GameCategory::Arcade => write!(f, "Arcade"),
            GameCategory::Educational => write!(f, "Educational"),
            GameCategory::Experimental => write!(f, "Experimental"),
        }
    }
}

// Template constants
const BASIC_GAME_TEMPLATE: &str = r#"// {{description}}
// Generated with Bevy AI

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "{{project_name}}".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            rotate_camera,
            {{#each features}}
            // TODO: Implement {{this}}
            {{/each}}
        ))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6.0, 12.0)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ..default()
        },
        MainCamera,
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(8.0, 8.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // Sample cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}

fn rotate_camera(
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<DirectionalLight>)>,
) {
    for mut transform in camera_query.iter_mut() {
        let radius = 12.0;
        let angle = time.elapsed_seconds() * 0.3;
        transform.translation.x = angle.cos() * radius;
        transform.translation.z = angle.sin() * radius;
        transform.look_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y);
    }
}
"#;

const PLATFORMER_2D_TEMPLATE: &str = r#"// {{description}}
// 2D Platformer generated with Bevy AI

use bevy::prelude::*;

const PLAYER_SPEED: f32 = 200.0;
const JUMP_STRENGTH: f32 = 400.0;
const GRAVITY: f32 = 800.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "{{project_name}}".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement,
            apply_gravity,
            camera_follow,
        ))
        .run();
}

#[derive(Component)]
struct Player {
    velocity: Vec2,
    grounded: bool,
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..default()
        },
        Player {
            velocity: Vec2::ZERO,
            grounded: false,
        },
    ));

    // Ground platforms
    let platform_positions = [
        Vec3::new(0.0, -200.0, 0.0),
        Vec3::new(200.0, -100.0, 0.0),
        Vec3::new(-200.0, 0.0, 0.0),
        Vec3::new(400.0, 50.0, 0.0),
    ];

    for position in platform_positions {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(100.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        });
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        let mut movement = 0.0;
        
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            movement -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            movement += 1.0;
        }
        
        player.velocity.x = movement * PLAYER_SPEED;
        
        if (keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::ArrowUp)) && player.grounded {
            player.velocity.y = JUMP_STRENGTH;
            player.grounded = false;
        }
        
        transform.translation.x += player.velocity.x * time.delta_seconds();
        transform.translation.y += player.velocity.y * time.delta_seconds();
    }
}

fn apply_gravity(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        if !player.grounded {
            player.velocity.y -= GRAVITY * time.delta_seconds();
        }
        
        // Simple ground collision (y = -200 is ground level)
        if transform.translation.y <= -184.0 {
            transform.translation.y = -184.0;
            player.velocity.y = 0.0;
            player.grounded = true;
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
"#;

const FPS_3D_TEMPLATE: &str = r#"// {{description}}
// 3D FPS generated with Bevy AI

use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::input::mouse::MouseMotion;

const MOVEMENT_SPEED: f32 = 5.0;
const MOUSE_SENSITIVITY: f32 = 0.002;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "{{project_name}}".to_string(),
                cursor: bevy::window::Cursor {
                    grab_mode: CursorGrabMode::Locked,
                    visible: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement,
            mouse_look,
        ))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerCamera {
    pitch: f32,
    yaw: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player (invisible, just a transform)
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 1.5, 3.0),
            ..default()
        },
        Player,
    )).with_children(|parent| {
        // Camera as child of player
        parent.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            PlayerCamera {
                pitch: 0.0,
                yaw: 0.0,
            },
        ));
    });

    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // Some cubes to shoot at
    for i in 0..5 {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb(0.8, 0.2, 0.2)),
            transform: Transform::from_xyz(
                (i as f32 - 2.0) * 3.0,
                0.5,
                -5.0,
            ),
            ..default()
        });
    }

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in player_query.iter_mut() {
        let mut movement = Vec3::ZERO;
        
        if keyboard.pressed(KeyCode::KeyW) {
            movement += transform.forward();
        }
        if keyboard.pressed(KeyCode::KeyS) {
            movement -= transform.forward();
        }
        if keyboard.pressed(KeyCode::KeyA) {
            movement -= transform.right();
        }
        if keyboard.pressed(KeyCode::KeyD) {
            movement += transform.right();
        }
        
        movement.y = 0.0; // Don't move up/down
        movement = movement.normalize_or_zero();
        
        transform.translation += movement * MOVEMENT_SPEED * time.delta_seconds();
    }
}

fn mouse_look(
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &mut PlayerCamera)>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    let mut delta = Vec2::ZERO;
    for motion in mouse_motion.read() {
        delta += motion.delta;
    }
    
    if delta.length_squared() > 0.0 {
        for (mut camera_transform, mut camera) in camera_query.iter_mut() {
            camera.yaw -= delta.x * MOUSE_SENSITIVITY;
            camera.pitch -= delta.y * MOUSE_SENSITIVITY;
            camera.pitch = camera.pitch.clamp(-1.5, 1.5);
            
            camera_transform.rotation = Quat::from_rotation_y(camera.yaw) * Quat::from_rotation_x(camera.pitch);
        }
        
        // Update player Y rotation to match camera yaw
        for mut player_transform in player_query.iter_mut() {
            if let Ok((_, camera)) = camera_query.get_single() {
                player_transform.rotation = Quat::from_rotation_y(camera.yaw);
            }
        }
    }
}
"#;

const PUZZLE_GAME_TEMPLATE: &str = r#"// {{description}}
// Puzzle Game generated with Bevy AI

use bevy::prelude::*;

const GRID_SIZE: usize = 8;
const TILE_SIZE: f32 = 32.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "{{project_name}}".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameGrid>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            update_visual_grid,
        ))
        .run();
}

#[derive(Resource)]
struct GameGrid {
    tiles: [[u8; GRID_SIZE]; GRID_SIZE],
    selected: Option<(usize, usize)>,
}

impl Default for GameGrid {
    fn default() -> Self {
        let mut tiles = [[0; GRID_SIZE]; GRID_SIZE];
        
        // Initialize with a simple pattern
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                tiles[x][y] = ((x + y) % 3) as u8;
            }
        }
        
        Self {
            tiles,
            selected: None,
        }
    }
}

#[derive(Component)]
struct GridTile {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct Selected;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: Res<GameGrid>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Create visual grid
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let world_x = (x as f32 - GRID_SIZE as f32 / 2.0) * TILE_SIZE;
            let world_y = (y as f32 - GRID_SIZE as f32 / 2.0) * TILE_SIZE;
            
            let color = match grid.tiles[x][y] {
                0 => Color::RED,
                1 => Color::GREEN,
                2 => Color::BLUE,
                _ => Color::WHITE,
            };
            
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(TILE_SIZE - 2.0, TILE_SIZE - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_x, world_y, 0.0),
                    ..default()
                },
                GridTile { x, y },
            ));
        }
    }
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut grid: ResMut<GameGrid>,
) {
    let mut new_selected = grid.selected;
    
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        if let Some((x, y)) = grid.selected {
            if y < GRID_SIZE - 1 {
                new_selected = Some((x, y + 1));
            }
        } else {
            new_selected = Some((GRID_SIZE / 2, GRID_SIZE / 2));
        }
    }
    
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        if let Some((x, y)) = grid.selected {
            if y > 0 {
                new_selected = Some((x, y - 1));
            }
        } else {
            new_selected = Some((GRID_SIZE / 2, GRID_SIZE / 2));
        }
    }
    
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        if let Some((x, y)) = grid.selected {
            if x > 0 {
                new_selected = Some((x - 1, y));
            }
        } else {
            new_selected = Some((GRID_SIZE / 2, GRID_SIZE / 2));
        }
    }
    
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        if let Some((x, y)) = grid.selected {
            if x < GRID_SIZE - 1 {
                new_selected = Some((x + 1, y));
            }
        } else {
            new_selected = Some((GRID_SIZE / 2, GRID_SIZE / 2));
        }
    }
    
    if keyboard.just_pressed(KeyCode::Space) {
        if let Some((x, y)) = grid.selected {
            // Cycle tile color
            grid.tiles[x][y] = (grid.tiles[x][y] + 1) % 3;
        }
    }
    
    grid.selected = new_selected;
}

fn update_visual_grid(
    grid: Res<GameGrid>,
    mut commands: Commands,
    mut tile_query: Query<(Entity, &mut Sprite, &GridTile), Without<Selected>>,
    selected_query: Query<Entity, With<Selected>>,
) {
    // Remove old selection
    for entity in selected_query.iter() {
        commands.entity(entity).remove::<Selected>();
    }
    
    // Update tile colors and add selection
    for (entity, mut sprite, tile) in tile_query.iter_mut() {
        let color = match grid.tiles[tile.x][tile.y] {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            _ => Color::WHITE,
        };
        
        sprite.color = color;
        
        // Add selection highlight
        if let Some((sel_x, sel_y)) = grid.selected {
            if tile.x == sel_x && tile.y == sel_y {
                sprite.color = Color::YELLOW;
                commands.entity(entity).insert(Selected);
            }
        }
    }
}
"#;

const STRATEGY_GAME_TEMPLATE: &str = r#"// {{description}}
// Strategy Game generated with Bevy AI

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "{{project_name}}".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .init_resource::<SelectedUnits>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            unit_selection,
            unit_movement,
            resource_generation,
            camera_movement,
        ))
        .run();
}

#[derive(Resource, Default)]
struct GameState {
    resources: u32,
}

#[derive(Resource, Default)]
struct SelectedUnits {
    units: Vec<Entity>,
}

#[derive(Component)]
struct Unit {
    health: f32,
    max_health: f32,
    unit_type: UnitType,
}

#[derive(Component)]
struct Selectable {
    selected: bool,
}

#[derive(Component)]
struct ResourceNode {
    resource_type: ResourceType,
    amount: u32,
}

#[derive(Component)]
struct MainCamera;

#[derive(Clone)]
enum UnitType {
    Worker,
    Soldier,
    Scout,
}

enum ResourceType {
    Gold,
    Stone,
    Wood,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 10.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // Spawn some units
    for i in 0..3 {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 1.0, 0.5)),
                material: materials.add(Color::BLUE),
                transform: Transform::from_xyz(i as f32 * 2.0 - 2.0, 0.5, 0.0),
                ..default()
            },
            Unit {
                health: 100.0,
                max_health: 100.0,
                unit_type: UnitType::Worker,
            },
            Selectable { selected: false },
        ));
    }

    // Spawn resource nodes
    let resource_positions = [
        (Vec3::new(5.0, 0.5, 5.0), ResourceType::Gold),
        (Vec3::new(-5.0, 0.5, 5.0), ResourceType::Stone),
        (Vec3::new(0.0, 0.5, -5.0), ResourceType::Wood),
    ];

    for (position, resource_type) in resource_positions {
        let color = match resource_type {
            ResourceType::Gold => Color::YELLOW,
            ResourceType::Stone => Color::GRAY,
            ResourceType::Wood => Color::rgb(0.6, 0.3, 0.1),
        };

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cylinder::new(0.8, 1.5)),
                material: materials.add(color),
                transform: Transform::from_translation(position),
                ..default()
            },
            ResourceNode {
                resource_type,
                amount: 100,
            },
        ));
    }
}

fn unit_selection(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut selectable_query: Query<&mut Selectable>,
    mut selected_units: ResMut<SelectedUnits>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        // Simple selection - select all units for now
        // In a real game, you'd use raycasting to select specific units
        selected_units.units.clear();
        
        for (entity, mut selectable) in selectable_query.iter_mut().enumerate() {
            selectable.selected = entity == 0; // Select first unit only
            if selectable.selected {
                selected_units.units.push(Entity::from_raw(entity as u32));
            }
        }
    }
}

fn unit_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    selected_units: Res<SelectedUnits>,
    mut unit_query: Query<(&mut Transform, &Selectable), With<Unit>>,
) {
    let mut movement = Vec3::ZERO;
    
    if keyboard.pressed(KeyCode::KeyW) {
        movement.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }
    
    if movement.length() > 0.0 {
        movement = movement.normalize() * 3.0 * time.delta_seconds();
        
        for (mut transform, selectable) in unit_query.iter_mut() {
            if selectable.selected {
                transform.translation += movement;
            }
        }
    }
}

fn resource_generation(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut last_generation: Local<f32>,
) {
    *last_generation += time.delta_seconds();
    
    if *last_generation >= 1.0 {
        game_state.resources += 10;
        *last_generation = 0.0;
        info!("Resources: {}", game_state.resources);
    }
}

fn camera_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in camera_query.iter_mut() {
        let mut movement = Vec3::ZERO;
        
        if keyboard.pressed(KeyCode::ArrowUp) {
            movement.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            movement.z += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowLeft) {
            movement.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            movement.x += 1.0;
        }
        
        if movement.length() > 0.0 {
            movement = movement.normalize() * 8.0 * time.delta_seconds();
            transform.translation += movement;
        }
    }
}
"#;
