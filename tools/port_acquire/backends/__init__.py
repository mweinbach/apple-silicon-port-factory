"""Pluggable store backends for `tools/port-acquire`.

Each store backend lives in its own module under this package (e.g. `epic.py`,
`steam.py`) and exposes `BACKEND_CLASS = <StoreBackend subclass>` or
`create_backend() -> StoreBackend`.
"""
