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

    // - if due to one's jump then other's who stumble are more: BAD
    // - if due to one's jump then other's keep running are more: OK
    // - if nobody's jumping, then choose max distance moving move: OK
    fn move_with_safest_approach(games: &mut Vec<Self>) -> String {
        let mut moves = Vec::new();

        for game in games.iter_mut() {
            let closest_hurdle = game
                .find_closest_hurdle()
                .or_else(|| Some(game.track.as_ref().unwrap().len()))
                .unwrap();
            game.closest_hurdle = Some(closest_hurdle);

            let dist = closest_hurdle - game.my_player.pos as usize;
            let curr_move = Game::move_based_on_dist(dist);

            let mut curr_move_stats = MoveStatistics::default();
            curr_move_stats.set_name(curr_move);
            dbg!(&curr_move_stats.name);

            moves.push(curr_move_stats.clone());
        }

        // calculate each move's stat for each game
        for this_stats in moves.iter_mut() {
            // complete move's stats for all games
            for game in games.iter() {
                let dist = game.closest_hurdle.unwrap() - game.my_player.pos as usize;
                let fut_dist = dist as i8 - Game::move_to_value(&this_stats.name) as i8;
                match fut_dist.cmp(&0) {
                    Ordering::Equal => this_stats.num_stumbling += 1,
                    Ordering::Greater => this_stats.num_running += 1,
                    Ordering::Less => this_stats.num_jumping += 1,
                }
            }
            // calculate points for each move stats
        }

        // chose the move from moves which is better:
        // b1: - if due to one's jump then other's who stumble are more: BAD
        // b2: - if due to one's jump then other's keep running are more: OK
        // b3: - if nobody's jumping, then choose max safe distance moving move: OK
        // each condition has different value: b1-10*(diff), b2-5*(diff),b3-2*(diff)
        // for this_move in moves {
        //     // let b1 = this_move.num_running < this_move.num_stumbling
        // }

        Game::move_based_on_dist(1)

        // if hurdle_freq_map.contains_key(&1) {
        //     if hurdle_freq_map.contains_key(&2) {
        //         if hurdle_freq_map.contains_key(&3) {
        //             Game::right()
        //         } else {
        //             Game::down()
        //         }
        //     } else {
        //         Game::left()
        //     }
        // } else if hurdle_freq_map.get(&2).is_none()
        //     || hurdle_freq_map.get(&1).unwrap() > hurdle_freq_map.get(&2).unwrap()
        // {
        //     Game::up()
        // } else {
        //     Game::left()
        // }
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
#[derive(Debug, Clone, Default)]

struct MoveStatistics {
    game_id: u8,
    name: String,
    num_stumbling: u8,
    g: u8,
    num_running: u8,
    points: u8,
}

impl MoveStatistics {
    // fn new(name: String, num_jumping: u8, num_running: u8, num_stumbling: u8) -> Self {
    //     MoveStatistics {
    //         name,
    //         num_jumping,
    //         num_running,
    //         num_stumbling,
    //     }
    // }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_points(&mut self, points: u8) {
        self.points = points;
    }

    fn set_game_id(&mut self, id: u8) {
        self.game_id = id;
    }

    fn calculate_points(&self) -> u8 {
        // chose the move from moves which is better:
        // b1: - if due to one's jump then other's who stumble are more: BAD
        // b2: - if due to one's jump then other's keep running are more: OK
        // b3: - if nobody's jumping, then choose max safe distance moving move: OK
        // each condition has different value: b1-10*(diff), b2-5*(diff),b3-2*(diff)

        let mut points = 0;
        if self.num_jumping < self.num_stumbling {
            points += 10 * (self.num_stumbling - self.num_jumping)
        }

        0
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
        // let safest_move = Game::move_with_safest_approach(&mut my_games);
        // Game::make_move(safest_move);

        let leading_move = Game::move_with_leading_approach(&mut my_games);
        Game::make_move(leading_move);
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

        let safest_move = Game::move_with_safest_approach(&mut games);
        dbg!(safest_move);
        // assert_eq!(safest_move, UP.to_owned());
        // assert_eq!(safest_move, DOWN.to_owned());
        // assert_eq!(safest_move, LEFT.to_owned());
        // assert_eq!(safest_move, RIGHT.to_owned());
    }

    #[test]
    fn calculate_move_stats() {
        let mut moves = Vec::new();
        let mut games = create_multiple_games(4);

        // dbg!(games.clone());

        // calculate game's suitable move
        for game in games.iter_mut() {
            let closest_hurdle = game.find_closest_hurdle();
            game.closest_hurdle = closest_hurdle;

            let dist = closest_hurdle.unwrap() - game.my_player.pos as usize;
            let curr_move = Game::move_based_on_dist(dist);

            let mut curr_move_stats = MoveStatistics::default();
            curr_move_stats.set_name(curr_move);
            curr_move_stats.set_game_id(game.id);
            // dbg!(&curr_move_stats);

            moves.push(curr_move_stats.clone());
        }

        // calculate each move's stat for each game
        for this_stats in moves.iter_mut() {
            // complete move's stats for all games
            for game in games.iter() {
                let dist = game.closest_hurdle.unwrap() - game.my_player.pos as usize;
                let fut_dist = dist as i8 - Game::move_to_value(&this_stats.name) as i8;
                match fut_dist.cmp(&0) {
                    Ordering::Greater => this_stats.num_running += 1,
                    Ordering::Equal => {this_stats.num_saved += 1},
                    Ordering::Less => this_stats.num_stumbling += 1,
                }
            }
            dbg!(&this_stats);
            // calculate points for each move stats
        }
    }

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
