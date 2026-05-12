#![allow(clippy::unwrap_used, clippy::expect_used, clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::must_use_candidate, clippy::uninlined_format_args, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_lossless, clippy::redundant_field_names, clippy::trivially_copy_pass_by_ref, clippy::needless_pass_by_value, clippy::needless_lifetimes, clippy::elidable_lifetime_names, clippy::items_after_statements, clippy::unreadable_literal, clippy::single_match_else, clippy::manual_range_contains, clippy::many_single_char_names, clippy::ref_patterns, clippy::map_unwrap_or, clippy::extra_unused_lifetimes, clippy::collapsible_match, clippy::collapsible_else_if, clippy::cloned_instead_of_copied, clippy::if_not_else, clippy::nonminimal_bool, clippy::default_trait_access, clippy::len_zero, clippy::let_underscore_future, clippy::enum_glob_use, clippy::wildcard_imports, clippy::match_same_arms, clippy::too_many_lines, clippy::large_stack_arrays, clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_wrap, clippy::bool_to_int_with_if, clippy::redundant_closure_for_method_calls, clippy::redundant_closure, clippy::used_underscore_binding, clippy::manual_let_else, clippy::explicit_iter_loop, clippy::explicit_into_iter_loop, clippy::match_bool, clippy::useless_conversion, clippy::semicolon_if_nothing_returned, clippy::print_literal, clippy::println_empty_string, clippy::needless_borrow, clippy::collapsible_if, clippy::range_plus_one, clippy::return_self_not_must_use, clippy::needless_late_init, clippy::unnecessary_wraps, clippy::redundant_static_lifetimes)]
use libtw2_teehistorian::Buffer;
use libtw2_teehistorian::Error;
use libtw2_teehistorian::Item;
use libtw2_teehistorian::Reader;
use serde_derive::Serialize;
use std::io;
use std::path::Path;
use std::process;

#[derive(Serialize)]
struct TickAndItem<'a> {
    tick: i32,
    item: Item<'a>,
}

fn process(path: &Path, json: bool) -> Result<(), Error> {
    let mut buffer = Buffer::new();
    let (_, mut reader) = Reader::open(path, &mut buffer)?;
    let mut tick = None;
    if json {
        println!("[");
    }
    let mut first = true;
    while let Some(item) = reader.read(&mut buffer)? {
        match item {
            Item::TickStart(t) => {
                assert!(tick.is_none());
                tick = Some(t);
            }
            Item::TickEnd(t) => {
                assert_eq!(tick, Some(t));
                tick = None;
            }
            _ => {
                if !first {
                    if json {
                        println!(",");
                    }
                } else {
                    first = false;
                }
                if json {
                    let stdout = io::stdout();
                    serde_json::to_writer(
                        stdout.lock(),
                        &TickAndItem {
                            tick: tick.unwrap(),
                            item: item,
                        },
                    )
                    .unwrap();
                } else {
                    println!("{} {:?}", tick.expect("in tick"), item);
                }
            }
        }
    }
    assert!(tick.is_none());
    if json {
        println!();
        println!("]");
    }
    Ok(())
}

fn main() {
    use clap::App;
    use clap::Arg;

    libtw2_logger::init();

    let matches = App::new("Teehistorian reader")
        .about(
            "Reads teehistorian file and dumps its contents in a human-readable\
                text stream",
        )
        .arg(
            Arg::with_name("TEEHISTORIAN")
                .help("Sets the teehistorian file to dump")
                .required(true),
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .help("Output machine-readable JSON"),
        )
        .get_matches();

    let path = Path::new(matches.value_of_os("TEEHISTORIAN").unwrap());
    let json = matches.is_present("json");

    match process(path, json) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}: {:?}", path.display(), err);
            process::exit(1);
        }
    }
}
