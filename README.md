# Rust Odds or Evens CLI Game

This game was developed on the series Rust Basics: A friendly introduction.

## Clone

Open the terminal and run the following commands to clone this repository and move inside it.

```bash
git clone https://github.com/andrekardec/rust-odds-or-events.git
cd rust-odds-or-events
```

## Local
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

### Requirements


Check if you have Rust and Cargo installed on your computer.

```bash
# should return rustc <version>
rustc --version
# should return cargo <version>
cargo --version
```

If Rust or Cargo is not installed on your computer, please follow the [installation guide](https://www.rust-lang.org/tools/install) or jump to the Docker section.

### Run 

From inside the `./rust-odds-or-events`: 

```bash
cargo run
```

## Docker

If you do not want to install Rust, use this instructions. The binaries will be generated inside a Docker container and then moved to the project's root folder, inside `targets`. This folder will be generated after running `./build.sh`.

![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

#### Build the Docker Image
If you're running the project from Windows, use the shell script `build-windows.sh`. It will target `x86_64-pc-windows-gnu`.

```bash
# MacOS or Linux
./build.sh
# Windows
./build-windows.sh
```

#### Run the app
Once the image is already built, you can just run to play. If you make any changes to the source code, remember to build the fresh version before running.https://opensource.org/licenses/ECL-2.0

```bash
./run.sh
```

## Show your support
I do not ask for donations, all I ask is that you star this repo if somehow I've helped you.
somehow
## License
> "Put knowledge where people trip over it.” —Carla O’Dell

- Any future work done must follow the guidelines mentioned in MIT.
- [MIT](https://opensource.org/licenses/MIT)