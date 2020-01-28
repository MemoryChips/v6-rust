# Notes

## Cargo Commands

```bash
cargo build
cargo test
cargo run --example sandbox
```

## Setup

### Examples

See glfw-rs examples here
[glfw-rs examples](https://github.com/PistonDevelopers/glfw-rs)

### GLFW

[Compile and install GLFW 3.x](http://www.glfw.org/docs/latest/compile.html)

Setup here: \$HOME/Training/cpp/glfw.

```bash
git clone git@github.com:glfw/glfw.git
cd glfw
cmake -DCMAKE_C_FLAGS=-fPIC

mkdir build
cd build
cmake .. -DCMAKE_BUILD_TYPE=Debug
make -j3 # to build
make clean

# Add to path in .profile?
# /home/robert/Training/cpp/glfw/build/src
export PATH="$HOME/Training/cpp/glfw/build/src:$PATH"

```
