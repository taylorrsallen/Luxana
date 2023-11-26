use crate::*;

use bevy::render::camera::Viewport;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum TwoPlayerSplitPref {
    #[default]
    Horizontal,
    Vertical,
}

/// Where should the larger viewport be positioned?
#[derive(Default, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum ThreePlayerSplitPref {
    Left,
    Right,
    Bottom,
    #[default]
    Top,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource, Default)]
pub struct SplitscreenSettings {
    two_player_pref: TwoPlayerSplitPref,
    three_player_pref: ThreePlayerSplitPref,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn sys_mark_splitscreen_changes(
    mut window_query: Query<&mut Window>,
    removed_cameras: RemovedComponents<Camera>,
    removed_players: RemovedComponents<Player>,
) {
    if !removed_cameras.is_empty() || !removed_players.is_empty() {
        for mut window in window_query.iter_mut() { window.set_changed(); }
    }
}

pub fn sys_update_resized_camera_viewports(
    mut camera_query: Query<&mut Camera>,
    player_query: Query<(&Id, &PlayerMainCameraRef), With<Player>>,
    changed_window_query: Query<Entity, Changed<Window>>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    window_query: Query<&Window>,
    splitscreen_settings: Res<SplitscreenSettings>,
) {
    for changed_window_entity in changed_window_query.iter() {
        // (PlayerId, GuiCamera, MainCamera)
        let mut cameras_in_window = vec![];
        for (player_id, main_camera_ref) in player_query.iter() {
            let main_camera_entity = if let Some(entity) = main_camera_ref.try_get() { *entity } else { continue };
            let main_camera = if let Ok(camera) = camera_query.get(main_camera_entity) { camera } else { continue };
            let window_entity = Cameras::window_entity_from_camera(&main_camera, &primary_window_query);
            if window_entity == changed_window_entity { cameras_in_window.push((player_id.get(), main_camera_entity)); }
        }
        
        let camera_count = cameras_in_window.len() as u32;
        if camera_count == 0 || camera_count > 4 { continue; }
        cameras_in_window.sort_by_key(|(id, _)| { *id });
        
        let window = if let Ok(window) = window_query.get(changed_window_entity) { window } else { continue };
        let physical_size = Vec2::new(window.resolution.physical_width() as f32, window.resolution.physical_height() as f32);


        for (i, (player_id, main_camera_entity)) in cameras_in_window.iter().copied().enumerate() {
            let viewport = get_splitscreen_viewport(i as u32, camera_count, physical_size, &splitscreen_settings);
            camera_query.get_mut(main_camera_entity).unwrap().viewport = viewport.clone();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_splitscreen_viewport(order: u32, camera_count: u32, physical_size: Vec2, splitscreen_settings: &Res<SplitscreenSettings>) -> Option<Viewport> {
    match camera_count {
        2 => { get_two_player_viewport(order, physical_size, splitscreen_settings.two_player_pref) },
        3 => { get_three_player_viewport(order, physical_size, splitscreen_settings.three_player_pref) },
        4 => {
            Some(Viewport {
                physical_position: UVec2::new((physical_size.x * 0.5) as u32 * (order % 2), if order < 2 { 0 } else { (physical_size.y * 0.5) as u32 }),
                physical_size: (physical_size * 0.5).as_uvec2(),
                ..default()
            })
        },
        _ => { None },
    }
}

fn get_two_player_viewport(order: u32, physical_size: Vec2, preference: TwoPlayerSplitPref) -> Option<Viewport> {
    if preference == TwoPlayerSplitPref::Horizontal {
        Some(Viewport {
            physical_position: UVec2::new(0, (physical_size.y * 0.5) as u32 * order),
            physical_size: UVec2::new(physical_size.x as u32, (physical_size.y * 0.5) as u32),
            ..default()
        })
    } else {
        Some(Viewport {
            physical_position: UVec2::new((physical_size.x * 0.5) as u32 * order, 0),
            physical_size: UVec2::new((physical_size.x * 0.5) as u32, physical_size.y as u32),
            ..default()
        })
    }
}

fn get_three_player_viewport(order: u32, physical_size: Vec2, preference: ThreePlayerSplitPref) -> Option<Viewport> {
    if order == 0 {
        match preference {
            ThreePlayerSplitPref::Left => {
                Some(Viewport { physical_size: UVec2::new((physical_size.x * 0.5) as u32, physical_size.y as u32), ..default() })
            },
            ThreePlayerSplitPref::Right => {
                Some(Viewport {
                    physical_position: UVec2::new((physical_size.x * 0.5) as u32, 0),
                    physical_size: UVec2::new((physical_size.x * 0.5) as u32, physical_size.y as u32),
                    ..default()
                })
            },
            ThreePlayerSplitPref::Bottom => {
                Some(Viewport {
                    physical_position: UVec2::new(0, (physical_size.y * 0.5) as u32),
                    physical_size: UVec2::new(physical_size.x as u32, (physical_size.y * 0.5) as u32),
                    ..default()
                })
            },
            ThreePlayerSplitPref::Top => {
                Some(Viewport { physical_size: UVec2::new(physical_size.x as u32, (physical_size.y * 0.5) as u32), ..default() })
            },
        }
    } else {
        let position_offset = 0.5 * (order - 1) as f32;
        let mut position = physical_size;
        match preference {
            ThreePlayerSplitPref::Left => {
                position.x *= 0.5;
                position.y *= position_offset;
            },
            ThreePlayerSplitPref::Right => {
                position.x = 0.0;
                position.y *= position_offset;
            },
            ThreePlayerSplitPref::Bottom => {
                position.x *= position_offset;
                position.y = 0.0;
            },
            ThreePlayerSplitPref::Top => {
                position.x *= position_offset;
                position.y *= 0.5;
            },
        }

        Some(Viewport { physical_position: position.as_uvec2(), physical_size: (physical_size * 0.5).as_uvec2(), ..default() })
    }
}