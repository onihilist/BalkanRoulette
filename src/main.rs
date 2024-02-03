<<<<<<< HEAD
use crate::ammo::AmmoData;
=======
>>>>>>> 0e34cea7993ea588f7e9cc267844f677be6365f3
use crate::player::PlayerData;
use crate::dealer::DealerData;
use crate::events::EventData;
use crate::rounds::RoundData;

mod player;
mod dealer;
mod events;
mod utils;
mod rounds;
mod ammo;
mod items;
mod rules;
mod menu;

fn main() {

    let mut player_data: PlayerData = player::init_player_data("Nihilist");
    let mut dealer_data: DealerData = dealer::init_dealer_data();
    let mut event_data: EventData = events::init_event_data(
        "shot".to_string(),
        player_data.health,
        dealer_data.health,
        false,
        None
    );
    let mut round_data: RoundData = rounds::init_round_data();
<<<<<<< HEAD
    let mut ammo_stage_one: [i16; 3] = ammo::generate_stage_one();

    // WHILE ROUND < 6 STAY STAGE 1
    /*while round_data.round < 6 { // MOVE THIS CONDITION INTO EVENT_SHOT
        if player_data.health > 0 {
            if dealer_data.health > 0 {
                println!("Round : {}", round_data.round);
                println!("Stage : {}", round_data.stage);
                events::event_shot(ammo_stage_one, &mut round_data, &mut player_data, &mut dealer_data);
                println!("Player health : {}", player_data.health);
                println!("Dealer health : {}", dealer_data.health);
            } else {
                println!("You defeat the dealer !");
                break;
            }
        } else {
            println!("You die...");
            break;
        }
    }*/

    println!("{}", items::found_item("handcuffs"));
=======

    // WHILE ROUND < 6 STAY STAGE 1
    println!("Round : {}", round_data.round);
    println!("Stage : {}", round_data.stage);
    events::event_shot(ammo::generate_stage_one(), &mut round_data, &mut player_data, &mut dealer_data);
>>>>>>> 0e34cea7993ea588f7e9cc267844f677be6365f3

}
