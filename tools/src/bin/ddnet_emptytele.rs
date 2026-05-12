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
use libtw2_map::Error;
use std::path::Path;
use std::process;

fn tele_tile_name(index: u8) -> Option<&'static str> {
    Some(match index {
        10 => "TELEINEVIL",
        14 => "TELEINWEAPON",
        15 => "TELEINHOOK",
        26 => "TELEIN",
        27 => "TELEOUT",
        29 => "TELECHECK",
        30 => "TELECHECKOUT",
        31 => "TELECHECKIN",
        63 => "TELECHECKINEVIL",
        _ => return None,
    })
}

fn process(path: &Path) -> Result<(), Error> {
    let mut map = libtw2_map::Reader::open(path)?;
    let game_layers = map.game_layers()?;

    let tele = if let Some(t) = game_layers.teleport() {
        t
    } else {
        return Ok(());
    };
    let tele_tiles = map.tele_layer_tiles(tele)?;
    for ((y, x), &t) in tele_tiles.indexed_iter() {
        if t.index != 0 && t.number == 0 {
            if let Some(name) = tele_tile_name(t.index) {
                println!("{}: {}: ({}, {})", path.display(), name, x, y);
            } else {
                println!("{}: unknown ({}): ({}, {})", path.display(), t.index, x, y);
            }
        }
    }
    Ok(())
}

fn main() {
    use clap::App;
    use clap::Arg;

    libtw2_logger::init();

    let matches = App::new("DDNet teleporter scanner")
        .about("Scans map files for weird teleporters.")
        .arg(
            Arg::with_name("MAP")
                .help("Sets the map file to analyse")
                .multiple(true)
                .required(true),
        )
        .get_matches();

    let maps = matches.values_of_os("MAP").unwrap();

    let mut error = false;
    for map in maps {
        let map = Path::new(map);
        match process(map) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("{}: {:?}", map.display(), err);
                error = true;
            }
        }
    }
    if error {
        process::exit(1);
    }
}
