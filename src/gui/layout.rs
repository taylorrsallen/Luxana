// use crate::*;

// use std::fmt;
// use bevy::{window::{WindowScaleFactorChanged, WindowResized}, utils::HashMap};
// use taffy::{prelude::Size, style_helpers::TaffyMaxContent, Taffy};

// /// - Guis are viewport bound.
// /// - When a Gui is displayed to multiple viewports, we just copy the output image.
// /// - 

// ////////////////////////////////////////////////////////////////////////////////////////////////////
// pub struct LayoutContext {
//     pub scale_factor: f64,
//     pub physical_size: Vec2,
//     pub min_size: f32,
//     pub max_size: f32,
// }

// impl LayoutContext {
//     fn new(scale_factor: f64, physical_size: Vec2) -> Self {
//         Self {
//             scale_factor,
//             physical_size,
//             min_size: physical_size.x.min(physical_size.y),
//             max_size: physical_size.x.max(physical_size.y),
//         }
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////////////////////////
// pub fn sys_update_gui_layouts(
//     mut scale_factor_events: EventReader<WindowScaleFactorChanged>,
//     mut resize_events: EventReader<WindowResized>,
//     root_query: Query<&GuiRoot>,
//     primary_window_query: Query<Entity, With<PrimaryWindow>>,
//     window_query: Query<&Window>,
//     camera_query: Query<&Camera, With<Camera2d>>,
// ) {
//     let resize_events: Vec<&WindowResized> = resize_events.iter().collect();
//     for gui_root in root_query.iter() {
//         let camera_entity = if let Some(camera) = gui_root.camera { camera } else { continue };
//         let camera = if let Ok(camera) = camera_query.get(camera_entity) { camera } else { continue };
//         let window_entity = Cameras::window_entity_from_camera(camera, &primary_window_query);
//         let window = if let Ok(window) = window_query.get(window_entity) { window } else { continue };
    
//         let scale_factor = window.resolution.scale_factor();
//         let inverse_scale_factor = 1.0 / scale_factor;
//         let physical_size: Vec2;
//         if let Some(camera_viewport) = &camera.viewport {
//             physical_size = Vec2::new(camera_viewport.physical_size.x as f32, camera_viewport.physical_size.y as f32);
//         } else {
//             physical_size = Vec2::new(window.physical_width() as f32, window.physical_height() as f32);
//         }

//         let resized = resize_events.iter().any(|resized_window| resized_window.window == window_entity);
//         let layout_context = LayoutContext::new(scale_factor, physical_size);

//         if !scale_factor_events.is_empty() || resized {
//             // Update gui elements?
//         }

//         fn update_uinode_geometry_recursive(
//             entity: Entity,
//             ui_surface: &UiSurface,
//             node_transform_query: &mut Query<(&mut Node, &mut Transform)>,
//             children_query: &Query<&Children>,
//             inverse_scale_factor: f32,
//             parent_size: Vec2,
//             mut absolute_location: Vec2,
//         ) {
//             if let Ok((mut node, mut transform)) = node_transform_query.get_mut(entity) {
//                 let layout = ui_surface.get_layout(entity).unwrap();
//                 let layout_size = Vec2::new(layout.size.width, layout.size.height);
//                 let layout_location = Vec2::new(layout.location.x, layout.location.y);
    
//                 absolute_location += layout_location;
//                 let rounded_location = round_layout_coords(layout_location);
//                 let rounded_size = round_layout_coords(absolute_location + layout_size)
//                     - round_layout_coords(absolute_location);
    
//                 let new_size = inverse_scale_factor * rounded_size;
//                 let new_position =
//                     inverse_scale_factor * rounded_location + 0.5 * (new_size - parent_size);
    
//                 // only trigger change detection when the new values are different
//                 if node.calculated_size != new_size {
//                     node.calculated_size = new_size;
//                 }
//                 if transform.translation.truncate() != new_position {
//                     transform.translation = new_position.extend(0.);
//                 }
//                 if let Ok(children) = children_query.get(entity) {
//                     for &child_uinode in children {
//                         update_uinode_geometry_recursive(
//                             child_uinode,
//                             ui_surface,
//                             node_transform_query,
//                             children_query,
//                             inverse_scale_factor,
//                             new_size,
//                             absolute_location,
//                         );
//                     }
//                 }
//             }
//         }
//     }
// }