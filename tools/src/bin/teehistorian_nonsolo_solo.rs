#![allow(clippy::unwrap_used, clippy::expect_used, clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::must_use_candidate, clippy::uninlined_format_args, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_lossless, clippy::redundant_field_names, clippy::trivially_copy_pass_by_ref, clippy::needless_pass_by_value, clippy::needless_lifetimes, clippy::elidable_lifetime_names, clippy::items_after_statements, clippy::unreadable_literal, clippy::single_match_else, clippy::manual_range_contains, clippy::many_single_char_names, clippy::ref_patterns, clippy::map_unwrap_or, clippy::extra_unused_lifetimes, clippy::collapsible_match, clippy::collapsible_else_if, clippy::cloned_instead_of_copied, clippy::if_not_else, clippy::nonminimal_bool, clippy::default_trait_access, clippy::len_zero, clippy::let_underscore_future, clippy::enum_glob_use, clippy::wildcard_imports, clippy::match_same_arms, clippy::too_many_lines, clippy::large_stack_arrays, clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_wrap, clippy::bool_to_int_with_if, clippy::redundant_closure_for_method_calls, clippy::redundant_closure, clippy::used_underscore_binding, clippy::manual_let_else, clippy::explicit_iter_loop, clippy::explicit_into_iter_loop, clippy::match_bool, clippy::useless_conversion, clippy::semicolon_if_nothing_returned, clippy::print_literal, clippy::println_empty_string, clippy::needless_borrow, clippy::collapsible_if, clippy::range_plus_one, clippy::return_self_not_must_use, clippy::needless_late_init, clippy::unnecessary_wraps, clippy::redundant_static_lifetimes)]
use itertools::sorted;
use libtw2_teehistorian::format;
use libtw2_teehistorian::Buffer;
use libtw2_teehistorian::Reader;
use serde_derive::Serialize;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use uuid::Uuid;
use walkdir::WalkDir;

#[allow(dead_code)] // We add fields just for their `Debug` implementation.
#[derive(Debug)]
enum Error {
    Csv(csv::Error),
    Io(io::Error),
    Teehistorian(format::Error),
    WalkDir(walkdir::Error),
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Error {
        Error::Csv(e)
    }
}

impl From<libtw2_teehistorian::Error> for Error {
    fn from(e: libtw2_teehistorian::Error) -> Error {
        use libtw2_teehistorian::Error::*;
        match e {
            Teehistorian(i) => Error::Teehistorian(i),
            Io(i) => Error::Io(i),
        }
    }
}

impl From<walkdir::Error> for Error {
    fn from(e: walkdir::Error) -> Error {
        Error::WalkDir(e)
    }
}

#[derive(Debug, Serialize)]
struct Record<'a> {
    path: &'a Path,
    game_uuid: Uuid,
    map_name: Cow<'a, str>,
    reset_file: Cow<'a, str>,
}

struct Config {
    ignore_ext: bool,
}

fn handle_dir<'a>(
    writer: &mut csv::Writer<Box<dyn Write>>,
    dir: &Path,
    config: &Config,
) -> Result<(), ()> {
    fn helper<'a>(
        writer: &mut csv::Writer<Box<dyn Write>>,
        dir: &Path,
        config: &Config,
    ) -> Result<(), Error> {
        let mut buffer = Buffer::new();
        for entry in WalkDir::new(dir).sort_by(|a, b| a.file_name().cmp(b.file_name())) {
            let entry = entry?;
            if !config.ignore_ext && entry.path().extension() != Some(OsStr::new("teehistorian")) {
                continue;
            }
            if entry.file_type().is_dir() {
                continue;
            }
            buffer.clear();
            match Reader::open(entry.path(), &mut buffer) {
                Ok((mut header, _)) => {
                    writer.serialize(Record {
                        path: entry.path(),
                        game_uuid: header.game_uuid,
                        map_name: header.map_name,
                        reset_file: header
                            .config
                            .remove("sv_reset_file")
                            .unwrap_or(Default::default()),
                    })?;
                }
                Err(e) => {
                    eprintln!("{}: {:?}", entry.path().display(), e);
                }
            }
        }
        Ok(())
    }
    helper(writer, dir, config).map_err(|e| eprintln!("{}: {:?}", dir.display(), e))
}

fn handle_args(dirs: Vec<PathBuf>, config: &Config) -> Result<(), ()> {
    let mut csv_out: csv::Writer<Box<dyn Write>> = csv::Writer::from_writer(Box::new(io::stdout()));

    for dir in dirs {
        handle_dir(&mut csv_out, &dir, config)?;
    }

    Ok(())
}

fn main() {
    use clap::App;
    use clap::Arg;

    libtw2_logger::init();

    let matches = App::new("Teehistorian indexer")
        .about(
            "Checks folders of teehistorian files for solo maps with \
                non-solo flexreset files",
        )
        .arg(
            Arg::with_name("DIRECTORY")
                .help("Directories to scan (current directory if none are given)")
                .multiple(true),
        )
        .arg(Arg::with_name("ignore-ext").long("--ignore-ext").help(
            "Don't check for the .teehistorian file extension before \
                   checking a file",
        ))
        .get_matches();

    let paths = matches.values_of_os("DIRECTORY");
    let config = Config {
        ignore_ext: matches.is_present("ignore-ext"),
    };

    let dirs = if let Some(p) = paths {
        sorted(p.into_iter().map(|p| PathBuf::from(p)))
    } else {
        vec![PathBuf::from(".")]
    };

    if handle_args(dirs, &config).is_err() {
        process::exit(1);
    }
}
