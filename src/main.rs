const NUM_EDGES: u64 = 100;

#[derive(Debug, Copy, Clone)]
enum Color {
    COOPERATE,
    DEFECT,
    TITFORTAT,
}
impl Color {
    fn new_random() -> Self {
        match fastrand::u8(0..3) {
            0 => Color::COOPERATE,
            1 => Color::DEFECT,
            2 => Color::TITFORTAT,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct Player {
    points: u64,
    prev_response: bool,
    color: Color,
}
impl Player {
    pub fn get_response(&self) -> bool {
        match self.color {
            Color::COOPERATE => true,
            Color::DEFECT => false,
            Color::TITFORTAT => !self.prev_response,
        }
    }
}

struct Matchup {
    a: Player,
    b: Player,
}
impl Matchup {
    fn play_round(&mut self) {
        let (a_prev, b_prev) = (self.a.get_response(), self.b.get_response());
        let (a_score, b_score) = Matchup::get_payoffs(a_prev, b_prev);

        self.a = Player { points: self.a.points + a_score, prev_response: b_prev, color: self.a.color };
        self.b = Player { points: self.b.points + b_score, prev_response: a_prev, color: self.b.color }; 
    }
    fn get_payoffs(a: bool, b: bool) -> (u64, u64) {
        match (a, b) {
            (false, false) => (1, 1),
            (false,  true) => (5, 0),
            ( true, false) => (0, 5),
            ( true,  true) => (3, 3),
        }
    }
}

struct Game {
    edges: Vec<Matchup>
}
impl Game {
    fn play_round(&mut self) {
        for edge in &mut self.edges {
            edge.play_round();
        }
    }
}

fn main() {
    (0..NUM_EDGES).map(|x| Matchup {
        a: Player { points: 0, prev_response: false, color: Color::new_random() },
        b: Player { points: 0, prev_response: false, color: Color::new_random() },
    }).collect::<Vec<_>>()

    let gaem = Game { edges: vec![
        Matchup { a:  },
    ] }
}
