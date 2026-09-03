# GitHub Actions Workflows

Optima ships with a comprehensive set of GitHub Actions workflows. They are grouped by purpose.

## Quality & Code Health

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `clippy.yml` | push/PR | Enforce `cargo clippy -D warnings` |
| `rustfmt.yml` | push/PR | Check code formatting |
| `cargo-audit.yml` | weekly/push | Dependency vulnerability audit |
| `cargo-deny.yml` | weekly/push | License, ban, and source checks |
| `cargo-udeps.yml` | push/PR | Detect unused dependencies (nightly) |
| `rust-semver.yml` | PR | Check API semver compatibility |
| `secret-scan.yml` | push/PR | Detect committed secrets (gitleaks) |
| `codespell.yml` | push/PR | Typo check across the repo |
| `license-check.yml` | push/PR | Verify license metadata |

## Build, Test & Release

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `ci.yml` | push/PR | Main CI: check + Android matrix + Linux |
| `test.yml` | push/PR | Run unit tests |
| `e2e.yml` | push/PR | Run end-to-end/integration tests (Xvfb) |
| `examples.yml` | push/PR | Build all examples |
| `bench.yml` | push/PR | Run benchmarks (criterion) |
| `coverage.yml` | push/PR | Test coverage via tarpaulin + Codecov |
| `docs.yml` | push/tag | Build & deploy rustdoc to GitHub Pages |
| `android-aar.yml` | push/PR/tag | Build per-ABI `.so` and assemble AAR |
| `publish.yml` | tag/dispatch | Build AAR and publish to GitHub Packages |
| `release.yml` | tag/dispatch | Draft GitHub release with changelog |
| `version-bump.yml` | dispatch | Bump version across Cargo/Kotlin/Android + tag |
| `changelog.yml` | push | Generate `CHANGELOG.md` via git-cliff (`cliff.toml`) |
| `cargo-publish.yml` | release | Publish crate to crates.io |

## Registry & Packaging

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `docker.yml` | push/tag/PR | Build & push GHCR Docker image (`Dockerfile`) |
| `gh-packages.yml` | push | Build AAR and publish to GitHub Packages |
| `maven-central.yml` | release | Publish AAR to Maven Central (Sonatype) |

## Repository Automation

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `auto-label.yml` | PR | Label PRs by changed paths (`.github/labeler.yml`) |
| `assign-author.yml` | PR | Auto-assign PR to its author |
| `size-label.yml` | PR | Label PRs by size (XS..XL) |
| `pr-title.yml` | PR | Enforce conventional commit PR titles |
| `welcome.yml` | issue/PR | First-interaction welcome |
| `stale.yml` | schedule | Close stale issues/PRs |
| `lock-threads.yml` | schedule | Lock inactive issue/PR threads |
| `todo-comments.yml` | push/PR | Report TODO/FIXME references |

## Configuration Files

- `.github/dependabot.yml` — automated dependency updates (Cargo, Gradle, GitHub Actions)
- `.github/labeler.yml` — path-based PR labels
- `cliff.toml` — git-cliff changelog configuration
- `Dockerfile` — desktop container build

## Secrets Used

| Secret | Workflow(s) | Required |
|--------|-------------|----------|
| `CARGO_REGISTRY_TOKEN` | cargo-publish | crates.io publishing |
| `OSSRH_USERNAME` / `OSSRH_TOKEN` | maven-central | Maven Central publishing |
| `SIGNING_KEY` / `SIGNING_PASSWORD` | maven-central | GPG signing |
| `GITHUB_TOKEN` | many | Auto-provided by GitHub |
