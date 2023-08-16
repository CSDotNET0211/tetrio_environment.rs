mod constants;
mod enums;
mod environment;
mod falling;
mod gameData;
mod options;
mod player_options;
mod stats;
mod structs;

pub extern crate tetrio_loader;
use tetrio_loader::enums::spin_bonuses_type::SpinBonusesType;
fn main() {
    let data = SpinBonusesType::None;
    println!("aaa:{:?}", data);
    //let mut data = Environment::new();
}
