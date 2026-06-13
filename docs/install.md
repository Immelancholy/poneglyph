# Installing poneglyph

## From source

```bash
git clone https://github.com/ShamanicArts/poneglyph.git
cd poneglyph
cargo install --path .
```

Then run:

```bash
poneglyph README.md
```

## From GitHub Releases

Download the archive for your platform from the Releases page, extract it, and put the `poneglyph` binary somewhere on your `PATH`.

Suggested install location:

```bash
mkdir -p ~/.local/bin
tar -xzf poneglyph-*-linux-x86_64.tar.gz
mv poneglyph ~/.local/bin/
```

## From crates.io

Once published:

```bash
cargo install poneglyph
```

## Configuration

User config:

```text
~/.config/poneglyph/config.toml
```

Project config:

```text
.poneglyph.toml
```

Example:

```toml
[ui]
theme = "tokyo-night"
cursorStyle = "block"
boxedChrome = true
themeSwatches = "square"
themeSwatchSpacing = 0
```
