use crossterm::{
    cursor,
    event::KeyCode,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};
use std::rc::Rc;

use crate::lang::{parser, interpreter, types};

pub struct Repl {
    history: Vec<String>,
    current_input: Vec<char>,
    cursor_pos: usize,
    scroll_offset: usize,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            history: Vec::new(),
            current_input: Vec::new(),
            cursor_pos: 0,
            scroll_offset: 0,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.print_welcome()?;

        loop {
            self.render()?;

            let key = super::read_key()?;

            if super::is_ctrl_c(&key) || (key.code == KeyCode::Char('q') && self.current_input.is_empty()) {
                break;
            }

            match key.code {
                KeyCode::Esc => break,
                KeyCode::Enter => {
                    if !self.current_input.is_empty() {
                        self.execute_input()?;
                    }
                }
                KeyCode::Char(c) => {
                    self.current_input.insert(self.cursor_pos, c);
                    self.cursor_pos += 1;
                }
                KeyCode::Backspace => {
                    if self.cursor_pos > 0 {
                        self.cursor_pos -= 1;
                        self.current_input.remove(self.cursor_pos);
                    }
                }
                KeyCode::Delete => {
                    if self.cursor_pos < self.current_input.len() {
                        self.current_input.remove(self.cursor_pos);
                    }
                }
                KeyCode::Left => {
                    if self.cursor_pos > 0 {
                        self.cursor_pos -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.cursor_pos < self.current_input.len() {
                        self.cursor_pos += 1;
                    }
                }
                KeyCode::Home => self.cursor_pos = 0,
                KeyCode::End => self.cursor_pos = self.current_input.len(),
                _ => {}
            }
        }

        Ok(())
    }

    fn print_welcome(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();
        queue!(stdout, Clear(ClearType::All))?;

        // Title
        queue!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::CurrentLine))?;
        queue!(stdout, SetForegroundColor(Color::Cyan), Print("═══ REPL - Interactive Mode ═══"), ResetColor)?;

        queue!(stdout, cursor::MoveTo(0, 1), Clear(ClearType::CurrentLine))?;

        queue!(stdout, cursor::MoveTo(0, 2), Clear(ClearType::CurrentLine))?;
        queue!(stdout, SetForegroundColor(Color::Yellow), Print("Type expressions to evaluate. Press Esc or Ctrl+C to exit."), ResetColor)?;

        queue!(stdout, cursor::MoveTo(0, 3), Clear(ClearType::CurrentLine))?;
        queue!(stdout, Print("Examples:"))?;

        queue!(stdout, cursor::MoveTo(0, 4), Clear(ClearType::CurrentLine))?;
        queue!(stdout, Print("  5 + 3"))?;

        queue!(stdout, cursor::MoveTo(0, 5), Clear(ClearType::CurrentLine))?;
        queue!(stdout, Print("  map (\\x -> x * 2) [1, 2, 3]"))?;

        queue!(stdout, cursor::MoveTo(0, 6), Clear(ClearType::CurrentLine))?;
        queue!(stdout, Print("  filter (\\x -> x > 5) [1..10]"))?;

        queue!(stdout, cursor::MoveTo(0, 7), Clear(ClearType::CurrentLine))?;

        stdout.flush()?;
        Ok(())
    }

    fn execute_input(&mut self) -> io::Result<()> {
        let input: String = self.current_input.iter().collect();

        // Add to history with the input
        self.history.push(format!("> {}", input));

        // Try to parse and evaluate
        match self.eval_expr(&input) {
            Ok(result) => {
                self.history.push(format!("  {}", result));
            }
            Err(e) => {
                self.history.push(format!("  Error: {}", e));
            }
        }

        // Clear input
        self.current_input.clear();
        self.cursor_pos = 0;

        // Scroll to bottom
        let (_, height) = terminal::size()?;
        let visible_lines = height.saturating_sub(12) as usize;
        if self.history.len() > visible_lines {
            self.scroll_offset = self.history.len() - visible_lines;
        }

        Ok(())
    }

    fn eval_expr(&self, input: &str) -> Result<String, String> {
        // Parse
        let expr = parser::parse(input)
            .map_err(|e| e.to_string())?;

        // Type check
        let mut type_checker = types::TypeChecker::new();
        let mut type_env = types::get_builtin_env();
        let ty = type_checker.infer(&expr, &mut type_env)
            .map_err(|e| e.to_string())?;

        // Evaluate
        let mut interp = interpreter::Interpreter::new();
        let env = Rc::new(interpreter::get_builtin_env());
        let value = interp.eval(&expr, &env)
            .map_err(|e| e.to_string())?;

        Ok(format!("{} : {}", value.to_string_repr(), ty))
    }

    fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        let (width, height) = terminal::size()?;

        let history_start_y = 8;
        let history_height = height.saturating_sub(12);

        // Render history line by line
        let visible_lines = history_height as usize;
        let end = self.history.len();
        let start = if end > visible_lines {
            end - visible_lines
        } else {
            0
        };

        for (line_offset, i) in (start..end).enumerate() {
            let y = history_start_y + line_offset as u16;
            queue!(stdout, cursor::MoveTo(0, y), Clear(ClearType::CurrentLine))?;

            let line = &self.history[i];
            // Truncate if too long
            let display_line = if line.len() > width as usize {
                &line[..width as usize]
            } else {
                line
            };

            if line.starts_with(">") {
                queue!(stdout, SetForegroundColor(Color::Green), Print(display_line), ResetColor)?;
            } else if line.contains("Error") {
                queue!(stdout, SetForegroundColor(Color::Red), Print(display_line), ResetColor)?;
            } else {
                queue!(stdout, SetForegroundColor(Color::White), Print(display_line), ResetColor)?;
            }
        }

        // Clear any remaining lines in history area
        for line_offset in (end - start)..visible_lines {
            let y = history_start_y + line_offset as u16;
            queue!(stdout, cursor::MoveTo(0, y), Clear(ClearType::CurrentLine))?;
        }

        // Input prompt
        let prompt_y = height.saturating_sub(3);
        queue!(stdout, cursor::MoveTo(0, prompt_y), Clear(ClearType::CurrentLine))?;
        queue!(stdout, SetForegroundColor(Color::DarkGrey))?;
        for _ in 0..width {
            queue!(stdout, Print("─"))?;
        }
        queue!(stdout, ResetColor)?;

        // Input line
        queue!(stdout, cursor::MoveTo(0, prompt_y + 1), Clear(ClearType::CurrentLine))?;
        queue!(stdout, SetForegroundColor(Color::Cyan), Print(">> "), ResetColor)?;

        let input_str: String = self.current_input.iter().collect();
        // Truncate input if too long
        if input_str.len() > width as usize - 3 {
            queue!(stdout, Print(&input_str[..width as usize - 3]))?;
        } else {
            queue!(stdout, Print(&input_str))?;
        }

        // Position cursor
        let cursor_x = (3 + self.cursor_pos).min(width as usize - 1) as u16;
        queue!(stdout, cursor::MoveTo(cursor_x, prompt_y + 1))?;

        stdout.flush()?;
        Ok(())
    }
}
