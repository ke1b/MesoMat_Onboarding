in_nix := env_var_or_default("IN_NIX_SHELL", "")

default: help

help:
    @just --list


_check_nix_shell:
    @if [ -z "{{in_nix}}" ]; then \
        echo  "\033[33mWarning: Not in a Nix shell. Consider using 'nix develop'.\033[0m"; \
    fi

[working-directory("./temp-humid-sensor/")]
build: _check_nix_shell
	@echo "Building project..."
	cargo build

[working-directory("./temp-humid-sensor/")]
run: _check_nix_shell
	cargo run
