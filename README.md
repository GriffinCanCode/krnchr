# krnchr

Combine files into a single output file. Supports multiple output formats.

## Features

- Parallel file reading
- Binary-safe with base64 encoding
- Multiple output formats: JSON, Text, XML, YAML, CSV
- Recursive directory processing
- Simple command-line interface

## Installation

Install from [crates.io](https://crates.io/crates/krnchr):

```bash
cargo install krnchr
```

Or build from source:

```bash
git clone https://github.com/GriffinCanCode/krnchr
cd krnchr
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

Uses parallel file reading for multi-core performance.

## Links

- **GitHub:** https://github.com/GriffinCanCode/krnchr
- **crates.io:** https://crates.io/crates/krnchr
- **Releases:** https://github.com/GriffinCanCode/krnchr/releases

## License

See [LICENSE](LICENSE) for full license text.