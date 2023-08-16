use crate::constants::constants;
use crate::enums::*;
use crate::environment::Environment;
use crate::options::Options;
use crate::player_options::PlayerOptions;
use crate::tetrio_loader;
use crate::SpinBonusesType;
use std::collections::{HashMap, VecDeque};

struct GameData {
    spinbonuses: SpinBonusesType,
    board: [MinoType; constants::BOARD_WIDTH * constants::BOARD_HEIGHT],
    current_btb_chain_power: u32,
    next: VecDeque<MinoType>,
    next_bag: Vec<MinoType>,
    options: Options,
    subframe: f32,
    lshift: bool,
    rshift: bool,
    last_shift: i32,
    ldas: f32,
    rdas: f32,
    handling: PlayerOptions,
    ldasiter: f32,
    rdasiter: f32,
    softdrop: bool,
    falling: Falling,
    hold_locked: bool,
    hold: MinoType,
    gravity: f32,
    falling_rotations: i32,
    total_rotations: i32,
    //impending_damages:Vec<igedata?>,
    garbage_actnowledgements: (HashMap<String, i32>, u32),
    garbage_id: i32,
    not_yet_received_attack: i32,
    targets: Vec<String>,
    interaction_id: i32,
    //waiting_frames:Vec<WaitingFramedata>
}

impl GameData {
    pub fn new(env: &Environment) {
        //  GameData {}
    }
}
