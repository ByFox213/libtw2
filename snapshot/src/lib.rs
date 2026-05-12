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
    clippy::expect_used,
    clippy::comparison_chain,
    clippy::default_trait_access,
    clippy::derivable_impls,
    clippy::elidable_lifetime_names,
    clippy::if_not_else,
    clippy::items_after_statements,
    clippy::map_unwrap_or,
    clippy::needless_continue,
    clippy::redundant_else,
    clippy::redundant_pattern_matching,
    clippy::semicolon_if_nothing_returned,
    clippy::single_match_else,
)]

use self::read_int::ReadInt;
use libtw2_common::num::Cast;
use std::ops;

pub mod format;
pub mod manager;
pub mod receiver;
pub mod snap;
pub mod storage;

pub use self::manager::Manager;
pub use self::receiver::DeltaReceiver;
pub use self::receiver::ReceivedDelta;
pub use self::snap::Delta;
pub use self::snap::Snap;
pub use self::storage::Storage;

mod read_int;

fn to_usize(r: ops::Range<u32>) -> ops::Range<usize> {
    r.start.usize()..r.end.usize()
}
