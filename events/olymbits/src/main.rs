#![allow(dead_code)]
use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    io,
};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone)]
struct PlayerDetails {
    pos: i32,
    stun: i32,
    id: i32,
}

impl PlayerDetails {
    fn default() -> Self {
        PlayerDetails {
            pos: 0,
            stun: 0,
            id: 0,
        }
    }

    fn new(my_pos: i32, my_stun: i32, my_id: i32) -> Self {
        PlayerDetails {
            pos: my_pos,
            stun: my_stun,
            id: my_id,
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u8,
    track: Option<String>,
    closest_hurdle: Option<usize>,
    my_player: PlayerDetails,
    other_players: Vec<PlayerDetails>,
}

impl Game {
    fn default() -> Self {
        Game {
            id: 0,
            track: None,
            closest_hurdle: None,
            my_player: PlayerDetails::default(),
            other_players: Vec::new(),
        }
    }

    fn new(id: u8, my_player: PlayerDetails, other_players: Vec<PlayerDetails>) -> Self {
        Game {
            id,
            track: None,
            closest_hurdle: None,
            my_player,
            other_players,
        }
    }

    // move 1 space
    fn left() -> String {
        "LEFT".to_owned()
    }

    // moves 3 spaces
    fn right() -> String {
        "RIGHT".to_owned()
    }

    // jump and move 2 spaces
    fn up() -> String {
        "UP".to_owned()
    }

    // move 2 spaces
    fn down() -> String {
        "DOWN".to_owned()
    }

    // do nothing
    fn nothing() -> String {
        "LEFT".to_owned()
    }

    fn add_track(&mut self, track: String) {
        self.track = Some(track);
    }

    fn set_id(&mut self, id: u8) {
        self.id = id;
    }

    fn add_player_details(&mut self, details: PlayerDetails) {
        self.my_player = details;
    }

    fn decide(&mut self) -> String {
        if self.track.is_some() {
            let my_pos = self.my_player.pos as usize;

            // find closest hurdle if unknown
            if self.closest_hurdle.is_none() {
                self.closest_hurdle = self.find_closest_hurdle();
            }

            // when no hurlde then is it len of track or len of track + 1
            let closest_hurdle = self
                .closest_hurdle
                .or_else(|| Some(self.track.as_ref().unwrap().len()))
                .unwrap();
            // .map_or(self.track.as_ref().unwrap().len(), |val| val);

            let dist = closest_hurdle - my_pos;
            dbg!(dist);

            let to_move = Game::move_based_on_dist(dist);
            if to_move == *"UP" {
                self.closest_hurdle = None;
            }
            return to_move;
        }
        Self::nothing()
    }

    fn find_closest_hurdle(&self) -> Option<usize> {
        let track = self.track.as_ref().unwrap();
        let my_pos = self.my_player.pos as usize;
        let ahead_track = &track[my_pos..];
        ahead_track
            .split_once('#')
            .map(|parts| parts.0.len() + my_pos)
            .or_else(|| Some(self.track.as_ref().unwrap().len()))
    }

    fn make_move(my_move: String) {
        println!("{my_move}");
    }

    fn move_based_on_dist(dist: usize) -> String {
        match dist {
            1 => Self::up(),
            2 => Self::left(),
            3 => Self::down(),
            _ => Self::right(),
        }
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
        let mut moves = Vec::new();
        let mut stumbles: Vec<u8> = Vec::new();

        for game in games.iter_mut() {
            let closest_hurdle = game
                .find_closest_hurdle()
                .or_else(|| Some(game.track.as_ref().unwrap().len()))
                .unwrap();
            game.closest_hurdle = Some(closest_hurdle);

            let dist = closest_hurdle - game.my_player.pos as usize;
            let curr_move = Game::move_based_on_dist(dist);
            moves.push(curr_move);
        }

        // Choose the move which stumbles the least
        // Stumble:
        // if my_new_pos >= hurdle_pos => True iff Move is not UP
        // else => False 

        // calculate each move's stat for each game
        for this_move in moves.iter() {
            // complete move's stats for all games
            let mut num_stumbles = 0;
            for game in games.iter() {
                let my_pos = game.my_player.pos;
                let my_new_pos = my_pos + Game::move_to_value(this_move) as i32;
                let hurdle_pos = game.closest_hurdle.unwrap() as i32;
                let mv_is_up = *this_move == Game::up();
                if my_new_pos >= hurdle_pos && !mv_is_up {
                    num_stumbles += 1;
                }else if mv_is_up && my_new_pos == hurdle_pos {
                    num_stumbles+=1;
                }
            }
            stumbles.push(num_stumbles);
        }

        // a single mv or many
        let (best_mv_id, least_stumbles) = stumbles.iter().enumerate().min().unwrap();
        let best_moves_id: Vec<usize> = stumbles
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if v == least_stumbles {
                    return Some(i);
                } else {
                    None
                }
            })
            .collect();
        let mv_count = best_moves_id.len();
        if mv_count == 1 {
            moves.get(best_mv_id).unwrap().to_owned()
        } else {
            let max_mv_val = best_moves_id
                .iter()
                .map(|id| Game::move_to_value(moves.get(*id).unwrap()))
                .max()
                .unwrap();
            let val_id = best_moves_id
                .iter()
                .find(|&id| Game::move_to_value(moves.get(*id).unwrap()) == max_mv_val)
                .unwrap();

            moves[*val_id].clone()
        }
    }

    fn move_with_leading_approach(games: &mut Vec<Self>) -> String {
        let mut weakest_lead = 0;
        let mut weakest_game = None;
        for game in games {
            if game.track.is_some() {
                let mut my_lead = 0;
                let my_p = game.my_player.pos;
                let p2 = game.other_players[0].pos;
                let p3 = game.other_players[1].pos;
                let mut positions = [my_p, p2, p3];
                positions.sort();
                if my_p == positions[2] {
                    my_lead = my_p - positions[1];
                } else {
                    my_lead = my_p - positions[2];
                }

                if game.my_player.stun == 0 && (weakest_game.is_none() || my_lead < weakest_lead) {
                    weakest_lead = my_lead;
                    weakest_game = Some(game.clone());
                }
            }
        }
        weakest_game
            .map_or(Some("DOWN".to_string()), |mut game| Some(game.decide()))
            .unwrap()
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
            let ids = [0, 1, 2];

            let mut players = vec![];
            for _i in ids {
                let p = PlayerDetails {
                    id: _i,
                    pos: positions[_i as usize],
                    stun: stuns[_i as usize],
                };
                players.push(p);
            }
            let my_player = players.remove(player_idx as usize);

            if gpu != "GAME_OVER" && my_player.stun == 0 {
                let mut game = Game::new(_i as u8, my_player, players);
                game.add_track(gpu);
                my_games.push(game);
            }
        }
        let safest_move = Game::move_with_safest_approach(&mut my_games);
        Game::make_move(safest_move);

        // let leading_move = Game::move_with_leading_approach(&mut my_games);
        // Game::make_move(leading_move);
    }
}

#[cfg(test)]
mod test_hurdle_up {
    use rand::Rng;

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
        let mut games = create_multiple_games(3);
        dbg!(games.clone());
        let safest_move = Game::move_with_safest_approach(&mut games);
        dbg!(safest_move);
        // assert_eq!(safest_move, UP.to_owned());
        // assert_eq!(safest_move, DOWN.to_owned());
        // assert_eq!(safest_move, LEFT.to_owned());
        // assert_eq!(safest_move, RIGHT.to_owned());
    }

    #[test]
    fn calculate_move_stats() {}

    fn create_multiple_games(n: u32) -> Vec<Game> {
        let track = TRACK1.to_string();
        let mut games = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..n {
            let mut game = Game::default();
            let a = rng.gen_range(0..5);
            let details = PlayerDetails::new(a, 0, 0);
            game.add_track(track.clone());
            game.set_id(i as u8);
            game.add_player_details(details.clone());
            games.push(game);
        }
        games
    }
}
