use gdnative::prelude::*;

mod extensions;
mod statik;
mod moving;

fn init(handle: InitHandle) {
    handle.add_class::<moving::MovingKinematicBody2D>();
    handle.add_class::<statik::StaticArea2D>();
}

godot_init!(init);
