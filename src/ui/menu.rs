use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

use crate::challenges::Challenge;
use crate::storage::{PersonalBest, Storage};

pub struct Menu {
    challenges: Vec<Challenge>,
    selected: usize,
    scroll_offset: usize,
    storage: Storage,
}

impl Menu {
    pub fn new(challenges: Vec<Challenge>) -> io::Result<Self> {
        let storage = Storage::new().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Database error: {}", e))
        })?;

        Ok(Menu {
            challenges,
            selected: 0,
            scroll_offset: 0,
            storage,
        })
    }

    pub fn run(&mut self) -> io::Result<MenuAction> {
        loop {
            self.render()?;

            let key = super::read_key()?;

            if super::is_ctrl_c(&key) {
                return Ok(MenuAction::Exit);
            }

            match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.move_up(),
                KeyCode::Down | KeyCode::Char('j') => self.move_down(),
                KeyCode::Enter => {
                    let challenge = self.challenges[self.selected].clone();
                    return Ok(MenuAction::SelectChallenge(challenge));
                }
                KeyCode::Char('r') => return Ok(MenuAction::OpenRepl),
                KeyCode::Char('h') => return Ok(MenuAction::OpenReference),
                KeyCode::Char('l') => return Ok(MenuAction::ShowLeaderboard),
                KeyCode::Char('q') | KeyCode::Esc => return Ok(MenuAction::Exit),
                _ => {}
            }
        }
    }

    fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            if self.selected < self.scroll_offset {
                self.scroll_offset = self.selected;
            }
        }
    }

    fn move_down(&mut self) {
        if self.selected < self.challenges.len() - 1 {
            self.selected += 1;
            let (_, height) = terminal::size().unwrap_or((80, 24));
            let visible_items = (height as usize).saturating_sub(10);
            if self.selected >= self.scroll_offset + visible_items {
                self.scroll_offset = self.selected - visible_items + 1;
            }
        }
    }

    fn render(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();
        let (width, height) = terminal::size()?;

        queue!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        // Title
        self.render_title(&mut stdout)?;

        // Stats
        self.render_stats(&mut stdout)?;

        // Challenge list
        let list_start_y = 6;
        let list_height = height.saturating_sub(10);
        self.render_challenge_list(&mut stdout, list_height, list_start_y)?;

        // Help bar
        queue!(stdout, cursor::MoveTo(0, height - 2))?;
        queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("─".repeat(width as usize)),
            ResetColor,
            Print("\n")
        )?;

        queue!(
            stdout,
            SetForegroundColor(Color::White),
            Print(" ↑/↓: Navigate | Enter: Select | R: REPL | H: Help | L: Leaderboard | Q: Quit"),
            ResetColor
        )?;

        stdout.flush()?;
        Ok(())
    }

    fn render_title(&self, stdout: &mut impl Write) -> io::Result<()> {
        queue!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print("╔════════════════════════════════════════════════════════════╗\n"),
            Print("║          CODE GOLF - Functional Language Edition          ║\n"),
            Print("╚════════════════════════════════════════════════════════════╝\n"),
            ResetColor,
            Print("\n")
        )?;
        Ok(())
    }

    fn render_stats(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        let total_score = self.storage.get_total_score().unwrap_or(0);
        let bests = self.storage.get_all_personal_bests().unwrap_or_default();
        let completed = bests.len();
        let beat_par = bests.iter().filter(|b| b.beat_par).count();

        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!(" Total Score: {} | Completed: {}/{} | Beat Par: {}\n\n",
                total_score, completed, self.challenges.len(), beat_par)),
            ResetColor
        )?;

        Ok(())
    }

    fn render_challenge_list(&mut self, stdout: &mut impl Write, height: u16, start_y: u16) -> io::Result<()> {
        let visible_items = height as usize;
        let end = (self.scroll_offset + visible_items).min(self.challenges.len());

        for (line_num, i) in (self.scroll_offset..end).enumerate() {
            let challenge = &self.challenges[i];
            let is_selected = i == self.selected;

            let best = self.storage.get_personal_best(challenge.id).unwrap_or(None);

            // Move to the correct line and clear it
            queue!(
                stdout,
                cursor::MoveTo(0, start_y + line_num as u16),
                Clear(ClearType::CurrentLine)
            )?;

            // Selection marker
            if is_selected {
                queue!(stdout, SetForegroundColor(Color::Green), Print(" > "))?;
            } else {
                queue!(stdout, Print("   "))?;
            }

            // Challenge number and name (shortened to fit better)
            let mut name_color = Color::White;
            if challenge.is_tutorial {
                name_color = Color::Cyan;
            }

            let name = if challenge.name.len() > 25 {
                format!("{:.22}...", challenge.name)
            } else {
                format!("{:<25}", challenge.name)
            };

            queue!(
                stdout,
                SetForegroundColor(name_color),
                Print(format!("{:2}. {}", challenge.id, name)),
                ResetColor
            )?;

            // Difficulty stars
            let stars = "★".repeat(challenge.difficulty) + &"☆".repeat(5 - challenge.difficulty);
            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print(format!(" {} ", stars)),
                ResetColor
            )?;

            // Par score
            queue!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("Par:{:3} ", challenge.par_score)),
                ResetColor
            )?;

            // Personal best
            if let Some(ref pb) = best {
                let color = if pb.beat_par {
                    Color::Green
                } else if pb.char_count <= challenge.par_score + 10 {
                    Color::Yellow
                } else {
                    Color::Red
                };

                queue!(
                    stdout,
                    SetForegroundColor(color),
                    Print(format!("Best:{:3}", pb.char_count)),
                    ResetColor
                )?;

                if pb.beat_par {
                    queue!(stdout, SetForegroundColor(Color::Green), Print(" ✓"), ResetColor)?;
                }
            } else {
                queue!(
                    stdout,
                    SetForegroundColor(Color::DarkGrey),
                    Print("Best:---"),
                    ResetColor
                )?;
            }
        }

        Ok(())
    }

    pub fn get_storage(&self) -> &Storage {
        &self.storage
    }
}

pub enum MenuAction {
    SelectChallenge(Challenge),
    OpenRepl,
    OpenReference,
    ShowLeaderboard,
    Exit,
}
