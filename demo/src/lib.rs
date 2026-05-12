#![allow(
    clippy::cloned_instead_of_copied,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::expect_used,
    clippy::large_stack_arrays,
    clippy::manual_let_else,
    clippy::match_same_arms,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::semicolon_if_nothing_returned,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps,
    clippy::unwrap_used
)]

pub mod ddnet;
mod format;
mod reader;
mod writer;

pub use self::format::DemoKind;
pub use self::format::RawChunk;
pub use self::format::Version;
pub use self::format::Warning;
pub use self::reader::ReadError;
pub use self::reader::Reader;
pub use self::writer::WriteError;
pub use self::writer::Writer;
