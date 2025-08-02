# Release Process

This document describes how to release a new version of bubbletea-rs to crates.io.

## Prerequisites

1. **crates.io Account**: You need an account on [crates.io](https://crates.io/)
2. **API Token**: Generate an API token from your crates.io account settings
3. **GitHub Repository Access**: You need push access to create tags

## Setup GitHub Secrets

1. Go to your GitHub repository settings
2. Navigate to Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Add a secret named `CARGO_REGISTRY_TOKEN` with your crates.io API token

## Release Process

### 1. Update Version

Update the version in `Cargo.toml`:

```toml
version = "0.0.2"  # Increment as needed
```

### 2. Commit Changes

```bash
git add Cargo.toml
git commit -m "chore: bump version to 0.0.2"
git push origin main
```

### 3. Create and Push Tag

Create a tag matching the version:

```bash
git tag v0.0.2
git push origin v0.0.2
```

### 4. Automated Publishing

Once you push the tag, the GitHub Actions workflow will:

1. Run all tests on Ubuntu
2. Check code formatting
3. Run clippy lints
4. Publish to crates.io using your API token
5. Create a GitHub release with auto-generated release notes

### 5. Monitor the Release

- Check the Actions tab in GitHub to monitor the release progress
- Visit [crates.io/crates/bubbletea-rs](https://crates.io/crates/bubbletea-rs) to verify publication
- The GitHub release will appear in the Releases section

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):
- MAJOR version for incompatible API changes
- MINOR version for backwards-compatible functionality additions
- PATCH version for backwards-compatible bug fixes

## Troubleshooting

### Publishing Fails

1. **Authentication Error**: Verify your `CARGO_REGISTRY_TOKEN` secret is set correctly
2. **Version Conflict**: Ensure the version hasn't been published already
3. **Build Errors**: Check that all tests pass locally before tagging

### Manual Publishing

If needed, you can publish manually:

```bash
cargo publish
```

You'll be prompted for your crates.io credentials or can use:

```bash
cargo publish --token YOUR_TOKEN
```