use gdnative::prelude::*;
use gdnative::api::{Area2D, PhysicsBody2D};

#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_static)]
pub struct StaticArea2D {}

#[methods]
impl StaticArea2D {
    fn new(_owner: &Node) -> Self {
        StaticArea2D {}
    }

    fn register_static(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: crate::extensions::SIGNAL_NAME,
            args: &[],
        });
    }

    #[export]
    fn _on_body_entered(&self, owner: &Area2D, _body: Ref<PhysicsBody2D>) {
        godot_dbg!("_on_body_entered");
        owner.emit_signal(crate::extensions::SIGNAL_NAME, &[]);
    }

}
