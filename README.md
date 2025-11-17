# Code Golf - Functional Language Edition

A terminal-based code golf game featuring a custom purely functional programming language with Haskell-inspired syntax. Challenge yourself to write the shortest solutions to programming problems!

## Features

### Custom Functional Language
- **Haskell-inspired syntax** with enhanced readability
- **Purely functional** - immutable data, no statements
- **Type inference** - automatic type checking
- **Lazy evaluation** with tail-call optimization
- **Pattern matching** on lists and values
- **List comprehensions** for concise list operations
- **Function composition** using `>>` (forward) and `<<` (backward)
- **Partial application** and currying built-in

### Language Highlights

```haskell
-- Lambda functions
\x -> x * 2

-- Pattern matching
let factorial = \n -> match n with
  0 -> 1
  _ -> n * factorial (n - 1)
in factorial

-- List comprehensions
[x * 2 | x <- [1..10], x > 5]

-- Function composition
filter (\x -> x > 0) >> map (\x -> x * 2) >> sum
```

### Game Features
- **25 Challenges** - from easy tutorials to advanced problems
- **5 Tutorial Challenges** - learn the language step by step
- **Real-time feedback** - see test results instantly
- **Par scores** - beat the challenge to earn points
- **Syntax highlighting** - readable code editor
- **Persistent storage** - track your best solutions
- **REPL mode** - experiment with the language interactively
- **Built-in reference** - comprehensive language guide

## Installation

### Easy Install (Recommended)

```bash
# Run the installer
./install.sh
```

This will:
- Check for Rust (and offer to install it if missing)
- Build the game in release mode
- Install to `~/.local/bin/less`
- Make it available from anywhere in your terminal

After installation, just run:
```bash
less
```

### Manual Installation

```bash
# Build the release version
cargo build --release

# Copy to your local bin
cp target/release/less ~/.local/bin/

# Make sure ~/.local/bin is in your PATH
export PATH="$HOME/.local/bin:$PATH"
```

### Uninstall

```bash
./uninstall.sh
```

## How to Play

1. **Main Menu** - Select a challenge from the list
2. **Code Editor** - Write your solution
3. **Test (Ctrl+R)** - Run against test cases
4. **Submit** - Save your solution when all tests pass
5. **Beat Par** - Try to minimize your character count!

### Controls

#### Main Menu
- `↑/↓` or `j/k` - Navigate challenges
- `Enter` - Select challenge
- `R` - Open REPL
- `H` - View language reference
- `L` - Show leaderboard
- `Q` or `Esc` - Quit

#### Code Editor
- Type to write code
- `Ctrl+R` - Run tests
- `Esc` - Return to menu
- `Ctrl+C` - Exit game

#### REPL
- Type expressions and press Enter
- `Esc` or `Ctrl+C` - Exit REPL

## Language Reference

### Basic Syntax

```haskell
-- Numbers, Booleans, Strings
42, -17, 0
true, false
"hello world"

-- Lists and Ranges
[1, 2, 3]
[1..10]  -- produces [1,2,3,4,5,6,7,8,9,10]
```

### Functions

```haskell
-- Lambda syntax
\x -> x * 2
\x y -> x + y

-- Let bindings
let double = \x -> x * 2 in double 5

-- Function application
map (\x -> x * 2) [1, 2, 3]
```

### Operators

```haskell
-- Arithmetic
+ - * / % ^

-- Comparison
== != < > <= >=

-- Logical
&& ||

-- List operations
::  -- cons (prepend)
++  -- concatenation

-- Composition
>>  -- forward pipe: x >> f is f(x)
<<  -- backward pipe: f << x is f(x)
```

### Pattern Matching

```haskell
match list with
  [] -> 0
  h::t -> h + sum t

-- Patterns: _, variable, number, [1,2,3], h::tail
```

### List Comprehensions

```haskell
[x * 2 | x <- [1..10]]
[x * 2 | x <- list, x > 5]
[x + y | x <- [1,2], y <- [10,20]]
```

### Built-in Functions

```haskell
map f list          -- Apply f to each element
filter f list       -- Keep elements where f returns true
fold f init list    -- Left fold with accumulator
foldr f init list   -- Right fold
zip list1 list2     -- Combine into pairs
take n list         -- First n elements
drop n list         -- Skip first n elements
reverse list        -- Reverse order
sort list           -- Sort numbers ascending
length list         -- Count elements
head list           -- First element
tail list           -- All but first
sum list            -- Sum of numbers
product list        -- Product of numbers
concat lists        -- Flatten one level
elem item list      -- Check if item in list
```

## Example Solutions

### Sum a List (Challenge 6)

```haskell
-- Shortest (beat par):
sum

-- Using fold:
fold (\a x->a+x) 0

-- Explicit recursion:
\l -> match l with [] -> 0 | h::t -> h + sum t
```

### Filter Even Numbers (Challenge 8)

```haskell
-- Using filter:
filter(\x->x%2==0)

-- List comprehension (shorter):
\l->[x|x<-l,x%2==0]
```

### Quicksort (Challenge 12)

```haskell
let qsort = \l -> match l with
  [] -> []
  p::r ->
    qsort [x|x<-r,x<p] ++ [p] ++ qsort [x|x<-r,x>=p]
in qsort
```

## Tips for Code Golf

1. **Use partial application**: `map (* 2)` is shorter than `map (\x -> x * 2)` (if implemented)
2. **List comprehensions** are often shorter than `map`/`filter`
3. **Pattern matching** can replace `if-then-else`
4. **Built-in functions** like `sum`, `product` save many characters
5. **Function composition** with `>>` chains operations elegantly
6. **Avoid spaces** where not needed: `\x->x*2` vs `\x -> x * 2`

## Challenge Categories

### Tutorials (1-5)
- Learn basic syntax and operations
- Hints provided for each challenge

### Basic (6-9)
- Sum, reverse, filter operations
- Fibonacci and basic recursion

### Intermediate (10-17)
- Prime checking
- Quicksort implementation
- Implementing map/filter with fold
- Cartesian products

### Advanced (18-25)
- Pascal's triangle
- Run-length encoding
- Group consecutive duplicates
- Partial sums and complex list operations

## Data Storage

Solutions are stored in `~/.code_golf_game/solutions.db` (SQLite database)

## Technical Details

- **Language**: Rust
- **Parser**: Pest (PEG parser)
- **UI**: Crossterm (terminal manipulation)
- **Storage**: SQLite with rusqlite
- **Type System**: Hindley-Milner style type inference
- **Evaluation**: Lazy with tail-call optimization

## Architecture

```
src/
├── lang/           # Language implementation
│   ├── ast.rs      # Abstract syntax tree
│   ├── parser.rs   # Parser using Pest
│   ├── types.rs    # Type inference
│   └── interpreter.rs  # Lazy evaluator
├── challenges/     # Challenge definitions
├── storage/        # SQLite persistence
├── runner.rs       # Test runner with timeout
├── ui/             # Terminal interface
│   ├── editor.rs   # Code editor
│   ├── menu.rs     # Main menu
│   ├── repl.rs     # Interactive REPL
│   └── reference.rs # Help system
└── main.rs         # Application entry point
```

## License

This is a personal project created for educational and entertainment purposes.

## Contributing

This is a complete, self-contained project. Feel free to fork and modify for your own use!

## Credits

Built with Rust and inspired by functional programming languages like Haskell, ML, and Lisp.
