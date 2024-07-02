use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

// ! Skills: Binary Search in 2D and Intervals 

#[derive(Default, Debug, Clone)]
pub struct SearchArea {
    left_col: i32,
    right_col: i32,
    top_row: i32,
    bottom_row: i32,
}

#[derive(Default, Debug, Clone)]
pub struct Player {
    pos_x: i32,
    pos_y: i32,
    jumps_left: i32,
    search_area: SearchArea,
}

// Jumps required: log2(W*H)

impl SearchArea {
    fn new(width: i32, height: i32) -> Self {
        Self {
            left_col: 0,
            right_col: width-1,
            top_row: 0,
            bottom_row: height-1,
        }
    }

    fn set_rows(&mut self, top: i32, bottom: i32) {
        self.top_row = top;
        self.bottom_row = bottom;
    }

    fn set_columns(&mut self, left: i32, right: i32) {
        self.left_col = left;
        self.right_col = right;
    }
}

impl Player {
    fn new(pos_x: i32, pos_y: i32, jumps_left: i32, width: i32, height: i32) -> Self {
        Self {
            pos_x,
            pos_y,
            jumps_left,
            search_area: SearchArea::new(width, height),
        }
    }

    fn change_search_area(&mut self, bomb_dir: &str) -> &mut Self {
        match bomb_dir {
            "U" => self.up(),
            "UR" => self.up_right(),
            "UL" => self.up_left(),
            "D" => self.down(),
            "DL" => self.down_left(),
            "DR" => self.down_right(),
            "L" => self.left(),
            "R" => self.right(),
            _ => {}
        }
        dbg!(self.search_area.clone());
        self
    }

    /// jump to centre of jump area  
    fn next_jump_position(&mut self, bomb_dir: &str) -> (i32, i32) {
        self.change_search_area(bomb_dir);
        let new_row = (self.search_area.top_row + self.search_area.bottom_row) / 2;
        let new_col = (self.search_area.left_col + self.search_area.right_col) / 2;
        self.jumps_left -=1;
        // dbg!((new_row, new_col));
        self.pos_x = new_col;
        self.pos_y = new_row;
        (new_col, new_row)
    }

    fn up(&mut self) {
        let bottom = self.pos_y - 1;
        let top = self.search_area.top_row;
        let left = self.pos_x;
        let right = self.pos_x;

        self.search_area.set_rows(top, bottom);
        self.search_area.set_columns(left, right);
    }

    fn down(&mut self) {
        let bottom = self.search_area.bottom_row;
        let top = self.pos_y + 1;
        let left = self.pos_x;
        let right = self.pos_x;

        self.search_area.set_rows(top, bottom);
        self.search_area.set_columns(left, right);
    }

    fn right(&mut self) {
        let bottom = self.pos_y;
        let top = self.pos_y;
        let right = self.search_area.right_col;
        let left = self.pos_x + 1;

        self.search_area.set_rows(top, bottom);
        self.search_area.set_columns(left, right);
    }

    fn left(&mut self) {
        let bottom = self.pos_y;
        let top = self.pos_y;
        let right = self.pos_x-1;
        let left = self.search_area.left_col;

        self.search_area.set_rows(top, bottom);
        self.search_area.set_columns(left, right);
    }

    fn up_right(&mut self) {
        let bottom = self.pos_y - 1;
        let top = self.search_area.top_row;
        let left = self.pos_x + 1;
        let right = self.search_area.right_col;

        self.search_area.set_rows(top, bottom);
        self.search_area.set_columns(left, right);
    }

    fn up_left(&mut self) {
        let bottom = self.pos_y - 1;
        let top = self.search_area.top_row;
        let right = self.pos_x - 1;
        let left = self.search_area.left_col;

        self.search_area.set_rows(top, bottom);
        self.search_area.set_columns(left, right);
    }

    fn down_right(&mut self) { 
        let top = self.pos_y + 1;
        let bottom = self.search_area.bottom_row;
        let left = self.pos_x + 1;
        let right = self.search_area.right_col;

        self.search_area.set_rows(top, bottom);
        self.search_area.set_columns(left, right);
    }

    fn down_left(&mut self) {
        let top = self.pos_y + 1;
        let bottom = self.search_area.bottom_row;
        let left = self.search_area.left_col;
        let right = self.pos_x - 1;

        self.search_area.set_rows(top, bottom);
        self.search_area.set_columns(left, right);
    }

    fn debug_position(&self){
        println!("x: {}, y: {}", self.pos_x, self.pos_y);
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // maximum number of turns before game over.

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x0 = parse_input!(inputs[0], i32);
    let y0 = parse_input!(inputs[1], i32);

    let mut player = Player::new(x0, y0, n, w, h);
    // dbg!(player.clone());
    dbg!((x0, y0, n, w, h));

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

        // dbg!(player.clone());
        dbg!(bomb_dir.clone());
        let (x, y) = player.next_jump_position(&bomb_dir);

        // the location of the next window Batman should jump to.
        println!("{x} {y}");
    }
}
