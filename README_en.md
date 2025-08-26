# minigrep

A simple grep tool for searching text in files.

## Usage

### Basic Usage
```bash
cargo run -- <query> <file_path>
```

For example:
```bash
cargo run -- "the" src/poem.txt
```

### Case Sensitivity Control

#### Using Command Line Arguments
```bash
cargo run -- <query> <file_path> --ignore-case
```

Or using the short form:
```bash
cargo run -- <query> <file_path> -i
```

#### Using Environment Variables
```bash
IGNORE_CASE=1 cargo run -- <query> <file_path>
```

### Priority

Environment variables have higher priority. If both command-line arguments and environment variables are set, the environment variable setting will be used.

For example, the following command will perform a case-sensitive search (because environment variables have higher priority):
```bash
IGNORE_CASE=0 cargo run -- <query> <file_path> --ignore-case
```

Even if the `--ignore-case` flag is set, if the `IGNORE_CASE=0` environment variable is set, the program will perform a case-sensitive search.