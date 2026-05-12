#![allow(
    clippy::bool_to_int_with_if,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::doc_markdown,
    clippy::elidable_lifetime_names,
    clippy::enum_glob_use,
    clippy::large_enum_variant,
    clippy::match_same_arms,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_borrow,
    clippy::needless_lifetimes,
    clippy::needless_pass_by_value,
    clippy::nonminimal_bool,
    clippy::ptr_as_ptr,
    clippy::pub_underscore_fields,
    clippy::redundant_field_names,
    clippy::redundant_static_lifetimes,
    clippy::result_unit_err,
    clippy::return_self_not_must_use,
    clippy::semicolon_if_nothing_returned,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::used_underscore_binding,
    clippy::unwrap_used,
)]

#[rustfmt::skip]
pub mod enums;
#[rustfmt::skip]
pub mod msg;
#[rustfmt::skip]
pub mod snap_obj;

mod traits;

pub use self::snap_obj::SnapObj;
pub use self::traits::Protocol;
pub use libtw2_gamenet_common::error;
pub use libtw2_gamenet_common::error::Error;
