use rand::Rng;

const H: usize = 3;
const W: usize = 4;
const END_TURN: i32 = 4;

#[derive(Debug)]
#[derive(Clone)]
struct Coord {
    y: usize,
    x: usize,
}
impl Coord {
    fn new(x: usize, y: usize) -> Self
    {
        Coord {x: x, y: y}
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct MazeState {
    points: Vec<Vec<i32>>,
    turn: i32,
    character: Coord,
    game_score: i32
}
impl MazeState {
    const DX: &[i32] = &[1, 0, -1, 0];
    const DY: &[i32] = &[0, 1, 0, -1];
    fn new() -> Self
    {
        let mut rng = rand::thread_rng();
        let chara = Coord::new(rng.gen_range(0..W), rng.gen_range(0..H));
        let mut points = vec![vec![0; W]; H];
        for i in 0..H {
            for j in 0..W {
                if chara.y == i && chara.x == j {
                    continue;
                }
                points[i][j] = rng.gen_range(0..10);
            }
        }
        MazeState {
            points: points,
            turn: 0,
            character: chara,
            game_score: 0,
        }
    }

    fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    fn legal_action(&self) -> Vec<usize> {
        let mut actions: Vec<usize> = vec![];
        for i in 0..4 {
            let next_x = self.character.x as i32 + Self::DX[i];
            let next_y = self.character.y as i32 + Self::DY[i];
            if next_x >= 0 && next_x < W as i32 && next_y >= 0 && next_y < H as i32 {
                actions.push(i);
            }
        }
        actions
    }

    fn advance(&mut self, action: usize) {
        let next_x = (self.character.x as i32 + Self::DX[action]) as usize;
        let next_y = (self.character.y as i32 + Self::DY[action]) as usize;
        let point = self.points[next_y][next_x];
        if point > 0 {
            self.game_score += point;
            self.points[next_y][next_x] = 0;
        }
        self.character.x = next_x;
        self.character.y = next_y;
        self.turn += 1;
    }

    fn print_maze_state(&self) {
        println!("Turn {}", self.turn);
        println!("Score {}", self.game_score);
        for y in 0..H {
            for x in 0..W {
                if self.character.y == y && self.character.x == x {
                    print!("@");
                } else if self.points[y][x] > 0 {
                    print!("{}", self.points[y][x]);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }

    fn random_action(&self) -> usize {
        let legal_actions = self.legal_action();
        let n = legal_actions.len();
        let mut rng = rand::thread_rng();
        let m = rng.gen_range(0..n);
        legal_actions[m]
    }

    fn greedy_action(&self) -> usize {
        let legal_actions = self.legal_action();
        let mut best_score: i32 = 0;
        let mut best_action = 0;
        for action in legal_actions {
            let mut now_state = self.clone();
            now_state.advance(action);
            if now_state.game_score > best_score {
                best_score = now_state.game_score;
                best_action = action;
            }
        }
        best_action
    }

    fn play_game(&mut self) {
        self.print_maze_state();
        while !self.is_done() {
            self.advance(self.greedy_action());
            self.print_maze_state();
        }
    }

}

fn main() {
    let mut maz = MazeState::new();
    maz.play_game();
}
