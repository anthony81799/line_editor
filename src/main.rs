use crossterm::{
    cursor::{position, MoveToColumn, MoveToNextLine, RestorePosition, SavePosition},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand, Result,
};
use line_buffer::LineBuffer;

use std::collections::VecDeque;
use std::io::{stdout, Stdout, Write};

mod line_buffer;

const HISTORY_SIZE: usize = 100;

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

fn buffer_repaint(stdout: &mut Stdout, buffer: &LineBuffer, prompt_offset: u16) -> Result<()> {
    let raw_buffer = buffer.get_buffer();
    let new_index = buffer.get_insertion_point();
    stdout
        .queue(MoveToColumn(prompt_offset))?
        .queue(Print(&raw_buffer[0..new_index]))?
        .queue(SavePosition)?
        .queue(Print(&raw_buffer[new_index..]))?
        .queue(Clear(ClearType::UntilNewLine))?
        .queue(RestorePosition)?;
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    let mut buffer = LineBuffer::new();
    let mut history = VecDeque::with_capacity(HISTORY_SIZE);
    let mut history_cursor = -1i64;
    let mut has_history = false;

    'repl: loop {
        stdout
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print(">"))?
            .execute(ResetColor)?;
        let (mut prompt_offset, ..) = position()?;
        prompt_offset += 1;

        'input: loop {
            match read()? {
                Event::Key(KeyEvent {
                    code,
                    modifiers: KeyModifiers::ALT,
                }) => match code {
                    KeyCode::Left => {
                        buffer.move_word_left();
                        buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                    }
                    KeyCode::Right => {
                        buffer.move_word_right();
                        buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                    }
                    _ => {}
                },
                Event::Key(KeyEvent {
                    code,
                    modifiers: KeyModifiers::CONTROL,
                }) => match code {
                    KeyCode::Char('a') => {
                        buffer.set_insertion_point(0);
                        buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                    }
                    KeyCode::Char('d') => {
                        stdout.queue(MoveToNextLine(1))?.queue(Print("exit"))?;
                        break 'repl;
                    }
                    KeyCode::Char('k') => {
                        buffer.clear_to_end();
                        buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                    }
                    _ => {}
                },
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Char(c) => {
                            buffer.insert_char(buffer.get_insertion_point(), c);
                            buffer.inc_insertion_point();
                            buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                        }
                        KeyCode::Backspace => {
                            if buffer.get_insertion_point() == buffer.get_buffer_length()
                                && !buffer.is_empty()
                            {
                                buffer.pop();
                                buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                            } else if buffer.get_insertion_point() < buffer.get_buffer_length()
                                && !buffer.is_empty()
                            {
                                buffer.dec_insertion_point();
                                buffer.remove_char(buffer.get_insertion_point());
                                buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                            }
                        }
                        KeyCode::Delete => {
                            if buffer.get_insertion_point() < buffer.get_buffer_length()
                                && !buffer.is_empty()
                            {
                                buffer.remove_char(buffer.get_insertion_point());
                                buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                            }
                        }
                        KeyCode::Home => {
                            buffer.set_insertion_point(0);
                            buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                        }
                        KeyCode::End => {
                            buffer.set_insertion_point(buffer.get_buffer_length());
                            buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                        }
                        KeyCode::Enter => {
                            if buffer.get_buffer() == "exit" {
                                break 'repl;
                            } else {
                                if history.len() + 1 == HISTORY_SIZE {
                                    history.pop_back();
                                }
                                history.push_front(String::from(buffer.get_buffer()));
                                has_history = true;
                                history_cursor = -1;
                                print_message(
                                    &mut stdout,
                                    &format!("Our buffer: {}", buffer.get_buffer()),
                                )?;
                                buffer.clear();
                                break 'input;
                            }
                        }
                        KeyCode::Up => {
                            if has_history && history_cursor < (history.len() as i64 - 1) {
                                history_cursor += 1;
                                let history_entry =
                                    history.get(history_cursor as usize).unwrap().clone();
                                buffer.set_buffer(history_entry.clone());
                                buffer.move_to_end();
                                buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                            }
                        }
                        KeyCode::Down => {
                            if history_cursor >= 0 {
                                history_cursor -= 1;
                            }
                            let new_buffer = if history_cursor < 0 {
                                String::new()
                            } else {
                                history.get(history_cursor as usize).unwrap().clone()
                            };
                            buffer.set_buffer(new_buffer.clone());
                            buffer.move_to_end();
                            buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                        }
                        KeyCode::Left => {
                            if buffer.get_insertion_point() > 0 {
                                buffer.dec_insertion_point();
                                buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
                            }
                        }
                        KeyCode::Right => {
                            if buffer.get_insertion_point() < buffer.get_buffer_length() {
                                buffer.inc_insertion_point();
                                buffer_repaint(&mut stdout, &buffer, prompt_offset)?;
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
