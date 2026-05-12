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
#![cfg(not(test))]

use libtw2_datafile as df;
use libtw2_map::reader;
use std::collections::HashMap;
use std::path::Path;

fn process(
    _: &Path,
    dfr: df::Reader,
    tilesets: &mut HashMap<Vec<u8>, u64>,
) -> Result<(), libtw2_map::Error> {
    let mut map = libtw2_map::Reader::from_datafile(dfr);
    for i in map.group_indices() {
        let group = map.group(i)?;
        for k in group.layer_indices.clone() {
            let layer = map.layer(k)?;
            let image_index = if let Some(i) = match layer.t {
                reader::LayerType::Quads(q) => q.image,
                reader::LayerType::Tilemap(t) => t.type_.to_normal().and_then(|n| n.image),
                reader::LayerType::DdraceSounds(_) => continue,
            } {
                i
            } else {
                continue;
            };
            let image = map.image(image_index)?;
            let name = map.image_name(image.name)?;
            *tilesets.entry(name).or_insert(0) += 1;
        }
    }
    Ok(())
}

fn print_stats(tilesets: &HashMap<Vec<u8>, u64>) {
    for (name, &c) in tilesets.iter() {
        println!("{:14} {:5}", String::from_utf8_lossy(name), c);
    }
}

fn main() {
    libtw2_tools::map_stats::stats(process, print_stats);
}
