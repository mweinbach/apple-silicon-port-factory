---
name: audit-repo-hygiene
description: Audit the repository before commits, PRs, or public pushes to verify zero copyrighted game files or assets, zero hardcoded personal paths or emails, zero git submodules, correct db/*.tsv storage (plain-text catalogs, Git LFS only for large tables), strict crate boundaries, and db catalog sync via cargo opsdb -- doctor. Use when preparing a public release, reviewing a PR, or checking repo cleanliness.
---

# Audit Repository Hygiene, Boundaries & Public-Release Readiness

Use this workflow before opening a PR, pushing to a public remote, or after bringing up a new game
to guarantee the repository stays clean, legal, portable, and single-repo.

## Quick Automated Audit

Run the bundled read-only audit script:

```sh
# Audit all tracked files (fast, read-only)
.agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh

# Audit staged changes before committing + verify db/*.tsv sync via doctor
.agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh --staged --doctor
```

## Manual & Policy Checklist

1. **Zero copyrighted game files, binaries, assets, or captures.**
   - Never commit `.exe`, `.dll`, `.rmdblob`, `.rmdtoc`, `.binrfx`, `.bnk`, `.wem`, `.bk2`, `.pak`,
     `.ucas`, `.utoc`, `.app/`, `.ipa`, `.mobileprovision`, game screenshots, or Steam HTML dumps.
   - All game-dependent tests must read paths from `--game-dir`, `--pe`, `PORT_GAME_DIR`,
     `AW2_GAME_DIR`, or `AW2_EXE` and skip cleanly when unset.
2. **Zero hardcoded personal machine paths or private metadata.**
   - No `/Users/<real_name>`, personal `/Volumes/<drive>`, personal emails, or Steam account IDs in
     tracked source, tests, scripts, or docs (use generic placeholders like `/path/to/Game` or
     `/Users/<name>` in docs).
   - Keep local git commits configured with your intended public/noreply author email.
   - Never push private/archive branches (such as `local/archive-full-history` or `refs/t3/*`) to a
     public remote.
3. **Single repo, zero submodules, plain-text `db/` catalogs, Git LFS only for large tables.**
   - `.gitmodules` must not exist; `third_party/` stays gitignored and is populated on demand via
     `./scripts/third_party.sh` (shallow `git clone`, defaulting to `--minimal`: `directx-headers`
     + `vgmstream`).
   - The catalogs that `cargo opsdb -- doctor` checks (`db/ops_x86_to_arm64.tsv`,
     `db/win32_apis.tsv`, `db/shader_formats.tsv`, `db/game_profiles.tsv`) and the other small
     tables are plain text, so pull requests show real diffs and no Git LFS is needed to build,
     test, or run `doctor`.
   - Tables over 1 MiB (the Alan Wake 2 analysis tables `db/progress.tsv`, `db/throws.tsv`,
     `db/dll_progress.tsv`, `db/ret_liveness.tsv`, `db/ret_liveness_dll.tsv`) must be listed in
     `.gitattributes` with `filter=lfs diff=lfs merge=lfs -text`. Verify with `git lfs ls-files`.
4. **Hard crate boundaries.**
   - `crates/` must contain zero game-specific VAs, hardcoded game names, or engine asset parsers.
   - `crates/win32-rt` must contain zero `objc2-metal` and zero `aw2-data`.
   - `crates/port-loader` default features (`port-runner`) must not depend on `aw2-game` or
     `aw2-data`.
5. **Catalog sync & experimental/BYOG framing.**
   - Run `cargo opsdb -- doctor` (exits non-zero on drift across `db/ops_x86_to_arm64.tsv`,
     `db/win32_apis.tsv`, `db/shader_formats.tsv`, `db/game_profiles.tsv`; `--fix` to sync).
   - Keep dual licensing (`LICENSE-MIT`, `LICENSE-APACHE`), `THIRD_PARTY_NOTICES.md`, and
     experimental / legally-owned-games-only / no-anticheat-bypass framing intact.
