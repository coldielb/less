use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

use crate::challenges::Challenge;
use crate::runner::{Runner, TestResult};

pub struct Editor {
    code: Vec<char>,
    cursor_pos: usize,
    scroll_offset: usize,
    challenge: Challenge,
    runner: Runner,
    last_results: Option<Vec<TestResult>>,
}

impl Editor {
    pub fn new(challenge: Challenge) -> Self {
        Editor {
            code: Vec::new(),
            cursor_pos: 0,
            scroll_offset: 0,
            challenge,
            runner: Runner::new(),
            last_results: None,
        }
    }

    pub fn load_code(&mut self, code: String) {
        self.code = code.chars().collect();
        self.cursor_pos = self.code.len();
    }

    pub fn run(&mut self) -> io::Result<EditorResult> {
        loop {
            self.render()?;

            let key = super::read_key()?;

            if super::is_ctrl_c(&key) {
                return Ok(EditorResult::Exit);
            }

            if super::is_ctrl_r(&key) {
                self.execute_code();
                continue;
            }

            match key.code {
                KeyCode::Esc => return Ok(EditorResult::Back),
                KeyCode::Char(c) => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        continue;
                    }
                    self.insert_char(c);
                }
                KeyCode::Backspace => self.backspace(),
                KeyCode::Delete => self.delete(),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),
                KeyCode::Home => self.cursor_pos = 0,
                KeyCode::End => self.cursor_pos = self.code.len(),
                KeyCode::Enter => self.insert_char('\n'),
                KeyCode::Tab => {
                    self.insert_char(' ');
                    self.insert_char(' ');
                }
                _ => {}
            }
        }
    }

    fn insert_char(&mut self, c: char) {
        self.code.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.code.remove(self.cursor_pos);
        }
    }

    fn delete(&mut self) {
        if self.cursor_pos < self.code.len() {
            self.code.remove(self.cursor_pos);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.code.len() {
            self.cursor_pos += 1;
        }
    }

    fn execute_code(&mut self) {
        let code_str: String = self.code.iter().collect();
        let results = self.runner.run_tests(&code_str, &self.challenge.test_cases);
        self.last_results = Some(results);
    }

    pub fn get_code(&self) -> String {
        self.code.iter().collect()
    }

    pub fn get_char_count(&self) -> usize {
        self.runner.count_chars(&self.get_code())
    }

    pub fn all_tests_passed(&self) -> bool {
        self.last_results.as_ref()
            .map(|results| results.iter().all(|r| r.passed))
            .unwrap_or(false)
    }

    fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        let (width, height) = terminal::size()?;

        queue!(stdout, Clear(ClearType::All))?;

        // Header section (lines 0-2)
        self.render_header(&mut stdout, width, 0)?;

        // Code editor section (starting at line 3)
        let editor_start = 3;
        let editor_height = if self.last_results.is_some() {
            (height.saturating_sub(10)) / 2
        } else {
            height.saturating_sub(5)
        };

        self.render_code_editor(&mut stdout, width, editor_height, editor_start)?;

        // Test results section (if available)
        if let Some(ref results) = self.last_results {
            let results_start = editor_start + editor_height;
            self.render_test_results(&mut stdout, width, results, results_start)?;
        }

        // Status bar (bottom)
        self.render_status_bar(&mut stdout, width, height)?;

        // Position cursor in editor
        let (cursor_x, cursor_y) = self.calculate_cursor_position();
        queue!(stdout, cursor::MoveTo(cursor_x, cursor_y + editor_start))?;

        stdout.flush()?;
        Ok(())
    }

    fn render_header(&self, stdout: &mut impl Write, width: u16, start_y: u16) -> io::Result<()> {
        // Line 0: Challenge name
        queue!(stdout, cursor::MoveTo(0, start_y), Clear(ClearType::CurrentLine))?;
        queue!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(format!("Challenge {}: {}", self.challenge.id, self.challenge.name)),
            ResetColor
        )?;

        // Line 1: Description (truncate if too long)
        queue!(stdout, cursor::MoveTo(0, start_y + 1), Clear(ClearType::CurrentLine))?;
        let desc = if self.challenge.description.len() > width as usize - 2 {
            format!("{}...", &self.challenge.description[..width as usize - 5])
        } else {
            self.challenge.description.clone()
        };
        queue!(
            stdout,
            SetForegroundColor(Color::White),
            Print(desc),
            ResetColor
        )?;

        // Line 2: Type signature
        queue!(stdout, cursor::MoveTo(0, start_y + 2), Clear(ClearType::CurrentLine))?;
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("Type: {}", self.challenge.type_signature)),
            ResetColor
        )?;

        Ok(())
    }

    fn render_code_editor(&self, stdout: &mut impl Write, width: u16, height: u16, start_y: u16) -> io::Result<()> {
        let code_str: String = self.code.iter().collect();
        let lines: Vec<&str> = code_str.split('\n').collect();

        for i in 0..height as usize {
            queue!(stdout, cursor::MoveTo(0, start_y + i as u16), Clear(ClearType::CurrentLine))?;

            let line_idx = i + self.scroll_offset;
            if line_idx < lines.len() {
                let line = lines[line_idx];
                // Truncate if too long
                if line.len() > width as usize {
                    let truncated = &line[..width as usize];
                    self.render_line_with_highlight(stdout, truncated)?;
                } else {
                    self.render_line_with_highlight(stdout, line)?;
                }
            }
        }

        Ok(())
    }

    fn render_line_with_highlight(&self, stdout: &mut impl Write, line: &str) -> io::Result<()> {
        let keywords = ["let", "in", "match", "with", "if", "then", "else", "true", "false"];

        let mut i = 0;
        let chars: Vec<char> = line.chars().collect();

        while i < chars.len() {
            let ch = chars[i];

            // Check for keywords
            if ch.is_alphabetic() {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();

                if keywords.contains(&word.as_str()) {
                    queue!(stdout, SetForegroundColor(Color::Magenta), Print(&word), ResetColor)?;
                } else if word.chars().all(|c| c.is_lowercase() || c == '_') && i < chars.len() {
                    // Likely a function name
                    queue!(stdout, SetForegroundColor(Color::Green), Print(&word), ResetColor)?;
                } else {
                    queue!(stdout, Print(&word))?;
                }
            } else if ch.is_numeric() {
                // Number
                let start = i;
                while i < chars.len() && chars[i].is_numeric() {
                    i += 1;
                }
                let num: String = chars[start..i].iter().collect();
                queue!(stdout, SetForegroundColor(Color::Blue), Print(&num), ResetColor)?;
            } else if ch == '"' {
                // String
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1;
                }
                let string: String = chars[start..i].iter().collect();
                queue!(stdout, SetForegroundColor(Color::Yellow), Print(&string), ResetColor)?;
            } else if ch == '\\' {
                // Lambda
                queue!(stdout, SetForegroundColor(Color::Red), Print("\\"), ResetColor)?;
                i += 1;
            } else if ch == '-' && i + 1 < chars.len() && (chars[i + 1] == '>' || chars[i + 1] == '-') {
                // Arrow or comment
                if chars[i + 1] == '>' {
                    queue!(stdout, SetForegroundColor(Color::Red), Print("->"), ResetColor)?;
                    i += 2;
                } else {
                    // Comment
                    let comment: String = chars[i..].iter().collect();
                    queue!(stdout, SetForegroundColor(Color::DarkGrey), Print(&comment), ResetColor)?;
                    break;
                }
            } else {
                queue!(stdout, Print(ch))?;
                i += 1;
            }
        }

        Ok(())
    }

    fn render_test_results(&self, stdout: &mut impl Write, width: u16, results: &[TestResult], start_y: u16) -> io::Result<()> {
        let mut current_line = start_y;

        // Header
        queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
        queue!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print("Test Results:"),
            ResetColor
        )?;
        current_line += 1;

        for result in results.iter() {
            let status = if result.passed { "✓ PASS" } else { "✗ FAIL" };
            let color = if result.passed { Color::Green } else { Color::Red };

            queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
            queue!(
                stdout,
                SetForegroundColor(color),
                Print(format!("  {} ", status)),
                ResetColor,
                Print(&result.description)
            )?;
            current_line += 1;

            if !result.passed {
                if let Some(ref error) = result.error {
                    queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
                    let err_msg = if error.len() > width as usize - 14 {
                        format!("{}...", &error[..width as usize - 17])
                    } else {
                        error.clone()
                    };
                    queue!(
                        stdout,
                        SetForegroundColor(Color::Red),
                        Print(format!("      Error: {}", err_msg)),
                        ResetColor
                    )?;
                    current_line += 1;
                } else {
                    queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
                    queue!(stdout, Print(format!("      Expected: {}", result.expected)))?;
                    current_line += 1;

                    queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
                    queue!(stdout, Print(format!("      Got:      {}", result.actual)))?;
                    current_line += 1;
                }
            }
        }

        Ok(())
    }

    fn render_status_bar(&self, stdout: &mut impl Write, width: u16, height: u16) -> io::Result<()> {
        queue!(stdout, cursor::MoveTo(0, height - 1), Clear(ClearType::CurrentLine))?;

        let char_count = self.get_char_count();
        let par = self.challenge.par_score;
        let delta = char_count as i32 - par as i32;

        let delta_color = if delta <= 0 {
            Color::Green
        } else if delta <= 10 {
            Color::Yellow
        } else {
            Color::Red
        };

        let status_text = format!(
            " Chars: {} | Par: {} | Δ: {:+} | Ctrl+R: Run | Esc: Back | Ctrl+C: Exit",
            char_count, par, delta
        );

        queue!(
            stdout,
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White)
        )?;

        // Print status, truncate if needed
        if status_text.len() > width as usize {
            queue!(stdout, Print(&status_text[..width as usize]))?;
        } else {
            queue!(stdout, Print(&status_text))?;
            // Fill rest of line
            for _ in status_text.len()..width as usize {
                queue!(stdout, Print(" "))?;
            }
        }

        queue!(stdout, ResetColor)?;

        Ok(())
    }

    fn calculate_cursor_position(&self) -> (u16, u16) {
        let code_before_cursor: String = self.code[..self.cursor_pos].iter().collect();
        let lines: Vec<&str> = code_before_cursor.split('\n').collect();
        let y = (lines.len() - 1) as u16;
        let x = lines.last().unwrap_or(&"").len() as u16;
        (x, y)
    }
}

pub enum EditorResult {
    Exit,
    Back,
}
