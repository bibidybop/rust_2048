use crate::board_manipulations;
use rand::Rng;

pub enum Action {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub tiles: [u32; 16],
}

impl Field {
    pub fn take_action(&mut self, action: &Action) {
        let tiles = match action {
            Action::Left => board_manipulations::left(&self.tiles),
            Action::Right => board_manipulations::right(&self.tiles),
            Action::Up => board_manipulations::up(&self.tiles),
            Action::Down => board_manipulations::down(&self.tiles),
        };
        if self.tiles != tiles {
            self.tiles = tiles;
            self.add_random_tile();
        }
    }
    pub fn take_random_action(&mut self) {
        let moves = self.available_moves();
        let i = rand::thread_rng().gen_range(0..moves.len());
        self.take_action(&moves[i]);
    }

    pub fn available_moves(&self) -> Vec<Action> {
        let mut moves = Vec::<Action>::new();
        if board_manipulations::can_move_left(&self.tiles) {
            moves.push(Action::Left);
        }
        if board_manipulations::can_move_right(&self.tiles) {
            moves.push(Action::Right);
        }
        if board_manipulations::can_move_up(&self.tiles) {
            moves.push(Action::Up);
        }
        if board_manipulations::can_move_down(&self.tiles) {
            moves.push(Action::Down);
        }
        moves
    }

    // Add a random tile after the player taken the action
    fn add_random_tile(&mut self) {
        let empty_tiles = self.index_empty_tiles();
        if empty_tiles.len() > 0 {
            let i = rand::thread_rng().gen_range(0..empty_tiles.len());
            self.tiles[empty_tiles[i]] = 2u32.pow(rand::thread_rng().gen_range(1..=2));
        }
    }

    // Locate indices of empty tiles
    fn index_empty_tiles(&self) -> Vec<usize> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|el| *el.1 == 0)
            .map(|el| el.0)
            .collect()
    }

    // Generate a random field with a maximum of max_nonzero_tiles nonzero tiles of 2 or 4
    pub fn random(max_nonzero_tiles: u8) -> Self {
        let mut tiles = [0; 16];
        for _ in 0..max_nonzero_tiles {
            let i = rand::thread_rng().gen_range(0..16);
            tiles[i] = 2u32.pow(rand::thread_rng().gen_range(1..=2));
        }
        Self { tiles }
    }
}

