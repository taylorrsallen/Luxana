use crate::*;

use bevy::{
    pbr::{
        ScreenSpaceAmbientOcclusionBundle,
        ScreenSpaceAmbientOcclusionSettings,
        ScreenSpaceAmbientOcclusionQualityLevel
    },
    core_pipeline::{
        experimental::taa::TemporalAntiAliasBundle,
        clear_color::ClearColorConfig,
        tonemapping::Tonemapping
    },
    render::{
        camera::TemporalJitter,
        view::RenderLayers
    }
};

////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn onsys_spawn_primary_player(
    mut commands: Commands,
) {
    let player_entity = Players::spawn_default_player(&mut commands);
    commands.entity(player_entity)
        .insert(PrimaryPlayer)
        .insert(Name::new("PrimaryPlayer"));
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// We go through the trouble of keeping player ids so that we know which player is better at any given moment.
pub fn sys_update_player_ids(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Id), (With<Player>, Without<PrimaryPlayer>)>,
    removed_players: RemovedComponents<Player>,
) {
    if removed_players.is_empty() { return; }
    
    let mut players: Vec<(Entity, u32)> = player_query.iter().map(|(entity, id)| { (entity, id.get()) }).collect();
    players.sort_by_key(|(_, id)| { *id });

    for (i, (player_entity, _)) in players.iter().copied().enumerate() {
        commands.entity(player_entity).insert(Id::new((i + 1) as u32));
    }
}

pub fn sys_init_added_players(
    mut commands: Commands,
    mut player_id_query: Query<&mut Id, With<Player>>,
    added_players: Query<Entity, (Added<Player>, Without<PrimaryPlayer>)>,
) {
    if added_players.is_empty() { return; }
    
    let mut next_id = player_id_query.iter().max_by_key(|id| id.get()).map(|id| id.get()).unwrap();
    for player_entity in added_players.iter() {
        next_id += 1;

        let mut id = if let Ok(id) = player_id_query.get_mut(player_entity) { id } else { continue };
        id.set(next_id);
    }
}

pub fn sys_update_changed_player_cameras(
    mut commands: Commands,
    mut camera_query: Query<&mut Camera>,
    player_id_query: Query<&Id, With<Player>>,
    changed_gui_camera_query: Query<(Entity, &PlayerGuiCameraRef), Changed<PlayerGuiCameraRef>>,
    changed_main_camera_query: Query<(Entity, &PlayerMainCameraRef), Changed<PlayerMainCameraRef>>,
) {
    for (player_entity, gui_camera_ref) in changed_gui_camera_query.iter() {
        let id = if let Ok(id) = player_id_query.get(player_entity) { id.get() } else { continue };
        let camera_entity = if let Some(entity) = *gui_camera_ref.try_get() { entity } else { continue };
        let mut camera = if let Ok(camera) = camera_query.get_mut(camera_entity) { camera } else { continue };

        camera.order = id as isize + 16;
        commands.entity(camera_entity).insert(RenderLayers::layer(id as u8 + 16));
    }

    for (player_entity, main_camera_ref) in changed_main_camera_query.iter() {
        let id = if let Ok(id) = player_id_query.get(player_entity) { id.get() } else { continue };
        let mut camera = if let Some(camera_entity) = *main_camera_ref.try_get() {
                if let Ok(camera) = camera_query.get_mut(camera_entity) { camera } else { continue }
            } else { continue };

        camera.order = id as isize;
    }
}

pub fn sys_update_primary_player_devices(
    mut commands: Commands,
    player_query: Query<&InputDeviceReceiver, Without<PrimaryPlayer>>,
    primary_player_query: Query<Entity, With<PrimaryPlayer>>,
    added_players: Query<Entity, Added<Player>>,
    removed_players: RemovedComponents<Player>,
) {
    if added_players.is_empty() && removed_players.is_empty() { return; }

    let primary_player_entity = if let Ok(entity) = primary_player_query.get_single() { entity } else { return };
    let mut primary_receiver = InputDeviceReceiver::from_devices(&[
        InputDevice::Keyboard(0),
        InputDevice::Mouse(0),
        InputDevice::Gamepad(0),
        InputDevice::Gamepad(1),
        InputDevice::Gamepad(2),
        InputDevice::Gamepad(3),
    ]);

    for receiver in player_query.iter() {
        for device in receiver.get_devices() { primary_receiver.disable_device(device); }
    }

    commands.entity(primary_player_entity).insert(primary_receiver);
}