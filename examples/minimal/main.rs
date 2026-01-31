//! TODO:

use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use hachiya::{Dylib, HachiyaPlugin};

#[derive(Resource)]
struct DylibWrapper(Handle<Dylib>);

fn load_dylib(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(DylibWrapper(
        asset_server.load("target/debug/libminimal.so"),
    ))
}

fn main() {
    App::new()
        .add_plugins((
            AssetPlugin::default(),
            LogPlugin::default(),
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
                // Run 60 times per second.
                Duration::from_secs_f64(1.0 / 60.0),
            )),
        ))
        .add_plugins(HachiyaPlugin)
        .add_systems(Startup, load_dylib)
        .run();
}
