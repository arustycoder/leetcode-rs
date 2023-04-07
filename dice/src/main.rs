use crate::DiceOP::{Down, Left, Right, Up};

#[tokio::main]
async fn main() {
    let mut dice_y = Dice::new(6, 4, 5);
    let mut dice_k = Dice::new(2, 1, 3);

    let count = 5;

    let mut sum_y = 0;
    let mut sum_k = 0;

    PLAYER_Y.iter().take(count).for_each(|op| {
        dice_y.roll_it(op);
        sum_y += dice_y.value();
        assert!(dice_y.check_valid());
    });

    PLAYER_K.iter().take(count).for_each(|op| {
        dice_k.roll_it(op);
        sum_k += dice_k.value();
        assert!(dice_k.check_valid());
    });

    println!(" count {count} sum_y {sum_y} sum_k {sum_k}");
}

enum DiceOP {
    Down(usize),
    Up(usize),
    Right(usize),
    Left(usize),
}

const PLAYER_Y: [DiceOP; 8] = [
    Right(1),
    Down(3),
    Left(5),
    Right(7),
    Down(9),
    Left(11),
    Right(13),
    Up(2),
];

const PLAYER_K: [DiceOP; 8] = [
    Right(2),
    Down(4),
    Left(6),
    Right(8),
    Down(10),
    Left(12),
    Right(14),
    Up(2),
];

// 0 -> down, 1 -> value, 2 -> right
struct Dice(u8, u8, u8);

impl Dice {
    fn new(down: u8, value: u8, right: u8) -> Self {
        Self(down, value, right)
    }

    fn value(&self) -> u8 {
        self.1
    }

    fn check_valid(&self) -> bool {
        self.0 + self.1 != 7 && self.1 + self.2 != 7 && self.0 + self.2 != 7
    }

    fn right_n(&mut self, n: usize) {
        for _ in 0..n {
            self.right();
        }
    }

    fn right(&mut self) {
        let value = self.value();
        self.1 = self.2;
        self.2 = 7 - value;
    }

    fn top_n(&mut self, n: usize) {
        for _ in 0..n {
            self.top();
        }
    }

    fn top(&mut self) {
        let value = self.value();
        self.1 = self.0;
        self.0 = 7 - value;
    }

    fn left_n(&mut self, n: usize) {
        for _ in 0..n {
            self.left();
        }
    }

    fn left(&mut self) {
        let value = self.value();
        self.1 = self.2;
        self.2 = 7 - value;
    }

    fn down_n(&mut self, n: usize) {
        for _ in 0..n {
            self.down();
        }
    }

    fn down(&mut self) {
        let value = self.value();
        self.1 = self.0;
        self.0 = 7 - value;
    }

    pub fn roll_it(&mut self, op: &DiceOP) {
        match op {
            Down(c) => self.down_n(*c),
            Up(c) => self.top_n(*c),
            Right(c) => self.right_n(*c),
            Left(c) => self.left_n(*c),
        }
    }
}
