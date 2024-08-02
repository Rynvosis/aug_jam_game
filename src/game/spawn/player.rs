//! Spawn the player.

use bevy::prelude::*;

// use crate::game::input::BoxMovement;
use crate::config::PIXEL_PERFECT_LAYERS;
use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        movement::{Movement, MovementController, WrapWithinWindow},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
    // app.add_systems(Update, move_player);
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: image_handles[&ImageKey::Ducky].clone_weak(),
            transform: Transform::default(),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        MovementController::default(),
        Movement { speed: 16.0 },
        WrapWithinWindow,
        player_animation,
        StateScoped(Screen::Playing),
        PIXEL_PERFECT_LAYERS,
    ));
}

// fn move_player(
//     mut query: Query<(&mut Transform, &ActionState<BoxMovement>)>,
//     camera: Query<(&Camera, &GlobalTransform)>,
// ) {
//     let Ok((mut box_transform, action_state)) = query.get_single_mut() else {
//         warn!("No player found.");
//         return;
//     };
//     let (camera, camera_transform) = camera.single();
//
//     // Note: Nothing is stopping us from doing this in the action update system instead!
//     let Some(cursor_movement) = action_state.axis_pair(&BoxMovement::MousePosition) else {
//         warn!("No cursor movement detected.");
//         return;
//     };
//     let ray = camera
//         .viewport_to_world(camera_transform, cursor_movement.xy())
//         .unwrap();
//     let box_pan_vector = ray.origin.truncate();
//
//     box_transform.translation.x = box_pan_vector.x;
//     box_transform.translation.y = box_pan_vector.y;
// }
