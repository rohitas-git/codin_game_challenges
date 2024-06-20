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

            let to_move = GameTurn::move_based_on_dist(dist);
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

    fn safe_approach(game_runs: &mut Vec<Self>) -> String {
        let mut moves = Vec::new();
        let mut stumbles: Vec<u8> = Vec::new();

        for game_run in game_runs.iter_mut() {
            let closest_hurdle = game_run
                .find_closest_hurdle()
                .or_else(|| Some(game_run.track.as_ref().unwrap().len()))
                .unwrap();
            game_run.closest_hurdle = Some(closest_hurdle);

            let dist = closest_hurdle - game_run.my_player.pos as usize;
            let curr_move = GameTurn::move_based_on_dist(dist);
            moves.push(curr_move);
        }

        // Choose the move which stumbles the least
        // Stumble:
        // if my_new_pos >= hurdle_pos => True iff Move is not UP
        // else => False

        // calculate each move's stat for each game_run
        for this_move in moves.iter() {
            // complete move's stats for all game_runs
            let mut num_stumbles = 0;
            for game_run in game_runs.iter() {
                let my_pos = game_run.my_player.pos;
                let my_new_pos = my_pos + GameTurn::move_to_value(this_move) as i32;
                let hurdle_pos = game_run.closest_hurdle.unwrap() as i32;
                let mv_is_up = *this_move == GameTurn::up();
                if my_new_pos >= hurdle_pos && !mv_is_up {
                    num_stumbles += 1;
                } else if mv_is_up && my_new_pos == hurdle_pos {
                    num_stumbles += 1;
                }
                // think of how any additional stumble in this game run affects
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
                .map(|id| GameTurn::move_to_value(moves.get(*id).unwrap()))
                .max()
                .unwrap();
            let val_id = best_moves_id
                .iter()
                .find(|&id| GameTurn::move_to_value(moves.get(*id).unwrap()) == max_mv_val)
                .unwrap();

            moves[*val_id].clone()
        }
    }

    // decide based on avg progress
    fn progress_approach(game_turns: &mut Vec<Self>) -> String {
        let mut moves = Vec::new();
        let mut avg_progresses = Vec::new();
        let mut stumbles: Vec<u8> = Vec::new();

        // storing curr moves each of which best suit atleast one game
        for game_turn in game_turns.iter_mut() {
            let closest_hurdle = game_turn
                .find_closest_hurdle()
                .or_else(|| Some(game_turn.track.as_ref().unwrap().len()))
                .unwrap();
            game_turn.closest_hurdle = Some(closest_hurdle);

            let dist = closest_hurdle - game_turn.my_player.pos as usize;
            let curr_move = GameTurn::move_based_on_dist(dist);
            moves.push(curr_move);
        }

        // Choose the move which stumbles the least && let us lead others
        for this_move in moves.iter() {
            let mut num_stumbles = 0;
            let mut avg_progess = GameTurn::move_to_value(this_move) as i32;
            let mut extra_leads = 0;

            for game_turn in game_turns.iter() {
                let mut game_lead = 0;
                let mut new_game_lead = 0;
                let my_pos = game_turn.my_player.pos;
                let my_stun = game_turn.my_player.stun;
                let my_new_pos = my_pos + GameTurn::move_to_value(this_move) as i32;
                let hurdle_pos = game_turn.closest_hurdle.unwrap() as i32;
                let mv_is_up = *this_move == GameTurn::up();
                let mut stumbled = false;

                // stumbles affecting progress
                if my_new_pos >= hurdle_pos && !mv_is_up {
                    num_stumbles += 1;
                    stumbled = true;
                } else if mv_is_up && my_new_pos == hurdle_pos {
                    num_stumbles += 1;
                    stumbled = true;
                }

                // progress compared to peers should be decent
                let p2 = game_turn.other_players[0].pos;
                let p3 = game_turn.other_players[1].pos;

                let mut s2 = game_turn.other_players[0].stun;
                if !s2.is_positive() {
                    s2 = 0;
                };
                let mut s3 = game_turn.other_players[1].stun;
                if !s3.is_positive() {
                    s3 = 0;
                };
                let mut positions = [my_pos, p2, p3];
                positions.sort();

                if my_pos == positions[2] {
                    if positions[1] == p2 && s2.is_positive() {
                        game_lead = my_pos - positions[1] + s2;
                    } else if positions[1] == p3 && s3.is_positive() {
                        game_lead = my_pos - positions[1] + s3;
                    }
                } else {
                    if positions[2] == p2 && s2.is_positive() {
                        game_lead = my_pos - positions[2] + s2;
                    } else if positions[2] == p3 && s3.is_positive() {
                        game_lead = my_pos - positions[2] + s3;
                    }
                }

                if stumbled {
                    if hurdle_pos == positions[2] {
                        new_game_lead = hurdle_pos - positions[1];
                    } else {
                        new_game_lead = hurdle_pos - positions[2];
                    }
                } else if my_new_pos == positions[2] {
                    new_game_lead = my_new_pos - positions[1];
                } else {
                    new_game_lead = my_new_pos - positions[2];
                }

                extra_leads += new_game_lead - game_lead;
            }

            stumbles.push(num_stumbles);
            let avg_leads = extra_leads / game_turns.len() as i32;
            avg_progess = avg_progess - (num_stumbles as i32 * 2 / 4) + avg_leads;
            avg_progresses.push(avg_progess);
        }

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
        if best_moves_id.len() == 1 {
            // if one move makes least stumbled: do it
            return moves.get(best_mv_id).unwrap().to_owned();
        } else {
            // else choose move which gives max progress
            let chosen_id = best_moves_id
                .iter()
                .map(|id| avg_progresses[*id])
                .max()
                .unwrap();

            return moves[chosen_id as usize].to_owned();
        }
    }

    // fn move_with_leading_approach(game_turns: &mut Vec<Self>) -> String {
    //     let mut weakest_lead = 0;
    //     let mut weakest_game = None;
    //     for game_turn in game_turns {
    //         if game_run.track.is_some() {
    //             let mut my_lead = 0;
    //             let my_p = game_run.my_player.pos;
    //             let p2 = game_run.other_players[0].pos;
    //             let p3 = game_run.other_players[1].pos;
    //             let mut positions = [my_p, p2, p3];
    //             positions.sort();
    //             if my_p == positions[2] {
    //                 my_lead = my_p - positions[1];
    //             } else {
    //                 my_lead = my_p - positions[2];
    //             }

    //             if game_run.my_player.stun == 0
    //                 && (weakest_game.is_none() || my_lead < weakest_lead)
    //             {
    //                 weakest_lead = my_lead;
    //                 weakest_game = Some(game_run.clone());
    //             }
    //         }
    //     }
    //     weakest_game
    //         .map_or(Some("DOWN".to_string()), |mut game_run| {
    //             Some(game_run.decide())
    //         })
    //         .unwrap()
    // }
}

#[derive(Debug, Default, Clone)]
struct MiniGameScore {
    game_id: u8,
    score: u32,
    gold: u8,
    silver: u8,
    bronze: u8,
    latest_run_turn: GameTurn,
}

#[derive(Debug, Default, Clone)]
struct GameRun {
    game_id: u8,
    turns: Vec<GameTurn>,
}

impl GameRun {
    fn new(game_id: u8) -> Self {
        GameRun {
            game_id,
            turns: Vec::new(),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct TotalPlayerInfo {
    player_id: i32,
    gold: u32,
    silver: u32,
    bronze: u32,
    final_score: u32,
    games_info: Vec<MiniGameScore>,
}

impl TotalPlayerInfo {
    fn new(id: i32, final_score: u32, gold: u32, silver: u32, bronze: u32) -> Self {
        TotalPlayerInfo {
            player_id: id,
            final_score,
            gold,
            silver,
            bronze,
            games_info: Vec::new(),
        }
    }

    fn calculate_final_score(&mut self) {
        self.final_score = self
            .games_info
            .iter()
            .fold(1u32, |acc, game| acc * game.score);
    }

    fn add_gold_medal(&mut self, game_id: u8) {
        self.gold += 1;
        self.games_info[game_id as usize].gold += 1;
        self.calculate_final_score();
    }

    fn add_silver_medal(&mut self, game_id: u8) {
        self.silver += 1;
        self.games_info[game_id as usize].silver += 1;
        self.calculate_final_score();
    }

    fn add_bronze_medal(&mut self, game_id: u8) {
        self.bronze += 1;
        self.games_info[game_id as usize].bronze += 1;
    }

    fn add_game_score(&mut self, game_id: usize, score: u32) {
        self.games_info[game_id].score += score;
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
                let mut game_run = GameTurn::new(_i as u8, num_runs, my_player, players);
                game_run.add_track(gpu);
                my_game_turns.push(game_run);
            } else if gpu == "GAME_OVER" {
                // GAME_OVER => Game run has finished, get medals and maybe start a game run
                num_runs += 1;
                my_game_turns.clear();
            }
        }
        let safest_move = GameTurn::safe_approach(&mut my_game_turns);
        GameTurn::make_move(safest_move);
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
        let details = PlayerTurn::new(4, 0, 0);
        let track = TRACK1.to_string();
        let mut game_run = GameTurn::default();
        game_run.add_track(track);
        game_run.add_player_details(details);
        dbg!(game_run.find_closest_hurdle());

        assert_eq!(game_run.decide(), UP.to_string());

        let details = PlayerTurn::new(29, 0, 0);
        let track = TRACK1.to_string();
        let mut game_run = GameTurn::default();
        game_run.add_track(track);
        game_run.add_player_details(details);

        dbg!(game_run.find_closest_hurdle());
        assert_eq!(game_run.decide(), UP.to_string());
    }

    #[test]
    fn test_down() {
        let details = PlayerTurn::new(2, 0, 0);
        let track = TRACK1.to_string();
        let mut game_run = GameTurn::default();
        game_run.add_track(track);
        game_run.add_player_details(details);
        dbg!(game_run.find_closest_hurdle());

        assert_eq!(game_run.decide(), DOWN.to_string());
        GameTurn::make_move(game_run.decide());
    }

    #[test]
    fn test_left() {
        let details = PlayerTurn::new(3, 0, 0);
        let track = TRACK1.to_string();
        let mut game_run = GameTurn::default();
        game_run.add_track(track);
        game_run.add_player_details(details);
        dbg!(game_run.find_closest_hurdle());

        assert_eq!(game_run.decide(), LEFT.to_string());
    }

    #[test]
    fn test_right() {
        let details = PlayerTurn::new(0, 0, 0);
        let track = TRACK1.to_string();
        let mut game_run = GameTurn::default();
        game_run.add_track(track);
        game_run.add_player_details(details);

        dbg!(game_run.find_closest_hurdle());
        assert_eq!(game_run.decide(), RIGHT.to_string());

        let details = PlayerTurn::new(14, 0, 0);
        let track = TRACK1.to_string();
        let mut game_run = GameTurn::default();
        game_run.add_track(track);
        game_run.add_player_details(details);

        dbg!(game_run.find_closest_hurdle());
        assert_eq!(game_run.decide(), RIGHT.to_string());
    }

    #[test]
    fn safest_move_in_all_games() {
        let mut game_runs = create_multiple_games(3);
        dbg!(game_runs.clone());
        let safest_move = GameTurn::safe_approach(&mut game_runs);
        dbg!(safest_move);
    }

    fn create_multiple_games(n: u32) -> Vec<GameTurn> {
        let track = TRACK1.to_string();
        let mut game_runs = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..n {
            let mut game_run = GameTurn::default();
            let a = rng.gen_range(0..5);
            let details = PlayerTurn::new(a, 0, 0);
            game_run.add_track(track.clone());
            game_run.set_run_id(i as u8);
            game_run.add_player_details(details.clone());
            game_runs.push(game_run);
        }
        game_runs
    }

    // - no two ## next to each other
    fn create_random_track() {}

    fn create_random_player_turn(my_pos: usize) -> PlayerTurn {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(my_pos - 3..my_pos + 3);
        PlayerTurn::new(a as i32, 0, 0)
    }
}
