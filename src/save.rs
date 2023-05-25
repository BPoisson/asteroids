use std::fs;

const SAVE_FILE_PATH: &str = "high_score_save.txt";

pub fn get_high_score() -> u64 {
    return match fs::read_to_string(SAVE_FILE_PATH) {
        Ok(score) => match score.parse::<u64>() {
            Ok(score_num) => score_num,
            Err(_) => 0
        }
        Err(_) => 0
    }
}

pub fn save_high_score(score: u64) -> () {
    if get_high_score() < score {
        fs::write(SAVE_FILE_PATH, score.to_string()).unwrap();
    }
}
