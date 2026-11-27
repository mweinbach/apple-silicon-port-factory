"""Dynamic backend discovery and registration for `tools/port-acquire`.

Any module placed at `tools/port_acquire/backends/<store>.py` that defines either
`create_backend() -> StoreBackend` or `BACKEND_CLASS = <StoreBackend subclass>` is
discovered automatically when `--store <store>` is requested.
"""

from __future__ import annotations

import importlib
from pathlib import Path
from typing import Callable

from .backend import BackendUnavailableError, StoreBackend

BackendFactory = Callable[[], StoreBackend]

_REGISTERED_BACKENDS: dict[str, BackendFactory] = {}


def register_backend(name: str, factory: BackendFactory) -> None:
    """Explicitly register a backend factory under `name` (primarily for tests/plugins)."""
    key = name.strip().lower()
    if not key:
        raise ValueError("backend name must be non-empty")
    _REGISTERED_BACKENDS[key] = factory


def _backends_dir() -> Path:
    return Path(__file__).resolve().parent / "backends"


def _discover_module_names() -> list[str]:
    bdir = _backends_dir()
    if not bdir.is_dir():
        return []
    names: list[str] = []
    for path in sorted(bdir.glob("*.py")):
        if path.name.startswith("_"):
            continue
        names.append(path.stem.lower())
    return names


def _load_backend_from_module(name: str) -> BackendFactory | None:
    mod_name = f"port_acquire.backends.{name}"
    try:
        module = importlib.import_module(mod_name)
    except ModuleNotFoundError as exc:
        if exc.name == mod_name:
            return None
        raise

    factory = getattr(module, "create_backend", None)
    if callable(factory):
        return factory

    cls = getattr(module, "BACKEND_CLASS", None)
    if isinstance(cls, type) and issubclass(cls, StoreBackend):
        return cls

    for attr in vars(module).values():
        if (
            isinstance(attr, type)
            and issubclass(attr, StoreBackend)
            and attr is not StoreBackend
            and getattr(attr, "name", "").lower() == name
        ):
            return attr
    return None


def available_backends() -> list[str]:
    """Return sorted names of all registered and discoverable store backends."""
    names = set(_REGISTERED_BACKENDS.keys()) | set(_discover_module_names())
    return sorted(names)


def get_backend(name: str) -> StoreBackend:
    """Instantiate the store backend named `name`."""
    key = name.strip().lower()
    if key in _REGISTERED_BACKENDS:
        return _REGISTERED_BACKENDS[key]()

    factory = _load_backend_from_module(key)
    if factory is not None:
        return factory()

    avail = available_backends()
    avail_str = ", ".join(avail) if avail else "(none)"
    raise BackendUnavailableError(
        f"unknown store backend {name!r} (available: {avail_str})",
        details={"requested": name, "available": avail},
    )
