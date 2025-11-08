# krnchr

A lightning-fast, lossless file aggregator written in Rust. Combines multiple files into a single output file with configurable formats.

## Features

- **Lightning Fast**: Parallel file reading with Rayon
- **100% Lossless**: Binary-safe with base64 encoding for structured formats
- **Multiple Formats**: JSON, Text, XML, YAML, CSV
- **Simple CLI**: Easy to use command-line interface
- **Recursive**: Automatically discovers all files in directories

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/krnch`

## Usage

```bash
krnch <path> <format>
```

### Arguments

- `path`: Directory or file path to process (relative or absolute)
- `format`: Output format (`json`, `text`, `xml`, `yaml`, `csv`)

### Examples

```bash
# Crunch all files in current directory to JSON
krnch ./ json

# Crunch specific directory to text format
krnch ./src text

# Crunch single file to XML
krnch ./README.md xml

# Crunch with relative path to YAML
krnch ../data yaml

# Crunch to CSV
krnch ./logs csv
```

## Output Formats

### JSON
Structured JSON with base64-encoded content, file metadata, and totals.

### Text
Plain text format with file separators and raw content.

### XML
XML format with base64-encoded content and file attributes.

### YAML
YAML format with file metadata and base64-encoded content.

### CSV
CSV format with columns: path, size, content_base64.

## Output Files

Output files are created in the current directory:
- `crunched.json`
- `crunched.txt`
- `crunched.xml`
- `crunched.yaml`
- `crunched.csv`

## Architecture

Modular design with focused modules:

- `main.rs`: Entry point and orchestration
- `cli.rs`: Command-line argument parsing
- `types.rs`: Shared data structures and enums
- `reader.rs`: Parallel file discovery and reading
- `formatter.rs`: Output format implementations
- `writer.rs`: File output handling

## Performance

- Parallel file reading using Rayon for multi-core performance
- Zero-copy operations where possible
- Efficient base64 encoding implementation
- Minimal allocations

## License

See [LICENSE](LICENSE) for full license text.