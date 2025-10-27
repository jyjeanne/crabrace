# CI/CD Pipeline Documentation

**Project:** Crabrace
**CI Platform:** GitHub Actions
**Status:** ✅ Active

---

## 📊 Overview

Crabrace uses GitHub Actions for continuous integration and continuous deployment. The CI pipeline runs on every push to `main` and on all pull requests.

---

## 🔄 Workflow: Rust CI

**File:** `.github/workflows/rust.yml`
**Triggers:**
- Push to `main` branch
- Pull requests to `main` branch

### Jobs

#### 1. Build and Test

**Runs on:** `ubuntu-latest`

**Steps:**

1. **Checkout Code**
   - Uses: `actions/checkout@v4`
   - Fetches repository code

2. **Setup Rust**
   - Uses: `dtolnay/rust-toolchain@stable`
   - Installs stable Rust toolchain
   - Includes `rustfmt` and `clippy` components

3. **Cache Dependencies**
   - Caches cargo registry, git index, and build artifacts
   - Speeds up subsequent builds
   - Cache key based on `Cargo.lock` hash

4. **Check Formatting**
   ```bash
   cargo fmt -- --check
   ```
   - Ensures code follows Rust formatting standards
   - Fails if code is not formatted

5. **Run Clippy**
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```
   - Runs Rust linter
   - Treats warnings as errors
   - Checks all targets and features

6. **Build Debug**
   ```bash
   cargo build --verbose
   ```
   - Compiles project in debug mode
   - Shows detailed output

7. **Run Tests**
   ```bash
   cargo test --verbose
   ```
   - Runs all unit and integration tests
   - Shows detailed test output

8. **Build Release**
   ```bash
   cargo build --release --verbose
   ```
   - Compiles optimized release binary
   - Ensures release builds work

#### 2. Validate JSON Configs

**Runs on:** `ubuntu-latest`

**Steps:**

1. **Checkout Code**
   - Uses: `actions/checkout@v4`

2. **Setup Python**
   - Uses: `actions/setup-python@v5`
   - Installs Python 3.x

3. **Validate Provider Configs**
   ```bash
   python verify_json.py
   ```
   - Validates all JSON provider configurations
   - Checks for required fields
   - Ensures data model compatibility

---

## 🎯 Pipeline Status

### Current Status

[![Rust](https://github.com/jyjeanne/crabrace/actions/workflows/rust.yml/badge.svg)](https://github.com/jyjeanne/crabrace/actions/workflows/rust.yml)

### Viewing Results

1. **GitHub Actions Tab**
   - Visit: https://github.com/jyjeanne/crabrace/actions
   - View all workflow runs

2. **PR Checks**
   - Checks appear automatically on pull requests
   - Must pass before merging

3. **Branch Protection**
   - Can be configured to require CI passing
   - Settings → Branches → Branch protection rules

---

## 🚀 Running Checks Locally

Before pushing, run these commands locally to match CI:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build
cargo build --verbose

# Run tests
cargo test --verbose

# Build release
cargo build --release --verbose

# Validate JSON
python verify_json.py
```

### All-in-One Check Script

Create `scripts/ci-check.sh`:

```bash
#!/bin/bash
set -e

echo "🔍 Checking formatting..."
cargo fmt -- --check

echo "🧹 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "🔨 Building..."
cargo build --verbose

echo "🧪 Running tests..."
cargo test --verbose

echo "📦 Building release..."
cargo build --release --verbose

echo "✅ Validating JSON..."
python verify_json.py

echo "✅ All checks passed!"
```

Run with: `bash scripts/ci-check.sh`

---

## 📈 Performance Metrics

### Build Times (Approximate)

| Step | Time (Cold) | Time (Cached) |
|------|-------------|---------------|
| Checkout | 5s | 5s |
| Setup Rust | 10s | 2s |
| Cache Restore | 10s | 5s |
| Format Check | 5s | 2s |
| Clippy | 60s | 10s |
| Build Debug | 120s | 30s |
| Tests | 30s | 10s |
| Build Release | 180s | 45s |
| Validate JSON | 5s | 3s |
| **Total** | **~7 min** | **~2 min** |

*Times are estimates and may vary*

---

## 🔧 Configuration

### Workflow File Location

```
.github/
└── workflows/
    └── rust.yml
```

### Environment Variables

```yaml
env:
  CARGO_TERM_COLOR: always
```

- `CARGO_TERM_COLOR: always` - Enables colored output in CI logs

### Rust Toolchain

- **Version:** Stable
- **Components:** rustfmt, clippy
- **Platform:** Linux (ubuntu-latest)

---

## 🎨 Customization

### Adding More Checks

To add additional checks, edit `.github/workflows/rust.yml`:

```yaml
- name: Check documentation
  run: cargo doc --no-deps --document-private-items

- name: Run benchmarks
  run: cargo bench --no-run

- name: Check security advisories
  run: cargo audit
```

### Matrix Testing

Test multiple Rust versions:

```yaml
strategy:
  matrix:
    rust: [stable, beta, nightly]
steps:
  - uses: dtolnay/rust-toolchain@master
    with:
      toolchain: ${{ matrix.rust }}
```

### Platform Testing

Test on multiple operating systems:

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest, macos-latest]
runs-on: ${{ matrix.os }}
```

---

## 🐛 Troubleshooting

### Build Fails in CI but Works Locally

1. **Different Rust Version**
   - CI uses stable
   - Check your local version: `rustc --version`

2. **Platform Differences**
   - CI runs on Linux
   - Windows-specific code may fail

3. **Missing Dependencies**
   - Ensure all dependencies are in `Cargo.toml`

### Cache Issues

If builds are slow or failing:

1. **Clear Cache**
   - Delete workflow runs
   - Cache will regenerate

2. **Update Cache Key**
   - Modify cache key in workflow file

### Test Failures

1. **Check Test Output**
   - View detailed logs in Actions tab

2. **Run Tests Locally**
   ```bash
   cargo test -- --nocapture
   ```

3. **Fix and Push**
   - Fix failing tests
   - Push changes to trigger new run

---

## 📋 Future Improvements

### Planned Additions

1. **Code Coverage**
   - Use `tarpaulin` or `grcov`
   - Upload to codecov.io

2. **Release Automation**
   - Automatic GitHub releases
   - Binary artifacts for download
   - Changelog generation

3. **Docker Builds**
   - Build Docker images in CI
   - Push to Docker Hub/GHCR

4. **Security Scanning**
   - `cargo-audit` for dependencies
   - `cargo-deny` for license compliance

5. **Performance Benchmarks**
   - Track performance over time
   - Alert on regressions

---

## 📊 Status Badge

Add to README.md:

```markdown
[![Rust](https://github.com/jyjeanne/crabrace/actions/workflows/rust.yml/badge.svg)](https://github.com/jyjeanne/crabrace/actions/workflows/rust.yml)
```

---

## 📚 Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI Setup Guide](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain)

---

## ✅ Checklist for Contributors

Before submitting a PR:

- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] All tests pass (`cargo test`)
- [ ] JSON configs valid (`python verify_json.py`)
- [ ] Documentation updated
- [ ] Commit messages clear

---

**CI Status:** ✅ Active and Running
**Last Updated:** October 27, 2025
