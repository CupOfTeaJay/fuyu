//! TODO: Document.

use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use dylib::DynamicLibrary;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DynamicAppLoaderError {}

#[derive(Asset, Default, TypePath)]
pub struct DynamicApp {
    _library: Option<DynamicLibrary>,
}

impl AssetLoader for DynamicApp {
    type Asset = DynamicApp;
    type Settings = ();
    type Error = DynamicAppLoaderError;

    async fn load(
        &self,
        _reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<DynamicApp, Self::Error> {
        let path = load_context.path().path();
        println!("{:#?}", path);
        let library = DynamicLibrary::open(Some(path)).unwrap();
        Ok(DynamicApp { _library: Some(library) })
    }

    fn extensions(&self) -> &[&str] {
        &["dll", "dylib", "so"]
    }
}

// TODO: Safety.
unsafe impl Send for DynamicApp {}
unsafe impl Sync for DynamicApp {}
