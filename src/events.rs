<<<<<<< HEAD

use crate::dealer::DealerData;
use crate::player::PlayerData;
use crate::rounds::RoundData;

use std::io::*;
use rand::Rng;
use std::convert::TryInto;

#[doc = include_str!("../docs/events/eventdata.md")]
pub struct EventData {
    pub event_type: String,
    pub shooter_health: i16,
    pub victim_health: i16,
    pub is_double_dmg: bool,
    pub used_items: Option<[String;3]>
}

#[doc = include_str!("../docs/events/init_event_data.md")]
pub fn init_event_data(current_event_type: String, shooter_h: i16, victim_h: i16, idd: bool, current_used_items: Option<[String; 3]>) -> EventData {

    if current_used_items != None {
        EventData {
            event_type: current_event_type,
            shooter_health: shooter_h,
            victim_health: victim_h,
            is_double_dmg: idd,
            used_items: current_used_items
        }
    } else {
        EventData {
            event_type: current_event_type,
            shooter_health: shooter_h,
            victim_health: victim_h,
            is_double_dmg: idd,
            used_items: None
        }
    }

}

pub fn event_shot(mut ammo_data: [i16;3], round_data: &mut RoundData, player_data: &mut PlayerData, dealer_data: &mut DealerData) -> bool {

    let mut status: bool = false;

        if player_data.turn == true {
            println!("Player turn to shot ! (health : {})", player_data.health);
            print!("You want to shoot you or the dealer ? (y/d): ");
            let mut input = String::new();
            stdout().flush().unwrap();
            stdin().read_line(&mut input).expect("An error occurred while reading your command.");

            if input.trim() == "y" {
                // CHECK IF THE AMMO IS BLANK
                // CHECK IDD -------- FEATURE TO MAKE ASAP --------
                // IF NOT BLANK SUBTRACT THE HEALTH OF THE PLAYER
                let mut rdm_ammo: i16 = rand::thread_rng().gen_range(1..ammo_data.len()+1).try_into().unwrap();
                if rdm_ammo == 1 {
                    if ammo_data[1] > 0 {
                        ammo_data[1] = ammo_data[1] - 1;
                        player_data.health = player_data.health - 1;
                        player_data.turn = false;
                        println!("Damn you seem to have a pretty strong headache...");
                    } else if ammo_data[2] == 0 && ammo_data[1] == 0 {
                        player_data.turn = true;
                        if round_data.stage < 4 {
                            if round_data.round > 0 || round_data.round < 5 {
                                round_data.round = round_data.round + 1;
                            } else if round_data.round == 5 {
                                round_data.round = 1;
                                round_data.stage = round_data.stage + 1;
                            } else {
                                println!("[ERROR] - Unexpected error while occurring round_data !");
                            }
                        }
                    } else {
                        // IT'S A BLANK AMMO
                        // IF rdm_ammo[2] > 0 { *run the code for a blank ammo* }
                        rdm_ammo = 2;
                    }
                } else if rdm_ammo == 2 {
                    if ammo_data[2] > 0 {
                        ammo_data[2] = ammo_data[2] - 1;
                        player_data.turn = true;
                        println!("Thanks god i'm safe !");
                    } else if ammo_data[2] == 0 || ammo_data[1] == 0 {
                        player_data.turn = true;
                        if round_data.stage < 4 {
                            if round_data.round > 0 || round_data.round < 5 {
                                round_data.round = round_data.round + 1;
                                println!("Next round !");
                            } else if round_data.round == 5 {
                                round_data.round = 1;
                                round_data.stage = round_data.stage + 1;
                                println!("I hope u are ready for the next stage...");
                            } else {
                                println!("[ERROR] - Unexpected error while occurring round_data !");
                            }
                        }
                    } else {
                        rdm_ammo = 1;
                    }
                }
            } else if input.trim() == "d" {
                // CHECK IF THE AMMO IS BLANK
                // CHECK IDD -------- FEATURE TO MAKE ASAP --------
                // IF NOT BLANK SUBTRACT THE HEALTH OF THE DEALER
                let mut rdm_ammo: i16 = rand::thread_rng().gen_range(1..ammo_data.len()+1).try_into().unwrap();
                if rdm_ammo == 1 {
                    if ammo_data[1] > 0 {
                        ammo_data[1] = ammo_data[1] - 1;
                        dealer_data.health = dealer_data.health - 1;
                        player_data.turn = false;
                        println!("Bang what u think about this Mrs. Dealer ?!");
                    } else if ammo_data[2] == 0 && ammo_data[1] == 0 {
                        player_data.turn = true;
                        if round_data.stage < 4 {
                            if round_data.round > 0 || round_data.round < 5 {
                                round_data.round = round_data.round + 1;
                                println!("Next round !");
                            } else if round_data.round == 5 {
                                round_data.round = 1;
                                round_data.stage = round_data.stage + 1;
                                println!("I hope u are ready for the next stage...");
                            } else {
                                println!("[ERROR] - Unexpected error while occurring round_data !");
                            }
                        }
                    } else {
                        // This don't run the ' if rdm_ammo == 2 '...
                        rdm_ammo = 2;
                    }
                } else if rdm_ammo == 2 {
                    if ammo_data[2] > 0 {
                        ammo_data[2] = ammo_data[2] - 1;
                        player_data.turn = true;
                        println!("Thanks god i'm safe !");
                    } else if ammo_data[2] == 0 || ammo_data[1] == 0 {
                        player_data.turn = true;
                        if round_data.stage < 4 {
                            if round_data.round > 0 || round_data.round < 5 {
                                round_data.round = round_data.round + 1;
                                println!("Next round !")
                            } else if round_data.round == 5 {
                                round_data.round = 1;
                                round_data.stage = round_data.stage + 1;
                                println!("I hope u are ready for the next stage...");
                            } else {
                                println!("[ERROR] - Unexpected error while occurring round_data !");
                            }
                        }
                    } else {
                        rdm_ammo = 1;
                    }
                }
            } else {
                let ipt_len = input.trim().len();
                println!("Wrong input ! (len : {ipt_len} - input : {input})");
            }

        } else {
            println!("Dealer turn to shot ! (health : {})", dealer_data.health);
            player_data.turn = true;
        }

    return status;

=======

use crate::dealer::DealerData;
use crate::player::PlayerData;
use crate::rounds::RoundData;

use std::io::*;
use rand::Rng;

pub struct EventData {
    pub event_type: String,
    pub shooter_health: i16,
    pub victim_health: i16,
    pub is_double_dmg: bool,
    pub used_items: Option<[String;3]>
}

pub fn init_event_data(current_event_type: String, shooter_h: i16, victim_h: i16, idd: bool, current_used_items: Option<[String; 3]>) -> EventData {

    if current_used_items != None {
        EventData {
            event_type: current_event_type,
            shooter_health: shooter_h,
            victim_health: victim_h,
            is_double_dmg: idd,
            used_items: current_used_items
        }
    } else {
        EventData {
            event_type: current_event_type,
            shooter_health: shooter_h,
            victim_health: victim_h,
            is_double_dmg: idd,
            used_items: None
        }
    }

}

pub fn event_shot(mut ammo_data: [i16;3], round_data: &mut RoundData, player_data: &mut PlayerData, dealer_data: &mut DealerData) -> bool {

    let mut status: bool = false;

        if player_data.turn == true {
            println!("Player turn to shot ! (health : {})", player_data.health);
            print!("You want to shoot you or the dealer ? (y/d): ");
            let mut input = String::new();
            stdout().flush().unwrap();
            stdin().read_line(&mut input).expect("An error occurred while reading your command.");

            if input.trim() == "y" {
                // CHECK IF THE AMMO IS BLANK
                // CHECK IDD -------- FEATURE TO MAKE ASAP --------
                // IF NOT BLANK SUBTRACT THE HEALTH OF THE PLAYER
                let mut rdm_ammo: i16 = rand::thread_rng().gen_range(1..ammo_data.len()+1).try_into().unwrap();
                if rdm_ammo == 1 {
                    if ammo_data[1] > 0 {
                        ammo_data[1] = ammo_data[1] - 1;
                        player_data.health = player_data.health - 1;
                        player_data.turn = false;
                        println!("Damn you seem to have a pretty strong headache...");
                    } else if ammo_data[2] == 0 && ammo_data[1] == 0 {
                        player_data.turn = true;
                        if round_data.stage < 4 {
                            if round_data.round > 0 || round_data.round < 5 {
                                round_data.round = round_data.round + 1;
                            } else if round_data.round == 5 {
                                round_data.round = 1;
                                round_data.stage = round_data.stage + 1;
                            } else {
                                println!("[ERROR] - Unexpected error while occurring round_data !");
                            }
                        }
                    } else {
                        rdm_ammo = 2;
                    }
                } else if rdm_ammo == 2 {
                    if ammo_data[2] > 0 {
                        ammo_data[2] = ammo_data[2] - 1;
                        player_data.turn = true;
                        println!("Thanks god i'm safe !");
                    } else if ammo_data[2] == 0 || ammo_data[1] == 0 {
                        player_data.turn = true;
                        if round_data.stage < 4 {
                            if round_data.round > 0 || round_data.round < 5 {
                                round_data.round = round_data.round + 1;
                                println!("Next round !");
                            } else if round_data.round == 5 {
                                round_data.round = 1;
                                round_data.stage = round_data.stage + 1;
                                println!("I hope u are ready for the next stage...");
                            } else {
                                println!("[ERROR] - Unexpected error while occurring round_data !");
                            }
                        }
                    } else {
                        rdm_ammo = 1;
                    }
                }
            } else if input.trim() == "d" {
                // CHECK IF THE AMMO IS BLANK
                // CHECK IDD -------- FEATURE TO MAKE ASAP --------
                // IF NOT BLANK SUBTRACT THE HEALTH OF THE DEALER
                let mut rdm_ammo: i16 = rand::thread_rng().gen_range(1..ammo_data.len()+1).try_into().unwrap();
                if rdm_ammo == 1 {
                    if ammo_data[1] > 0 {
                        ammo_data[1] = ammo_data[1] - 1;
                        dealer_data.health = dealer_data.health - 1;
                        player_data.turn = false;
                        println!("Bang what u think about this Mrs. Dealer ?!");
                    } else if ammo_data[2] == 0 && ammo_data[1] == 0 {
                        player_data.turn = true;
                        if round_data.stage < 4 {
                            if round_data.round > 0 || round_data.round < 5 {
                                round_data.round = round_data.round + 1;
                                println!("Next round !");
                            } else if round_data.round == 5 {
                                round_data.round = 1;
                                round_data.stage = round_data.stage + 1;
                                println!("I hope u are ready for the next stage...");
                            } else {
                                println!("[ERROR] - Unexpected error while occurring round_data !");
                            }
                        }
                    } else {
                        // This don't run the ' if rdm_ammo == 2 '...
                        rdm_ammo = 2;
                    }
                } else if rdm_ammo == 2 {
                    if ammo_data[2] > 0 {
                        ammo_data[2] = ammo_data[2] - 1;
                        player_data.turn = true;
                        println!("Thanks god i'm safe !");
                    } else if ammo_data[2] == 0 || ammo_data[1] == 0 {
                        player_data.turn = true;
                        if round_data.stage < 4 {
                            if round_data.round > 0 || round_data.round < 5 {
                                round_data.round = round_data.round + 1;
                                println!("Next round !")
                            } else if round_data.round == 5 {
                                round_data.round = 1;
                                round_data.stage = round_data.stage + 1;
                                println!("I hope u are ready for the next stage...");
                            } else {
                                println!("[ERROR] - Unexpected error while occurring round_data !");
                            }
                        }
                    } else {
                        rdm_ammo = 1;
                    }
                }
            } else {
                let ipt_len = input.trim().len();
                println!("Wrong input ! (len : {ipt_len} - input : {input})");
            }

        } else {
            println!("Dealer turn to shot ! (health : {})", dealer_data.health);
        }

    return status;

>>>>>>> 0e34cea7993ea588f7e9cc267844f677be6365f3
}