use bevy::{animation::transition, prelude::*, render::camera};
use bevy::core_pipeline::motion_blur::*;
use bevy::core_pipeline::bloom::*;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy_framepace::*;


pub struct BananaPlugin;

impl Plugin for BananaPlugin { // made a plugin that calls spawn_scene and move_banana
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off);
        app.add_systems(Startup, spawn_scene);
        app.add_systems(Update, move_banana);
    }
}

#[derive(Component)]
pub struct Banana {
    velocity: Vec2,
}

static BANANA_RATIO: f32 = 1470. / 986.; // y to x ratio of the banana image
pub fn spawn_scene (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: ResMut<AssetServer>,
    mut frame_settings: ResMut<FramepaceSettings>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::AgX,
            ..default()
        },
        MotionBlur {
            shutter_angle: 2.0,
            samples: 2,
            ..default()
        },
        BloomSettings::default(),
    ));

    let banana: Handle<Image> = asset_server.load("imgs/nana.png"); //loading asset
    let banana_height: f32 = 80.0; 
    commands.spawn( (//sprite spawning
        Banana{velocity: Vec2::new(0., 0.)},
        SpriteBundle {
            texture: banana.clone(), //image
            sprite: Sprite { //size
                custom_size: Some(Vec2::new(banana_height * BANANA_RATIO, banana_height)),
                color: Color::srgb(5.0, 5.0, 5.0),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0), //position
            ..default()
        },
    ));

    frame_settings.limiter = Limiter::from_framerate(60.);
}

static BANANA_SPEED: f32 = 150.0;
pub fn move_banana (
    mut banana: Query<(&mut Transform, &mut Banana, &mut Sprite)>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&Window>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (camera_transform, camera) = camera_query.single();
    let window = window_query.single();
    let mut direction = Vec2::ZERO;

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Some(destination) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    for (mut banana_transform, mut banana_struct, mut sprite) in banana.iter_mut() {
        let mut desired_velocity = destination - banana_transform.translation.truncate();
        let distance = desired_velocity.length();
        

        if distance > 0. {
            let mut distance_quotient = 0.05;

            desired_velocity = desired_velocity.normalize() * distance * 0.7;
            banana_struct.velocity = (desired_velocity * distance_quotient) + (banana_struct.velocity * (1. - distance_quotient));
        }
        
        banana_transform.translation.x += banana_struct.velocity.x * time.delta_seconds() * BANANA_SPEED;
        banana_transform.translation.y += banana_struct.velocity.y * time.delta_seconds() * BANANA_SPEED;

        if distance > 100. {
            sprite.flip_x = banana_transform.translation.x > destination.x;
        }
    }
}