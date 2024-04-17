use std::{env, fs::File, path::Path};

use bevy::{
    asset::io::{AssetReader, AssetReaderError, Reader},
    utils::BoxedFuture,
};

struct GameAssetReader(Box<dyn AssetReader>);

fn get_game_dir() -> String {
    env::var("GAME_DIR").unwrap_or(".".to_string())
}

impl AssetReader for GameAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        let full_path = path.join(get_game_dir());
        self.0.read(&full_path)
    }

    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        todo!()
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<bevy::asset::io::PathStream>, AssetReaderError>> {
        todo!()
    }

    fn is_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<bool, AssetReaderError>> {
        todo!()
    }
}
