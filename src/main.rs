use bevy::prelude::*;
use hachiya::DynamicApp;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_asset::<DynamicApp>()
        .init_asset_loader::<DynamicApp>()
        .add_systems(Startup, |asset_server: Res<AssetServer>| {
            let foo: Handle<DynamicApp> = asset_server.load("libmod_a.so");
        })
        .run();
}
