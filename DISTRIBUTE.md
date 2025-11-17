# Distribution Guide

## For Distributors

### Creating a Release Package

1. **Build and test locally first:**
   ```bash
   cargo build --release
   cargo test
   ./target/release/less
   ```

2. **Create distribution package:**
   ```bash
   ./package.sh
   ```

   This creates: `dist/code-golf-v1.0.0.tar.gz` (approx 44KB)

3. **Share the tarball** via:
   - GitHub Releases
   - File sharing service
   - Email
   - Your own server

### What's Included in the Package

- Source code (needed for compilation)
- Installation scripts (install.sh, uninstall.sh)
- Documentation (README.md, QUICKSTART.md, INSTALL.md)
- Build configuration (Cargo.toml, Cargo.lock)

### Security Notes

**Why source code is included:**
- Rust requires source to build binaries
- Users can inspect code before building (transparency)
- No pre-compiled malware concerns
- Solutions are hidden once compiled

**About "looking at answers":**
- Yes, users CAN view challenge solutions in source
- But the game is about the challenge, not the answers
- Most users want to solve problems themselves
- Consider it an "honor system" game

**For truly hidden solutions:**
- You'd need code signing ($99/year Apple Developer)
- Or distribute server-based version (more complex)
- Current approach is standard for Rust CLI tools

## For Users

### Installation Instructions

1. **Extract the archive:**
   ```bash
   tar -xzf code-golf-v1.0.0.tar.gz
   cd code-golf-v1.0.0
   ```

2. **Run the installer:**
   ```bash
   ./install.sh
   ```

3. **Play the game:**
   ```bash
   less
   ```

### macOS Security Warning

If you get "cannot verify developer" warning:

**Option 1: Remove quarantine flag**
```bash
xattr -d com.apple.quarantine less
```

**Option 2: System Preferences**
1. Right-click the binary
2. Select "Open"
3. Click "Open" in the security dialog

**Option 3: Build from source** (most secure)
```bash
./install.sh
```
This builds locally, avoiding all security warnings.

### System Requirements

- **OS:** macOS, Linux, or Windows with WSL
- **Terminal:** Any modern terminal with color support
- **Rust:** 1.70+ (installer will offer to install if missing)
- **Disk Space:** ~50MB for build, ~5MB for installed binary

### Uninstalling

```bash
cd code-golf-v1.0.0
./uninstall.sh
```

Or manually:
```bash
rm ~/.local/bin/less
rm -rf ~/.code_golf_game  # optional: removes save data
```

## Troubleshooting

### "Rust not found"
The installer will offer to install Rust automatically.

Or install manually:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### "less: command not found" after install
Add `~/.local/bin` to your PATH:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

(Replace `.bashrc` with `.zshrc` if using zsh)

### Build fails
Check Rust version:
```bash
rustc --version  # Should be 1.70+
```

Update Rust:
```bash
rustup update
```

### Permission denied
Make scripts executable:
```bash
chmod +x install.sh uninstall.sh
```

## Version History

### v1.0.0 (Current)
- Initial release
- 25 challenges (5 tutorials + 20 regular)
- Custom functional language
- REPL mode
- Language reference
- SQLite persistence
- Full terminal UI

## Support

For issues, feature requests, or questions:
- Check QUICKSTART.md for basic help
- Review INSTALL.md for installation issues
- See README.md for full documentation

Enjoy the challenge!
