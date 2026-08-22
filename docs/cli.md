# Command-Line Interface

Executable name: `webflow-runtime` on Linux and Windows.

The CLI uses options rather than subcommands:

```text
webflow-runtime [OPTIONS]
```

## Options

| Option | Description |
| --- | --- |
| `-a, --app <APP_ID>` | Launch an application directly by its ID without opening the Manager UI. |
| `-d, --debug` | Enable WebView developer tools. In Manager and direct application modes, also enable timestamped, color-coded logging for UI events and JavaScript diagnostics. Manager mode additionally logs IPC activity and backend command timings. On Windows, attach to the parent console when available. |
| `--debug-verbose` | Include background polling and other high-volume debug events. This option affects Manager debug logging and is intended to be used together with `--debug`. |
| `--debug-file` | Write a detailed session log to `logs/` next to the executable. The file includes background polling and high-volume debug events. Does not enable terminal output by itself. |
| `--create-from-template <TEMPLATE_ID>` | Create an application from a template and exit. |
| `--list-templates` | List available built-in and filesystem templates and exit. |
| `--help` | Display CLI help and exit. `-h` is also accepted. |
| `--version` | Display the executable version and exit. `-V` is also accepted. |
| `--autostart` | Internal flag used by operating-system autostart entries. It marks a Manager launch as system-initiated and is not intended for regular manual use. |
| `--restart` | Internal flag used during Manager self-restart. It delays initialization briefly so the previous Manager process can release its instance lock. |

## Execution modes

### Manager mode

Running without a mode-selecting option starts WebFlow Runtime Manager:

```bash
./webflow-runtime
```

`--debug` opens WebView developer tools and enables diagnostic output in the launching terminal. The default Manager debug level suppresses recurring background polling. Use `--debug-verbose` to include it:

```bash
./webflow-runtime --debug
./webflow-runtime --debug --debug-verbose
```

Use `--debug-file` to write detailed diagnostics without terminal output. Combine it with `--debug` to write the detailed log and keep the normal color-coded terminal output:

```bash
./webflow-runtime --debug-file
./webflow-runtime --debug --debug-file
```

Manager mode creates `logs/session-YYYYMMDD-HHMMSS-PID.log`; direct application mode creates `logs/app-session-YYYYMMDD-HHMMSS-PID.log`. Files are placed next to the executable. File logging is enabled only when explicitly requested.

### Direct application mode

`--app` starts the specified application without loading the Manager UI:

```bash
./webflow-runtime --app <APP_ID>
```

In this mode, `--debug` enables WebView developer tools for the application. Manager UI and Manager IPC diagnostics are not active.

In this mode, `--debug` logs application UI events, JavaScript errors, unhandled promise rejections, and console messages to the terminal. `--debug-file` writes the same application diagnostics to an `app-session-...log` file, and combining both options enables both outputs.

### Template listing mode

`--list-templates` prints the available built-in templates and templates found in the runtime template directory, then exits:

```bash
./webflow-runtime --list-templates
```

### Template creation mode

`--create-from-template` creates an application using the specified template ID, prints the generated application ID and path, then exits:

```bash
./webflow-runtime --create-from-template claude
```

Built-in template IDs currently handled by the CLI are:

- `claude`
- `chatgpt`
- `deepseek`
- `youtube`

Unknown IDs use the generic application defaults.

## Option precedence

When mutually exclusive mode-selecting options are provided together, the runtime evaluates them in this order:

1. `--list-templates`
2. `--create-from-template`
3. `--app`
4. Manager mode

The process exits with a non-zero status when the selected operation fails.
