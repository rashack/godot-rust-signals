use gdnative::prelude::*;
use gdnative::api::Area2D;

use crate::extensions::NodeExt as _;

const SPEED: f32 = 100.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct MovingKinematicBody2D {
    velocity: Vector2,
}

#[methods]
impl MovingKinematicBody2D {
    fn new(_owner: &Node) -> Self {
        MovingKinematicBody2D {
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    #[export]
    fn _ready(&self, owner: TRef<KinematicBody2D>) {
        let static_area = unsafe { owner.get_typed_node::<Area2D, _>("../StaticArea2D") };
        static_area.connect(crate::extensions::SIGNAL_NAME, owner, "area_entered", VariantArray::new_shared(), 0).unwrap();
    }

    #[export]
    fn _process(&mut self, owner: &KinematicBody2D, _delta: f32) {
        let _pos = owner.position();

        let input = Input::godot_singleton();
        if input.is_action_pressed("ui_right") {
            self.velocity.x = SPEED;
        }
        if input.is_action_pressed("ui_left") {
            self.velocity.x = -SPEED;
        }

        self.velocity = owner.move_and_slide(
            Vector2::new(self.velocity.x, self.velocity.y),
            Vector2::new(0.0, -1.0),
            false,
            4,
            0.785398,
            true);

        self.velocity.x = 0.0;
    }

    #[export]
    fn area_entered(&mut self, _owner: &KinematicBody2D) {
        godot_dbg!("area_entered");
    }
}
