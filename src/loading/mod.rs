use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_common_assets::ron::RonAssetPlugin;

pub mod data;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<ConfigSeatHandles>()
                .load_collection::<ConfigTileHandles>()
            )
            .add_plugins(RonAssetPlugin::<data::SeatData>::new(&["seat.ron"]))
            .add_plugins(RonAssetPlugin::<data::TileData>::new(&["tile.ron"]));
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/witcher_16x16.png")]
    pub player: Handle<Image>,
    #[asset(path = "textures/wall.png")]
    pub wall_tile: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ConfigSeatHandles {
    #[asset(path = "config/player.seat.ron")]
    pub player: Handle<data::SeatData>,
}

#[derive(AssetCollection, Resource)]
pub struct ConfigTileHandles {
    #[asset(path = "config/wall.tile.ron")]
    pub wall: Handle<data::TileData>,
}