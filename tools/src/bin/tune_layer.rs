#![allow(clippy::unwrap_used, clippy::expect_used, clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::must_use_candidate, clippy::uninlined_format_args, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_lossless, clippy::redundant_field_names, clippy::trivially_copy_pass_by_ref, clippy::needless_pass_by_value, clippy::needless_lifetimes, clippy::elidable_lifetime_names, clippy::items_after_statements, clippy::unreadable_literal, clippy::single_match_else, clippy::manual_range_contains, clippy::many_single_char_names, clippy::ref_patterns, clippy::map_unwrap_or, clippy::extra_unused_lifetimes, clippy::collapsible_match, clippy::collapsible_else_if, clippy::cloned_instead_of_copied, clippy::if_not_else, clippy::nonminimal_bool, clippy::default_trait_access, clippy::len_zero, clippy::let_underscore_future, clippy::enum_glob_use, clippy::wildcard_imports, clippy::match_same_arms, clippy::too_many_lines, clippy::large_stack_arrays, clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_wrap, clippy::bool_to_int_with_if, clippy::redundant_closure_for_method_calls, clippy::redundant_closure, clippy::used_underscore_binding, clippy::manual_let_else, clippy::explicit_iter_loop, clippy::explicit_into_iter_loop, clippy::match_bool, clippy::useless_conversion, clippy::semicolon_if_nothing_returned, clippy::print_literal, clippy::println_empty_string, clippy::needless_borrow, clippy::collapsible_if, clippy::range_plus_one, clippy::return_self_not_must_use, clippy::needless_late_init, clippy::unnecessary_wraps, clippy::redundant_static_lifetimes)]
#![cfg(not(test))]

use libtw2_common::num::Cast;
use libtw2_common::unwrap_or_return;
use libtw2_datafile as df;
use std::fmt;
use std::path::Path;

#[derive(Clone, Copy)]
struct Entity(u8);

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Entity(inner) = *self;
        write!(f, "{:4x}", inner)
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct Stats {
    tune_layers: u64,
    tiles: [u64; 256],
}

impl Default for Stats {
    fn default() -> Stats {
        Stats {
            tune_layers: 0,
            tiles: [0; 256],
        }
    }
}

fn process(path: &Path, dfr: df::Reader, stats: &mut Stats) -> Result<(), libtw2_map::Error> {
    let mut map = libtw2_map::Reader::from_datafile(dfr);
    let game_layers = map.game_layers()?;
    let tune_layer = unwrap_or_return!(game_layers.tune(), Ok(()));
    let tiles = map.tune_layer_tiles(tune_layer)?;

    stats.tune_layers += 1;
    let mut tiles_count = [0u64; 256];
    for tile in tiles.iter() {
        tiles_count[tile.index.usize()] += 1;
        stats.tiles[tile.index.usize()] += 1;
    }
    println!("{}", path.to_string_lossy());
    for (i, &c) in tiles_count.iter().enumerate() {
        let entity = Entity(i.assert_u8());
        if c != 0 {
            println!("{}: {:5}", entity, c);
        }
    }
    Ok(())
}

fn print_stats(stats: &Stats) {
    for (i, &c) in stats.tiles.iter().enumerate() {
        let entity = Entity(i.assert_u8());
        if c != 0 {
            println!("{}: {:5}", entity, c);
        }
    }
    println!("total: {}", stats.tune_layers);
}

fn main() {
    libtw2_tools::map_stats::stats(process, print_stats);
}
