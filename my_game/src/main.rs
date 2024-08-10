use std::io::{self, Write};
use std::{thread, time::Duration};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::{
    style::{Stylize, Print},
    cursor::{MoveTo, MoveLeft, MoveRight},
    terminal::{Clear, ClearType, SetSize},
    event::{read, poll, Event},
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let esc = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);

    loop {
        if read().unwrap() == Event::Key(esc) {
            println!("Bye bye");
            break
        }
    }
    // let mut x = 0;
    // let mut y = 0;
    // execute!(stdout, SetSize(120, 40), Clear(ClearType::All))?;
    // while y < 10 {
    //     while x <= 20 {
    //         execute!(
    //             stdout, 
    //             MoveTo(x,y),
    //             Print("0"),
    //         )?;
    //         thread::sleep(Duration::from_millis(50));
    //         x += 1;
    //     }
    //     execute!(stdout, Print("0"))?;
    //     y += 1;
    //     while x > 0 {
    //         execute!(
    //             stdout, 
    //             MoveTo(x,y),
    //             Print("0"),
    //         )?;
    //         thread::sleep(Duration::from_millis(50));
    //         x -= 1;
    //     }
    //     execute!(stdout, MoveLeft(2), Print("0"))?;
    //     y += 1
    // }
    Ok(())
}