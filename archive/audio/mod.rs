use crate::*;

use std::sync::{Arc, Mutex, MutexGuard};

use kira::{
    manager::{AudioManager, backend::DefaultBackend, AudioManagerSettings},
    track::{TrackBuilder, effect::{filter::{FilterBuilder, FilterHandle},
    reverb::{ReverbBuilder, ReverbHandle}}, TrackHandle},
    sound::{static_sound::{StaticSoundSettings, StaticSoundHandle}, PlaybackState},
    tween::Tween,
};

mod asset;
pub use asset::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LuxanaAudioPlugin;
impl Plugin for LuxanaAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                AudioAssetPlugin,
                ObjectDevPlugin::<AudioSpatialEmitter>::default(),
            ))
            .register_type::<AudioState>()
            .insert_resource(AudioState::default())
            .insert_resource(AudioPlayer::default())
            .add_event::<AudioUiEvent>()
            .add_event::<AudioSpatialEvent>()
            .add_event::<AudioBgmEvent>()
            .add_systems(Last, (
                evsys_spawn_ui_sound,
                evsys_spawn_spatial_sound,
                sys_update_spatial_audio,
            ).chain());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct AudioState {
    max_sounds: u32,
    pub current_sounds: u32,
}

impl Default for AudioState {
    fn default() -> Self { Self { max_sounds: 56, current_sounds: 0 } }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Resource)]
pub struct AudioPlayer(Arc<Mutex<AudioManager>>);

impl Default for AudioPlayer {
    fn default() -> Self { Self { 0: Arc::new(Mutex::new(AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap())) } }
}

impl AudioPlayer {
    fn manager(&mut self) -> Option<MutexGuard<'_, AudioManager, >> {
        if let Ok(manager) = self.0.lock() { Some(manager) } else { None }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
pub struct AudioSpatialReceiver;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
struct AudioSpatialEmitter {
    decibles: f32,
    sound: StaticSoundHandle,
    track: TrackHandle,
    cutoff: FilterHandle,
    reverb: ReverbHandle,
}

impl DevName for AudioSpatialEmitter {
    fn dev_name() -> &'static str { "AudioSpatialEmitter" }
}

#[derive(Bundle)]
struct SpatialEmitterBundle {
    transform: TransformBundle,
    emitter: AudioSpatialEmitter,
}

impl SpatialEmitterBundle {
    fn new(
        position: Vec3,
        decibles: f32,
        sound: StaticSoundHandle,
        track: TrackHandle,
        cutoff: FilterHandle,
        reverb: ReverbHandle,
    ) -> Self {
        Self {
            transform: TransformBundle {
                local: Transform::from_translation(position),
                global: GlobalTransform::from(Transform::from_translation(position)),
            },
            emitter: AudioSpatialEmitter { decibles, sound, track, cutoff, reverb },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Event)]
pub struct AudioUiEvent { pub sound_id: u32, pub volume: f32 }

#[derive(Event)]
pub struct AudioSpatialEvent { pub sound_id: u32, pub decibles: f32, pub position: Vec3 }

#[derive(Event)]
pub enum AudioBgmEvent {
    Play { sound_id: u32, volume: f32, layer: u8, repeat: bool },
    Stop { layer: u8 },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
struct SpatialSoundSettings {
    volume: f32,
    panning: f32,
    cutoff: f32,
}

impl SpatialSoundSettings {
    fn new(
        emitter_position: Vec3,
        emitter_decibles: f32,
        receiver_position: Vec3,
        receiver_right: Vec3,
    ) -> Self {
        let sound_path = emitter_position - receiver_position;
        let distance = sound_path.length();
        
        let base_intensity = 10.0_f32.powf(emitter_decibles / 10.0) * (1.0 * 10.0_f32.powi(-12));
        let final_intensity = (1.0 / distance.powi(2)) * base_intensity;
        let final_decibles = 10.0 * ((final_intensity / (1.0 * 10.0_f32.powi(-12)))).log10();
        let volume = (final_decibles / 120.0).clamp(0.0, 1.0);

        let right_ear_angle = receiver_right.angle_between(sound_path);
        let panning = (right_ear_angle.cos() + 1.0) / 2.0;

        let cutoff = 500.0 + volume * 19_500.0;

        Self { volume, panning, cutoff }
    }

    fn closest_receiver<'world>(
        emitter_position: Vec3,
        receiver_query: &'world Query<&GlobalTransform, With<AudioSpatialReceiver>>,
    ) -> Option<&'world GlobalTransform> {
        let (mut closest_distance, mut closest_receiver_transform) = (f32::MAX, None);
        for receiver_transform in receiver_query.iter() {
            let distance = emitter_position.distance(receiver_transform.translation());
            if distance < closest_distance {
                closest_distance = distance;
                closest_receiver_transform = Some(receiver_transform);
            }
        }

        closest_receiver_transform
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn evsys_spawn_ui_sound(
    mut commands: Commands,
    mut events: EventReader<AudioUiEvent>,
    mut audio_player: ResMut<AudioPlayer>,
    mut audio_state: ResMut<AudioState>,
    audio_sources: Res<Assets<SoundSource>>,
    receiver_query: Query<&GlobalTransform, With<AudioSpatialReceiver>>,
    packages: Res<Packages>,
) {
    let mut manager = if let Some(manager) = audio_player.manager() { manager } else { return };
    for event in events.iter() {
        if audio_state.current_sounds > audio_state.max_sounds - 1 { return; }
        // audio_state.current_sounds += 1;

        let cutoff;
        let reverb;
        let track = manager.add_sub_track({
                let mut track = TrackBuilder::new();
                cutoff = track.add_effect(FilterBuilder::new().cutoff(20_000.0));
                reverb = track.add_effect(ReverbBuilder::new().mix(0.35));
                track
            }).unwrap();

        let source_handle = packages.sounds.handle(event.sound_id);
        let source = audio_sources.get(&source_handle).unwrap();

        let sound_data = source.sound.with_settings(
            StaticSoundSettings::new()
                .output_destination(&track)
                .volume(event.volume as f64)
        );

        let sound = manager.play(sound_data).unwrap();

        // commands.spawn(SpatialEmitterBundle::new(event.position, event.decibles, sound, track, cutoff, reverb));
    }
}

fn evsys_spawn_spatial_sound(
    mut commands: Commands,
    mut events: EventReader<AudioSpatialEvent>,
    mut audio_player: ResMut<AudioPlayer>,
    mut audio_state: ResMut<AudioState>,
    audio_sources: Res<Assets<SoundSource>>,
    receiver_query: Query<&GlobalTransform, With<AudioSpatialReceiver>>,
    packages: Res<Packages>,
) {
    let mut manager = if let Some(manager) = audio_player.manager() { manager } else { return };
    for event in events.iter() {
        if audio_state.current_sounds > audio_state.max_sounds - 1 { return; }
        audio_state.current_sounds += 1;

        let mut sound_settings = SpatialSoundSettings::default();
        if let Some(receiver_transform) = SpatialSoundSettings::closest_receiver(event.position, &receiver_query) {
            sound_settings = SpatialSoundSettings::new(event.position, event.decibles, receiver_transform.translation(), receiver_transform.right());
        }

        let cutoff;
        let reverb;
        let track = manager.add_sub_track({
                let mut track = TrackBuilder::new();
                cutoff = track.add_effect(FilterBuilder::new().cutoff(sound_settings.cutoff as f64));
                reverb = track.add_effect(ReverbBuilder::new().mix(0.0));
                track
            }).unwrap();

        let source_handle = packages.sounds.handle(event.sound_id);
        let source = audio_sources.get(&source_handle).unwrap();

        let sound_data = source.sound.with_settings(
            StaticSoundSettings::new()
                .output_destination(&track)
                .volume(sound_settings.volume as f64)
                .panning(sound_settings.panning as f64)
        );

        let sound = manager.play(sound_data).unwrap();

        commands.spawn(SpatialEmitterBundle::new(event.position, event.decibles, sound, track, cutoff, reverb));
    }
}

fn sys_update_spatial_audio(
    mut commands: Commands,
    mut emitter_query: Query<(Entity, &GlobalTransform, &mut AudioSpatialEmitter)>,
    mut audio_state: ResMut<AudioState>,
    receiver_query: Query<&GlobalTransform, With<AudioSpatialReceiver>>,
) {
    for (emitter_entity, emitter_transform, mut emitter) in emitter_query.iter_mut() {
        if emitter.sound.state() == PlaybackState::Stopped {
            commands.entity(emitter_entity).despawn_recursive();
            audio_state.current_sounds -= 1;
            continue;
        }

        let mut sound_settings = SpatialSoundSettings::default();
        if let Some(receiver_transform) = SpatialSoundSettings::closest_receiver(emitter_transform.translation(), &receiver_query) {
            sound_settings = SpatialSoundSettings::new(emitter_transform.translation(), emitter.decibles, receiver_transform.translation(), receiver_transform.right());
        }

        if emitter.sound.set_volume(sound_settings.volume as f64, Tween::default()).is_err() { println!("Could not set volume!") };
        if emitter.sound.set_panning(sound_settings.panning as f64, Tween::default()).is_err() { println!("Could not set panning!") };
        if emitter.cutoff.set_cutoff(sound_settings.cutoff as f64, Tween::default()).is_err() { println!("Could not set cutoff!") };
    }
}