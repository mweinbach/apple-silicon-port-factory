#!/usr/bin/env bash
# Worktree environment bootstrap and settle cleanup for T3 Code (and manual git worktrees).
#
# Links gitignored, read-mostly local caches/references from the primary checkout into a
# linked git worktree so builds and tools work immediately without duplicating gigabytes:
#   - third_party/              (Wwise Vorbis codebooks for wwise-audio, DirectX/middleware refs)
#   - db/functions.sqlite       (local fndb index, if built in primary checkout)
#   - db/ghidra_names.tsv       (local Ghidra name export, if present)
#   - db/ghidra_functions.jsonl (local Ghidra call-graph export, if present)
#   - re/ghidra/AW2.{gpr,rep}   (local Ghidra project, if present)
#   - out/{symbols,decomp}      (shared symbol table & decomp cache; crash/perf logs stay local)
#   - .agents/mcp_config.json   (local machine MCP config, if present)
#
# `target/` is intentionally kept isolated per worktree: `crates/win64-abi` mirrors crate
# sources into `target/win64-abi-aarch64/<pkg>/src/` and Cargo holds `.cargo-lock` there,
# so sharing `target/` across concurrent worktrees would corrupt builds. `clean` removes
# only a linked worktree's local `target/` directory on thread settle.
#
# Usage:
#   ./scripts/worktree.sh setup    # link gitignored caches/refs and check host prerequisites
#   ./scripts/worktree.sh clean    # remove linked worktree's local target/ (safe no-op on primary)
#   ./scripts/worktree.sh status   # inspect resolved paths and worktree link status
set -euo pipefail

ok()   { printf '  [ok]   %s\n' "$*"; }
note() { printf '  [info] %s\n' "$*"; }
warn() { printf '  [warn] %s\n' "$*"; }

canon_dir() {
    (cd "$1" 2>/dev/null && pwd -P)
}

resolve_roots() {
    local wt_raw="${T3CODE_WORKTREE_PATH:-}"
    if [[ -z "$wt_raw" ]]; then
        wt_raw="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
    fi
    WORKTREE_ROOT="$(canon_dir "$wt_raw")"

    local main_raw="${T3CODE_PROJECT_ROOT:-${T3_PROJECT_ROOT:-}}"
    if [[ -n "$main_raw" && -d "$main_raw" ]]; then
        MAIN_ROOT="$(canon_dir "$main_raw")"
    else
        local common_dir
        common_dir="$(git -C "$WORKTREE_ROOT" rev-parse --path-format=absolute --git-common-dir 2>/dev/null || true)"
        if [[ -n "$common_dir" && "$(basename "$common_dir")" == ".git" ]]; then
            MAIN_ROOT="$(canon_dir "$(dirname "$common_dir")")"
        else
            MAIN_ROOT="$WORKTREE_ROOT"
        fi
    fi

    IS_LINKED_WORKTREE=0
    if [[ -f "$WORKTREE_ROOT/.git" && "$WORKTREE_ROOT" != "$MAIN_ROOT" ]]; then
        IS_LINKED_WORKTREE=1
    fi
}

# Link a gitignored path from MAIN_ROOT into WORKTREE_ROOT.
# Never overwrites a real (non-symlink) path in the worktree and never touches git-tracked paths.
link_shared() {
    local rel="$1"
    local kind="$2" # file | dir
    local src="$MAIN_ROOT/$rel"
    local dst="$WORKTREE_ROOT/$rel"

    if [[ "$kind" == "dir" ]]; then
        if [[ ! -d "$src" ]]; then
            return 1
        fi
    else
        if [[ ! -f "$src" ]]; then
            return 1
        fi
    fi

    # Guard against older branches where a path (e.g. third_party/*) may still be tracked.
    if [[ -n "$(git -C "$WORKTREE_ROOT" ls-files -- "$rel" 2>/dev/null)" ]]; then
        note "skip $rel (tracked by git on this branch)"
        return 0
    fi

    if [[ -e "$dst" && ! -L "$dst" ]]; then
        note "keep local $rel (already exists in worktree)"
        return 0
    fi

    mkdir -p "$(dirname "$dst")"
    ln -sfn "$src" "$dst"

    # On older branches whose .gitignore uses a trailing slash (e.g. `/third_party/`),
    # Git treats directory symlinks as untracked files. Fall back cleanly so `git status`
    # stays clean even when spinning up a worktree from an older commit.
    if [[ "$kind" == "dir" ]] && [[ -n "$(git -C "$WORKTREE_ROOT" status --porcelain -- "$rel" 2>/dev/null)" ]]; then
        rm -f "$dst"
        if [[ "$rel" == "third_party" ]]; then
            mkdir -p "$dst"
            local child
            for child in "$src"/*; do
                [[ -e "$child" ]] || continue
                ln -sfn "$child" "$dst/$(basename "$child")"
            done
            ok "linked $rel/* -> $src/* (older .gitignore compatibility)"
        else
            note "skip $rel (directory symlink is not gitignored on this branch)"
        fi
        return 0
    fi

    ok "linked $rel -> $src"
    return 0
}

cmd_setup() {
    echo "=== Apple Silicon Port Factory — Worktree Setup ==="
    note "worktree: $WORKTREE_ROOT"
    note "primary:  $MAIN_ROOT"

    if [[ "$IS_LINKED_WORKTREE" -eq 0 ]]; then
        note "primary checkout detected; skipping cross-worktree symlinks"
    else
        if ! link_shared "third_party" "dir"; then
            warn "third_party/ not present in primary checkout (optional; run ./scripts/third_party.sh for Wwise Vorbis codebooks)"
        fi

        # Optional local analysis databases & Ghidra project (gitignored)
        link_shared "db/functions.sqlite" "file" || true
        link_shared "db/ghidra_names.tsv" "file" || true
        link_shared "db/ghidra_functions.jsonl" "file" || true
        link_shared "re/ghidra/AW2.gpr" "file" || true
        link_shared "re/ghidra/AW2.rep" "dir" || true

        # Shared read-mostly output caches; keep out/ itself local so crash/perf/hitch logs stay isolated
        link_shared "out/symbols" "dir" || true
        link_shared "out/decomp" "dir" || true

        # Optional local machine agent MCP config (gitignored)
        link_shared ".agents/mcp_config.json" "file" || true
    fi

    # Non-blocking host environment checks
    if head -n 1 "$WORKTREE_ROOT/db/progress.tsv" 2>/dev/null | grep -q '^version https://git-lfs.github.com/spec/v1'; then
        warn "db/progress.tsv is a Git LFS pointer (only needed for Alan Wake 2 work; run 'git lfs pull' if needed)"
    fi

    local msc=""
    local cand
    for cand in \
        "${METAL_IRCONVERTER_DYLIB:-}" \
        "$HOME/.local/opt/metal-shaderconverter/lib/libmetalirconverter.dylib" \
        "/opt/homebrew/lib/libmetalirconverter.dylib" \
        "/usr/local/lib/libmetalirconverter.dylib" \
        "/opt/metal-shaderconverter/lib/libmetalirconverter.dylib"; do
        if [[ -n "$cand" && -f "$cand" ]]; then
            msc="$cand"
            break
        fi
    done
    if [[ -n "$msc" ]]; then
        ok "Metal Shader Converter: $msc"
    else
        warn "Metal Shader Converter not found (optional for CPU/Win32 tests; run ./scripts/install_metal_shader_converter.sh)"
    fi

    note "target/ stays isolated per worktree (required by win64-abi mirroring & Cargo locking)"
    echo "Worktree ready."
}

cmd_clean() {
    echo "=== Apple Silicon Port Factory — Worktree Build Cleanup ==="
    if [[ "$IS_LINKED_WORKTREE" -eq 0 ]]; then
        note "primary checkout detected ($WORKTREE_ROOT); refusing to clean target/"
        return 0
    fi

    local target_dir="$WORKTREE_ROOT/target"
    if [[ -L "$target_dir" ]]; then
        warn "target/ is a symlink; refusing to remove symlink target"
        return 0
    fi
    if [[ -d "$target_dir" ]]; then
        local size
        size="$(du -sh "$target_dir" 2>/dev/null | awk '{print $1}')"
        rm -rf "$target_dir"
        ok "removed worktree target/ (${size:-unknown} reclaimed)"
    else
        note "no worktree target/ directory to clean"
    fi
}

cmd_status() {
    echo "worktree: $WORKTREE_ROOT"
    echo "primary:  $MAIN_ROOT"
    echo "linked:   $IS_LINKED_WORKTREE"
    local rel
    for rel in \
        "third_party" \
        "db/functions.sqlite" \
        "db/ghidra_names.tsv" \
        "db/ghidra_functions.jsonl" \
        "re/ghidra/AW2.gpr" \
        "re/ghidra/AW2.rep" \
        "out/symbols" \
        "out/decomp" \
        ".agents/mcp_config.json"; do
        local p="$WORKTREE_ROOT/$rel"
        if [[ -L "$p" ]]; then
            printf '  symlink  %s -> %s\n' "$rel" "$(readlink "$p")"
        elif [[ -e "$p" ]]; then
            printf '  local    %s\n' "$rel"
        else
            printf '  absent   %s\n' "$rel"
        fi
    done
    if [[ -d "$WORKTREE_ROOT/target" && ! -L "$WORKTREE_ROOT/target" ]]; then
        printf '  target/  %s\n' "$(du -sh "$WORKTREE_ROOT/target" 2>/dev/null | awk '{print $1}')"
    else
        printf '  target/  absent\n'
    fi
}

resolve_roots

case "${1:-setup}" in
    setup)  cmd_setup ;;
    clean)  cmd_clean ;;
    status) cmd_status ;;
    -h|--help)
        sed -n '2,23p' "$0"
        ;;
    *)
        echo "Usage: $0 [setup|clean|status]" >&2
        exit 2
        ;;
esac
