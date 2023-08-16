use crate::enums::MinoType;
use crate::environment::Environment;

pub enum Last {
    None,
    Rotate,
    Move,
    Fall,
}

pub enum SpinType {
    Null,
    Normal,
    Mini,
}

pub enum LastRotation {
    None,
    Right,
    Left,
    Vertical,
    Horizontal,
}

pub struct Falling {
    env: Environment,
    _type: MinoType,
    x: i32,
    aox: i32,
    y: f32,
    aoy: i32,
    r: i32,
    irs: i32,
    ihs: i32,
    safelock: i32,
    forcelock: bool,
    sleep: bool,
    deepsleep: bool,
    last: Last,
    lastrotation: LastRotation,
    spintype: SpinType,
    lastkick: i32,
    clamped: bool,
    lockresets: i32,
    locking: f32,
    floored: bool,
    highesty: f32,
}

impl Falling {
    pub fn new() {}

    pub fn init() {}

    pub fn is_highesty_over20() {}
}
