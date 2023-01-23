#[inline]
pub fn left(tiles: &[u32; 16]) -> [u32; 16] {
    let mut write_index: usize = 0;
    let mut last_seen_value: u32 = tiles[0];
    let mut new_tiles = [0u32; 16];
    new_tiles[0] = tiles[0];
    for i in 1..16 {
        // when new row encountered
        if i % 4 == 0 {
            write_index = i;
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
            continue;
        }
        if tiles[i] == 0 {
            continue;
        }
        if last_seen_value == 0 {
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
        } else if tiles[i] == last_seen_value {
            new_tiles[write_index] *= 2;
            write_index += 1;
            last_seen_value = 0;
        } else {
            write_index += 1;
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
        }
    }
    new_tiles
}

pub fn right(tiles: &[u32; 16]) -> [u32; 16] {
    let mut write_index: usize = 15;
    let mut last_seen_value: u32 = tiles[15];
    let mut new_tiles = [0u32; 16];
    new_tiles[15] = tiles[15];

    for i in (0..15).rev() {
        // when new row encountered
        if i % 4 == 3 {
            write_index = i;
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
            continue;
        }
        if tiles[i] == 0 {
            continue;
        }
        if last_seen_value == 0 {
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
        } else if tiles[i] == last_seen_value {
            new_tiles[write_index] <<= 1;
            write_index -= 1;
            last_seen_value = 0;
        } else {
            write_index -= 1;
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
        }
    }
    new_tiles
}

pub fn up(tiles: &[u32; 16]) -> [u32; 16] {
    let mut write_index: usize = 0;
    let mut last_seen_value: u32 = tiles[0];
    let mut new_tiles = [0u32; 16];
    new_tiles[0] = tiles[0];

    for &i in &[4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15] {
        if i < 4 {
            write_index = i;
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
            continue;
        }
        if tiles[i] == 0 {
            continue;
        }
        if last_seen_value == 0 {
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
        } else if tiles[i] == last_seen_value {
            new_tiles[write_index] <<= 1;
            write_index += 4;
            last_seen_value = 0;
        } else {
            write_index += 4;
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
        }
    }
    new_tiles
}

pub fn down(tiles: &[u32; 16]) -> [u32; 16] {
    let mut write_index: usize = 15;
    let mut last_seen_value: u32 = tiles[15];
    let mut new_tiles = [0u32; 16];
    new_tiles[15] = tiles[15];

    // for &i in &[4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15] {
    for &i in &[11, 7, 3, 14, 10, 6, 2, 13, 9, 5, 1, 12, 8, 4, 0] {
        if i >= 12 {
            write_index = i;
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
            continue;
        }
        if tiles[i] == 0 {
            continue;
        }
        if last_seen_value == 0 {
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
        } else if tiles[i] == last_seen_value {
            new_tiles[write_index] <<= 1;
            write_index -= 4;
            last_seen_value = 0;
        } else {
            write_index -= 4;
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
        }
    }
    new_tiles
}

pub fn can_move_left(tiles: &[u32; 16]) -> bool {
    for &i in &[0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14] {
        if (tiles[i] == 0 && tiles[i + 1] != 0) || (tiles[i] != 0 && tiles[i] == tiles[i + 1]) {
            return true;
        }
    }
    false
}

pub fn can_move_right(tiles: &[u32; 16]) -> bool {
    for &i in &[0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14] {
        if (tiles[i] != 0 && tiles[i + 1] == 0) || (tiles[i + 1] != 0 && tiles[i] == tiles[i + 1]) {
            return true;
        }
    }
    false
}

pub fn can_move_up(tiles: &[u32; 16]) -> bool {
    for i in 0..12 {
        if (tiles[i] == 0 && tiles[i + 4] != 0) || (tiles[i] != 0 && tiles[i] == tiles[i + 4]) {
            return true;
        }
    }
    false
}

pub fn can_move_down(tiles: &[u32; 16]) -> bool {
    for i in 4..16 {
        if (tiles[i] == 0 && tiles[i - 4] != 0) || (tiles[i] != 0 && tiles[i] == tiles[i - 4]) {
            return true;
        }
    }
    false
}

pub fn can_move(tiles: &[u32; 16]) -> bool {
    can_move_left(tiles) || can_move_right(tiles) || can_move_up(tiles) || can_move_down(tiles)
}

pub fn next_tiles_after_player_move(tiles: &[u32; 16]) -> Vec<[u32; 16]> {
    let mut next_tiles = Vec::new();
    if can_move_left(tiles) {
        next_tiles.push(left(tiles));
    }
    if can_move_right(tiles) {
        next_tiles.push(right(tiles));
    }
    if can_move_up(tiles) {
        next_tiles.push(up(tiles));
    }
    if can_move_down(tiles) {
        next_tiles.push(down(tiles));
    }
    return next_tiles;
}

pub fn next_tiles_after_spawn(tiles: &[u32; 16]) -> Vec<[u32; 16]> {
    let mut next_states = Vec::with_capacity(30); // max of 15 zero tiles, at each tile there can be 2 or 4
    let mut possible_board_state = tiles.clone();
    (0..16).filter(|&i| tiles[i] == 0).for_each(|i| {
        possible_board_state[i] = 2;
        next_states.push(possible_board_state.clone());
        possible_board_state[i] = 4;
        next_states.push(possible_board_state.clone());
        possible_board_state[i] = 0;
    });

    next_states
}

//unit-tests go here, integration tests go into a separate <tests> folder
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left() {
        let b = [2, 2, 4, 4, 2, 4, 4, 4, 2, 4, 8, 4, 2, 4, 8, 8];
        let e: [u32; 16] = [4, 8, 0, 0, 2, 8, 4, 0, 2, 4, 8, 4, 2, 4, 16, 0];
        assert_eq!(left(&b), e);
        let b = [0, 2, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let e = [2, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(left(&b), e);
        let tiles = [0, 2, 2, 0, 0, 0, 2, 2, 4, 0, 0, 4, 2, 0, 2, 4];
        let expected: [u32; 16] = [4, 0, 0, 0, 4, 0, 0, 0, 8, 0, 0, 0, 4, 4, 0, 0];
        assert_eq!(left(&tiles), expected);
        let tiles = [4, 4, 2, 2, 4, 4, 4, 2, 2, 4, 8, 4, 8, 8, 4, 2];
        let expected: [u32; 16] = [8, 4, 0, 0, 8, 4, 2, 0, 2, 4, 8, 4, 16, 4, 2, 0];
        assert_eq!(left(&tiles), expected);
    }

    #[test]
    fn test_right() {
        let b = [2, 2, 4, 4, 2, 4, 4, 4, 2, 4, 8, 4, 2, 4, 8, 8];
        let e: [u32; 16] = [0, 0, 4, 8, 0, 2, 4, 8, 2, 4, 8, 4, 0, 2, 4, 16];
        assert_eq!(right(&b), e);
        let b = [0, 2, 2, 0, 0, 0, 2, 2, 4, 0, 0, 4, 2, 0, 2, 4];
        let e: [u32; 16] = [0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 8, 0, 0, 4, 4];
        assert_eq!(right(&b), e);
        let b = [0, 2, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let e = [0, 0, 2, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(right(&b), e);
        let b = [0, 2, 2, 0, 0, 0, 2, 2, 4, 0, 0, 4, 2, 0, 2, 4];
        let e: [u32; 16] = [0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 8, 0, 0, 4, 4];
        assert_eq!(right(&b), e);
        let b = [4, 4, 2, 2, 4, 4, 4, 2, 2, 4, 8, 4, 8, 8, 4, 2];
        let e: [u32; 16] = [0, 0, 8, 4, 0, 4, 8, 2, 2, 4, 8, 4, 0, 16, 4, 2];
        assert_eq!(right(&b), e);
    }

    #[test]
    fn test_up() {
        let b = [2, 2, 2, 2, 2, 4, 4, 4, 4, 4, 8, 8, 4, 8, 8, 4];
        let e: [u32; 16] = [4, 2, 2, 2, 8, 8, 4, 4, 0, 8, 16, 8, 0, 0, 0, 4];
        assert_eq!(up(&b), e);
    }

    #[test]
    fn test_down() {
        let tiles = [4, 8, 8, 4, 4, 4, 8, 8, 2, 4, 4, 4, 2, 2, 2, 2];
        let expected: [u32; 16] = [0, 0, 0, 4, 0, 8, 16, 8, 8, 8, 4, 4, 4, 2, 2, 2];
        assert_eq!(down(&tiles), expected);
    }

    #[test]
    fn test_can_move_left() {
        assert!(can_move_left(&[
            2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]));
        assert!(!can_move_left(&[
            2, 4, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]));
        assert!(!can_move_left(&[
            2, 4, 8, 4, 2, 4, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]));
    }

    #[test]
    fn test_can_move_right() {
        assert!(!can_move_right(&[
            2, 4, 8, 16, 0, 2, 4, 8, 2, 4, 2, 4, 0, 0, 0, 0
        ]));
        assert!(can_move_right(&[
            2, 4, 8, 0, 16, 2, 4, 8, 2, 4, 2, 4, 0, 0, 0, 0
        ]));
        assert!(can_move_right(&[
            2, 4, 8, 16, 0, 2, 4, 8, 2, 4, 2, 4, 0, 2, 0, 0
        ]));
    }

    #[test]
    fn test_can_move_up() {
        assert!(can_move_left(&[
            2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]));
        assert!(!can_move_left(&[
            2, 4, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]));
        assert!(!can_move_left(&[
            2, 4, 8, 4, 2, 4, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]));
    }

    #[test]
    fn test_can_move_down() {
        assert!(can_move_left(&[
            2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]));
        assert!(!can_move_left(&[
            2, 4, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]));
        assert!(!can_move_left(&[
            2, 4, 8, 4, 2, 4, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]));
    }
}
