#!/usr/bin/env bash
# Single-pass offline bring-up audit wrapper for a Windows x86-64 game directory.
# Delegates directly to `cargo opsdb -- audit-game` (one PE parse pass, unified readiness
# summary with Win32 impl_kind breakdown, and optional pre-filled `game.toml` via `--init-manifest`).
#
# Usage:
#   .agents/skills/bring-up-game/scripts/audit_game.sh /path/to/GameDir [MainGame.exe] [--record] [--json] [--init-manifest [OUT_TOML]] [--force]
set -euo pipefail

if [[ $# -lt 1 || "$1" == "-h" || "$1" == "--help" ]]; then
    echo "Usage: $0 <GAME_DIR> [MAIN_EXE_NAME_OR_PATH] [--record] [--json] [--init-manifest [OUT_TOML]] [--force]" >&2
    exit 2
fi

GAME_DIR="$1"
shift

if [[ ! -e "$GAME_DIR" ]]; then
    echo "error: game path not found: $GAME_DIR" >&2
    exit 1
fi

EXTRA_ARGS=()
MAIN_EXE=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --record|--json|--force)
            EXTRA_ARGS+=("$1")
            ;;
        --init-manifest)
            EXTRA_ARGS+=("$1")
            if [[ $# -gt 1 && "$2" != -* ]]; then
                shift
                EXTRA_ARGS+=("$1")
            fi
            ;;
        --exe|--id|--name)
            if [[ $# -lt 2 ]]; then
                echo "error: $1 requires a value" >&2
                exit 2
            fi
            EXTRA_ARGS+=("$1" "$2")
            shift
            ;;
        -*)
            EXTRA_ARGS+=("$1")
            ;;
        *)
            if [[ -z "$MAIN_EXE" ]]; then
                MAIN_EXE="$1"
            else
                EXTRA_ARGS+=("$1")
            fi
            ;;
    esac
    shift
done

if [[ -n "$MAIN_EXE" ]]; then
    EXTRA_ARGS+=(--exe "$MAIN_EXE")
fi

exec cargo opsdb -- audit-game "$GAME_DIR" ${EXTRA_ARGS[@]+"${EXTRA_ARGS[@]}"}
