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
use clap::App;
use libtw2_packer::UnexpectedEnd;
use libtw2_packer::Unpacker;
use libtw2_tools::warn_stderr::Stderr;
use std::io;
use std::io::Read as _;
use std::io::Write as _;

fn main() -> Result<(), io::Error> {
    let _ = App::new("Teeworlds variable-length integer decoding")
        .about(
            "Decodes stdin as a list of Teeworlds variable-length integers\
                to big-endian 32-bit integers",
        )
        .get_matches();

    let mut stdin = Vec::new();
    io::stdin().read_to_end(&mut stdin)?;
    let mut unpacker = Unpacker::new(&stdin);
    let mut result = Vec::new();
    while !unpacker.is_empty() {
        result.extend_from_slice(
            &unpacker
                .read_int(&mut Stderr)
                .map_err(|UnexpectedEnd| io::Error::other("unexpected end"))?
                .to_be_bytes(),
        );
    }
    io::stdout().write_all(&result)?;
    Ok(())
}
