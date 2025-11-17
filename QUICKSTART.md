# Quick Start Guide

## Installation

```bash
./install.sh
```

That's it! The script will handle everything.

## First Time Usage

After installation, start the game:

```bash
less
```

## Navigating the Game

### Main Menu
- **Up/Down** or **j/k** - Move between challenges
- **Enter** - Select a challenge
- **R** - Open REPL (practice mode)
- **H** - View language reference
- **L** - View leaderboard
- **Q** - Quit

### Code Editor
- Type your solution
- **Ctrl+R** - Run tests
- **Esc** - Return to menu
- **Ctrl+C** - Exit game

## Your First Challenge

1. Start with "Double a Number" (Challenge 1)
2. It asks you to write a function that doubles a number
3. Try this solution:
   ```
   \x -> x * 2
   ```
4. Press **Ctrl+R** to test
5. If all tests pass, press **Esc** to save and return

## Learning the Language

Press **H** from the main menu to see the full language reference.

Quick syntax guide:
- Lambda: `\x -> x * 2`
- List: `[1, 2, 3]`
- Range: `1..10`
- List comprehension: `[x * 2 | x <- [1..10], x > 5]`
- Pattern matching: `match list with [] -> 0 | h::t -> h`

## Tips

1. **Start with tutorials** (Challenges 1-5) - They teach the language
2. **Use the REPL** - Press R to experiment with expressions
3. **Beat par** - Try to minimize your character count
4. **Read the reference** - Press H to learn all language features

## Scoring

- Each challenge has a "par score" (target character count)
- Beat par to earn points
- Whitespace doesn't count toward your score
- Shorter code = higher score

## Getting Help

- Press **H** in the main menu for the language reference
- Use the **REPL** (press R) to test expressions
- Tutorial challenges have hints after failed attempts

## Uninstalling

```bash
./uninstall.sh
```

Enjoy the challenge!
