# Roo - Unofficial Goo Format Parser for Elegoo 3D Printers

Roo is a Rust library that enables you to parse `.goo` files, the proprietary format used by Elegoo 3D printers. This library is not officially associated with Elegoo and is a community-driven project.

Roo is inspired by the UVTools project, and is essentially a rewrite of the `GooFile.cs` class found in that project. The purpose of this library is to make it easier for Rust developers to work with `.goo` files.

## Installation

To use Roo in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
roo = "0.1.0"
```

Then run `cargo build` to download and compile Roo.

## Usage

Here is a basic example of how you can use Roo to load and count the layers in a `.goo` file:

```rust
use std::fs::File;
use roo::Roo;

fn main() {
    // Specify the path to your .goo file
    let path = "/path/to/your/file.goo";
    
    // Open the file
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };

    // Create a Roo object from the file
    let mut roo = Roo::from_file(file).expect("Failed to parse the .goo file");

    // Iterate over the layers in the .goo file
    let mut layer_count = 0;
    while let Some(layer) = roo.next() {
        println!("Layer {}: {:?}", layer_count, layer);
        layer_count += 1;
    }

    println!("Total layers: {}", layer_count);
}

```

In this example, we open a `.goo` file, create a `Roo` object from it, and then we iterate over each layer in the file, printing it out and incrementing a count. At the end, we assert that the count is 32, which is the expected number of layers in our test file.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request or open an Issue.

## License

MIT License

## Disclaimer

This library is not officially associated with Elegoo. It is a community project and is not endorsed by or affiliated with Elegoo in any way. Use this library at your own risk.