# DroidTUI Development Tasks
# Install just: cargo install just
# Usage: just <task>

# Default task - show available commands
default:
    @just --list

# Build the project
build:
    cargo build

# Build release version
build-release:
    cargo build --release

# Run the application
run:
    cargo run

# Run tests
test:
    cargo test

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Check if code is formatted
fmt-check:
    cargo fmt --check

# Run clippy linter
clippy:
    cargo clippy -- -D warnings

# Run all checks (fmt, clippy, test)
check-all: fmt-check clippy test
    @echo "✅ All checks passed!"

# Clean build artifacts
clean:
    cargo clean

# Install the application locally
install:
    cargo install --path .

# Bump version (usage: just bump 0.2.5)
bump version:
    @echo "Bumping version to {{version}}..."
    @./scripts/bump_version.sh {{version}}

# Quick release: format, check, test, and build
release-check: fmt clippy test build-release
    @echo "✅ Ready for release!"

# Publish to crates.io (dry run)
publish-dry:
    cargo publish --dry-run

# Publish to crates.io
publish:
    cargo publish

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated

# Generate documentation
doc:
    cargo doc --no-deps --open

# Watch and auto-run on file changes (requires cargo-watch)
watch:
    cargo watch -x run

# Git: commit current changes
commit message:
    git add .
    git commit -m "{{message}}"

# Git: push to origin
push:
    git push origin main

# Git: push tags
push-tags:
    git push --tags

# Full release workflow: bump version and push
release version: (bump version)
    @echo "Pushing to remote..."
    git push origin main
    git push origin v{{version}}
    @echo "✅ Release v{{version}} complete!"

# Show current version
version:
    @grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'

# Show project info
info:
    @echo "Project: DroidTUI"
    @echo "Version: $(just version)"
    @echo "Author: Sorin Albu-Irimies"
    @echo "License: MIT"
