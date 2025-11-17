mod lang;
mod challenges;
mod storage;
mod runner;
mod ui;

use challenges::get_all_challenges;
use storage::{Solution, Storage};
use ui::{editor::{Editor, EditorResult}, menu::{Menu, MenuAction}, repl::Repl, reference::Reference};
use std::io;

fn main() -> io::Result<()> {
    // Setup terminal
    ui::setup_terminal()?;

    let result = run_app();

    // Restore terminal
    ui::restore_terminal()?;

    // Handle any errors
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn run_app() -> io::Result<()> {
    let challenges = get_all_challenges();
    let mut menu = Menu::new(challenges)?;

    loop {
        match menu.run()? {
            MenuAction::SelectChallenge(challenge) => {
                // Load any existing best solution
                let storage = menu.get_storage();
                let best = storage.get_personal_best(challenge.id)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                let mut editor = Editor::new(challenge.clone());

                if let Some(pb) = best {
                    editor.load_code(pb.code);
                }

                match editor.run()? {
                    EditorResult::Exit => break,
                    EditorResult::Back => {
                        // Save solution if all tests passed
                        if editor.all_tests_passed() {
                            let code = editor.get_code();
                            let char_count = editor.get_char_count();
                            let beat_par = char_count <= challenge.par_score;

                            let solution = Solution {
                                challenge_id: challenge.id,
                                code,
                                char_count,
                                passed: true,
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs() as i64,
                            };

                            storage.save_solution(&solution)
                                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                            storage.update_beat_par(challenge.id, beat_par)
                                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                        }
                    }
                }
            }
            MenuAction::OpenRepl => {
                let mut repl = Repl::new();
                repl.run()?;
            }
            MenuAction::OpenReference => {
                let mut reference = Reference::new();
                reference.run()?;
            }
            MenuAction::ShowLeaderboard => {
                show_leaderboard(menu.get_storage())?;
            }
            MenuAction::Exit => break,
        }
    }

    Ok(())
}

fn show_leaderboard(storage: &Storage) -> io::Result<()> {
    use crossterm::{
        cursor,
        queue,
        style::{Color, Print, ResetColor, SetForegroundColor},
        terminal::{self, Clear, ClearType},
    };
    use std::io::Write;

    let mut stdout = io::stdout();
    let (width, height) = terminal::size()?;

    queue!(stdout, Clear(ClearType::All))?;

    let mut current_line = 0u16;

    // Title
    queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
    queue!(stdout, SetForegroundColor(Color::Cyan), Print("═══ Your Leaderboard ═══"), ResetColor)?;
    current_line += 2;

    let bests = storage.get_all_personal_bests()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let total_score = storage.get_total_score()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Stats
    queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
    queue!(stdout, SetForegroundColor(Color::Yellow), Print(format!(" Total Score: {} points", total_score)), ResetColor)?;
    current_line += 1;

    queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
    queue!(stdout, SetForegroundColor(Color::Yellow), Print(format!(" Challenges Completed: {}", bests.len())), ResetColor)?;
    current_line += 1;

    queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
    queue!(stdout, SetForegroundColor(Color::Yellow), Print(format!(" Beat Par: {}", bests.iter().filter(|b| b.beat_par).count())), ResetColor)?;
    current_line += 2;

    // Header
    queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
    queue!(stdout, SetForegroundColor(Color::White), Print(format!(" {:<4} {:<12} {:<8}", "ID", "Chars", "Status")), ResetColor)?;
    current_line += 1;

    queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
    queue!(stdout, SetForegroundColor(Color::DarkGrey), Print(" ────────────────────────────────"), ResetColor)?;
    current_line += 1;

    // List
    for best in &bests {
        if current_line >= height - 3 {
            break; // Leave room for footer
        }

        let status = if best.beat_par { "✓ Beat Par" } else { "  Solved" };
        let color = if best.beat_par { Color::Green } else { Color::Yellow };

        queue!(stdout, cursor::MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
        queue!(
            stdout,
            SetForegroundColor(Color::White),
            Print(format!(" {:>3}  ", best.challenge_id)),
            Print(format!("{:>4} chars  ", best.char_count)),
            SetForegroundColor(color),
            Print(status),
            ResetColor
        )?;
        current_line += 1;
    }

    // Footer
    queue!(stdout, cursor::MoveTo(0, height - 2), Clear(ClearType::CurrentLine))?;
    queue!(stdout, SetForegroundColor(Color::DarkGrey), Print(" Press any key to return to menu..."), ResetColor)?;

    stdout.flush()?;

    ui::read_key()?;

    Ok(())
}
