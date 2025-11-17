# DEPRECATED Installation Instructions

## Quick Install (Recommended)

```bash
# Install Rust if you don't have it
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and install
cargo install --path . --locked

# Run the game
less
```

## Alternative: Run without installing

```bash
cargo run --release
```

## For macOS Users

If you downloaded a pre-built binary and get a security warning:

```bash
# Remove the quarantine flag
xattr -d com.apple.quarantine ./less

# Or right-click the binary, select "Open", then click "Open" in the dialog
```

## System Requirements

- macOS, Linux, or Windows
- Terminal with color support
- Rust 1.70+ (for building from source)
