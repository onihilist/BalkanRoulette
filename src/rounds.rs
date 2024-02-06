

pub struct RoundData {
    pub round: i16,
    pub stage: i16,
    pub round_winner: [String;5],
    pub stage_winner: [String;3]
}

pub fn init_round_data() -> RoundData {

    RoundData {
        round: 1,
        stage: 1,
        round_winner: ["null".to_string(), "null".to_string(), "null".to_string(), "null".to_string(), "null".to_string()],
        stage_winner: ["null".to_string(), "null".to_string(), "null".to_string()]
    }

}

pub fn set_round_data(round_data: &mut RoundData, round: i16) {
    round_data.round = round;
}

pub fn set_stage_data(round_data: &mut RoundData, stage: i16) {
    round_data.stage = stage;
}

pub fn set_round_winner(round_data: &mut RoundData, round_winner: String) {
    let current_index = round_data.round_winner.len();
    round_data.round_winner[current_index + 1] = round_winner;
}