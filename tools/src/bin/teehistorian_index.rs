#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::uninlined_format_args,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::redundant_field_names,
    clippy::trivially_copy_pass_by_ref,
    clippy::needless_pass_by_value,
    clippy::needless_lifetimes,
    clippy::elidable_lifetime_names,
    clippy::items_after_statements,
    clippy::unreadable_literal,
    clippy::single_match_else,
    clippy::manual_range_contains,
    clippy::many_single_char_names,
    clippy::ref_patterns,
    clippy::map_unwrap_or,
    clippy::extra_unused_lifetimes,
    clippy::collapsible_match,
    clippy::collapsible_else_if,
    clippy::cloned_instead_of_copied,
    clippy::if_not_else,
    clippy::nonminimal_bool,
    clippy::default_trait_access,
    clippy::len_zero,
    clippy::let_underscore_future,
    clippy::enum_glob_use,
    clippy::wildcard_imports,
    clippy::match_same_arms,
    clippy::too_many_lines,
    clippy::large_stack_arrays,
    clippy::similar_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::bool_to_int_with_if,
    clippy::redundant_closure_for_method_calls,
    clippy::redundant_closure,
    clippy::used_underscore_binding,
    clippy::manual_let_else,
    clippy::explicit_iter_loop,
    clippy::explicit_into_iter_loop,
    clippy::match_bool,
    clippy::useless_conversion,
    clippy::semicolon_if_nothing_returned,
    clippy::print_literal,
    clippy::println_empty_string,
    clippy::needless_borrow,
    clippy::collapsible_if,
    clippy::range_plus_one,
    clippy::return_self_not_must_use,
    clippy::needless_late_init,
    clippy::unnecessary_wraps,
    clippy::redundant_static_lifetimes
)]
use chrono::DateTime;
use chrono::FixedOffset;
use itertools::sorted;
use itertools::Itertools;
use libtw2_teehistorian::format;
use libtw2_teehistorian::Buffer;
use libtw2_teehistorian::Reader;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::slice;
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct HexU32(u32);

impl serde::Serialize for HexU32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:08x}", self.0))
    }
}

struct HexU32Visitor;

impl<'de> serde::de::Visitor<'de> for HexU32Visitor {
    type Value = HexU32;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("8 character hex value")
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<HexU32, E> {
        let len = v.chars().count();
        if len != 8 {
            return Err(E::invalid_length(len, &self));
        }
        let value = u32::from_str_radix(v, 16)
            .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(v), &self))?;
        Ok(HexU32(value))
    }
}

impl<'de> serde::Deserialize<'de> for HexU32 {
    fn deserialize<D>(deserializer: D) -> Result<HexU32, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(HexU32Visitor)
    }
}

impl From<u32> for HexU32 {
    fn from(i: u32) -> HexU32 {
        HexU32(i)
    }
}

#[derive(Debug, Serialize)]
struct Record<'a> {
    path: &'a Path,
    game_uuid: Uuid,
    timestamp: DateTime<FixedOffset>,
    server_port: u16,
    map_name: Cow<'a, str>,
    map_crc: HexU32,
    map_size: u32,
}

impl<'a> From<&'a ReadRecord> for Record<'a> {
    fn from(r: &'a ReadRecord) -> Record<'a> {
        Record {
            path: &r.path,
            game_uuid: r.game_uuid,
            timestamp: r.timestamp,
            server_port: r.server_port,
            map_name: Cow::from(&r.map_name[..]),
            map_crc: r.map_crc.into(),
            map_size: r.map_size,
        }
    }
}

fn contains<'a>(
    base: &mut slice::Iter<'a, ReadRecord>,
    writer: &mut csv::Writer<Box<dyn Write>>,
    path: &Path,
) -> Result<bool, Error> {
    let mut found = false;
    for record in base.peeking_take_while(|r| r.path <= path) {
        if record.path == path {
            found = true;
        }
        writer.serialize(Record::from(record))?;
    }
    Ok(found)
}

struct Config {
    ignore_ext: bool,
}

fn handle_dir<'a>(
    base: &mut slice::Iter<'a, ReadRecord>,
    writer: &mut csv::Writer<Box<dyn Write>>,
    dir: &Path,
    config: &Config,
) -> Result<(), ()> {
    fn helper<'a>(
        base: &mut slice::Iter<'a, ReadRecord>,
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
            if contains(base, writer, entry.path())? {
                continue;
            }
            buffer.clear();
            match Reader::open(entry.path(), &mut buffer) {
                Ok((header, _)) => {
                    writer.serialize(Record {
                        path: entry.path(),
                        game_uuid: header.game_uuid,
                        timestamp: header.timestamp,
                        server_port: header.server_port,
                        map_name: header.map_name,
                        map_crc: header.map_crc.into(),
                        map_size: header.map_size,
                    })?;
                }
                Err(e) => {
                    eprintln!("{}: {:?}", entry.path().display(), e);
                }
            }
        }
        Ok(())
    }
    helper(base, writer, dir, config).map_err(|e| eprintln!("{}: {:?}", dir.display(), e))
}

// Why do I need a separate one for this. :(
//
// `Ord` is implemented using mainly `path`.
#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
struct ReadRecord {
    path: PathBuf,
    game_uuid: Uuid,
    timestamp: DateTime<FixedOffset>,
    server_port: u16,
    map_name: String,
    map_crc: HexU32,
    map_size: u32,
}

fn read_index(path: &Path) -> Result<Vec<ReadRecord>, Error> {
    fn read(path: &Path) -> Result<Vec<ReadRecord>, Error> {
        csv::Reader::from_path(path)?
            .into_deserialize()
            .map(|r| r.map_err(|e| e.into()))
            .collect()
    }
    read(path).map(|mut v| {
        v.sort();
        v
    })
}

fn handle_args(
    base: Option<&Path>,
    output: Option<&Path>,
    dirs: Vec<PathBuf>,
    config: &Config,
) -> Result<(), ()> {
    let base = base
        .map(|b| read_index(b).map_err(|e| eprintln!("{}: {:?}", b.display(), e)))
        .transpose()?
        .unwrap_or(Vec::new());

    let mut base_iter = base.iter();

    let mut csv_out: csv::Writer<Box<dyn Write>> = csv::Writer::from_writer(match output {
        // `csv::Writer::from_writer` already uses a `BufWriter`
        Some(o) => Box::new(File::create(o).map_err(|e| eprintln!("{}: {:?}", o.display(), e))?),
        None => Box::new(io::stdout()),
    });

    for dir in dirs {
        handle_dir(&mut base_iter, &mut csv_out, &dir, config)?;
    }

    for record in base_iter {
        csv_out
            .serialize(Record::from(record))
            .map_err(|e| eprintln!("{:?}", e))?;
    }

    Ok(())
}

fn main() {
    use clap::App;
    use clap::Arg;

    libtw2_logger::init();

    let matches = App::new("Teehistorian indexer")
        .about(
            "Indexes folders of teehistorian files and dumps the index into \
                a CSV file",
        )
        .arg(
            Arg::with_name("base")
                .short("b")
                .long("base")
                .value_name("BASE")
                .help("Sets a base index file"),
        )
        .arg(
            Arg::with_name("inplace")
                .short("i")
                .long("in-place")
                .value_name("INDEX")
                .help("Sets the index file to update")
                .conflicts_with("base"),
        )
        .arg(
            Arg::with_name("DIRECTORY")
                .help("Directories to scan (current directory if none are given)")
                .multiple(true),
        )
        .arg(Arg::with_name("ignore-ext").long("--ignore-ext").help(
            "Don't check for the .teehistorian file extension before \
                   indexing a file",
        ))
        .get_matches();

    let paths = matches.values_of_os("DIRECTORY");
    let base = matches.value_of_os("base");
    let inplace = matches.value_of_os("inplace");
    let (input, output) = match (base, inplace) {
        (None, None) => (None, None),
        (Some(b), None) => (Some(Path::new(b)), None),
        (None, Some(i)) => (Some(Path::new(i)), Some(Path::new(i))),
        (Some(_), Some(_)) => unreachable!(),
    };
    let config = Config {
        ignore_ext: matches.is_present("ignore-ext"),
    };

    let dirs = if let Some(p) = paths {
        sorted(p.into_iter().map(|p| PathBuf::from(p)))
    } else {
        vec![PathBuf::from(".")]
    };

    if handle_args(input, output, dirs, &config).is_err() {
        process::exit(1);
    }
}
