use tetrio_loader::enums::garbageblocking_type::GarbageBlockingType;
use tetrio_loader::enums::passthrough_type::PassthroughType;
pub struct Options {
    version: i32,
    btb_chaining: bool,
    seed: i32,
    hasgarbage: i32,
    gravity_margin: i32,
    gravity_increase: f32,
    gravity_multiplier: f32,
    garbage_margin: i32,
    garbage_speed: i32,
    garbage_increase: f32,
    infinite_movement: bool,
    garbage_cap: f32,
    garbage_cap_max: f32,
    garbage_cap_increase: f32,
    allow180: bool,
    allow_harddrop: bool,
    display_hold: Option<bool>,
    lockresets: Option<i32>,
    locktime: Option<i32>,
    clip_listenids: bool,
    passthrough: PassthroughType,
    garbageblocking: GarbageBlockingType,
    nolockout: bool,
}
