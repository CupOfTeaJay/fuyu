//! TODO: Document.

use bevy::prelude::*;

use crate::registrar::Registrar;

/// TODO: Document.
pub struct Applicator {
    /// TODO: Document.
    _registrar: Registrar,
}

impl Applicator {
    /// TODO: Document.
    fn _add_systems(&mut self, world: &mut World) -> &mut Self {
        world.resource_scope(|_world: &mut World, mut schedules: Mut<Schedules>| {
            for (name, systems) in self._registrar.systems() {
                if let Some(schedule) = schedules.get_mut(systems.0) {
                    println!("Scheduling {}", name);
                    schedule.add_systems(systems.1);
                }
            }
        });
        self
    }

    /// TODO: Document.
    fn _register_components(&mut self, world: &mut World) -> &mut Self {
        for component in self._registrar.components() {
            world.register_component_with_descriptor(component);
        }
        self
    }

    /// TODO: Document.
    fn _register_resources(&mut self, world: &mut World) -> &mut Self {
        for resource in self._registrar.resources() {
            world.register_resource_with_descriptor(resource);
        }
        self
    }

    /// TODO: Document.
    pub fn _apply(&mut self, world: &mut World) {
        self._register_resources(world)
            ._register_components(world)
            ._add_systems(world);
    }

    /// TODO: Document.
    pub fn new() -> Self {
        Applicator {
            _registrar: Registrar::new(),
        }
    }

    /// TODO: Document.
    pub fn _registrar(&mut self) -> &mut Registrar {
        &mut self._registrar
    }
}
