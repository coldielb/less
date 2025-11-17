use crossterm::{
    cursor,
    event::KeyCode,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

pub struct Reference {
    scroll_offset: usize,
    content: Vec<String>,
}

impl Reference {
    pub fn new() -> Self {
        let content = Self::generate_content();
        Reference {
            scroll_offset: 0,
            content,
        }
    }

    fn generate_content() -> Vec<String> {
        vec![
            "╔════════════════════════════════════════════════════════════╗".to_string(),
            "║              Language Reference - Quick Guide              ║".to_string(),
            "╚════════════════════════════════════════════════════════════╝".to_string(),
            "".to_string(),
            "BASIC SYNTAX".to_string(),
            "".to_string(),
            "  Numbers:      42, -17, 0".to_string(),
            "  Booleans:     true, false".to_string(),
            "  Strings:      \"hello world\"".to_string(),
            "  Lists:        [1, 2, 3], [], [1..10]".to_string(),
            "  Ranges:       1..5  produces [1, 2, 3, 4, 5]".to_string(),
            "".to_string(),
            "FUNCTIONS".to_string(),
            "".to_string(),
            "  Lambda:       \\x -> x * 2".to_string(),
            "  Multi-arg:    \\x y -> x + y".to_string(),
            "  Let binding:  let double = \\x -> x * 2 in double 5".to_string(),
            "  Application:  map (\\x -> x * 2) [1, 2, 3]".to_string(),
            "".to_string(),
            "OPERATORS".to_string(),
            "".to_string(),
            "  Arithmetic:   + - * / % ^".to_string(),
            "  Comparison:   == != < > <= >=".to_string(),
            "  Logical:      && ||".to_string(),
            "  List ops:     :: (cons), ++ (concat)".to_string(),
            "  Composition:  >> (forward), << (backward)".to_string(),
            "".to_string(),
            "PATTERN MATCHING".to_string(),
            "".to_string(),
            "  match list with".to_string(),
            "    [] -> 0".to_string(),
            "    h::t -> h + sum t".to_string(),
            "".to_string(),
            "  Patterns: _, variable, number, [1,2,3], h::tail".to_string(),
            "".to_string(),
            "LIST COMPREHENSIONS".to_string(),
            "".to_string(),
            "  [x * 2 | x <- [1..10]]".to_string(),
            "  [x * 2 | x <- list, x > 5]".to_string(),
            "  [x + y | x <- [1,2], y <- [10,20]]  (nested)".to_string(),
            "".to_string(),
            "BUILT-IN FUNCTIONS".to_string(),
            "".to_string(),
            "  map f list          - Apply f to each element".to_string(),
            "  filter f list       - Keep elements where f returns true".to_string(),
            "  fold f init list    - Left fold with accumulator".to_string(),
            "  foldr f init list   - Right fold".to_string(),
            "  zip list1 list2     - Combine into pairs".to_string(),
            "  take n list         - First n elements".to_string(),
            "  drop n list         - Skip first n elements".to_string(),
            "  reverse list        - Reverse order".to_string(),
            "  sort list           - Sort numbers ascending".to_string(),
            "  length list         - Count elements".to_string(),
            "  head list           - First element".to_string(),
            "  tail list           - All but first".to_string(),
            "  sum list            - Sum of numbers".to_string(),
            "  product list        - Product of numbers".to_string(),
            "  concat lists        - Flatten one level".to_string(),
            "  elem item list      - Check if item in list".to_string(),
            "".to_string(),
            "EXAMPLES".to_string(),
            "".to_string(),
            "  Sum a list:".to_string(),
            "    fold (\\acc x -> acc + x) 0".to_string(),
            "    sum  (builtin shortcut)".to_string(),
            "".to_string(),
            "  Double all elements:".to_string(),
            "    map (\\x -> x * 2)".to_string(),
            "    \\list -> [x * 2 | x <- list]".to_string(),
            "".to_string(),
            "  Filter evens:".to_string(),
            "    filter (\\x -> x % 2 == 0)".to_string(),
            "".to_string(),
            "  Quicksort:".to_string(),
            "    let qsort = \\list -> match list with".to_string(),
            "      [] -> []".to_string(),
            "      p::rest ->".to_string(),
            "        let smaller = filter (\\x -> x < p) rest in".to_string(),
            "        let larger = filter (\\x -> x >= p) rest in".to_string(),
            "        qsort smaller ++ [p] ++ qsort larger".to_string(),
            "    in qsort".to_string(),
            "".to_string(),
            "  Fibonacci:".to_string(),
            "    let fib = \\n -> match n with".to_string(),
            "      0 -> 0".to_string(),
            "      1 -> 1".to_string(),
            "      _ -> fib (n - 1) + fib (n - 2)".to_string(),
            "    in fib".to_string(),
            "".to_string(),
            "PARTIAL APPLICATION".to_string(),
            "".to_string(),
            "  All functions support partial application:".to_string(),
            "    add3 = (\\x y z -> x + y + z) 1 2    -- returns (\\z -> 1 + 2 + z)".to_string(),
            "    map (+ 3) [1,2,3]   -- Note: (+) is not an operator section".to_string(),
            "".to_string(),
            "TIPS FOR CODE GOLF".to_string(),
            "".to_string(),
            "  1. Use partial application: map (* 2) instead of map (\\x -> x * 2)".to_string(),
            "  2. List comprehensions are often shorter than map/filter".to_string(),
            "  3. Pattern matching can replace if-then-else".to_string(),
            "  4. Fold can implement most list operations".to_string(),
            "  5. Use >> for function composition: x >> f >> g".to_string(),
            "  6. Built-ins like sum, product save characters".to_string(),
            "".to_string(),
            "Press any key to return to menu...".to_string(),
        ]
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.render()?;

            let key = super::read_key()?;

            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.scroll_offset > 0 {
                        self.scroll_offset -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let (_, height) = terminal::size()?;
                    let visible_lines = height.saturating_sub(2) as usize;
                    if self.scroll_offset + visible_lines < self.content.len() {
                        self.scroll_offset += 1;
                    }
                }
                KeyCode::PageUp => {
                    self.scroll_offset = self.scroll_offset.saturating_sub(10);
                }
                KeyCode::PageDown => {
                    let (_, height) = terminal::size()?;
                    let visible_lines = height.saturating_sub(2) as usize;
                    self.scroll_offset = (self.scroll_offset + 10)
                        .min(self.content.len().saturating_sub(visible_lines));
                }
                KeyCode::Home => {
                    self.scroll_offset = 0;
                }
                KeyCode::End => {
                    let (_, height) = terminal::size()?;
                    let visible_lines = height.saturating_sub(2) as usize;
                    self.scroll_offset = self.content.len().saturating_sub(visible_lines);
                }
                _ => break,
            }
        }

        Ok(())
    }

    fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        let (width, height) = terminal::size()?;

        queue!(stdout, Clear(ClearType::All))?;

        let visible_lines = height.saturating_sub(2) as usize;
        let end = (self.scroll_offset + visible_lines).min(self.content.len());

        for (line_offset, i) in (self.scroll_offset..end).enumerate() {
            queue!(stdout, cursor::MoveTo(0, line_offset as u16), Clear(ClearType::CurrentLine))?;

            let line = &self.content[i];

            // Truncate if too long
            let display_line = if line.len() > width as usize {
                &line[..width as usize]
            } else {
                line
            };

            if line.starts_with("╔") || line.starts_with("║") || line.starts_with("╚") {
                queue!(stdout, SetForegroundColor(Color::Cyan), Print(display_line), ResetColor)?;
            } else if line.chars().all(|c| c.is_uppercase() || c.is_whitespace()) && !line.is_empty() {
                queue!(stdout, SetForegroundColor(Color::Yellow), Print(display_line), ResetColor)?;
            } else if line.starts_with("  ") && line.contains("->") {
                queue!(stdout, SetForegroundColor(Color::Green), Print(display_line), ResetColor)?;
            } else {
                queue!(stdout, Print(display_line))?;
            }
        }

        // Scroll indicator
        queue!(stdout, cursor::MoveTo(0, height - 1), Clear(ClearType::CurrentLine))?;
        if self.content.len() > visible_lines {
            let scroll_percent = (self.scroll_offset * 100) / (self.content.len() - visible_lines);
            queue!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print(format!(" Scroll: {}% (↑/↓, PgUp/PgDn)", scroll_percent)),
                ResetColor
            )?;
        } else {
            queue!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print(" Press any key to return"),
                ResetColor
            )?;
        }

        stdout.flush()?;
        Ok(())
    }
}
