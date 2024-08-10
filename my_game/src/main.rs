use std::io;

use crossterm::cursor::{Hide, MoveTo, MoveToColumn, RestorePosition, SavePosition, Show};
use crossterm::event::KeyCode;
use crossterm::{execute, terminal};
use crossterm::terminal::{size, SetSize};
use crossterm::{
    style::Print,
    terminal::{Clear, ClearType, enable_raw_mode},
    event::{read, Event},
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut x = 0;
    let mut y = 0;
    enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), SetSize(80, 30), Show, MoveTo(x, y))?;

    loop {
        match read().unwrap() {
            Event::Key(key) => {
                if key.code == KeyCode::Esc {
                    execute!(stdout, Clear(ClearType::All))?;
                    break
                } else {
                    match key.code {
                        KeyCode::Left => {
                            if x > 0 {                                
                                x -= 1;
                            }
                        },
                        KeyCode::Right => {
                            if x < size().unwrap().0 {
                                x += 1;
                            }
                        },
                        KeyCode::Up => {
                            if y > 0 {
                                y -= 1;
                            }
                        },
                        KeyCode::Down => {
                            if y < size().unwrap().1 {
                                y += 1;
                            }
                        },
                        _ => {}
                    }

                    execute!(stdout, Clear(ClearType::All), MoveTo(x, y), SavePosition,  MoveTo(0, size().unwrap().1), Print(x.to_string()), Print(" "), Print(y.to_string()), RestorePosition)?;
                }
            },
            _ => {},
        }
    }
    Ok(())
}