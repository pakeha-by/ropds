# Coverage Report

Generated: 2026-08-24 08:06:12 UTC

## Scope

- Test types: unit tests (`src/lib.rs` harness) + integration tests (`tests/integration_tests.rs`)
- Coverage workflow uses: `cargo test`, `llvm-profdata`, `llvm-cov`
- Container-based docker tests were not included (feature-gated)

## Test execution result

- Unit tests: 285 passed, 0 failed
- Integration tests: 110 passed, 0 failed

## Coverage summary (project files)

- Regions coverage: 83.15%
- Functions coverage: 83.95%
- Lines coverage: 81.94%

### Source only (`src/`)

- Lines: 18978
- Missed lines: 4110
- Line coverage: 78.34%

### Integration test code only (`tests/integration/`)

- Lines: 3811
- Missed lines: 5
- Line coverage: 99.87%

## Lowest covered source files (by line coverage)

| File | Line coverage |
|---|---|
| `src/web/admin/genres.rs` | 0.00% |
| `src/web/admin/oauth_requests.rs` | 0.00% |
| `src/web/admin/scan.rs` | 0.00% |
| `src/web/oauth.rs` | 10.20% |
| `src/web/admin/user_pages.rs` | 15.91% |
| `src/email.rs` | 33.96% |
| `src/web/admin/book_edit.rs` | 37.66% |
| `src/opds/v2/feeds.rs` | 38.59% |
| `src/db/models.rs` | 42.86% |
| `src/lib.rs` | 42.86% |
