use crate::*;

use bevy::{utils::{HashSet, petgraph::Graph}, window::PrimaryWindow};

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankGuiHitboxPlugin;
impl Plugin for TankGuiHitboxPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GuiInteractableCollider>()
            .register_type::<GuiCollidingEntities>()
            .add_event::<GuiCollisionEvent>()
            .add_systems(Update, (
                sys_update_collisions,
                evsys_update_gui_colliding_entities,
            ).chain());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Event, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GuiCollisionEvent {
    Started(Entity, Entity),
    Stopped(Entity, Entity),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Collides ONLY with GuiCursors. Scales with the Transform of the Entity.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GuiInteractableCollider {
    pub dims: Vec2,
}

impl GuiInteractableCollider {
    pub fn is_point_colliding(&self, point_px: Vec2, collider_pos_px: Vec2, collider_scale: &Vec3) -> bool {
        let scaled_collider = self.dims * Vec2::new(collider_scale.x, collider_scale.y);
        let upper_left_bound = collider_pos_px - scaled_collider + 1.0;
        let lower_right_bound = collider_pos_px + scaled_collider + 1.0;
        
        if point_px.x < upper_left_bound.x || point_px.x > lower_right_bound.x || point_px.y < upper_left_bound.y || point_px.y > lower_right_bound.y {
            false
        } else {
            true
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Component which will be filled (if present) with a list of entities with which the current entity is currently in contact.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GuiCollidingEntities(HashSet<Entity>);

impl GuiCollidingEntities {
    #[must_use] pub fn len(&self) -> usize { self.0.len() }
    #[must_use] pub fn is_empty(&self) -> bool { self.0.is_empty() }
    #[must_use] pub fn contains(&self, entity: Entity) -> bool { self.0.contains(&entity) }
    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ { self.0.iter().copied() }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_collisions(
    mut collision_events: EventWriter<GuiCollisionEvent>,
    root_query: Query<(&GuiRoot, &GuiViewport, &Children)>,
    cursor_query: Query<(Entity, &GuiCursor, &GuiCollidingEntities)>,
    collider_query: Query<(&GuiInteractableCollider, &GuiPos, &GuiZLayer, &Transform), With<Parent>>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    for (cursor_entity, cursor, cursor_colliding_entities) in cursor_query.iter() {
        let cursor_pos = if let Some(pos) = cursor.pos { pos } else { continue };
        let gui_root_entity = if let Some(entity) = cursor.gui_root { entity } else { continue };
        let (gui_root, gui_viewport, gui_children) = if let Ok(root) = root_query.get(gui_root_entity) { root } else { continue };

        let mut colliding_entity = None;
        let mut colliding_z_layer = i32::MIN;

        for child_entity in gui_children.iter() {
            let (collider, pos, z_layer, transform) = if let Ok(child) = collider_query.get(*child_entity) { child } else { continue };
            if z_layer.get() <= colliding_z_layer { continue; }
            if !collider.is_point_colliding(cursor_pos, pos.as_px_vec2(gui_viewport.get()), &transform.scale) { continue; }
            colliding_entity = Some(*child_entity);
            colliding_z_layer = z_layer.get();
        }
        
        if let Some(new_colliding_entity) = colliding_entity {
            for existing_colliding_entity in cursor_colliding_entities.iter() {
                if new_colliding_entity != existing_colliding_entity { collision_events.send(GuiCollisionEvent::Stopped(cursor_entity, existing_colliding_entity)); }
            }
            
            collision_events.send(GuiCollisionEvent::Started(cursor_entity, new_colliding_entity));
        } else {
            for existing_colliding_entity in cursor_colliding_entities.iter() {
                collision_events.send(GuiCollisionEvent::Stopped(cursor_entity, existing_colliding_entity));
            }
        }
    }
}

fn evsys_update_gui_colliding_entities(
    mut collision_events: EventReader<GuiCollisionEvent>,
    mut colliding_entities: Query<&mut GuiCollidingEntities>,
) {
    for event in collision_events.iter() {
        match event.to_owned() {
            GuiCollisionEvent::Started(entity0, entity1) => {
                if let Ok(mut entities) = colliding_entities.get_mut(entity0) { entities.0.insert(entity1); }
                if let Ok(mut entities) = colliding_entities.get_mut(entity1) { entities.0.insert(entity0); }
            }
            GuiCollisionEvent::Stopped(entity0, entity1) => {
                if let Ok(mut entities) = colliding_entities.get_mut(entity0) { entities.0.remove(&entity1); }
                if let Ok(mut entities) = colliding_entities.get_mut(entity1) { entities.0.remove(&entity0); }
            }
        }
    }
}