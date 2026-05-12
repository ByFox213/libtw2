#![allow(
    clippy::assertions_on_constants,
    clippy::bool_to_int_with_if,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::default_constructed_unit_structs,
    clippy::doc_markdown,
    clippy::elidable_lifetime_names,
    clippy::enum_glob_use,
    clippy::identity_op,
    clippy::items_after_statements,
    clippy::large_enum_variant,
    clippy::map_identity,
    clippy::match_same_arms,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::never_loop,
    clippy::new_without_default,
    clippy::redundant_closure,
    clippy::redundant_else,
    clippy::redundant_field_names,
    clippy::redundant_static_lifetimes,
    clippy::semicolon_if_nothing_returned,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::unnecessary_wraps,
    clippy::nonminimal_bool,
    clippy::unnecessary_cast,
    clippy::while_let_on_iterator,
)]

pub mod collections;
pub mod connection;
pub mod connection7;
pub mod net;
pub mod protocol;
pub mod protocol7;
pub mod time;

pub use self::connection::Connection;
pub use self::net::Net;
pub use self::time::Timeout;
pub use self::time::Timestamp;
