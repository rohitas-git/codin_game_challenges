#![allow(dead_code)]
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    fn left() -> String {
        // println!("LEFT");
        "LEFT".to_owned()
    }

    // moves 3 spaces
    fn right() -> String {
        // println!("RIGHT");
        "RIGHT".to_owned()
    }

    // jump and move 2 spaces
    fn up() -> String {
        // println!("UP");
        "UP".to_owned()
    }

    // move 2 spaces
    fn down() -> String {
        // println!("DOWN");
        "DOWN".to_owned()
    }

    // do nothing
    fn nothing() -> String {
        // println!("LEFT");
        "LEFT".to_owned()
    }

    fn add_track(&mut self, track: String) {
        self.track = Some(track);
    }

    fn add_player_details(&mut self, details: PlayerDetails) {
        self.my_player = details;
    }

    fn decide(&mut self) -> String {
        if self.track.is_some() {
            let my_pos = self.my_player.my_pos as usize;

            // find closest hurdle if unknown
            if self.closest_hurdle.is_none() {
                self.closest_hurdle = self.find_closest_hurdle();
            }

            // when no hurlde then is it len of track or len of track + 1
            let closest_hurdle = self
                .closest_hurdle
                .map_or(self.track.as_ref().unwrap().len(), |val| val);

            let dist = closest_hurdle - my_pos;
            dbg!(dist);
            match dist {
                1 => {
                    self.closest_hurdle = None;
                    return Self::up();
                }
                2 => return Self::left(),
                3 => return Self::down(),
                _ => return Self::right(),
            }
        }
        Self::nothing()
    }

    fn find_closest_hurdle(&self) -> Option<usize> {
        let track = self.track.as_ref().unwrap();
        let my_pos = self.my_player.my_pos as usize;
        let ahead_track = &track[my_pos..];
        ahead_track
            .split_once('#')
            .map(|parts| parts.0.len() + my_pos)
    }

    fn make_move(my_move: String) {
        println!("{my_move}");
    }

    fn move_to_value(this_move: &String) -> u8 {
        match this_move.as_str() {
            "LEFT" => 1,
            "UP" => 2,
            "DOWN" => 2,
            "RIGHT" => 3,
            _ => 99,
        }
    }

    fn value_to_move(val: u8) -> String {
        match val {
            1 => "LEFT".to_owned(),
            2 => "DOWN".to_owned(),
            3 => "RIGHT".to_owned(),
            _ => "NOTHING".to_owned(),
        }
    }

    fn move_with_safest_approach(games: &mut Vec<Self>) -> String {
        let mut my_moves: Vec<String> = Vec::new();
        let mut my_move = "LEFT".to_owned();

        for game in games {
            let todo_move = game.decide();
            my_moves.push(todo_move.clone());
            if todo_move == *"UP" {
                my_move = todo_move;
            }
        }
        if my_move != *"UP" {
            my_move = Game::get_safest_move(&my_moves);
        }
        my_move
    }

    fn get_safest_move(my_moves: &[String]) -> String {
        let min_move = my_moves
            .iter()
            .map(Game::move_to_value)
            .min()
            .expect("Not empty vector");
        Game::value_to_move(min_move)
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
        // let mut player_scores: Vec<PlayerScore> = Vec::new();
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

            // let player_score = PlayerScore::new(
            //     final_score,
            //     nb_gold_medals,
            //     nb_silver_medals,
            //     nb_bronze_medals,
            // );
            // player_scores.push(player_score);
        }

        // my games:
        // - if i am stunned in a game, doesn't take that game into consideration
        // - if i am next to a hurdle in any game, then just jump
        // - play the best move that doesn't leave me stunned in any game
        //    - find the nearest hurdle in all games and move according to that hurdle

        // choose my move for all 4 games
        let mut my_games: Vec<Game> = Vec::new();

        // let mut weakest_track = "".to_owned();
        // let mut weakest_pos = -1;
        // let mut weakest_player = PlayerDetails::default();

        // start each mini-game
        for _i in 0..nb_games as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();

            // GPU: ASCII representation of the racetrack (GAME_OVER during reset turn)
            let gpu = inputs[0].trim().to_string();
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

            let my_player = PlayerDetails {
                my_id: player_idx,
                my_pos: positions[player_idx as usize],
                my_stun: stuns[player_idx as usize],
            };

            if gpu != "GAME_OVER" && my_player.my_stun == 0 {
                // if weakest_pos == -1 || my_player.my_pos < weakest_pos{
                //     weakest_track = gpu;
                //     weakest_pos = my_player.my_pos;
                //     weakest_player = my_player;
                // }
                let mut game = Game::new(my_player);
                game.add_track(gpu);
                my_games.push(game);
            }
        }
        let safest_move = Game::move_with_safest_approach(&mut my_games);
        Game::make_move(safest_move);
        // let mut game = Game::new(weakest_player);
        // game.add_track(weakest_track);
        // Game::make_move(game.decide())
    }
}

#[cfg(test)]
mod test_hurdle_up {
    use super::*;
    const RIGHT: &str = "RIGHT";
    const UP: &str = "UP";
    const DOWN: &str = "DOWN";
    const LEFT: &str = "LEFT";
    const TRACK1: &str = ".....#...#...#................";
    const TRACK2: &str = ".....#...#...#................";
    const TRACK3: &str = ".....#...#...#................";
    const TRACK4: &str = ".....#...#...#................";

    #[test]
    fn right_closest_hurdle() {
        let track = TRACK1.to_string();
        let my_pos = 2;
        let ahead_track = &track[my_pos..];
        let h_pos = ahead_track
            .split_once('#')
            .map(|parts| parts.0.len() + my_pos);
        assert_eq!(h_pos.unwrap(), 5);

        let track = TRACK1.to_string();
        let my_pos = 7;
        let ahead_track = &track[my_pos..];
        let h_pos = ahead_track
            .split_once('#')
            .map(|parts| parts.0.len() + my_pos);
        assert_eq!(h_pos.unwrap(), 9);

        let track = TRACK1.to_string();
        let my_pos = 11;
        let ahead_track = &track[my_pos..];
        let h_pos = ahead_track
            .split_once('#')
            .map(|parts| parts.0.len() + my_pos);
        assert_eq!(h_pos.unwrap(), 13);
    }

    #[test]
    fn test_up() {
        let details = PlayerDetails::new(4, 0, 0);
        let track = TRACK1.to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);
        dbg!(game.find_closest_hurdle());

        assert_eq!(game.decide(), UP.to_string());

        let details = PlayerDetails::new(29, 0, 0);
        let track = TRACK1.to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);

        dbg!(game.find_closest_hurdle());
        assert_eq!(game.decide(), UP.to_string());
    }

    #[test]
    fn test_down() {
        let details = PlayerDetails::new(2, 0, 0);
        let track = TRACK1.to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);
        dbg!(game.find_closest_hurdle());

        assert_eq!(game.decide(), DOWN.to_string());
        Game::make_move(game.decide());
    }

    #[test]
    fn test_left() {
        let details = PlayerDetails::new(3, 0, 0);
        let track = TRACK1.to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);
        dbg!(game.find_closest_hurdle());

        assert_eq!(game.decide(), LEFT.to_string());
    }

    #[test]
    fn test_right() {
        let details = PlayerDetails::new(0, 0, 0);
        let track = TRACK1.to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);

        dbg!(game.find_closest_hurdle());
        assert_eq!(game.decide(), RIGHT.to_string());

        let details = PlayerDetails::new(14, 0, 0);
        let track = TRACK1.to_string();
        let mut game = Game::default();
        game.add_track(track);
        game.add_player_details(details);

        dbg!(game.find_closest_hurdle());
        assert_eq!(game.decide(), RIGHT.to_string());
    }

    #[test]
    fn safest_move_in_all_games() {
        let details = PlayerDetails::new(0, 0, 0);
        let track = TRACK1.to_string();
        let mut game1 = Game::default();
        game1.add_track(track.clone());
        game1.add_player_details(details.clone());

        let details = PlayerDetails::new(1, 0, 0);
        let mut game2 = Game::default();
        game2.add_track(track.clone());
        game2.add_player_details(details.clone());

        let details = PlayerDetails::new(0, 0, 0);
        let mut game3 = Game::default();
        game3.add_track(track.clone());
        game3.add_player_details(details.clone());

        let mut games = Vec::new();
        games.push(game1);
        games.push(game2);
        games.push(game3);

        let safest_move = Game::move_with_safest_approach(&mut games);
        // assert_eq!(safest_move, UP.to_owned());
        // assert_eq!(safest_move, DOWN.to_owned());
        // assert_eq!(safest_move, LEFT.to_owned());
        assert_eq!(safest_move, RIGHT.to_owned());
    }
}
