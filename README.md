# Dozo

Dozo(どうぞ) is a Japanese word meaning "please" or "here you go" used when offering something.

A unified CLI tool for managing coding agent configurations across Cursor, Claude, and Devin.

## What it does

Dozo centralizes prompt and rule management for different coding agents. Instead of maintaining separate configuration files for each tool, you write your coding standards once and sync them everywhere.

The key innovation is **cross-tool knowledge integration** - pull knowledge from any tool and make it available to all others.


## Installation

```bash
cargo install --git https://github.com/funwarioisii/dozo
```


## Quick Start

Here's a typical workflow showing what Dozo can do:

```bash
# 1. Pull relevant knowledge from Devin API  
export DEVIN_API_KEY="your_api_key"
dozo pull --from devin
# → Creates .agentic-coding/devin/Project_Guidelines.md, etc.

# 2. Pull existing Cursor configuration
dozo pull --from cursor  
# → Creates .agentic-coding/cursor/existing_rules.md

# 3. Add your own manual rules (optional)
echo "# Security Rules\nAlways validate input" > .agentic-coding/security.md

# 4. Now push the COMBINED knowledge to Claude
dozo push --target claude
# → CLAUDE.md contains:
#   - All relevant Devin knowledge  
#   - All Cursor rules
#   - Your manual security.md
#   - Properly organized with section headers

# 5. Also make it available to Cursor in .mdc format
dozo push --target cursor
# → .cursor/rules/ contains all knowledge in .mdc format
```

## Usage

### Pull configuration from tools

```bash
# Pull from existing tool configurations
dozo pull --from cursor
dozo pull --from claude
dozo pull --from devin    # Pulls knowledge from Devin API (requires DEVIN_API_KEY)

# Merge with existing configuration
dozo pull --from cursor --merge
dozo pull --from claude --merge
dozo pull --from devin --merge
```

**Note**: The Devin pull feature requires the `DEVIN_API_KEY` environment variable to be set for API authentication.

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
- **Claude**: Combines **all** content (manual + pulled knowledge) into `CLAUDE.md` + copies `commands/` to `.claude/commands/`
- **Devin**: ⚠️ **Push not yet implemented** (only pull is currently supported)

### Cross-tool knowledge integration

One of Dozo's key features is **cross-tool knowledge integration**. When you pull knowledge from different tools, you can then push the combined knowledge to other tools:

```bash
# Pull knowledge from multiple sources
dozo pull --from devin    # API knowledge → .agentic-coding/devin/
dozo pull --from cursor   # Existing rules → .agentic-coding/cursor/

# Push combined knowledge to Claude (includes all sources)
dozo push --target claude # All knowledge combined → CLAUDE.md
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
│   ├── general/              # Manual rules
│   │   ├── coding-style.md
│   │   └── security.md
│   ├── frontend/             # Manual rules
│   │   ├── react-rules.md
│   │   └── styling.md
│   ├── devin/               # From: dozo pull --from devin
│   │   ├── Knowledge_Item_1.md
│   │   └── Knowledge_Item_2.md
│   ├── cursor/              # From: dozo pull --from cursor  
│   │   └── existing-rules.md
│   └── commands/            # Claude commands (optional)
│       ├── deploy.md
│       └── test.md
└── src/                     # Your project files
```

### Output Structure

```
my-project/
├── .cursor/rules/            # dozo push --target cursor
│   ├── general/
│   │   ├── coding-style.mdc  # From manual rules
│   │   └── security.mdc
│   ├── frontend/
│   │   ├── react-rules.mdc   # From manual rules
│   │   └── styling.mdc
│   ├── devin/               # From pulled Devin knowledge
│   │   ├── Knowledge_Item_1.mdc
│   │   └── Knowledge_Item_2.mdc
│   └── cursor/              # From pulled Cursor rules
│       └── existing-rules.mdc
├── .claude/commands/         # Claude commands (if commands/ exists)
│   ├── deploy.md
│   └── test.md
└── CLAUDE.md                 # dozo push --target claude
                              # Contains ALL: manual + devin + cursor knowledge
```

**Key Feature**: `CLAUDE.md` includes content from:
- `general/coding-style.md` (manual)
- `frontend/react-rules.md` (manual)  
- `devin/Knowledge_Item_1.md` (from Devin API)
- `cursor/existing-rules.md` (from Cursor pull)
- Everything except `commands/` directory

## How it works

### Hierarchical Organization

1. **Set up once**: Create a `.agentic-coding/` directory with your configuration and markdown files
2. **Organize by domain**: Use subdirectories to organize rules by topic (general/, frontend/, backend/, etc.)
3. **Special directories**: 
   - `commands/` → Copied to `.claude/commands/` for Claude-specific commands
   - Everything else → Combined for main configuration files

### Tool-Specific Generation

- **Cursor**: Preserves your directory structure in `.cursor/rules/` and converts `.md` files to `.mdc` format
- **Claude**: Combines **all** markdown files (including those from `devin/`, `cursor/` subdirectories) into `CLAUDE.md` + copies `commands/` separately
- **Devin**: 
  - **Push**: ⚠️ Not yet implemented
  - **Pull**: Fetches knowledge from Devin API, filters by project relevance, saves as individual `.md` files in `.agentic-coding/devin/`

### Cross-Tool Integration

The key innovation is that when you push to any tool, **all knowledge sources are included**:
- Your manually created `.md` files
- Knowledge pulled from Devin (in `devin/` directory)
- Rules pulled from Cursor (in `cursor/` directory)
- Any other tool-specific knowledge

This means you can pull knowledge from Devin and immediately make it available to Claude and Cursor.

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