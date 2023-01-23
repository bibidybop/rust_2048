use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use std::io::{Stdout, Write};

pub fn render_field_boundaries(stdout: &mut Stdout) {
    stdout.queue(MoveTo(0, 0)).unwrap();
    for i in 0..=9 {
        stdout
            .write(if i % 2 != 0 {
                b"+-------+-------+-------+-------+"
            } else {
                b"|       |       |       |       |"
            })
            .unwrap();
        stdout.queue(MoveTo(0, i)).unwrap();
    }
}

pub fn render(stdout: &mut Stdout, tiles: &[u32; 16]) {
    // stdout.queue(Clear(ClearType::All)).unwrap();
    // render_field_boundaries(stdout);
    tiles.iter().enumerate().for_each(|(i, &val)| {
        stdout
            .queue(MoveTo((8 * (i % 4) as u16) + 1, 2 * (i / 4) as u16 + 1))
            .unwrap();
        if val > 0u32 {
            stdout
                .write(format!("\x1b[9{}m{:>6}\x1b[0m", val.trailing_zeros() % 8, val).as_bytes())
                .unwrap();
        } else {
            stdout.write(b"      ").unwrap();
        }
    });
    stdout.flush().unwrap();
}
