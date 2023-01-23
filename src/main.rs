use crossterm::event::{self, Event, KeyCode};
use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use game_2048::field::{Action, Field};
use game_2048::render::{render, render_field_boundaries};
use std::error::Error;
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    //terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?; // ? to crash if raw mode is unaccessible
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?; // hide thread

    //field
    let mut field = Field::random(4);
    render_field_boundaries(&mut stdout);

    //game
    'gameloop: loop {
        thread::sleep(Duration::from_millis(10));

        // input from user
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Left => field.take_action(&Action::Left),
                    KeyCode::Right => field.take_action(&Action::Right),
                    KeyCode::Up => field.take_action(&Action::Up),
                    KeyCode::Down => field.take_action(&Action::Down),
                    _ => {
                        continue 'gameloop; // skip the render step
                    }
                }
            }
        }
        // update the tiles on the screen
        render(&mut stdout, &field.tiles);
    }

    //cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}
