# VastAI CLI Tool

A command-line interface for interacting with the VastAI API, built with Rust.

## Features

- 🔑 **Automatic API Key Management** - Stores API key in `.vast_key` file
- 🖥️ **Machine Management** - List and manage your VastAI machines
- 🔧 **Maintenance Mode** - Set maintenance mode with custom schedules
- 📊 **Formatted Output** - Clean, tabular display of machine information
- 💾 **Machine Cache** - Saves machine list for quick selection
- ⏰ **Smart DateTime Input** - User-friendly date/time format with automatic timezone
- 🎨 **Colored Output** - Beautiful terminal UI with syntax highlighting
- 🔍 **Verbose Mode** - Optional detailed request information with `-i` flag
- ✅ **Interactive Prompts** - User-friendly interface for all operations

## Installation

### Build from source:

```bash
cargo build --release
```

The binary will be available at `target/release/VastAI`

### Optional: Add to PATH

```bash
# Copy to a directory in your PATH
cp target/release/VastAI /usr/local/bin/vastai
```

## Usage

### Global Flags

- `-i, --info` - Show detailed request information (verbose mode)

### List Machines

Display all your VastAI machines with formatted output:

```bash
vastai list

# With verbose output
vastai list -i
```

**Output includes:**
- Machine ID
- Hostname
- GPU Temperature (°C)
- Reliability (%)
- GPU Occupancy (rented/total)
- Hourly Earnings ($/h)
- Driver Version

**Example:**
```
=== VastAI List Machines ===
========================================================================================================================
Machine ID   Hostname             GPU Temp     Reliability  GPU Occupancy   Earning      Driver Ver
========================================================================================================================
40459        vastai2-desktop      76.0°C       99.19%       4/8             $1.3433/h    580.95.05
43976        vastai3              79.0°C       95.73%       8/8             $2.5391/h    580.95.05
========================================================================================================================

Success: Saved 2 machines to .machines
```

### Maintenance Mode

Set maintenance mode for a machine with custom schedule:

```bash
# Interactive selection from saved machines
vastai maint

# Specify machine ID
vastai maint -d 12345

# With verbose output
vastai maint -d 12345 -i
```

**The tool will prompt for:**
- Machine ID (select from saved machines or enter custom)
- Start date/time (format: `YYYY-MM-DD HH:MM`, uses local timezone)
- Duration in hours
- Maintenance reason
- Maintenance category (software/hardware/network/other)

**Date/Time Format:**
- Input: `2025-12-24 14:30`
- Automatically converts to ISO 8601 with your timezone: `2025-12-24T14:30:00+01:00`

**Example:**
```
=== VastAI Maintenance Mode ===

Select a machine:
> 40459 - vastai2-desktop
  43976 - vastai3
  Enter custom machine ID

Start date (YYYY-MM-DD HH:MM): 2025-12-24 14:30
Duration (hours): 2
Maintenance reason: Routine hardware check
Maintenance category:
> software
  hardware
  network
  other
```

### Unlist Instance

Remove an instance from listing:

```bash
vastai unlist -d 12345
```

## Configuration

### API Key

On first run, the tool will prompt for your VastAI API key and save it to `.vast_key`.

**Get your API key:**
The tool displays a clickable link to: https://cloud.vast.ai/manage-keys/

**Manual setup:**
```bash
echo "your-api-key-here" > .vast_key
```

### Saved Machines

Machine IDs and hostnames are automatically saved to `.machines` after running `vastai list`.
This enables quick machine selection in the `maint` command.

**Files (auto-managed, gitignored):**
- `.vast_key` - Your API key
- `.machines` - Cached machine list

## Project Architecture

Clean, modular architecture following Rust best practices:

```
src/
├── main.rs          # Application entry point (30 lines)
├── cli.rs           # Command-line argument parsing
├── config.rs        # API key management
├── models.rs        # Data structures (Machine, MaintRequest, etc.)
├── handlers.rs      # Business logic for each command
├── ui.rs            # User interface and interactive prompts
└── api.rs           # HTTP client and API communication
```

### Module Responsibilities

- **main.rs**: Application entry point, command routing, API key initialization
- **cli.rs**: CLI structure with clap (commands, arguments, flags)
- **config.rs**: API key file operations (read, write, prompt)
- **models.rs**: Data structures with serde serialization
- **handlers.rs**: Business logic for maint, list, unlist commands
- **ui.rs**: Interactive prompts, formatted output, machine display
- **api.rs**: VastAI API client with HTTP requests

### Design Patterns

- **Separation of Concerns** - Each module has a single responsibility
- **Dependency Injection** - Handlers receive data from UI prompts
- **Repository Pattern** - Config handles file persistence
- **Client Pattern** - VastAiClient encapsulates API communication
- **Error Handling** - Result types with user-friendly error messages

## Dependencies

```toml
clap = "4.5"          # CLI argument parsing with derive macros
reqwest = "0.12"      # HTTP client (blocking + JSON)
serde = "1.0"         # Serialization framework
serde_json = "1.0"    # JSON support
dialoguer = "0.11"    # Interactive terminal prompts
colored = "2.1"       # Terminal colors
chrono = "0.4"        # Date and time handling
```

## API Endpoints

- `GET /api/v0/machines` - List all machines
- `PUT /api/v0/machines/{id}/dnotify` - Set maintenance mode
- `POST /api/v0/instances/{id}/unlist` - Unlist instance

Base URL: `https://console.vast.ai/api/v0`

## Development

### Run in development mode:

```bash
cargo run -- list
cargo run -- maint -d 12345
cargo run -- list -i  # verbose mode
```

### Build for release:

```bash
cargo build --release
```

### Code Quality

- ✅ Zero compiler warnings
- ✅ Clean module separation
- ✅ Consistent error handling
- ✅ Type safety with Rust's type system
- ✅ Idiomatic Rust code

## Examples

### Quick machine check

```bash
# List all machines with current status
vastai list
```

### Schedule maintenance

```bash
# Interactive - select from saved machines
vastai maint

# Direct - specify machine ID
vastai maint -d 40459
```

### Debug API requests

```bash
# Show full request details
vastai list -i
vastai maint -d 40459 -i
```

## Features Timeline

### ✅ Implemented
- API key management with file storage
- List machines with formatted table output
- Maintenance mode scheduling
- Machine caching for quick selection
- DateTime input with timezone support
- GPU occupancy display (rented/total format)
- Reliability as percentage
- Verbose mode with `-i` flag
- Input validation with helpful error messages

### 🔮 Future Enhancements
- Machine filtering and sorting
- Export to CSV/JSON
- Batch operations
- Configuration file for preferences
- Async operations for better performance
- Unit and integration tests

## Troubleshooting

### API Key Issues

If you get "API key file not found or empty":
```bash
# Create the API key file
echo "your-vast-api-key" > .vast_key
```

### Invalid Date Format

Date format must be: `YYYY-MM-DD HH:MM`
- ✅ Correct: `2025-12-24 14:30`
- ❌ Wrong: `2025-13-24 14:30` (invalid month)
- ❌ Wrong: `2025-12-24 14-30` (use `:` for time)

### No Saved Machines

Run `vastai list` first to populate the `.machines` cache.

## Security

- API keys are stored in `.vast_key` (gitignored)
- Keys are masked in output (shows only last 4 characters)
- Machine cache in `.machines` (gitignored)
- No credentials in version control

## License

This project is private and proprietary.

## Support

For issues or questions, please refer to the VastAI API documentation:
- API Documentation: https://vast.ai/docs/api
- Manage API Keys: https://cloud.vast.ai/manage-keys/
