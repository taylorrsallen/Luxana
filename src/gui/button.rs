use crate::*;

use bevy::{window::PrimaryWindow, render::view::RenderLayers};

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankGuiButtonPlugin;
impl Plugin for TankGuiButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GuiButtonState>()
            .register_type::<GuiButtonAction>()
            .register_type::<GuiButtonColors>()
            .add_event::<GuiButtonActionEvent>()
            .add_systems(Update, (
                sys_update_button_states,
                sys_update_button_colors,
                sys_deserialize_button_sounds,
                sys_update_button_sounds,
            ).chain());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle, Default)]
pub struct GuiButtonBundle {
    sprite: SpriteBundle,
    render_layers: RenderLayers,
    pos: GuiPos,
    color: GuiColor,
    button_state: GuiButtonState,
    button_action: GuiButtonAction,
    interactable_collider: GuiInteractableCollider,
    colliding_entities: GuiCollidingEntities,
    z_layer: GuiZLayer,
}

impl GuiButtonBundle {
    pub fn new(player_id: u32, menu_id: u16, action_id: u16) -> Self {
        Self {
            render_layers: RenderLayers::layer(player_id as u8 + 16),
            button_action: GuiButtonAction { menu_id, action_id },
            ..default()
        }
    }

    pub fn with_texture(mut self, texture: Handle<Image>, dims: Vec2) -> Self {
        self.sprite.texture = texture;
        self.interactable_collider.dims = dims;
        self
    }

    pub fn with_pos(mut self, pos: GuiPos, z_layer: i32) -> Self {
        self.pos = pos;
        self.z_layer = GuiZLayer::new(z_layer);
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.sprite.transform.scale = scale;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color.set(color);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// TODO: Goal is a subscriptor model for Gui events like button activations, but it has to wait.
/// 
/// For now just give buttons a unique ID and use a match. More complex Guis will be a pain.
#[derive(Event)]
pub struct GuiButtonActionEvent(pub Entity);

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum GuiButtonState {
    #[default]
    Idle,
    Selected,
    Pressed,
    Released,
}

impl GuiButtonState {
    fn try_set_selected(
        &mut self,
        released_this_update: bool,
    ) {
        if *self != GuiButtonState::Pressed && !released_this_update { *self = GuiButtonState::Selected; }
    }

    fn try_set_pressed(
        &mut self,
        button_entity: Entity,
        cursor: &mut GuiCursor,
    ) {
        if cursor.previous_state() != GuiCursorState::Pressed {
            *self = GuiButtonState::Pressed;
            cursor.active_button = Some(button_entity);
        } else if let Some(active_button) = cursor.active_button {
            if active_button == button_entity {
                *self = GuiButtonState::Pressed;
            } else {
                *self = GuiButtonState::Selected;
            }
        } else {
            *self = GuiButtonState::Selected;
        }
    }

    fn try_set_released(
        &mut self,
        button_entity: Entity,
        cursor: &GuiCursor,
        button_action_events: &mut EventWriter<GuiButtonActionEvent>,
        button_action_query: &Query<&GuiButtonAction>,
    ) -> bool {
        if let Some(active_button) = cursor.active_button {
            if active_button == button_entity {
                *self = GuiButtonState::Released;

                if let Ok(button_action) = button_action_query.get(button_entity) {
                    button_action_events.send(GuiButtonActionEvent { 0: button_entity });
                }

                return true;
            }
        }

        false
    }
}

fn sys_update_button_states(
    mut button_action_events: EventWriter<GuiButtonActionEvent>,
    mut button_query: Query<(Entity, &mut GuiButtonState, &GuiCollidingEntities), Changed<GuiCollidingEntities>>,
    mut cursor_query: Query<&mut GuiCursor, Changed<GuiCursor>>,
    button_action_query: Query<&GuiButtonAction>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (button_entity, mut button_state, colliding_entities) in button_query.iter_mut() {
        if colliding_entities.is_empty() { *button_state = GuiButtonState::Idle; continue; }

        let mut released_this_update = false;
        let mut new_button_state = *button_state;
        for colliding_entity in colliding_entities.iter() {
            let mut cursor = if let Ok(cursor) = cursor_query.get_mut(colliding_entity) { cursor } else { continue };
    
            match cursor.state() {
                GuiCursorState::Idle => { new_button_state.try_set_selected(released_this_update); }
                GuiCursorState::Pressed => { new_button_state.try_set_pressed(button_entity, &mut cursor); }
                GuiCursorState::Released => { released_this_update = new_button_state.try_set_released(button_entity, &cursor, &mut button_action_events, &button_action_query); }
            }
    
            if *button_state != new_button_state { *button_state = new_button_state; }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GuiButtonAction {
    pub menu_id: u16,
    pub action_id: u16,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GuiButtonColors {
    pub idle: Color,
    pub selected: Color,
    pub pressed: Color,
    pub released: Color,
}

impl GuiButtonColors {
    pub fn new(idle: Color, selected: Color, pressed: Color, released: Color) -> Self {
        Self { idle, selected, pressed, released }
    }

    pub fn color_from_state(&self, state: &GuiButtonState) -> Color {
        match state {
            GuiButtonState::Idle => { self.idle }
            GuiButtonState::Selected => { self.selected }
            GuiButtonState::Pressed => { self.pressed }
            GuiButtonState::Released => { self.released }
        }
    }
}

fn sys_update_button_colors(
    mut button_query: Query<(&mut GuiColor, &GuiButtonColors, &GuiButtonState), Changed<GuiButtonState>>,
) {
    for (mut color, colors, state) in button_query.iter_mut() {
        color.set(colors.color_from_state(state));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct SerializedGuiButtonSounds {
    pub selected: Option<String>,
    pub pressed: Option<String>,
    pub released: Option<String>,
}

impl SerializedGuiButtonSounds {
    pub fn new(selected: &str, pressed: &str, released: &str) -> Self {
        Self { selected: Some(selected.into()), pressed: Some(pressed.into()), released: Some(released.into()) }
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GuiButtonSounds {
    pub selected: Option<u32>,
    pub pressed: Option<u32>,
    pub released: Option<u32>,
}

impl GuiButtonSounds {
    pub fn try_get_sound_from_state(&self, state: &GuiButtonState) -> Option<u32> {
        match state {
            GuiButtonState::Selected => { self.selected }
            GuiButtonState::Pressed => { self.pressed }
            GuiButtonState::Released => { self.released }
            _ => { None }
        }
    }
}

fn sys_deserialize_button_sounds(
    mut commands: Commands,
    button_query: Query<(Entity, &SerializedGuiButtonSounds), Added<SerializedGuiButtonSounds>>,
    packages: Res<Packages>,
) {
    for (entity, sounds) in button_query.iter() {
        commands.entity(entity).remove::<SerializedGuiButtonSounds>()
            .insert(GuiButtonSounds {
                selected: if let Some(selected) = &sounds.selected { Some(packages.sounds.fetch_id(selected)) } else { None },
                pressed: if let Some(pressed) = &sounds.pressed { Some(packages.sounds.fetch_id(pressed)) } else { None },
                released: if let Some(released) = &sounds.released { Some(packages.sounds.fetch_id(released)) } else { None },
            });
    }
}

fn sys_update_button_sounds(
    mut events: EventWriter<StaticAudioEvent>,
    button_query: Query<(&GuiButtonSounds, &GuiButtonState), Changed<GuiButtonState>>,
) {
    for (sounds, state) in button_query.iter() {
        if let Some(sound_id) = sounds.try_get_sound_from_state(state) {
            events.send(StaticAudioEvent { sound_id, volume: 1.0 });
        }
    }
}