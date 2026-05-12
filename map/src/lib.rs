#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::doc_markdown,
    clippy::enum_glob_use,
    clippy::large_enum_variant,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::ptr_as_ptr,
    clippy::redundant_field_names,
    clippy::result_unit_err,
    clippy::return_self_not_must_use,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::used_underscore_binding,
    clippy::unwrap_used,
    clippy::cast_precision_loss,
    clippy::copy_iterator,
    clippy::identity_op,
    clippy::iter_without_into_iter,
    clippy::len_zero,
    clippy::manual_contains,
    clippy::manual_slice_size_calculation,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::mem_replace_option_with_some,
    clippy::nonminimal_bool,
    clippy::pub_underscore_fields,
    clippy::redundant_closure_for_method_calls,
)]

pub use self::reader::Error;
pub use self::reader::Reader;

#[rustfmt::skip]
pub mod format;
pub mod reader;
