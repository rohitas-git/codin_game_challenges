#![allow(dead_code)]
use std::{
    borrow::BorrowMut,
    cmp::{self, Ordering},
    collections::{btree_map::Range, HashMap},
    io,
};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone, Default)]
struct PlayerTurn {
    pos: i32,
    stun: i32,
    id: i32,
}

impl PlayerTurn {
    fn default() -> Self {
        PlayerTurn {
            pos: 0,
            stun: 0,
            id: 0,
        }
    }

    fn new(my_pos: i32, my_stun: i32, my_id: i32) -> Self {
        PlayerTurn {
            pos: my_pos,
            stun: my_stun,
            id: my_id,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct GameTurn {
    game_id: u8,
    run_id: u8,
    track: Option<String>,
    closest_hurdle: Option<usize>,
    my_player: PlayerTurn,
    other_players: Vec<PlayerTurn>,
    suitable_move: Option<String>,
}

impl GameTurn {
    fn new(game_id: u8, run_id: u8, my_player: PlayerTurn, other_players: Vec<PlayerTurn>) -> Self {
        GameTurn {
            game_id,
            run_id,
            track: None,
            closest_hurdle: None,
            my_player,
            other_players,
            suitable_move: None,
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

    fn set_run_id(&mut self, id: u8) {
        self.run_id = id;
    }

    fn add_player_details(&mut self, details: PlayerTurn) {
        self.my_player = details;
    }

    fn decide(&mut self) -> String {
        if self.track.is_some() {
            let my_pos = self.my_player.pos as usize;

            // find closest hurdle if unknown
            if self.closest_hurdle.is_none() {
                self.closest_hurdle = Some(self.find_closest_hurdle());
            }

            let closest_hurdle = self.closest_hurdle.unwrap();
            let dist = closest_hurdle - my_pos;
            dbg!(dist);

            let to_move = GameTurn::move_based_on_dist(dist);
            self.suitable_move = Some(to_move.clone());
            return to_move;
        }
        self.suitable_move = None;
        Self::down()
    }

    fn find_closest_hurdle(&self) -> usize {
        let track = self.track.as_ref().unwrap();
        let my_pos = self.my_player.pos as usize;
        let mut closest_hurdle = 4;
        for i in my_pos+1..(my_pos+4){
            if i < track.len() && track.get(i..i+1).unwrap() == "#"{
                closest_hurdle = i - my_pos;
                break;
            }
        }
        closest_hurdle
    }

    fn make_move(my_move: String) {
        println!("{my_move}");
    }

    fn move_based_on_dist(dist: usize) -> String {
        match dist {
            1 => Self::up(),
            2 => Self::left(),
            3 => Self::up(),
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

    fn aggressive_strategy(closest_hurdle: usize) -> String {
        let mut chosen_mv = GameTurn::up();
        if closest_hurdle != 1{
            chosen_mv = GameTurn::right();
        }
        chosen_mv
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

    // the number of simultaneously running mini-game_run = 4 (depends)
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let nb_games = parse_input!(input_line, i32);

    let mut num_runs = 0;
    // game_run loop
    loop {
        // get score info of each player before start of game_runs
        let mut player_infos: Vec<TotalPlayerInfo> = Vec::new();
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

            let player_info = TotalPlayerInfo::new(
                i as i32,
                final_score,
                nb_gold_medals,
                nb_silver_medals,
                nb_bronze_medals,
            );
            player_infos.push(player_info);
        }

        // choose my move for all 4 game_runs
        let mut my_game_turns: Vec<GameTurn> = Vec::new();
        // let mut mini_games: Vec<MiniGamesScore> = [0, 1, 2, 3u8].iter().map(|&i| ).collect();
        let mut closest_hurdle = 4;
        // start each mini-game_run:
        // - multiple runs in one mini-game
        // - multiple turns in one run
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
                let p = PlayerTurn {
                    id: _i,
                    pos: positions[_i as usize],
                    stun: stuns[_i as usize],
                };
                players.push(p);
            }
            let my_player = players.remove(player_idx as usize);

            if gpu != "GAME_OVER" && my_player.stun == 0 {
                let mut game_turn = GameTurn::new(_i as u8, num_runs, my_player, players);
                game_turn.add_track(gpu);
                my_game_turns.push(game_turn.clone());
                closest_hurdle = std::cmp::min(closest_hurdle, game_turn.find_closest_hurdle());
            } else if gpu == "GAME_OVER" {
                // GAME_OVER => Game run has finished, get medals and maybe start a game run
                num_runs += 1;
                my_game_turns.clear();
            }
        }
        dbg!(closest_hurdle);
        GameTurn::make_move(GameTurn::aggressive_strategy(closest_hurdle));
    }
}