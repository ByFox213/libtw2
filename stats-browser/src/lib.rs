#![cfg(not(test))]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cloned_instead_of_copied,
    clippy::collapsible_else_if,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::elidable_lifetime_names,
    clippy::enum_glob_use,
    clippy::expect_used,
    clippy::explicit_iter_loop,
    clippy::if_not_else,
    clippy::iter_without_into_iter,
    clippy::large_enum_variant,
    clippy::match_like_matches_macro,
    clippy::map_unwrap_or,
    clippy::mem_replace_option_with_some,
    clippy::mem_replace_with_default,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_borrowed_reference,
    clippy::needless_pass_by_value,
    clippy::never_loop,
    clippy::new_without_default,
    clippy::nonminimal_bool,
    clippy::println_empty_string,
    clippy::ptr_as_ptr,
    clippy::question_mark,
    clippy::range_plus_one,
    clippy::redundant_closure,
    clippy::redundant_field_names,
    clippy::redundant_static_lifetimes,
    clippy::result_unit_err,
    clippy::return_self_not_must_use,
    clippy::similar_names,
    clippy::single_match,
    clippy::single_match_else,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::uninlined_format_args,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unwrap_used,
    clippy::used_underscore_binding,
)]

#[macro_use]
extern crate log;

pub use self::stats_browser::StatsBrowser;
pub use self::stats_browser::StatsBrowserCb;

pub mod addr;
pub mod base64;
pub mod config;
pub mod entry;
pub mod hashmap_ext;
pub mod lookup;
pub mod socket;
pub mod stats_browser;
pub mod time;
pub mod tracker_fstd;
pub mod tracker_json;
pub mod vec_map;
pub mod work_queue;
