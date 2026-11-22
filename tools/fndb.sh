#!/usr/bin/env bash
# Thin wrapper: tools/fndb.sh <subcommand> ... (see tools/README.md)
exec python3 "$(dirname "$0")/fndb/fndb.py" "$@"
