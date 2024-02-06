
pub struct PlayerData {
    pub name: String,
    pub health: i16,
    pub turn: bool,
    pub last_shot: String,
    pub items: Option<[String;8]>
}

pub fn init_player_data(username: &str) -> PlayerData {
    PlayerData {
        name: username.to_string(),
        health: 3,
        turn: true,
        last_shot: "".to_string(),
        items: None,
    }
}

pub fn get_player_health(player_data: PlayerData) -> i16 {
    return player_data.health;
}

pub fn set_player_health(player_data: &mut PlayerData, health: i16) {
    player_data.health = health;
}
