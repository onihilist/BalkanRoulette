
use rand::Rng;

#[doc = include_str!("../docs/ammo/ammodata.md")]
pub struct AmmoData {
    blank_ammo: i16,
    letal_ammo: i16,
    total_ammo: i16
}

#[doc = include_str!("../docs/ammo/generate_stage.md")]
pub fn generate_stage_one() -> [i16;3] {

    let total_ammo: i16 = rand::thread_rng().gen_range(2..4);
    let blank_ammo: i16 = rand::thread_rng().gen_range(1..total_ammo);
    let letal_ammo: i16 = total_ammo - blank_ammo;

    return [total_ammo, letal_ammo, blank_ammo];
}

pub fn generate_stage_two() -> [i16;3] {
    let total_ammo: i16 = rand::thread_rng().gen_range(3..7);
    let substract_ammo: i16 = total_ammo - 1;
    let blank_ammo: i16 = rand::thread_rng().gen_range(2..substract_ammo);
    let letal_ammo: i16 = total_ammo - blank_ammo;

    return [total_ammo, letal_ammo, blank_ammo];
}
