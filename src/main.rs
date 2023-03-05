use crossterm::{
    cursor::{position, MoveLeft, MoveRight, MoveToColumn, MoveToNextLine},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::io::{stdout, Stdout, Write};

mod line_buffer;

fn main() -> Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    let mut buffer = line_buffer::LineBuffer::new();

    'repl: loop {
        stdout
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print(">"))?
            .execute(ResetColor)?;
        let (mut prompt_offset, ..) = position()?;
        prompt_offset += 1;

        'input: loop {
            match read()? {
                Event::Key(KeyEvent { code, modifiers }) => {
                    match code {
                        KeyCode::Char(c) => {
                            if modifiers == KeyModifiers::CONTROL && c == 'd' {
                                stdout.queue(MoveToNextLine(1))?.queue(Print("exit"))?;
                                break 'repl;
                            }
<<<<<<< HEAD
                            let insertion_point = buffer.get_insertion_point();
                            if insertion_point == buffer.get_buffer_length() {
=======
                            let insertion_point =
                                caret_position as usize - input_start_col as usize;
                            if insertion_point == buffer.len() {
>>>>>>> master
                                stdout.queue(Print(c))?;
                            } else {
                                stdout
                                    .queue(Print(c))?
                                    .queue(Print(buffer.slice_buffer(insertion_point)))?
                                    .queue(MoveToColumn(
                                        insertion_point as u16 + 1 + prompt_offset,
                                    ))?;
                            }
                            stdout.flush()?;
                            buffer.inc_insertion_point();
                            buffer.insert_char(insertion_point, c);
                        }
                        KeyCode::Backspace => {
                            let insertion_point = buffer.get_insertion_point();
                            if insertion_point == buffer.get_buffer_length() && !buffer.is_empty() {
                                buffer.pop();
                                buffer.dec_insertion_point();
                                stdout
                                    .queue(MoveLeft(1))?
                                    .queue(Print(' '))?
                                    .queue(MoveLeft(1))?;
                                stdout.flush()?;
                            } else if insertion_point < buffer.get_buffer_length()
                                && !buffer.is_empty()
                            {
                                buffer.remove_char(insertion_point - 1);
                                buffer.dec_insertion_point();
                                stdout
                                    .queue(MoveLeft(1))?
                                    .queue(Print(buffer.slice_buffer(insertion_point - 1)))?
                                    .queue(Print(' '))?
                                    .queue(MoveToColumn(
                                        insertion_point as u16 + prompt_offset - 1,
                                    ))?;
                                stdout.flush()?;
                            }
                        }
                        KeyCode::Delete => {
                            let insertion_point = buffer.get_insertion_point();
                            if insertion_point < buffer.get_buffer_length() && !buffer.is_empty() {
                                buffer.remove_char(insertion_point);
                                stdout
                                    .queue(Print(buffer.slice_buffer(insertion_point)))?
                                    .queue(Print(' '))?
                                    .queue(MoveToColumn(insertion_point as u16 + prompt_offset))?;
                                stdout.flush()?;
                            }
                        }
                        KeyCode::Enter => {
                            if buffer.get_buffer() == "exit" {
                                break 'repl;
                            } else {
                                print_message(
                                    &mut stdout,
                                    &format!("Our buffer: {}", buffer.get_buffer()),
                                )?;
                                buffer.clear();
                                buffer.set_insertion_point(0);
                                break 'input;
                            }
                        }
                        KeyCode::Left => {
<<<<<<< HEAD
                            if buffer.get_insertion_point() > 0 {
                                if modifiers == KeyModifiers::ALT {
                                    let new_insertion_point = buffer.move_word_left();
                                    stdout.queue(MoveToColumn(
                                        new_insertion_point as u16 + prompt_offset,
                                    ))?;
                                } else {
                                    stdout.queue(MoveLeft(1))?;
                                    buffer.dec_insertion_point();
=======
                            if caret_position > input_start_col {
                                if modifiers == KeyModifiers::ALT {
                                    let whitespace_index = buffer
                                        .rmatch_indices(&[' ', '\t'][..])
                                        .find(|(index, _)| {
                                            index
                                                < &(caret_position as usize
                                                    - input_start_col as usize
                                                    - 1)
                                        });
                                    match whitespace_index {
                                        Some((index, _)) => {
                                            stdout.queue(MoveToColumn(
                                                index as u16 + input_start_col + 1,
                                            ))?;
                                            caret_position = input_start_col + index as u16 + 1;
                                        }
                                        None => {
                                            stdout.queue(MoveToColumn(input_start_col))?;
                                            caret_position = input_start_col;
                                        }
                                    }
                                } else {
                                    stdout.queue(MoveLeft(1))?;
                                    caret_position -= 1;
>>>>>>> master
                                }
                                stdout.flush()?;
                            }
                        }
                        KeyCode::Right => {
<<<<<<< HEAD
                            if buffer.get_insertion_point() < buffer.get_buffer_length() {
                                if modifiers == KeyModifiers::ALT {
                                    let new_insertion_point = buffer.move_word_right();
                                    stdout.queue(MoveToColumn(
                                        new_insertion_point as u16 + prompt_offset,
                                    ))?;
                                } else {
                                    stdout.queue(MoveRight(1))?;
                                    buffer.inc_insertion_point();
=======
                            if (caret_position as usize) < (input_start_col as usize) + buffer.len()
                            {
                                if modifiers == KeyModifiers::ALT {
                                    let whitespace_index = buffer
                                        .rmatch_indices(&[' ', '\t'][..])
                                        .find(|(index, _)| {
                                            index
                                                > &(caret_position as usize
                                                    - input_start_col as usize)
                                        });
                                    match whitespace_index {
                                        Some((index, _)) => {
                                            stdout.queue(MoveToColumn(
                                                index as u16 + input_start_col + 1,
                                            ))?;
                                            caret_position = input_start_col + index as u16 + 1;
                                        }
                                        None => {
                                            stdout.queue(MoveToColumn(
                                                buffer.len() as u16 + input_start_col,
                                            ))?;
                                            caret_position = buffer.len() as u16 + input_start_col;
                                        }
                                    }
                                } else {
                                    stdout.queue(MoveRight(1))?;
                                    caret_position += 1;
>>>>>>> master
                                }
                                stdout.flush()?;
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
        .queue(Print("\n"))?
        .queue(MoveToColumn(1))?
        .queue(Print(msg))?
        .queue(Print("\n"))?
        .queue(MoveToColumn(1))?;
    stdout.flush()?;
    Ok(())
}
