#!/opt/homebrew/bin/bash

# Path to your Rust file
RUST_FILE="/Users/wervand/Desktop/des/rust/test/factorial.rs"

# Loop to continuously check and compile
while true; do
    # Compile the Rust code
    rustc $RUST_FILE -o main
    # If it compiles successfully, run the code
    if [ $? -eq 0 ]; then
        ./main
    fi
    # Wait for changes to the Rust file before looping again
    inotifywait -e close_write $RUST_FILE
done
