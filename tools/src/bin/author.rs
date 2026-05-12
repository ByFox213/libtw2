#![allow(clippy::unwrap_used, clippy::expect_used, clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::must_use_candidate, clippy::uninlined_format_args, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_lossless, clippy::redundant_field_names, clippy::trivially_copy_pass_by_ref, clippy::needless_pass_by_value, clippy::needless_lifetimes, clippy::elidable_lifetime_names, clippy::items_after_statements, clippy::unreadable_literal, clippy::single_match_else, clippy::manual_range_contains, clippy::many_single_char_names, clippy::ref_patterns, clippy::map_unwrap_or, clippy::extra_unused_lifetimes, clippy::collapsible_match, clippy::collapsible_else_if, clippy::cloned_instead_of_copied, clippy::if_not_else, clippy::nonminimal_bool, clippy::default_trait_access, clippy::len_zero, clippy::let_underscore_future, clippy::enum_glob_use, clippy::wildcard_imports, clippy::match_same_arms, clippy::too_many_lines, clippy::large_stack_arrays, clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_wrap, clippy::bool_to_int_with_if, clippy::redundant_closure_for_method_calls, clippy::redundant_closure, clippy::used_underscore_binding, clippy::manual_let_else, clippy::explicit_iter_loop, clippy::explicit_into_iter_loop, clippy::match_bool, clippy::useless_conversion, clippy::semicolon_if_nothing_returned, clippy::print_literal, clippy::println_empty_string, clippy::needless_borrow, clippy::collapsible_if, clippy::range_plus_one, clippy::return_self_not_must_use, clippy::needless_late_init, clippy::unnecessary_wraps, clippy::redundant_static_lifetimes)]
#![cfg(not(test))]

use libtw2_datafile as df;
use libtw2_map::format;
use std::path::Path;

#[derive(Default)]
struct Stats {
    author: u64,
    version: u64,
    credits: u64,
    license: u64,
    settings: u64,
    info: u64,
    total: u64,
}

fn process(_: &Path, dfr: df::Reader, stats: &mut Stats) -> Result<(), libtw2_map::Error> {
    let map = libtw2_map::Reader::from_datafile(dfr);
    let info = match map.info() {
        Ok(i) => i,
        Err(format::Error::MissingInfo) => {
            stats.total += 1;
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };
    if info.author.is_some() {
        stats.author += 1;
    }
    if info.version.is_some() {
        stats.version += 1;
    }
    if info.credits.is_some() {
        stats.credits += 1;
    }
    if info.license.is_some() {
        stats.license += 1;
    }
    if info.settings.is_some() {
        stats.settings += 1;
    }
    stats.info += 1;
    stats.total += 1;
    Ok(())
}

fn print_stats(stats: &Stats) {
    println!("author:   {:5}", stats.author);
    println!("version:  {:5}", stats.version);
    println!("credits:  {:5}", stats.credits);
    println!("license:  {:5}", stats.license);
    println!("settings: {:5}", stats.settings);
    println!("info:     {:5}", stats.info);
    println!("total:    {:5}", stats.total);
}

fn main() {
    libtw2_tools::map_stats::stats(process, print_stats);
}
