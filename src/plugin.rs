//! TODO: Document.

use bevy::prelude::*;

use crate::dylib::Dylib;

fn process_dylibs(mut messages: MessageReader<AssetEvent<Dylib>>) {
    for message in messages.read() {
        println!("{:#?}", message);
    }
}

pub struct HachiyaPlugin;

impl Plugin for HachiyaPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Dylib>()
            .init_asset_loader::<Dylib>()
            .add_systems(Last, process_dylibs);
    }
}
