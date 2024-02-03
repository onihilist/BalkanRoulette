<<<<<<< HEAD

pub struct DealerData {
    pub name: String,
    pub health: i16,
    pub turn: bool,
    pub last_shot: String,
    pub items: Option<[String;8]>
}

pub fn init_dealer_data() -> DealerData {

    DealerData {
        name: "Dealer".to_string(),
        health: 3,
        turn: false,
        last_shot: "".to_string(),
        items: None,
    }

}

pub fn get_dealer_health(dealer_data: DealerData) -> i16 {
    return dealer_data.health;
}
=======

pub struct DealerData {
    pub name: String,
    pub health: i16,
    pub turn: bool,
    pub last_shot: String,
    pub items: Option<[String;8]>
}

pub fn init_dealer_data() -> DealerData {

    DealerData {
        name: "Dealer".to_string(),
        health: 3,
        turn: false,
        last_shot: "".to_string(),
        items: None,
    }

}

pub fn get_dealer_health(dealer_data: DealerData) -> i16 {
    return dealer_data.health;
}
>>>>>>> 0e34cea7993ea588f7e9cc267844f677be6365f3
