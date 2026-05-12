#![allow(clippy::unwrap_used, clippy::expect_used, clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::must_use_candidate, clippy::uninlined_format_args, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_lossless, clippy::redundant_field_names, clippy::trivially_copy_pass_by_ref, clippy::needless_pass_by_value, clippy::needless_lifetimes, clippy::elidable_lifetime_names, clippy::items_after_statements, clippy::unreadable_literal, clippy::single_match_else, clippy::manual_range_contains, clippy::many_single_char_names, clippy::ref_patterns, clippy::map_unwrap_or, clippy::extra_unused_lifetimes, clippy::collapsible_match, clippy::collapsible_else_if, clippy::cloned_instead_of_copied, clippy::if_not_else, clippy::nonminimal_bool, clippy::default_trait_access, clippy::len_zero, clippy::let_underscore_future, clippy::enum_glob_use, clippy::wildcard_imports, clippy::match_same_arms, clippy::too_many_lines, clippy::large_stack_arrays, clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_wrap, clippy::bool_to_int_with_if, clippy::redundant_closure_for_method_calls, clippy::redundant_closure, clippy::used_underscore_binding, clippy::manual_let_else, clippy::explicit_iter_loop, clippy::explicit_into_iter_loop, clippy::match_bool, clippy::useless_conversion, clippy::semicolon_if_nothing_returned, clippy::print_literal, clippy::println_empty_string, clippy::needless_borrow, clippy::collapsible_if, clippy::range_plus_one, clippy::return_self_not_must_use, clippy::needless_late_init, clippy::unnecessary_wraps, clippy::redundant_static_lifetimes)]
extern crate libtw2_gamenet_teeworlds_0_6 as libtw2_gamenet;

use libtw2_demo::RawChunk;
use libtw2_gamenet::msg::Game;
use libtw2_warn as warn;
use libtw2_warn::Warn;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
enum Error {
    DemoRead(libtw2_demo::ReadError),
    DemoWrite(libtw2_demo::WriteError),
    Io(io::Error),
    Gamenet(libtw2_gamenet::Error),
}

#[derive(Debug)]
enum Warning {
    Demo(libtw2_demo::Warning),
    Gamenet(libtw2_packer::Warning),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<libtw2_demo::ReadError> for Error {
    fn from(err: libtw2_demo::ReadError) -> Self {
        match err.io_error() {
            Ok(io) => Error::Io(io),
            Err(demo) => Error::DemoRead(demo),
        }
    }
}

impl From<libtw2_demo::WriteError> for Error {
    fn from(err: libtw2_demo::WriteError) -> Error {
        match err.io_error() {
            Ok(io) => Error::Io(io),
            Err(demo) => Error::DemoWrite(demo),
        }
    }
}

impl From<libtw2_gamenet::Error> for Error {
    fn from(e: libtw2_gamenet::Error) -> Error {
        Error::Gamenet(e)
    }
}

impl From<libtw2_demo::Warning> for Warning {
    fn from(w: libtw2_demo::Warning) -> Warning {
        Warning::Demo(w)
    }
}

impl From<libtw2_packer::Warning> for Warning {
    fn from(w: libtw2_packer::Warning) -> Warning {
        Warning::Gamenet(w)
    }
}

#[derive(Default)]
struct ErrorStats {
    demo_warnings: HashMap<libtw2_demo::Warning, u64>,
    demo_read_errors: Vec<libtw2_demo::ReadError>,
    demo_write_errors: Vec<libtw2_demo::WriteError>,
    gamenet_warnings: HashMap<libtw2_packer::Warning, u64>,
    gamenet_errors: HashMap<libtw2_gamenet::Error, u64>,
    io_errors: Vec<io::Error>,
    ok: u64,
}

fn update_warning_stats(stats: &mut ErrorStats, warning: Warning) {
    match warning {
        Warning::Demo(w) => *stats.demo_warnings.entry(w).or_insert(0) += 1,
        Warning::Gamenet(w) => *stats.gamenet_warnings.entry(w).or_insert(0) += 1,
    }
}

fn update_error_stats(stats: &mut ErrorStats, err: Error) {
    match err {
        Error::DemoRead(e) => stats.demo_read_errors.push(e),
        Error::DemoWrite(e) => stats.demo_write_errors.push(e),
        Error::Gamenet(e) => *stats.gamenet_errors.entry(e).or_insert(0) += 1,
        Error::Io(e) => stats.io_errors.push(e),
    }
}

fn print_error_stats(error_stats: &ErrorStats) {
    for e in &error_stats.demo_read_errors {
        println!("{}", e);
    }
    for e in &error_stats.demo_write_errors {
        println!("{}", e);
    }
    for (w, c) in &error_stats.demo_warnings {
        println!("{:?}: {}", w, c);
    }
    for (e, c) in &error_stats.gamenet_errors {
        println!("{:?}: {}", e, c);
    }
    for (w, c) in &error_stats.gamenet_warnings {
        println!("{:?}: {}", w, c);
    }
    for e in &error_stats.io_errors {
        println!("{:?}", e);
    }
    println!("ok: {}", error_stats.ok);
}

fn process<W: Warn<Warning>>(warn: &mut W, path: &Path) -> Result<(), Error> {
    let file = fs::File::open(path)?;
    let mut reader = libtw2_demo::Reader::new(file, warn::wrap(warn))?;
    println!("{}", path.display());
    println!("version: {:?}", reader.version());
    println!(
        "net_version: {}",
        String::from_utf8_lossy(reader.net_version())
    );
    println!("map_name: {}", String::from_utf8_lossy(reader.map_name()));
    println!("map_size: {}", reader.map_size());
    println!("map_crc: {:x}", reader.map_crc());
    println!("timestamp: {}", String::from_utf8_lossy(reader.timestamp()));
    while let Some(chunk) = reader.read_chunk(warn::wrap(warn))? {
        match chunk {
            RawChunk::Message(bytes) => {
                let mut u = libtw2_packer::Unpacker::new_from_demo(bytes);
                println!("message {:?}", Game::decode(warn::wrap(warn), &mut u)?);
            }
            RawChunk::Tick { tick, .. } => println!("tick={}", tick),
            RawChunk::Snapshot(_) => println!("snapshot"),
            RawChunk::SnapshotDelta(_) => println!("snapshot_delta"),
            RawChunk::Unknown => println!("Unknown chunk"),
        }
    }
    println!();
    Ok(())
}

fn main() {
    libtw2_logger::init();

    let mut args = env::args_os();
    let mut have_args = false;
    let program_name = args.next().unwrap();

    let mut error_stats = ErrorStats::default();
    for arg in args {
        have_args = true;
        let path = Path::new(&arg);
        match process(
            warn::closure(&mut |w| {
                println!("{}: {:?}", path.display(), w);
                update_warning_stats(&mut error_stats, w);
            }),
            path,
        ) {
            Ok(()) => error_stats.ok += 1,
            Err(err) => {
                println!("{}: {:?}", path.display(), err);
                update_error_stats(&mut error_stats, err);
            }
        }
    }
    if !have_args {
        println!("USAGE: {} <DEMO>...", program_name.to_string_lossy());
        return;
    }
    print_error_stats(&error_stats);
}
