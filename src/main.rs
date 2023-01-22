use crossterm::{
    cursor::{position, MoveLeft, MoveRight, MoveToColumn},
    event::{read, Event, KeyCode, KeyEvent},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ScrollUp},
    ExecutableCommand, QueueableCommand, Result,
};
use std::io::{stdout, Stdout, Write};

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdout = stdout();
    let mut caret_position;

    terminal::enable_raw_mode()?;

    'repl: loop {
        stdout
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print(">"))?
            .execute(ResetColor)?;
        let (mut input_start_col, ..) = position()?;
        input_start_col += 1;
        caret_position = input_start_col;

        'input: loop {
            match read()? {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Char(c) => {
                            let insertion_point =
                                caret_position as usize - input_start_col as usize;
                            if insertion_point == buffer.len() {
                                stdout.queue(Print(c))?;
                            } else {
                                stdout
                                    .queue(Print(c))?
                                    .queue(Print(&buffer[insertion_point..]))?
                                    .queue(MoveToColumn(caret_position + 1))?;
                            }
                            stdout.flush()?;
                            caret_position += 1;
                            buffer.insert(insertion_point, c);
                        }
                        KeyCode::Backspace => {
                            let insertion_point =
                                caret_position as usize - input_start_col as usize;
                            if insertion_point == buffer.len() && !buffer.is_empty() {
                                buffer.pop();
                                stdout
                                    .queue(MoveLeft(1))?
                                    .queue(Print(' '))?
                                    .queue(MoveLeft(1))?;
                                stdout.flush()?;
                                caret_position -= 1;
                            } else if insertion_point < buffer.len() && !buffer.is_empty() {
                                buffer.remove(insertion_point - 1);
                                stdout
                                    .queue(MoveLeft(1))?
                                    .queue(Print(&buffer[(insertion_point - 1)..]))?
                                    .queue(Print(' '))?
                                    .queue(MoveToColumn(caret_position - 1))?;
                                stdout.flush()?;
                                caret_position -= 1;
                            }
                        }
                        KeyCode::Delete => {
                            let insertion_point =
                                caret_position as usize - input_start_col as usize;
                            if insertion_point < buffer.len() && !buffer.is_empty() {
                                buffer.remove(insertion_point);
                                stdout
                                    .queue(Print(&buffer[insertion_point..]))?
                                    .queue(Print(' '))?
                                    .queue(MoveToColumn(caret_position))?;
                                stdout.flush()?;
                            }
                        }
                        KeyCode::Enter => {
                            if buffer == "exit" {
                                break 'repl;
                            } else {
                                print_message(&mut stdout, &format!("Our buffer: {buffer}"))?;
                                buffer.clear();
                                break 'input;
                            }
                        }
                        KeyCode::Left => {
                            if caret_position > input_start_col {
                                stdout.queue(MoveLeft(1))?;
                                stdout.flush()?;
                                caret_position -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if (caret_position as usize) < (input_start_col as usize) + buffer.len()
                            {
                                stdout.queue(MoveRight(1))?;
                                stdout.flush()?;
                                caret_position += 1;
                            }
                        }
                        _ => {}
                    };
                }
                Event::Mouse(event) => {
                    print_message(&mut stdout, &format!("{event:?}"))?;
                }
                Event::Resize(width, height) => {
                    print_message(&mut stdout, &format!("width: {width} and height: {height}"))?;
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    println!();

    Ok(())
}

fn print_message(stdout: &mut Stdout, msg: &str) -> Result<()> {
    stdout
        .queue(ScrollUp(1))?
        .queue(MoveToColumn(1))?
        .queue(Print(msg))?
        .queue(ScrollUp(1))?
        .queue(MoveToColumn(1))?;
    stdout.flush()?;
    Ok(())
}
