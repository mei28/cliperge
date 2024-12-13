# Cliperge

📋 Cliperge is a CLI tool to combine the contents of multiple files and copy them to the clipboard.

## Features

- Combine contents of multiple files
- Copy combined contents to the clipboard
- Support for full paths (starting from `~/`), relative paths, or file names
- Exclude specific files using patterns

## Installation

To install Cliperge, you'll need to have Rust and Cargo installed. Then you can build and install the tool from source:

```sh
cargo install cliperge
```

or 

```sh
git clone https://github.com/mei28/cliperge.git
cd cliperge
cargo install --path .
```

## Usage 

```sh
cliperge [OPTIONS] <file1> <file2> ...
```

### Options
- `-f`, `--full`: Use full file paths (starting from ~/ if applicable)
- `-r`, `--relative`: Use relative paths
- `-e`, `--exclude <pattern>`: Exclude files matching the specified pattern
- Default behavior: Use file names only

### Examples

#### Combine files using only file names:
```sh
cliperge file1.txt file2.txt
cliperge file{1..3}.txt 
cliperge src/**/*.rs 
```
#### Combine files with relative paths:

```sh
cliperge -r file1.txt file2.txt
```
#### Combine files with full paths:
```sh
cliperge -f file1.txt file2.txt
```
#### Exclude specific files:
```sh
cliperge src/*.rs -f -e log file1.txt file2.txt logs/error.log
```

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](https://github.com/mei28/cliperge/blob/main/LICENSE) file for details.
