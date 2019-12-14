use super::*;

enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn parse(num: i64) -> Tile {
        match num {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => unimplemented!(),
        }
    }
    fn c(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => 'X',
            Tile::Paddle => '_',
            Tile::Ball => '@',
        }
    }
}

struct Game {
    screen: Vec<char>,
    width: usize,
    _height: usize,
    ball_pos_x: i64,
    pad_pos_x: i64,
    score: i64,
}

impl Game {
    fn new(vec: Vec<char>, width: usize, _height: usize) -> Game {
        Game {
            screen: vec,
            width,
            _height,
            ball_pos_x: 0,
            pad_pos_x: 0,
            score: 0,
        }
    }
    fn update_pixel(&mut self, x: usize, y: usize, tile_num: i64) {
        if tile_num == 4 {
            self.ball_pos_x = x as i64;
        } else if tile_num == 3 {
            self.pad_pos_x = x as i64;
        }
        let index = x + y * self.width;
        let c = Tile::parse(tile_num).c();
        self.screen[index] = c;
    }
    fn _print_screen(&self) {
        for i in 0..self._height {
            let slice = &self.screen[i * self.width..(i + 1) * self.width];
            let out: String = slice.into_iter().collect();
            println!("{}", out);
        }
    }
    fn suggest_input(&self) -> i64 {
        (self.ball_pos_x - self.pad_pos_x).signum()
    }
}

fn do_frame(machine: &mut IntMachine, game: &mut Game, input: Vec<i64>) {
    let out = machine.run(input, false);
    for i in (0..out.len()).step_by(3) {
        if out[i] == -1 && out[i + 1] == 0 {
            game.score = out[i + 2];
        } else {
            let x = out[i] as usize;
            let y = out[i + 1] as usize;
            let t = out[i + 2];
            game.update_pixel(x, y, t);
        }
    }
}

pub fn p13(input: &str) -> Answer {
    let mut mem = parse_intcode(input);
    mem[0] = 2;
    let mut machine = IntMachine::new(mem);
    let out = machine.run(vec![], false);
    let mut tiles = HashMap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    for i in (0..out.len()).step_by(3) {
        let x = out[i];
        let y = out[i + 1];
        if x > x_max {
            x_max = x;
        }
        if y > y_max {
            y_max = y;
        }
        tiles.insert((x, y), out[i + 2]);
    }
    let ans1 = tiles
        .into_iter()
        .fold(0, |acc, (_k, v)| if v == 2 { acc + 1 } else { acc });

    let mut screen = Vec::with_capacity(46 * 26);
    for i in (2..out.len()).step_by(3) {
        screen.push(Tile::parse(out[i]).c());
    }
    let mut game = Game::new(screen, (x_max + 1) as usize, (y_max + 1) as usize);
    while !machine.halted() {
        let input = vec![game.suggest_input()];
        do_frame(&mut machine, &mut game, input);
    }
    Answer::new(ans1, game.score)
}