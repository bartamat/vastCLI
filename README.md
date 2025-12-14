# VastAI CLI Tool

A command-line interface for interacting with the VastAI API.

## Features

- Interactive prompts for collecting request information
- Pretty-printed preview of requests before execution
- Confirmation prompt requiring "accept" or "cancel"
- Colored output for better readability
- Support for multiple VastAI operations

## Installation

Build the project:

```bash
cargo build --release
```

The binary will be available at `target/release/VastAI`

## Usage

### Maintenance Mode

Set maintenance mode for an instance:

```bash
# With interactive prompts
cargo run -- maint

# With instance ID provided
cargo run -- maint --id 12345
```

The tool will prompt you for:
- Instance ID (if not provided)
- Maintenance mode (Enable/Disable)
- Reason for maintenance

### List Instances

List all instances:

```bash
cargo run -- list
```

The tool will prompt you for:
- API Key

### Unlist Instance

Remove an instance from listing:

```bash
# With interactive prompts
cargo run -- unlist

# With instance ID provided
cargo run -- unlist --id 12345
```

The tool will prompt you for:
- Instance ID (if not provided)

## Workflow

1. Run the command with the desired operation
2. Provide required information through interactive prompts
3. Review the preview of the request
4. Type "accept" to proceed or "cancel" to abort
5. Provide your API key when prompted (for actual execution)

## Example Session

```
$ cargo run -- maint

=== VastAI Maintenance Mode ===

Instance ID: 12345
Maintenance mode:
  > Enable
    Disable
Reason [User requested]: Scheduled hardware upgrade

Maintenance Request:
  {
    "id": 12345,
    "maintenance": true,
    "reason": "Scheduled hardware upgrade"
  }

Type 'accept' to confirm or 'cancel' to abort: accept

Executing request...
VastAI API Key: ********

Request details:
  URL: https://console.vast.ai/api/v0
  Body: {"id":12345,"maintenance":true,"reason":"Scheduled hardware upgrade"}
```

## Project Architecture

The project follows clean code principles with a modular structure:

```
src/
├── main.rs          # Application entry point
├── cli.rs           # Command-line argument parsing
├── models.rs        # Data structures and domain models
├── handlers.rs      # Business logic for each command
├── ui.rs            # User interface and interactive prompts
└── api.rs           # HTTP client and API communication
```

### Module Responsibilities

- **main.rs**: Application entry point that orchestrates command routing
- **cli.rs**: Defines CLI structure using clap, including all commands and arguments
- **models.rs**: Contains data structures (MaintRequest, UnlistRequest) with constructors
- **handlers.rs**: Implements business logic for each command (maint, list, unlist)
- **ui.rs**: Handles all user interaction, prompts, and output formatting
- **api.rs**: Manages HTTP requests to the VastAI API with the VastAiClient

### Design Patterns

- **Separation of Concerns**: Each module has a single, well-defined responsibility
- **Dependency Injection**: Handlers receive data from UI prompts rather than collecting it themselves
- **Builder Pattern**: Models use constructors to ensure valid object creation
- **Client Pattern**: VastAiClient encapsulates all API communication logic

## Dependencies

- `clap` - Command-line argument parsing
- `reqwest` - HTTP client
- `serde` & `serde_json` - JSON serialization
- `dialoguer` - Interactive prompts
- `colored` - Terminal colors

## Notes

- API requests are sent to the VastAI API endpoints
- Update the BASE_URL in api.rs if needed
- API keys are masked in the preview for security
