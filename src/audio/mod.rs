use crate::*;

pub use bevy_kira_audio::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankAudioPlugin;
impl Plugin for TankAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                bevy_kira_audio::AudioPlugin,
            ))
            .insert_resource(SpacialAudio { max_distance: 25.0 })
            .add_event::<StaticAudioEvent>()
            .add_event::<SpatialAudio3dEvent>()
            .add_event::<AudioBgmEvent>()
            .register_type::<SpatialAudio3dSource>()
            .add_systems(Last, (
                evsys_play_audio_ui,
                sys_init_spatial_audio_3d_source,
            ).chain());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Event)]
pub struct StaticAudioEvent {
    pub sound_id: u32,
    pub volume: f32,
}

fn evsys_play_audio_ui(
    mut events: EventReader<StaticAudioEvent>,
    audio: Res<Audio>,
    packages: Res<Packages>,
) {
    for event in events.read() {
        audio.play(packages.sounds.handle(event.sound_id).clone())
            .with_volume(event.volume as f64);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Event)]
pub struct SpatialAudio3dEvent {
    pub sound_id: u32,
    pub decibles: f32,
    pub position: Vec3,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Event)]
pub enum AudioBgmEvent {
    Play { sound_id: u32, volume: f32, layer: u8, repeat: bool },
    Stop { layer: u8 },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct SpatialAudio3dSource {
    pub sound_id: u32,
    pub looping: bool,
}

fn sys_init_spatial_audio_3d_source(
    mut commands: Commands,
    mut source_query: Query<(Entity, &mut SpatialAudio3dSource), Added<SpatialAudio3dSource>>,
    audio: Res<Audio>,
    packages: Res<Packages>,
) {
    for (entity, mut source) in source_query.iter_mut() {
        let mut audio_command = audio.play(packages.sounds.handle(source.sound_id).clone());
        if source.looping { audio_command.looped(); }
        commands.entity(entity).insert(AudioEmitter { instances: vec![audio_command.handle()]});
    }
}

// #[derive(Bundle)]
// pub struct SpatialAudio3dSourceBundle {
//     source: SpatialAudio3dSource,
//     audio_emitter: AudioEmitter,
//     transform: TransformBundle,
// }

// impl SpatialAudio3dSourceBundle {
//     pub fn new(sound_id: u32, looping: bool) -> Self {
//         Self {
            
//         }
//     }
// }