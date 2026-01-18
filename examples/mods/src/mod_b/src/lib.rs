use bevy::prelude::*;
use hachiya::Registrar;

#[unsafe(no_mangle)]
pub fn init(registrar: &mut Registrar) {
    registrar.add_systems(PostUpdate, hello);
}

fn hello() {
    println!("Hello from Mod B");
}

