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
            new_tiles[write_index] <<= 1;
            write_index += 1;
            last_seen_value = 0;
        } else {
            write_index += 1;
            last_seen_value = tiles[i];
            new_tiles[write_index] = last_seen_value;
        }
    }
    return new_tiles;
}

#[inline]
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
    return new_tiles;
}

#[inline]
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
    return new_tiles;
}

#[inline]
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
    return new_tiles;
}

#[inline]
pub fn can_move_left(tiles: &[u32; 16]) -> bool {
    for &i in &[0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14] {
        if (tiles[i] == 0 && tiles[i + 1] != 0) || (tiles[i] != 0 && tiles[i] == tiles[i + 1]) {
            return true;
        }
    }
    return false;
}

#[inline]
pub fn can_move_right(tiles: &[u32; 16]) -> bool {
    for &i in &[0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14] {
        if (tiles[i] != 0 && tiles[i + 1] == 0) || (tiles[i + 1] != 0 && tiles[i] == tiles[i + 1]) {
            return true;
        }
    }
    return false;
}

#[inline]
pub fn can_move_up(tiles: &[u32; 16]) -> bool {
    for i in 0..12 {
        if (tiles[i] == 0 && tiles[i + 4] != 0) || (tiles[i] != 0 && tiles[i] == tiles[i + 4]) {
            return true;
        }
    }
    return false;
}
#[inline]
pub fn can_move_down(tiles: &[u32; 16]) -> bool {
    for i in 4..16 {
        if (tiles[i] == 0 && tiles[i - 4] != 0) || (tiles[i] != 0 && tiles[i] == tiles[i - 4]) {
            return true;
        }
    }
    return false;
}

#[inline]
pub fn can_move(tiles: &[u32; 16]) -> bool {
    can_move_left(tiles) || can_move_right(tiles) || can_move_up(tiles) || can_move_down(tiles)
}
