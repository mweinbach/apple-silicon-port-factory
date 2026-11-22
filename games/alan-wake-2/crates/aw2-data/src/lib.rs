//! Northlight on-disk data formats.
//!
//! Ported from the legacy prototype (`legacy/prototype/src/{formats,runtime}.rs`).
//! These parsers were written from format inspection, not from decompiled code.
//! As the corresponding engine functions are decompiled (see `db/progress.tsv`),
//! annotate each parser with the original function VA it mirrors and add
//! differential tests against the hooked original.

pub mod formats;
pub mod repack;
pub mod vfs;

pub use formats::{BinRfx, RmdToc, TocFileEntry};
pub use vfs::{discover_game_dir, DdsTexture, NorthlightVfs};
