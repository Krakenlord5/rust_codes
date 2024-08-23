use std::fs::File;
use std::path::Path;
use rand::Rng;
use std::{ io::{self}, process::exit, thread::sleep, time::{Duration, Instant}};

use crossterm::{cursor::{Hide, MoveDown, MoveTo, MoveToColumn, MoveToRow, Show}, event::{poll, read, Event, KeyCode, KeyEvent}, execute, style::{Color, Print, PrintStyledContent, Stylize}, terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize}};
struct HitCircle {
    start_pos: u16,
    pos: u16,
    color: Color,
}
struct CircleList {
    circles: Vec<HitCircle>,
}

enum HitType {
    Miss,
    Great,
    Perfect,
}

impl HitCircle {
    fn new() -> Self {
        let random_pos: i32 = rand::thread_rng().gen_range(0..=1);
        if random_pos == 0 {
            HitCircle {
                start_pos: 0,
                pos: 0,
                color: Color::Red,
            }
        } else {
            HitCircle {
                start_pos: size().unwrap().0 - 3,
                pos: size().unwrap().0 - 3,
                color: Color::Cyan,
            }
        }
    }
}

impl CircleList {
    fn new() -> Self {
        CircleList {
            circles: Vec::new(),
        }
    }

    fn add_circle(&mut self) {
        self.circles.push(HitCircle::new());
    }

    fn delete(&mut self) {
        self.circles.remove(0);
    }

    fn move_circles(&mut self) {
        for item in &mut self.circles {
            if item.start_pos == 0 {
                item.pos += 1;
            } else {
                item.pos -= 1;
            }
        }
    }
}

// bullshit code
fn main() -> io::Result<()> {
    let original_terminal_size = size().unwrap();
    let mut stdout: io::Stdout = io::stdout();
    let mut spawn_timer: i32 = 5;
    let mut initial_spawn_timer: i32 = 10;
    let mut circle_list: CircleList = CircleList::new();
    let mut score: i32 = 0;
    let mut combo: i32 = 0;
    let mut multiplier: f64 = 1.0;
    let mut health: u16 = 10;
    let mut choose_diff: bool = true;
    let mut sleep_period: u64 = 100;
    let mut final_text = "Game Over! :((((";

    execute!(stdout, Clear(ClearType::All), Hide)?;
    execute!(stdout, MoveTo(0, 0), Print("Hello! Welcome to..."), MoveTo(0, 2), PrintStyledContent("TAI".with(Color::Red)), PrintStyledContent("KO".with(Color::Cyan)), PrintStyledContent(" V2! 🥁 🥁 🥁".with(Color::White)), MoveDown(2), MoveToColumn(0))?;

    println!("Here are some brief explanations of the game!");
    println!("\n• There will be 2 receptors in the middle of the screen, the left and the right receptor.");
    println!("\n• Circles will spawn in from both sides at ramdom times.");
    println!("• The spawn rate will depend on the difficulty you choose.");
    println!("\n• When the circles enter the left receptor area, tap the 's' or 'a' key on your keyboard.");
    println!("• When the circles enter the right receptor area, tap the 'l' or 'k' key on your keyboard.");
    println!("\n• The Esc key can be used to exit the game at any time.");
    println!("\n• There will be judgements based on how on time your taps are.");
    execute!(stdout, MoveDown(1), MoveToColumn(2), PrintStyledContent("Perfect! ".with(Color::Cyan)), Print("will give you +300 score."))?;
    execute!(stdout, MoveDown(1), MoveToColumn(2), PrintStyledContent("Great! ".with(Color::Green)), Print("will give you +100 score."))?;
    execute!(stdout, MoveDown(1), MoveToColumn(2), PrintStyledContent("Miss! ".with(Color::Red)), Print("will give you no score and deduct 1 health from you."), MoveDown(2), MoveToColumn(0))?;
    println!("• You lose when your health reaches 0.");
    println!("• You win when the timer reaches 0.");
    println!("\nAnd lastly... Have fun and enjoy the game!");
    println!("Press any key to continue...");

    enable_raw_mode()?;
    match read().unwrap() {
        Event::Key(key) => {
            match key.code {
                KeyCode::Esc => {
                    execute!(stdout, Clear(ClearType::All))?;
                    disable_raw_mode()?;
                    println!("Bye bye!");
                    exit(1)
                }
                _ => {}
            }
        }
        _ => {}
    }
    loop {
        if choose_diff {
            execute!(stdout, SetSize(60, 20), Clear(ClearType::All), MoveToRow(3), MoveToColumn(18), Print("Choose Difficulty"), MoveDown(3), MoveToColumn(18), PrintStyledContent("(1) Easy".with(Color::Cyan)), MoveDown(3), MoveToColumn(18), PrintStyledContent("(2) Normal".with(Color::Green)), MoveDown(3), MoveToColumn(18), PrintStyledContent("(3) Hard".with(Color::Red)), MoveDown(3), MoveToColumn(18), PrintStyledContent("(4) Lunatic".with(Color::Magenta)))?;
            match read().unwrap() {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char('1') => {
                            sleep_period = 60;
                            initial_spawn_timer = 8;
                        }
                        KeyCode::Char('2') => {
                            sleep_period = 45;
                            initial_spawn_timer = 6;
                        }
                        KeyCode::Char('3') => {
                            sleep_period = 35;
                            initial_spawn_timer = 6;
                        }
                        KeyCode::Char('4') => {
                            sleep_period = 25;
                            initial_spawn_timer = 5;
                        }
                        KeyCode::Esc => {
                            execute!(stdout, SetSize(original_terminal_size.0, original_terminal_size.1), Clear(ClearType::All), Show)?;
                            disable_raw_mode()?;
                            println!("Bye bye!");
                            exit(1)
                        }
                        _ => continue
                    }
                }
                _ => continue
            }
            execute!(stdout, Clear(ClearType::All), MoveTo(24, 9), Print("vvvv    vvvv"), MoveTo(24, 11), Print("∧∧∧∧    ∧∧∧∧"), MoveTo(29, 8), Print(format!("{}x", combo)), MoveTo(20, 0), Print("Time: 0"), MoveTo(51, 0), Print(format!("{:09}", score)), MoveToRow(5), PrintStyledContent("\r〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜".with(Color::Red)), MoveToRow(16), PrintStyledContent("\r〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜".with(Color::Cyan)))?;
            sleep(Duration::from_secs(1));
            choose_diff = false;
        }
        let start_time = Instant::now();
        let countdown = Duration::from_secs(60);
        let mut remaining = Duration::ZERO;
        while health > 0 {
            let time_passed = start_time.elapsed();
            if time_passed <= countdown {
                remaining = countdown - time_passed;
                execute!(stdout, MoveTo(20, 0), Print(format!("Time: {:?}", remaining.as_secs())))?;
            }
            if time_passed >= countdown {
                final_text = "Congratulations!";
                break
            }
            sleep(Duration::from_millis(sleep_period));

            if spawn_timer == 0 {
                circle_list.add_circle();
                spawn_timer = initial_spawn_timer;
            } else {
                spawn_timer -= 1
            }
            
            for i in 1..=health {
                execute!(stdout, MoveTo(i, 0), PrintStyledContent("█".with(crossterm::style::Color::Red)))?;
            }
            execute!(stdout, Print(format!(" {}", health)))?;
            
            if !circle_list.circles.is_empty() {
                
                circle_list.move_circles();

                if circle_list.circles[0].pos == 29 {
                    combo = 0;
                    circle_list.delete();
                    health -= 1;
                    multiplier = 1.0;
                    execute!(stdout, MoveTo(28, 10), Print("    "), MoveTo(0, 0), Print("              "))?;
                    execute!(io::stdout(), MoveTo(27, 14), Clear(ClearType::CurrentLine), PrintStyledContent("Miss!".with(crossterm::style::Color::Red)), MoveTo(29, 8), Clear(ClearType::CurrentLine), Print(format!("{}x", combo)))?;
                }
                for item in &circle_list.circles {
                    execute!(stdout, MoveTo(item.pos, 10), PrintStyledContent(" O ".with(item.color)))?;
                }
            }

            if poll(Duration::ZERO).unwrap() {
                if let Ok(Event::Key(KeyEvent { code, .. })) = read() {
                    match code {
                        KeyCode::Char('s') | KeyCode::Char('a') => {
                            if !circle_list.circles.is_empty() {
                                if circle_list.circles[0].start_pos == 0 {
                                    let hit_type = match circle_list.circles[0].pos {
                                        20..=23 => HitType::Great,
                                        24..=28 => HitType::Perfect,
                                        _ => HitType::Miss
                                    };
                                    execute!(stdout, MoveTo(circle_list.circles[0].pos, 10), Print("  "))?;
                                    match_hit(hit_type, &mut combo, &mut multiplier, &mut score, &mut health, &mut circle_list)?;
                                } 
                            }   
                        }
                        KeyCode::Char('l') | KeyCode::Char('k') => {
                            if !circle_list.circles.is_empty() {
                                if circle_list.circles[0].start_pos == size().unwrap().0 - 3 {
                                    let hit_type = match circle_list.circles[0].pos {
                                        35..=38 => HitType::Great,
                                        30..=34 => HitType::Perfect,
                                        _ => HitType::Miss
                                    };
                                    execute!(stdout, MoveTo(circle_list.circles[0].pos, 10), Print("  "))?;
                                    match_hit(hit_type, &mut combo, &mut multiplier, &mut score, &mut health, &mut circle_list)?;
                                }  
                            }   
                        }
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }  
        execute!(stdout, Clear(ClearType::All), MoveTo(size().unwrap().0 / 2 - 5, size().unwrap().1 / 2 - 4), Print(final_text.to_string()), MoveDown(2), MoveToColumn(28), Print("Score: "), Print(score.to_string()), MoveDown(2), MoveToColumn(16), PrintStyledContent("(q) Quit the game".with(Color::Red)), PrintStyledContent("   (r) Retry".with(Color::Green)), MoveDown(2), MoveToColumn(18), Print("(c) Choose new difficulty"))?;
        match read().unwrap() {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('r') => {
                        score = 0;
                        health = 10;
                        combo = 0;
                        multiplier = 1.0;
                        spawn_timer = 5;
                        circle_list = CircleList::new();
                        remaining = Duration::ZERO;
                        final_text = "Game Over! :((((";
                        execute!(stdout, Clear(ClearType::All),  MoveTo(24, 9), Print("vvvv    vvvv"), MoveTo(24, 11), Print("∧∧∧∧    ∧∧∧∧"), MoveTo(29, 8), Print(format!("{}x", combo)), MoveTo(20, 0), Print(format!("Time: {:?}", remaining.as_secs())), MoveTo(51, 0), Print(format!("{:09}", score)), MoveToRow(5), PrintStyledContent("\r〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜".with(Color::Red)), MoveToRow(16), PrintStyledContent("\r〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜〜".with(Color::Cyan)))?;
                        sleep(Duration::from_secs(1));
                    }
                    KeyCode::Char('c') => {
                        score = 0;
                        health = 10;
                        combo = 0;
                        multiplier = 1.0;
                        spawn_timer = 5;
                        circle_list = CircleList::new();
                        remaining = Duration::ZERO;
                        choose_diff = true;
                        final_text = "Game Over! :((((";
                    }
                    KeyCode::Char('q') => break,
                    _ => health = 0,
                }
            }
            _ => {}
        }
    }
    execute!(stdout, SetSize(original_terminal_size.0, original_terminal_size.1), Show, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, size().unwrap().1), Print("Bye bye!\n\r"))?;
    disable_raw_mode()?;
    Ok(())
}

fn match_hit(hit_type: HitType, combo: &mut i32, multiplier: &mut f64, score: &mut i32, health: &mut u16, circle_list: &mut CircleList) -> io::Result<()> {
    match hit_type {
        HitType::Perfect => {
            *score += (300.0 * *multiplier) as i32;
            *combo += 1;
            *multiplier += 0.01;
            execute!(io::stdout(), MoveTo(27, 14), Clear(ClearType::CurrentLine), PrintStyledContent("Perfect!".with(crossterm::style::Color::Cyan)))?;
        },
        HitType::Great => {
            *score += (100.0 * *multiplier) as i32;
            *combo += 1;
            *multiplier += 0.01;
            execute!(io::stdout(), MoveTo(27, 14), Clear(ClearType::CurrentLine), PrintStyledContent("Great!".with(crossterm::style::Color::Green)))?;
        },
        _ => {
            *combo = 0;
            *multiplier = 1.0;
            *health -= 1;
            execute!(io::stdout(), MoveTo(27, 14), Clear(ClearType::CurrentLine), PrintStyledContent("Miss!".with(crossterm::style::Color::Red)), MoveTo(0, 0), Print("              "))?;
        },
    }
    circle_list.delete();
    execute!(io::stdout(), MoveTo(29, 8), Print(format!("{}x", combo)), MoveTo(51, 0), Print(format!("{:09}", score)))?;
    Ok(())
}  

