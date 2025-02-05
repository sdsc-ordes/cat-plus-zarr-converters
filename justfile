set positional-arguments
set shell := ["bash", "-cue"]

root_dir := `git rev-parse --show-toplevel`

# Default recipe to list all recipes.
default:
    just --list

# Build the synth-converter.
build *args:
    cargo build --manifest-path "{{root_dir}}/synth-converter/Cargo.toml" "$@"

# Test the synth-converter.
test *args:
    cargo test --manifest-path "{{root_dir}}/synth-converter/Cargo.toml" "$@"

# Format the synth-converter.
fmt *args:
    cargo fmt --manifest-path "{{root_dir}}/synth-converter/Cargo.toml" "$@"

# Run the synth-converter.
run input_file output_file *args:
    cd "{{root_dir}}/synth-converter" && \
    cargo run --bin synth-converter {{root_dir}}/{{input_file}} {{root_dir}}/{{output_file}} {{args}}

# Enter a Nix development shell.
nix-develop *args:
    echo "Starting nix developer shell in './tools/nix/flake.nix'."
    cd "{{root_dir}}" && \
    cmd=("$@") && \
    { [ -n "${cmd:-}" ] || cmd=("zsh"); } && \
    nix develop ./tools/nix#default --accept-flake-config --command "${cmd[@]}"
