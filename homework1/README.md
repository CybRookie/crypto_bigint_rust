# Homework 1

Table of contents:  

- [Homework 1](#homework-1)
  - [Solution nuances](#solution-nuances)
  - [How to run](#how-to-run)
    - [Accepted arguments](#accepted-arguments)
    - [Example of how to run](#example-of-how-to-run)
    - [Docker](#docker)
  - [Logic behind the solution](#logic-behind-the-solution)
    - [Caesar](#caesar)
    - [Vigenere](#vigenere)
    - [Encoding](#encoding)
    - [Tests](#tests)
  - [CI/CD](#cicd)

Task description:  

```txt
Implement Caesar and Vigenere encryption and decryption.
```

## Solution nuances

Solution was implemented in the Rust programming language, as a command line tool, accepting user arguments alongside binary's name. 
Though, it was desired to follow the idea of "test-driven development", in reality, development went with the "flow". 
Then refactoring happened, tool's library was split into separate crates/modules. 
Unit-tests and integration tests were written, and some more refactoring happened. 
Some tests still managed to find a hole or two in the logic of the tool!  

> The binary/tool's name is `se`, standing for symmetric encryption.

## How to run

Rust compiler should be present for compilation of the Rust source code. 
The official instructions on installation of the Rust toolchain can be found here: [Install Rust](https://www.rust-lang.org/tools/install). 
More details can be found here [Other Rust Installation Methods](https://forge.rust-lang.org/infra/other-installation-methods.html) 
There is also an option to use Docker container to run Rust code: [Rust Docker container](https://hub.docker.com/_/rust/)

- To compile and run the solution from source, run command `cargo run <list of required args goes here>` inside `homework1` directory.
- To compile and run tests for the solution, run command `cargo test <list of required args goes here>` inside `homework1` directory.
- To only compile the solution, run command `cargo build` (for an unoptimized, debug version) or `cargo build --release` (for an optimized, release version) inside `homework1` directory.
- If you are using compiled binary, then you can call it with `se <list of required args goes here>` on Linux/MacOS/Unix systems and `se.exe <list of required args goes here>` on Windows.

> Compiled debug binaries under the name of `se(.exe)` should reside under the path of `homework1/target/debug`.  
> Optimized binaries compiled with `cargo build --release` should reside under the path of `homework1/target/release`.

### Accepted arguments

Usage pattern: `se(.exe) <cipher type> <encryption mode> <output mode> <plaintext or ciphertext> <key>`

Possible values for the listed arguments (total 5 arguments):  

- cipher mode: caesar/vigenere,
- encryption mode: encrypt/decrypt,
- output mode: console/file/both,
- plaintext or ciphertext: "your text/string/phrase to encrypt or decrypt"
- key: "your key to use for encryption decryption"

Notice:  

- This tool outputs encrypted information in a hexadecimal encoding format.
- This tool only accepts ciphertexts for decryption, previously encrypted with this tool in hex format.
- If the "file" or "both " output mode were chosen then the ciphertext will be saved in the file, named "ciphertext.txt".
- If the "file" or "both " output mode were chosen then the ciphertext will be saved in the same location of the tool.
- Caesar mode encryption and decryption accept only whole numbers as a key (both positive and negative).
- Vigenere mode encryption and decryption accept any string as a key.

> You can trigger output of a help message with a special argument `help` for information about the tool from the tool itself, like so: `cargo run help`  

### Example of how to run

Examples:

- To get help message: `cargo run help`.
- To encrypt provided strings with Vigenere cipher and output result to the console/standard output: `cargo run vigenere encrypt console YourTargetString YourVerySecretTextKey`
- To encrypt provided strings with Caesar cipher and output result to the file output: `cargo run caesar encrypt file YourTargetString 123`
- To decrypt provided strings with Caesar cipher and output result to the console/standard and a file output: `cargo run caesar decrypt both D4EAF0EDCFDCEDE2E0EFCEEFEDE4E9E2 123`

### Docker

Theoretically, you can spin up a container from the official Rust image, copy or link `homework1` directory to the running container
and run the above-mentioned commands. Another option is to compose the image from the provided `docker-compose.yml` file.

Run the following command inside the `homework1` directory, where the `docker-compose.yml` resides: `docker-compose run --rm  dev`

This Docker compose file links/passes through current `homework1` directory to the Docker container.
Inside the Docker container, that is built from the official Rust Docker image, you should be able to access `/homework1` directory and run `cargo run <args>` to compile and run the tool.

> Compose file may require modification, to tailor you Docker/docker-compose setup.
> The setup was tested with Docker version 20.10.7 and docker-compose 1.25.0 on Ubuntu 20.04.

## Logic behind the solution

Tool accepts 5 arguments and creates a custom configuration struct, which it will further pass to the function uniting the whole logic. 
The accepted arguments are tested on validity.  

### Caesar

> Note: all strings in Rust are encoded in UTF-8 by default.

Caesar encryption algorithm encrypts, or decrypts passed string of text byte by byte, by adding the numeric key to each byte, 
which before the operation goes through Euclidean modulus of 256 to get a positive numeric key that is not bigger than 255. 
This is done because of limitations of how much information can be held in one unsigned byte. 
To go around these limitations the byte under encryption or decryption is turned into a signed two-byte entity. 
During encryption, the key is added to the entity under processing. 
If the resulting numeric value equals 256 or bigger, 256 is subtracted from it; and it is turned back into an unsigned one-byte entity. 
But if the byte was going through the decryption process, 
then the key is subtracted from the target byte instead, if the result is a negative number, then 256 is added to the value.
It is turned back into an unsigned one-byte entity as well. 
Caesar algorithm accepts only whole numeric values, negative, positive and 0.

Snippet of code with Caesar function:  

```rust
// Function to encrypt or decrypt the target sting under Caesar cipher.
pub fn caesar(mode: &Mode, target: &mut str, key: &str) -> Result<String, Box<dyn Error>> {
    // Byte has only 256 variations, considering the algorithm used,
    // there is no need for key number bigger than 256;
    // the euclidean modulus is calculated to account for possible negative entries instead of
    // C-like remainder "%" operation.
    let key: i128 = key.parse()?;
    let key = key.rem_euclid(256);
    let key = key as u8;

    match mode {
        Mode::Encode => {
            // Convert string to the vector of unsigned one byte integers.
            let target = unsafe { target.as_bytes_mut() };

            // Encrypt vector of bytes one by one.
            for char in target.iter_mut() {
                caesar_encrypt_char(char, &key);
            }

            // Encode the vector of bytes into the hex string.
            string_hex_encode(target)
        }
        Mode::Decode => {
            // Convert received hex string into the vector of encrypted one bytes.
            let mut decoded_string = string_hex_decode(target)?;

            // Decrypt vector of bytes one by one.
            for char in decoded_string.iter_mut() {
                caesar_decrypt_char(char, &key);
            }

            let result = unsafe { from_utf8_unchecked(&decoded_string) };

            Ok(String::from(result))
        }
    }
}
```

Caesar algorithm related code can be found under the path of: `homework1/src/crypto/caesar.rs`.  

### Vigenere

Vigenere encryption algorithm encrypts or decrypts strings in the same manner as the Caesar encryption algorithm. 
This is the result of Vigenere function calling Caesar's function for byte-by-byte processing. 
The only difference is that a text string as a key is accepted for processing. 
The key is also used byte by byte, this is done by cycling through the vector of byte with remainder `%` operation.

Snippet of code with the Vigenere function:  

```rust
pub fn vigenere(mode: &Mode, target: &mut str, key: &str) -> Result<String, Box<dyn Error>> {
    // Turn key string into vector of bytes.
    let key = key.as_bytes();
    let key_len = key.len();

    match mode {
        Mode::Encode => {
            // Convert string to the vector of unsigned one byte integers.
            let target = unsafe { target.as_bytes_mut() };

            // Iterator over key.
            let mut i = 0;

            // Encrypt vector of bytes one by one.
            for char in target.iter_mut() {
                caesar_encrypt_char(char, &key[i]);
                i = (i + 1) % key_len;
            }

            // Encode the vector of bytes into the hex string.
            string_hex_encode(target)
        }
        Mode::Decode => {
            // Convert received hex string into the vector of encrypted one bytes.
            let mut decoded_string = string_hex_decode(target)?;

            // Iterator over key.
            let mut i = 0;

            // Decrypt vector of bytes one by one.
            for char in decoded_string.iter_mut() {
                caesar_decrypt_char(char, &key[i]);
                i = (i + 1) % key_len;
            }

            let result = unsafe { from_utf8_unchecked(&decoded_string) };

            Ok(String::from(result))
        }
    }
}
```

Vigenere algorithm related code can be found under the path of: `homework1/src/crypto/vigenere.rs`.

### Encoding

Encoding for the encryption results is utilised, because the produced string sequences may not be displayed by the terminals or text editors correctly. 
Information may be lost. The encryption results are encoded in hexadecimal format. 
Both Caesar and Vigenere algorithms can accept for decryption only hexadecimal strings, produced by the same algorithm.

Hexadecimal encoding/decoding related code can be found under the path of: `homework1/src/encoding/mod.rs`.

### Tests

Both unit-tests and integration tests were implemented for the project. 
Each library crate/file holds its own module with tests for the code residing in that crate/file. 
Integration tests mimicking `main` function or the main logic of the tool, can be found under the path of: `homework1/tests/integration_test.rs`.

To run the test, run the next command in the `homework1` directory: `cargo test`.  
If you want to see additional debugging output from the test functions run: `cargo test -- --nocapture`.

## CI/CD

Gitlab CI/CD was configured for learning purposes, to become more familiar with DevOps technologies. 
The present configuration `.gitlab-ci.yml` can test the `homework1` Rust code with Cargo. It can consistently build binary for Linux on amd64 architecture. 
Windows can be built as well, if the GitLab runners manage to connect to the Linux distribution's repository to set up required 
MinGW environment. Building for macOS (amd64 and ARM) is theoretically possible, practically proven on the local machine, but GitLab runners 
did not manage to install all the required tools over many attempts.  

The Docker environment is based of official Rust image and configured with `apt`, to install required software. 
This is very inefficient, a lot should be cached between separate jobs for better performance, but this is not possible. 
Ideally, a custom Docker image would be used with all the required tools for binary compilations. 
But this is, probably, a task for another day.

>For cross compilation from Linux to:  
>
> - Windows, `mingw-w64` package is required, Rust's `std` library can be installed with `rustup target add x86_64-pc-windows-gnu` command, 
>afterwards compilation can be done with `cargo build --verbose --release --target x86_64-pc-windows-gnu`
> - macOS, building of `clang` compiler for macOS is required, this can be achieved with [osxcross](https://github.com/tpoechtrager/osxcross) project. 
>The compiler is built using local Linux tools and then, the resulting compiler is utilised for building binaries for macOS 
with command `cargo build --verbose --release --target x86_64-apple-darwin`. `std` library can be acquired with `rustup target add x86_64-apple-darwin`
