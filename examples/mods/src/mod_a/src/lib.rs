use bevy::prelude::*;
use bevy::app::PostUpdate;

#[unsafe(no_mangle)]
pub fn init(world: &mut World) {
    world.resource_scope(|_world: &mut World, mut schedules: Mut<Schedules>| {
        if let Some(schedule) = schedules.get_mut(PostUpdate) {
            schedule.add_systems(hello);
        }
    });
}

fn hello() {
    println!("Hello from Mod A");
}
