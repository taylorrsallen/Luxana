use crate::*;

use std::io::Cursor;
use bevy::{asset::{LoadContext, AssetLoader, LoadedAsset}, utils::BoxedFuture, reflect::{TypeUuid, TypePath}};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct AudioAssetPlugin;
impl Plugin for AudioAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<SoundSource>()
            .init_asset_loader::<OggLoader>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, TypeUuid, TypePath)]
#[uuid = "6a9fc4ca-b5b5-94d6-613c-522e2d9fe86d"]
pub struct SoundSource {
    pub sound: StaticSoundData,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct OggLoader;

impl AssetLoader for OggLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let mut sound_bytes = vec![];
            for byte in bytes { sound_bytes.push(*byte); }
            
            let sound = StaticSoundData::from_cursor(Cursor::new(sound_bytes), StaticSoundSettings::default())?;
            load_context.set_default_asset(LoadedAsset::new(SoundSource { sound }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ogg", "oga", "spx"]
    }
}