# Homework 2

Table of contents:  

- [Homework 2](#homework-2)
  - [Solution nuances](#solution-nuances)
  - [How to run](#how-to-run)
    - [Accepted arguments](#accepted-arguments)
    - [Example of how to run](#example-of-how-to-run)
    - [Docker](#docker)
  - [Logic behind the solution](#logic-behind-the-solution)
    - [A custom BigInt](#a-custom-bigint)
    - [Diffie-Hellman](#diffie-hellman)
    - [RSA](#rsa)
    - [RSA private key bruteforce calculation](#rsa-private-key-bruteforce-calculation)
    - [Tests](#tests)
  - [CI/CD](#cicd)

Task description:  

```txt
Implement Diffie-Hellman algorithm, RSA encryption and decryption, and a brute force calculation of an RSA private key based on a public one.
```

## Solution nuances

Solution was implemented in the Rust programming language, as a command line tool, accepting user arguments alongside binary's name. 
For this homework a whole custom BigInt library was implemented for use in RSA cipher and Diffie-Hellman key-exchange algorithm. 
Moreover RSA was implemented as a block cipher, encrypting/decrypting 16 bytes (128 bit) blocks at a time. RSA bruteforcing 
was multi-threaded with the help of a custom thread-pool, containing workers listening for the new tasks from the main thread, 
and also being capable to send the signal themselves, if the goal was reached successfully or if a failure happened during execution. 
The thread-pool is also capable of a graceful shutdown of itself and its workers.

> The binary/tool's name is `enc`, standing for symmetric encryption.
> Caesar and Vigenere cipher are accessible as available as well.

## How to run

Rust compiler should be present for compilation of the Rust source code. 
The official instructions on installation of the Rust toolchain can be found here: [Install Rust](https://www.rust-lang.org/tools/install). 
More details can be found here [Other Rust Installation Methods](https://forge.rust-lang.org/infra/other-installation-methods.html). 
There is also an option to use Docker container to run Rust code: [Rust Docker container](https://hub.docker.com/_/rust/).  

- To compile and run the solution from source, run command `cargo run <list of required args goes here>` inside `homework2` directory.
- To compile and run tests for the solution, run command `cargo test <list of required args goes here>` inside `homework2` directory.
- To only compile the solution, run command `cargo build` (for an unoptimized, debug version) or `cargo build --release` (for an optimized, release version) inside `homework2` directory.
- If you are using compiled binary, then you can call it with `enc <list of required args goes here>` on Linux/MacOS/Unix systems and `enc.exe <list of required args goes here>` on Windows.

> Compiled debug binaries under the name of `enc(.exe)` should reside under the path of `homework2/target/debug`.  
> Optimized binaries compiled with `cargo build --release` should reside under the path of `homework2/target/release`.
> It is recommended to utilise the flag `--release` with `cargo test`, `cargo run`, `cargo build` commands to compile a more optimized solution and speed up costly mathematical operations.

### Accepted arguments

Usage pattern:  

- For symmetric encryption with Caesar or Vigenere: `enc(.exe) <cipher type> <encryption mode> <output mode> <plaintext or ciphertext> <key>`
- For Diffie-Hellman algorithm: `enc(.exe) <cipher type> generate <output mode> <none or shared prime> <none or shared base> <none or secret A> <none or secret B>`
- For RSA encryption/decryption: `enc(.exe) <cipher type> <encryption mode> <output mode> <plaintext or ciphertext> <public or private exponent> <public modulus>`
- For RSA key pair generation: `enc(.exe) <cipher type> generate <output mode>`
- For RSA public key bruteforcing: `enc(.exe) <cipher type> generate <output mode> <public or private exponent> <public modulus> <empty or a custom amount of threads>`

Possible values for the listed arguments (amount of required arguments varies on the requested operation):  

- cipher type: caesar/vigenere/rsa/df,
- encryption mode: encrypt/decrypt/generate/bruteforce,
- output mode: console/file/both,
- plaintext or ciphertext: "your text/string/phrase to encrypt or decrypt",
- key: "your key to use for encryption decryption",
- shared prime/shared base/secret A/secret B: "your appropriate numeric values for Diffie-Hellman algorithm or "none" to generate a random value",
- public or private exponent/public modulus: "your appropriate numeric values for RSA cipher",")?;
- amount of threads: "your custom of number of threads, values from 1 to 64 are allowed, or "none", or leave it empty to use a default value of 8",

Notice:  

- This tool outputs encrypted information in a hexadecimal encoding format.
- This tool only accepts ciphertexts for decryption, previously encrypted with this tool in hex format.
- If the "file" or "both " output mode were chosen then the ciphertext will be saved in the file, named "ciphertext.txt".
- If the "file" or "both " output mode were chosen then the ciphertext will be saved in the same location of the tool.
- Caesar mode encryption and decryption accept only whole numbers as a key (both positive and negative).
- Vigenere mode encryption and decryption accept any string as a key.
- If you use `cargo run` command to run the program, it is advised to run int with `--release` parameter to speed up calculations.

> You can trigger output of a help message with a special argument `help` for information about the tool from the tool itself, like so: `cargo run help`  

### Example of how to run

Examples:

- To get help message: `cargo run help`.
- To encrypt provided strings with Vigenere cipher and output result to the console/standard output: `cargo run vigenere encrypt console YourTargetString YourVerySecretTextKey`.
- To encrypt provided strings with Caesar cipher and output result to the file output: `cargo run caesar encrypt file YourTargetString 123`.
- To decrypt provided strings with Caesar cipher and output result to the console/standard and a file output: `cargo run caesar decrypt both .D4EAF0EDCFDCEDE2E0EFCEEFEDE4E9E2 123`.
- To generate Diffie-Hellman values: `cargo run df generate file`, `cargo run df generate console none none none none`, `cargo run df generate console none 123 none 12345`.
- To encrypt/decrypt with RSA cipher: `cargo run encrypt console "Target string!" 12 19784619`.
- To generate an RSA key pair: `cargo run rsa generate console`.
- To bruteforce a public RSA key: `cargo run rsa bruteforce both 12 19784619`, `cargo run rsa bruteforce both 12 19784619 32`.

### Docker

Theoretically, you can spin up a container from the official Rust image, copy or link `homework2` directory to the running container
and run the above-mentioned commands. Another option is to compose the image from the provided `docker-compose.yml` file.  

Run the following command inside the `homework2` directory, where the `docker-compose.yml` resides: `docker-compose run --rm  dev`.  

This Docker composes file links/passes through current `homework1`  and `homework2` directories to the Docker container.
Inside the Docker container, that is built from the official Rust Docker image, you should be able to access `/homework2` directory and run `cargo run <args>` to compile and run the tool.  

> Compose file may require modification, to tailor you Docker/docker-compose setup.
> The setup was tested with Docker version 20.10.7 and docker-compose 1.25.0 on Ubuntu 20.04.

## Logic behind the solution

Tool accepts a variable amount of arguments and creates a configuration struct suitable for a particular algorithm/cipher(-s), which it will further pass to the function uniting the whole logic. The accepted arguments are tested on validity. The requested algorithm implementations rely on the implementation of a custom BigInt library. Explanations for the Caesar and Vigenere implementations are available in the `homework1` dirctory. Here the RSA, Diffie-Hellman and bruteforcing will be discussed.  

### A custom BigInt

A custom BigInt is a struct containing a vector with digits and an enumeration defining the sign of the integer: positive, negative or zero/neutral. 
The vector stores the digits in a decimal format, in little endian. The decimal base was chosen to ease a more precise randomisation of a BigInt and to 
avoid possibly costly operations of transition between bases, considering that the decimal one will be often requested. The little endian was chosend, 
because it simplifies addition, subtraction and multiplication algorithms.  

The following mathematical operations are implemented and are available for use:  

- Addition `+`
- Subtraction `-`
- Multiplication `*`
- Division `/`
- Euclidean modulus `%`
- Binary exponentiation
- Modular exponentiation
- Negation `-`
- Comparison `==`, `!-`, `<`, `<=`, `>`, `>=`
- GCD
- EGCD
- Prime, coprime, primitive root generations
- Iterative and a Miller-Rabin primality tests
- Factorisation, prime fatorisation and RSA modulus factorisations
- And other operations not strictly math related

For the addition, subtraction and multiplication school arithmetics algorithms were implemented. The multiplication could be sped up with Karatsuba multiplication 
algorithm; considering that division, modulus and other rely on it, it could provide a decent speed boost. Division and modulus were implemented with 
Quotient Estimation Algorithm, turning the operation into one native division with small numbers and one-to-three custom multiplications. 
A binary type of exponentiation was implemented for its lighter runtime complexity of O(sqrt(n)), and a modular type was implemented as well.  

The BigInt related code can be found under the path of `homework2/src/logic` in a file `bigint.rs` and a `bigint` directory.  

### Diffie-Hellman

Diffie-Hellman algorithm has only optional parameters, if all of them are empty, the shared prime, base and both A and B secrets are randomised and calculated. 
Some values may contain a variant `None`, signifying that this particular field can be calculated and randomised, while other, is they received some data,
will align with the provided data. The whole exchange of values is calculated with the help of a custom BigInt library and when finished, the results are returned 
and are printed out.  

The Diffie-Hellman related code can be found under the path of `homework2/src/crypto` in a file `diffie_hellman.rs`.  

### RSA

RSA relies on the difficulty of factorisation of a product of two big prime integers. The current solution a krandom key-pair genearion, 
but it is limited to a modulus that has a length of ~46 digits. Going higher will bring a noticeable slowdown during calculations. 
Going below 40 is prohibited, this is caused by the block encryption implementation. The target string for encryption is split into 16 byte chunks of separate bytes, each one of them is fused together into one unsigned 128 integer (they are natively supported in Rust), which can contain values with the maximum length of 39. If the modulus for the encryption is smaller than the block size then the information will be lost during modulus operation. THe produce integer is turned 
into a custom BigInt, which has a `modpow` operation implemented, the result of it is calculated. Iteratively, from each produced BigInt 
an inner vector of decimal digits is extracted (BigInt represents numbers in a decimal system, in little endian); all the vectors are concatenated 
with a predetermined delimeter value. The final vector of bytes is encoded into hex and is returned. The decryption process reverses these operations.  

The RSA related code can be found under the path of `homework2/src/crypto` in a file `rsa.rs`.  

### RSA private key bruteforce calculation

When the bruteforce operation is requested, the tool takes the public modulus, calculates its digit length and splits it in half (there is no need to try option over the half of the whole range of possible values, factors come in pairs). A vector of 9's is initialized with the amount equal the half of the modulus length. 
Afterwards, the produced value is turned into a BigInt, and it is divided by the amount of threads. The retrieved value is used as a starting point, 
from which the workers/threads will start checking values. Thus the whole range of values is split among the number of threads.  

The thread-pool is a struct containing a vector of requested workers/threads and a sender part of a channel, through which, the main thread sends 
the new clojures/anonymous functions with tasks to the other end, which is shared among the threads and is controlled by a mutex, to prevent data races, 
and a smart pointer, observing the usage of the shared receiving point. When the thread is idle, it constantly listens to the receiver part of the channel 
for the new jobs. When a job received the worker executes it. The clojures used, contain one more channel, but this time they are sending the data to the 
main thread which listens to the successful signals with the produced data, or a termination signal, signifying that the modulus is not suitable for factoring, e.g. 
it was not a product of two primes.  

The operation is limited to modulus with the maximum length of 10 digits, if more is requested, a more significant amount of time will be needed to factorise
the target.  

The RSA bruteforce related code can be found under the path of `homework2/src/crypto` in a file `rsa.rs`, a directory `rsa` and partially in a file `factor.rs`, 
under the path of `homework1/src/logic/bigint`.  

### Tests

Both unit-tests and integration tests were implemented for the project. 
Each library crate/file holds its own module with tests for the code residing in that crate/file. 
Integration tests mimicking `main` function or the main logic of the tool, can be found under the path of: `homework2/tests/integration_test.rs`.

To run the test, run the next command in the `homework2` directory: `cargo test --release`.  
If you want to see additional debugging output from the test functions run: `cargo test --release -- --nocapture`.


## CI/CD

Gitlab CI/CD was configured for learning purposes, to become more familiar with DevOps technologies. 
The present configuration `.gitlab-ci.yml` can test the `homework1` and `homework2` Rust code with Cargo. It can consistently build binary for Linux on amd64 architecture. 
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
