#![allow(dead_code)]
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

struct PlayerScore {
    final_score: u32,
    nb_gold_medals: u32,
    nb_silver_medals: u32,
    nb_bronze_medals: u32,
}
struct PlayerDetails {
    my_pos: i32,
    my_stun: i32,
    my_id: i32,
}

impl PlayerDetails {
    fn default() -> Self {
        PlayerDetails {
            my_pos: 0,
            my_stun: 0,
            my_id: 0,
        }
    }

    fn new(my_pos: i32, my_stun: i32, my_id: i32) -> Self {
        PlayerDetails {
            my_pos,
            my_stun,
            my_id,
        }
    }
}

struct Game {
    track: Option<String>,
    closest_hurdle: Option<usize>,
    my_player: PlayerDetails,
}

impl Game {
    fn default() -> Self {
        Game {
            track: None,
            closest_hurdle: None,
            my_player: PlayerDetails::default(),
        }
    }

    fn new(my_player: PlayerDetails) -> Self {
        Game {
            track: None,
            closest_hurdle: None,
            my_player,
        }
    }

    // move 1 space
    fn left() {
        println!("LEFT");
    }

    // moves 3 spaces
    fn right() {
        println!("RIGHT");
    }

    // jump and move 2 spaces
    fn up() {
        println!("UP");
    }

    // move 2 spaces
    fn down() {
        println!("DOWN");
    }

    // do nothing
    fn nothing() {
        println!("LEFT");
    }

    fn add_track(&mut self, track: String) {
        self.track = Some(track);
    }

    fn add_player_details(&mut self, details: PlayerDetails) {
        self.my_player = details;
    }

    

    fn find_closest_hurdle(&self) -> Option<usize> {
        let track = self.track.as_ref().unwrap();
        let my_pos = self.my_player.my_pos as usize;
        let track = &track[my_pos+1..];
        track.split_once('#').map(|parts| parts.0.len())
    }

    fn decide(&mut self) -> String {
        if self.track.is_some() {
            let my_pos = self.my_player.my_pos as usize;

            // find closest hurdle if unknown
            if self.closest_hurdle.is_none() {
                self.closest_hurdle = self.find_closest_hurdle();
            }
            let closest_hurdle = self
                .closest_hurdle
                .map_or(self.track.as_ref().unwrap().len(), |val| val); // is len of track or len of track + 1
            let dist = closest_hurdle - my_pos;

            match dist {
                1 => {
                    self.closest_hurdle = None;
                    return String::from("UP");
                }
                2 => return String::from("LEFT"),
                3 => return String::from("DOWN"),
                _ => return String::from("RIGHT"),
            }
        }
        String::from("LEFT") // return nothing
    }
}

impl PlayerScore {
    fn calculate_final_score(&mut self) {
        self.final_score = self.nb_silver_medals + self.nb_gold_medals * 3;
    }

    fn increase_gold_medal(&mut self) {
        self.nb_gold_medals += 1;
        self.calculate_final_score();
    }

    fn increase_silver_medal(&mut self) {
        self.nb_silver_medals += 1;
        self.calculate_final_score();
    }

    fn increase_bronze_medal(&mut self) {
        self.nb_bronze_medals += 1;
    }

    fn final_score(&self) -> u32 {
        self.final_score
    }

    fn new(
        final_score: u32,
        nb_gold_medals: u32,
        nb_silver_medals: u32,
        nb_bronze_medals: u32,
    ) -> Self {
        PlayerScore {
            final_score,
            nb_gold_medals,
            nb_silver_medals,
            nb_bronze_medals,
        }
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // get my player id
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let player_idx = parse_input!(input_line, i32);

    // the number of simultaneously running mini-game = 1 (default)
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let nb_games = parse_input!(input_line, i32);

    // game loop
    loop {
        let mut player_scores: Vec<PlayerScore> = Vec::new();
        // get score info of each player before start of games
        for i in 0..3 as usize {
            // contains a breakdown of each player's final score
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let score_info = input_line.trim_matches('\n').to_string();
            let inputs = score_info.split(" ").collect::<Vec<_>>();
            assert_eq!(inputs.len(), 4);

            let final_score = parse_input!(inputs[0], u32);
            let nb_gold_medals = parse_input!(inputs[1], u32);
            let nb_silver_medals = parse_input!(inputs[2], u32);
            let nb_bronze_medals = parse_input!(inputs[3], u32);

            let player_score = PlayerScore::new(
                final_score,
                nb_gold_medals,
                nb_silver_medals,
                nb_bronze_medals,
            );
            player_scores.push(player_score);
        }

        // start each mini-game
        for i in 0..nb_games as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();

            // GPU: ASCII representation of the racetrack (GAME_OVER during reset turn)
            let gpu = inputs[0].trim().to_string();
            // assert_eq!(gp)

            // position of player 1
            let reg_0 = parse_input!(inputs[1], i32);

            // position of player 2
            let reg_1 = parse_input!(inputs[2], i32);

            // position of player 3
            let reg_2 = parse_input!(inputs[3], i32);

            // stun time for player 1
            let reg_3 = parse_input!(inputs[4], i32);

            // stun time for player 2
            let reg_4 = parse_input!(inputs[5], i32);

            // stun time for player 3
            let reg_5 = parse_input!(inputs[6], i32);

            // unused = -1
            let reg_6 = parse_input!(inputs[7], i32);

            let positions = [reg_0, reg_1, reg_2];
            let stuns = [reg_3, reg_4, reg_5];
            let my_pos = positions[player_idx as usize];
            let my_stun = stuns[player_idx as usize];
            let my_player = PlayerDetails {
                my_id: player_idx,
                my_pos,
                my_stun,
            };
            let mut game = Game::new(my_player);
            if gpu != "GAME_OVER" {
                game.add_track(gpu);
                game.decide();
            } else {
                Game::nothing();
            }
        }
    }
}

#[cfg(test)]
mod test_hurdle_up {
    use super::*;
    const RIGHT: &str = "RIGHT";
    const UP: &str = "UP";
    const DOWN: &str = "DOWN";
    const LEFT: &str = "LEFT";

    #[test]
    fn test_up() {
        let details = PlayerDetails::new(4, 0, 0);
        let track = ".....#...#...#................".to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);
        dbg!(game.find_closest_hurdle());

        assert_eq!(game.decide_v2(), UP.to_string());

        let details = PlayerDetails::new(29, 0, 0);
        let track = ".....#...#...#................".to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);

        dbg!(game.find_closest_hurdle());
        assert_eq!(game.decide_v2(), UP.to_string());
    }

    #[test]
    fn test_down() {
        let details = PlayerDetails::new(2, 0, 0);
        let track = ".....#...#...#................".to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);
        dbg!(game.find_closest_hurdle());

        assert_eq!(game.decide_v2(), DOWN.to_string());
    }

    #[test]
    fn test_left() {
        let details = PlayerDetails::new(3, 0, 0);
        let track = ".....#...#...#................".to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);
        dbg!(game.find_closest_hurdle());

        assert_eq!(game.decide_v2(), LEFT.to_string());
    }

    #[test]
    fn test_right() {
        let details = PlayerDetails::new(0, 0, 0);
        let track = ".....#...#...#................".to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);

        dbg!(game.find_closest_hurdle());
        assert_eq!(game.decide_v2(), RIGHT.to_string());

        let details = PlayerDetails::new(14, 0, 0);
        let track = ".....#...#...#................".to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);

        dbg!(game.find_closest_hurdle());
        assert_eq!(game.decide_v2(), RIGHT.to_string());

    }
}