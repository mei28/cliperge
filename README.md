# Cliperge

📋Cliperge is a CLI tool to combine the contents of multiple files and copy them to the clipboard.

## Features

- Combine contents of multiple files
- Copy combined contents to the clipboard
- Support for full paths, relative paths, or file names

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
cliperge [-f | -r] <file1> <file2> ...
```


```sh
-f: Use full file paths
-r: Use relative paths
Default: Use file names
``` 

## Examples

```sh
cliperge -f file1.txt file2.txt
cliperge -r file1.txt file2.txt
cliperge file1.txt file2.txt
```

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](https://github.com/mei28/cliperge/blob/main/LICENSE) file for details.
