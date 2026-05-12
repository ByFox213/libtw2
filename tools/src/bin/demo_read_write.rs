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
use clap::App;
use clap::Arg;
use libtw2_demo::ddnet;
use libtw2_gamenet_ddnet::Protocol as DDNet;
use libtw2_warn as warn;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::process;

fn main() {
    libtw2_logger::init();
    let matches = App::new("Teehistorian reader")
        .about(
            "Reads teehistorian file and dumps its contents in a human-readable\
                text stream",
        )
        .arg(
            Arg::with_name("INPUT_DEMO")
                .help("Sets the demo file to read")
                .required(true),
        )
        .arg(
            Arg::with_name("OUTPUT_DEMO")
                .help("Sets the path to write to")
                .required(true),
        )
        .arg(
            Arg::with_name("DDNET")
                .long("ddnet")
                .help("Interpret the demo as a DDNet demo"),
        )
        .get_matches();

    let input = matches.value_of("INPUT_DEMO").unwrap();
    let output = matches.value_of("OUTPUT_DEMO").unwrap();
    let as_ddnet = matches.is_present("DDNET");
    let rewrite = match as_ddnet {
        true => ddnet_read_write,
        false => read_write,
    };
    if let Err(err) = rewrite(input, output) {
        println!("Error: {}", err);
        process::exit(-1);
    }
}

fn read_write(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let input_file = BufReader::new(File::open(input)?);
    let output_file = BufWriter::new(File::create(output)?);
    let mut reader = libtw2_demo::Reader::new(input_file, &mut warn::Ignore)?;
    let mut writer = libtw2_demo::Writer::new(
        output_file,
        reader.net_version(),
        reader.map_name(),
        reader.map_sha256(),
        reader.map_crc(),
        reader.kind(),
        reader.length(),
        reader.timestamp(),
        reader.map_data(),
    )?;
    while let Some(chunk) = reader.read_chunk(&mut warn::Ignore)? {
        writer.write_chunk(chunk)?;
    }
    Ok(())
}

fn ddnet_read_write(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let input_file = BufReader::new(File::open(input)?);
    let output_file = BufWriter::new(File::create(output)?);
    let mut reader = ddnet::DemoReader::<DDNet>::new(input_file, &mut warn::Log)?;
    let mut writer = ddnet::DemoWriter::<DDNet>::new(
        output_file,
        reader.net_version(),
        reader.map_name(),
        reader.map_sha256(),
        reader.map_crc(),
        reader.kind(),
        reader.length(),
        reader.timestamp(),
        reader.map_data(),
    )?;
    let mut last_tick = None;
    while let Some(chunk) = reader.next_chunk(&mut warn::Log)? {
        match chunk {
            ddnet::Chunk::Message(msg) => writer.write_msg(&msg)?,
            ddnet::Chunk::Snapshot(snap) => match last_tick.take() {
                None => eprintln!("Snapshot without tick"),
                Some(t) => writer.write_snap(t, snap.map(|(obj, id)| (obj, *id)))?,
            },
            ddnet::Chunk::Tick(t) => last_tick = Some(t),
            ddnet::Chunk::Invalid => eprintln!("Invalid chunk!"),
        }
    }
    Ok(())
}
