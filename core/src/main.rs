use bevy::{prelude::*, render::texture::ImageSettings};

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
#[derive(Component)]
struct Player;
#[derive(Component)]
struct PlayerOne {
    pub move_animation: FourWayMoveAnimation,
}
#[derive(Component)]
struct FourWayMoveAnimation {
    pub up: Handle<TextureAtlas>,
    right: Handle<TextureAtlas>,
    down: Handle<TextureAtlas>,
    left: Handle<TextureAtlas>,
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("../assets/animations/character/BODY_skeleton.png");
    let mut texture_atlas_vec: Vec<TextureAtlas> = (1..=4)
        .into_iter()
        .map(|direction| {
            TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(64.0, 64.0), 9, direction)
        })
        .collect();
    let player_one: PlayerOne = PlayerOne {
        move_animation: FourWayMoveAnimation {
            left: texture_atlases.add(texture_atlas_vec.pop().unwrap()),
            down: texture_atlases.add(texture_atlas_vec.pop().unwrap()),
            right: texture_atlases.add(texture_atlas_vec.pop().unwrap()),
            up: texture_atlases.add(texture_atlas_vec.pop().unwrap()),
        },
    };
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: player_one.move_animation.up,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}
