use bevy::prelude::SystemSet;
use rand::{thread_rng, Rng};

#[derive(SystemSet, Debug, Hash, Eq, PartialEq, Clone)]
pub enum FrameSet {
    Input,
    GameLogic,
    EventHandling,
    Rendering,
}

pub fn get_random_meteor_sprite(big_ratio: f32) -> (usize, String) {
    let size_class: usize = if thread_rng().gen_bool(big_ratio as f64) {
        3
    } else {
        2
    };
    let size_name = match size_class {
        2 => "med",
        3 => "big",
        _ => panic!("Unsupported asteroid size class: {}", size_class),
    };
    let color = if thread_rng().gen_bool(0.5) {
        "Brown"
    } else {
        "Grey"
    };
    let variant = if size_class == 3 {
        thread_rng().gen_range(1..=4)
    } else {
        thread_rng().gen_range(1..=2)
    };

    (
        size_class,
        format!("Meteors/meteor{}_{}{}.png", color, size_name, variant),
    )
}
