# Dozo

Dozo(どうぞ) is a Japanese word meaning "please" or "here you go" used when offering something.

A unified CLI tool for managing coding agent configurations across Cursor, Claude, and Devin.

## What it does

Dozo centralizes prompt and rule management for different coding agents. Instead of maintaining separate configuration files for each tool, you write your coding standards once and sync them everywhere.

Create a `.agentic-coding/` directory in your project with your configuration files, then use Dozo to sync them to your tools. **Dozo supports hierarchical directory structures** - organize your rules in subdirectories and Dozo will handle the rest.

## Usage

### Push configuration to tools

```bash
# Push to all tools
dozo push

# Push to specific tool
dozo push --target cursor
dozo push --target claude
dozo push --target devin

# Force overwrite existing files
dozo push --force
```

This generates:
- **Cursor**: Copies your hierarchy to `.cursor/rules/` (converts `.md` → `.mdc`)
- **Claude**: Combines content into `CLAUDE.md` + copies `commands/` to `.claude/commands/`
- **Devin**: Syncs with Devin via API (when implemented)

### Pull configuration from tools

```bash
# Pull from existing tool configurations
dozo pull --from cursor
dozo pull --from claude

# Merge with existing configuration
dozo pull --from cursor --merge
```

### Check synchronization status

```bash
dozo status
```

Shows the current state of all tool configurations and when they were last updated.

## Installation

### From source

```bash
git clone https://github.com/your-org/dozo
cd dozo/dozo-rs
cargo build --release
cargo install --path .
```

### Using cargo

```bash
cargo install dozo
```

## Configuration

Create a `.agentic-coding/` directory in your project root and add your markdown files organized by topic or domain.

## Options

- `--verbose, -v` - Enable verbose output
- `--config, -c <DIR>` - Set configuration directory (default: `.agentic-coding`)

### Push command options

- `--target <TOOL>` - Target tool: `cursor`, `claude`, `devin`, or `all` (default: `all`)
- `--force, -f` - Force overwrite existing files

### Pull command options

- `--from <TOOL>` - Source tool: `cursor`, `claude`, or `devin`
- `--merge` - Merge with existing configuration instead of replacing

## File Structure

### Input Structure (Hierarchical)

```
my-project/
├── .agentic-coding/           # Configuration directory
│   ├── config.yaml           # Main configuration
│   ├── general/              # General rules
│   │   ├── coding-style.md
│   │   └── security.md
│   ├── frontend/             # Frontend-specific rules
│   │   ├── react-rules.md
│   │   └── styling.md
│   └── commands/             # Claude commands (optional)
│       ├── deploy.md
│       └── test.md
└── src/                      # Your project files
```

### Output Structure

```
my-project/
├── .cursor/rules/            # Cursor hierarchy (mirrors input)
│   ├── general/
│   │   ├── coding-style.mdc  # Converted to .mdc
│   │   └── security.mdc
│   └── frontend/
│       ├── react-rules.mdc
│       └── styling.mdc
├── .claude/commands/         # Claude commands (if commands/ exists)
│   ├── deploy.md
│   └── test.md
├── CLAUDE.md                 # Combined content for Claude
└── devin-knowledge.json      # Devin backup (placeholder)
```

## How it works

### Hierarchical Organization

1. **Set up once**: Create a `.agentic-coding/` directory with your configuration and markdown files
2. **Organize by domain**: Use subdirectories to organize rules by topic (general/, frontend/, backend/, etc.)
3. **Special directories**: 
   - `commands/` → Copied to `.claude/commands/` for Claude-specific commands
   - Everything else → Combined for main configuration files

### Tool-Specific Generation

- **Cursor**: Preserves your directory structure in `.cursor/rules/` and converts `.md` files to `.mdc` format
- **Claude**: Combines all non-command files into `CLAUDE.md` + copies `commands/` separately
- **Devin**: Creates JSON backup file (API integration planned)

### Benefits

- **Hierarchical**: Organize rules by domain, feature, or any structure that makes sense
- **Consistent**: Same rules everywhere, formatted appropriately for each tool
- **Flexible**: Each tool gets the format it expects while you maintain a single source

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## License

MIT