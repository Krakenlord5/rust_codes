use std::io::{stdout, Write};

use crossterm::{
    ExecutableCommand, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    event,
};

fn main() -> std::io::Result<()> {
    // using the macro
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
    )?;

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?;
    
    Ok(())
}