# Quick Release Guide 🚀

This is a quick reference for releasing a new version of DroidTUI.

## TL;DR - One Command Release

```bash
# Install just (first time only)
cargo install just

# Release new version - THAT'S IT!
just release 0.2.5
```

This single command will:
- ✅ Update `Cargo.toml` version
- ✅ Update `README.md` version badge
- ✅ Generate `CHANGELOG.md` with git-cliff
- ✅ Run `cargo fmt`
- ✅ Run `cargo clippy`
- ✅ Run `cargo test`
- ✅ Commit changes
- ✅ Create git tag
- ✅ Push to GitHub

Then GitHub Actions automatically:
- ✅ Run CI tests on Linux & Windows
- ✅ Build release binaries
- ✅ Update CHANGELOG.md and commit it
- ✅ Update README.md version badge
- ✅ Create GitHub Release with changelog notes
- ✅ Publish to crates.io

## Prerequisites (One-Time Setup)

```bash
# Install just command runner
cargo install just

# Install git-cliff for changelog generation
cargo install git-cliff

# Add CRATES_IO_TOKEN to GitHub secrets (for auto-publish)
# Get token from: https://crates.io/settings/tokens
# Add to: https://github.com/YOUR_USERNAME/droidtui/settings/secrets/actions
```

## Release Workflow

### 1. Make Your Changes

```bash
# Make changes, commit with conventional commits
git add .
git commit -m "feat: add new awesome feature"
git commit -m "fix: resolve bug in menu"
git commit -m "docs: update installation guide"
```

### 2. Release New Version

```bash
# For bug fixes: 0.2.4 → 0.2.5
just release 0.2.5

# For new features: 0.2.5 → 0.3.0
just release 0.3.0

# For breaking changes: 0.3.0 → 1.0.0
just release 1.0.0
```

### 3. Wait for GitHub Actions

After pushing, GitHub Actions will:
1. Run all tests (2-3 minutes)
2. Generate changelog and update docs
3. Build and publish release (5-10 minutes)

Check progress at: https://github.com/sorinirimies/droidtui/actions

### 4. Verify Release

- ✅ Check GitHub Releases: https://github.com/sorinirimies/droidtui/releases
- ✅ Check crates.io: https://crates.io/crates/droidtui
- ✅ Test installation: `cargo install droidtui`

## Conventional Commit Format

Use these prefixes for automatic changelog categorization:

```bash
feat:     🚀 New feature
fix:      🐛 Bug fix
docs:     📚 Documentation
refactor: 🚜 Code refactoring
perf:     ⚡ Performance improvement
style:    🎨 Code style/formatting
test:     🧪 Tests
chore:    ⚙️ Maintenance tasks
```

Examples:
```bash
git commit -m "feat: add scrollable command output"
git commit -m "fix: align menu borders correctly"
git commit -m "docs: update README with new features"
git commit -m "perf: optimize rendering performance"
```

## Alternative Methods

### Method 2: Use the Bash Script

```bash
./scripts/bump_version.sh 0.2.5
```

Then manually push:
```bash
git push origin main
git push origin v0.2.5
```

### Method 3: Manual (Not Recommended)

See [RELEASE.md](RELEASE.md) for detailed manual process.

## Useful Commands

```bash
# Check current version
just version

# Run all checks without releasing
just check-all

# Generate changelog without releasing
just changelog

# View current changelog
just view-changelog

# Build release binary locally
just build-release

# Show all available commands
just
```

## Troubleshooting

### Tests Fail Locally

```bash
# Run tests to see what's wrong
cargo test

# Fix clippy issues
cargo clippy --fix

# Format code
cargo fmt
```

### Release Failed on GitHub

1. Check GitHub Actions logs
2. Fix the issue
3. Delete the tag and retry:
   ```bash
   git tag -d v0.2.5
   git push origin :refs/tags/v0.2.5
   just release 0.2.5
   ```

### Can't Push to GitHub

```bash
# Make sure you're on main branch
git checkout main

# Pull latest changes
git pull origin main

# Try again
just release 0.2.5
```

## Version Numbers

Follow [Semantic Versioning](https://semver.org/):

- **Patch** (0.2.4 → 0.2.5) - Bug fixes, small improvements
- **Minor** (0.2.5 → 0.3.0) - New features, backwards compatible
- **Major** (0.3.0 → 1.0.0) - Breaking changes

## Support

- 📖 Full documentation: [RELEASE.md](RELEASE.md)
- 🐛 Issues: https://github.com/sorinirimies/droidtui/issues
- 💬 Discussions: https://github.com/sorinirimies/droidtui/discussions

---

**Happy Releasing! 🎉**