use crate::config::GRID_SIZE;
use crate::{AppSet, OuterCamera};
use bevy::app::{App, Update};
use bevy::math::{IVec2, Vec2};
use bevy::prelude::{
    Camera, GlobalTransform, Interaction, IntoSystemConfigs, Node, Query, ResMut, Resource, Window,
    With,
};
use bevy::window::PrimaryWindow;
use log::info;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MouseInfo>();
    app.add_systems(Update, update_mouse_info_system.in_set(AppSet::RecordInput));
}

#[derive(Resource, Default)]
pub struct MouseInfo {
    pub screen_pos: Vec2,
    pub grid_pos: IVec2,
    pub is_over_ui: bool,
}

fn update_mouse_info_system(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<OuterCamera>>,
    ui_nodes: Query<&Interaction, With<Node>>,
    mut mouse_info: ResMut<MouseInfo>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    if let Some(screen_position) = window.cursor_position() {
        let world_position = camera
            .viewport_to_world_2d(camera_transform, screen_position)
            .unwrap();
        // Update mouse info
        mouse_info.screen_pos = screen_position;
        mouse_info.grid_pos = (world_position / GRID_SIZE).floor().as_ivec2();

        info!("Mouse screen pos: {:?}", mouse_info.screen_pos);
        info!("Mouse grid pos: {:?}", mouse_info.grid_pos);
    }
    mouse_info.is_over_ui = ui_nodes
        .iter()
        .any(|interaction| !matches!(interaction, Interaction::None))
}
