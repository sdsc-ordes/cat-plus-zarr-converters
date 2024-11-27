set positional-arguments
set shell := ["bash", "-cue"]

root_dir := `git rev-parse --show-toplevel`

# Default recipe to list all recipes.
default:
  just --list

# Build the synth-converter.
build *args:
    cd "{{root_dir}}/synth-converter" && \
        cargo build --bin synth-converter {{args}}

# Test the synth-converter.
test *args:
    cd "{{root_dir}}/synth-converter" && \
        cargo test --bin synth-converter {{args}}

# Run the synth-converter.
run *args:
    cd "{{root_dir}}/synth-converter" && \
        cargo run --bin synth-converter {{args}}

# Enter a Nix development shell.
nix-develop *args:
    echo "Starting nix developer shell in './tools/nix/flake.nix'."
    cd "{{root_dir}}" && \
    cmd=("$@") && \
    { [ -n "${cmd:-}" ] || cmd=("zsh"); } && \
    nix develop ./tools/nix#default --accept-flake-config --command "${cmd[@]}"
