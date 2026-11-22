//! Vorbis I packet decoder: a fork of lewton 0.10.2 (`audio`, `bitpacking`,
//! `header`, `header_cached`, `huffman_tree`, `imdct`, `samples`).
//!
//! Copyright (c) 2016 est31 <MTest31@outlook.com> and contributors.
//! Licensed under the MIT license or the Apache License 2.0, at your option
//! (same terms as upstream; see https://github.com/RustAudio/lewton).
//!
//! Why a fork: Wwise's (AK) Vorbis decoder does not use libvorbis' floor1
//! synthesis (integer Bresenham line + `FLOOR1_fromdB_LOOKUP`). It
//! interpolates the floor continuously in float and evaluates the dB->linear
//! curve with a rational fast-exp2 in float-bit space (`rcpps`), see
//! [`audio::FloorSynthesis`]. lewton keeps the floor private, so it lives here.
//!
//! Local changes vs upstream: edition 2021 (`try!` -> `?`, paths), no
//! `byteorder`/`tinyvec` dependencies, debug record macros removed, the
//! floor1 synthesis selectable (`FloorSynthesis`).

#![allow(clippy::all)]
#![allow(dead_code, unused_parens, unused_imports, unused_mut, unused_variables, unused_assignments)]
#![allow(non_snake_case, non_upper_case_globals, bare_trait_objects, deprecated, mismatched_lifetime_syntaxes)]

pub mod audio;
pub mod ak_floor;
mod bitpacking;
pub mod header;
mod header_cached;
mod huffman_tree;
mod imdct;
#[cfg(test)]
mod imdct_test;
pub mod samples;

fn ilog(val: u64) -> u8 {
	64 - val.leading_zeros() as u8
}

#[test]
fn test_ilog() {
	// Test vectors from the Vorbis I spec.
	assert_eq!(ilog(0), 0);
	assert_eq!(ilog(1), 1);
	assert_eq!(ilog(2), 2);
	assert_eq!(ilog(3), 2);
	assert_eq!(ilog(4), 3);
	assert_eq!(ilog(7), 3);
}

fn bit_reverse(n: u32) -> u32 {
	n.reverse_bits()
}
