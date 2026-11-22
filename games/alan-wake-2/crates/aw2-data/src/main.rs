//! Small CLI for inspecting and repacking Northlight game data:
//! `cargo data status | ls <prefix> | cat <path> <out> | repack --src <DIR> --dst <DIR> [...]`.
use aw2_data::repack::{repack_game_archives, RepackOptions};
use aw2_data::{discover_game_dir, NorthlightVfs};
use std::path::PathBuf;

fn cmd_repack(args: &[String]) {
    let mut opts = RepackOptions::default();
    opts.keep_lang = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--src" => {
                i += 1;
                opts.src_game_dir = PathBuf::from(args.get(i).expect("--src requires DIR"));
            }
            "--dst" | "--out" => {
                i += 1;
                opts.dst_game_dir = PathBuf::from(args.get(i).expect("--dst requires DIR"));
            }
            "--hevc-videos" => opts.hevc_videos = true,
            "--video-max-height" => {
                i += 1;
                opts.video_max_height = args
                    .get(i)
                    .expect("--video-max-height requires N")
                    .parse()
                    .expect("invalid --video-max-height");
            }
            "--video-bitrate" => {
                i += 1;
                opts.video_bitrate_kbps = args
                    .get(i)
                    .expect("--video-bitrate requires KBPS")
                    .parse()
                    .expect("invalid --video-bitrate");
            }
            "--video-gop" => {
                i += 1;
                opts.video_gop = args
                    .get(i)
                    .expect("--video-gop requires N")
                    .parse()
                    .expect("invalid --video-gop");
            }
            "--max-tex-dim" => {
                i += 1;
                opts.max_tex_dim = args
                    .get(i)
                    .expect("--max-tex-dim requires N")
                    .parse()
                    .expect("invalid --max-tex-dim");
            }
            "--keep-lang" => {
                i += 1;
                opts.keep_lang = Some(args.get(i).expect("--keep-lang requires CODE").clone());
            }
            "--strip-dlc" => opts.strip_dlc = true,
            "--jobs" | "-j" => {
                i += 1;
                opts.jobs = args
                    .get(i)
                    .expect("--jobs requires N")
                    .parse()
                    .expect("invalid --jobs");
            }
            "--only-toc" => {
                i += 1;
                opts.only_toc = Some(args.get(i).expect("--only-toc requires FILE").clone());
            }
            "--dry-run" => opts.dry_run = true,
            other => {
                eprintln!("unknown repack flag: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    if opts.src_game_dir.as_os_str().is_empty() {
        opts.src_game_dir = std::env::var("AW2_GAME_DIR")
            .map(PathBuf::from)
            .ok()
            .or_else(discover_game_dir)
            .expect("pass --src <GAME_DIR> or set AW2_GAME_DIR");
    }
    if opts.dst_game_dir.as_os_str().is_empty() && !opts.dry_run {
        eprintln!("repack requires --dst <OUT_GAME_DIR>");
        std::process::exit(2);
    }

    match repack_game_archives(&opts) {
        Ok(s) => {
            println!("=== Northlight Archive Repack Summary ===");
            println!("  TOCs Repacked      : {}", s.tocs_repacked);
            if s.tocs_skipped_lang > 0 {
                println!(
                    "  Lang Packs Skipped : {} (-{:.2} GB)",
                    s.tocs_skipped_lang,
                    s.lang_bytes_saved as f64 / 1e9
                );
            }
            if s.videos_transcoded > 0 {
                println!(
                    "  Videos -> KB2v HEVC: {} ({:.2} GB -> {:.2} GB)",
                    s.videos_transcoded,
                    s.video_bytes_before as f64 / 1e9,
                    s.video_bytes_after as f64 / 1e9
                );
            }
            if s.textures_capped > 0 {
                println!(
                    "  DDS Mips Capped    : {} ({:.2} GB -> {:.2} GB)",
                    s.textures_capped,
                    s.texture_bytes_before as f64 / 1e9,
                    s.texture_bytes_after as f64 / 1e9
                );
            }
            if !opts.dry_run {
                let saved = s.total_blob_bytes_before.saturating_sub(s.total_blob_bytes_after);
                println!(
                    "  Repacked Blobs     : {:.2} GB -> {:.2} GB (saved {:.2} GB)",
                    s.total_blob_bytes_before as f64 / 1e9,
                    s.total_blob_bytes_after as f64 / 1e9,
                    saved as f64 / 1e9
                );
            }
        }
        Err(e) => {
            eprintln!("aw2-data repack failed: {e}");
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("repack") {
        cmd_repack(&args[1..]);
        return;
    }
    let game = std::env::var("AW2_GAME_DIR")
        .map(PathBuf::from)
        .ok()
        .or_else(discover_game_dir)
        .expect("set AW2_GAME_DIR to your Alan Wake 2 install");
    let vfs = NorthlightVfs::mount(&game).expect("mount VFS");
    match args.first().map(String::as_str) {
        Some("ls") => {
            let prefix = args.get(1).map(String::as_str).unwrap_or("");
            for toc in &vfs.archives {
                for e in &toc.files {
                    if e.path.starts_with(prefix) {
                        println!("{:>12}  {}", e.file_size, e.path);
                    }
                }
            }
        }
        Some("cat") => {
            let path = args.get(1).expect("cat <virtual path> <out file>");
            let out = args.get(2).expect("cat <virtual path> <out file>");
            let bytes = vfs.read_virtual_file(path).expect("read");
            std::fs::write(out, bytes).expect("write");
        }
        _ => {
            println!("game dir      : {}", game.display());
            println!("archives      : {}", vfs.total_blobs());
            println!("virtual files : {}", vfs.total_virtual_files());
            println!("shader bundles: {}", vfs.shader_paths.len());
        }
    }
}
