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
    clippy::cast_precision_loss,
    clippy::collapsible_match,
    clippy::default_trait_access,
    clippy::elidable_lifetime_names,
    clippy::if_not_else,
    clippy::items_after_statements,
    clippy::manual_assert,
    clippy::manual_next_back,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::mem_replace_with_default,
    clippy::missing_safety_doc,
    clippy::needless_question_mark,
    clippy::nonminimal_bool,
    clippy::range_plus_one,
    clippy::redundant_pattern_matching,
    clippy::redundant_static_lifetimes,
    clippy::semicolon_if_nothing_returned,
    clippy::single_match,
    clippy::single_match_else,
    clippy::uninlined_format_args,
    clippy::unnecessary_semicolon,
    clippy::unnecessary_trailing_comma,
    clippy::used_underscore_items
)]

extern crate libtw2_wireshark_dissector_sys as sys;

mod format;
mod intern;
mod spec;
#[allow(static_mut_refs)]
mod tw;
#[allow(static_mut_refs)]
mod tw7;

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    lazy_static! {
        pub static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }
}

use intern::intern;
use intern::Interned;
use libtw2_gamenet_spec::Identifier;
use libtw2_warn as warn;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::process;
use uuid::Uuid;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static plugin_want_major: c_int = 4;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static plugin_want_minor: c_int = 6;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static plugin_version: [u8; 6] = *b"0.0.1\0";

#[inline]
fn c(s: &'static str) -> *const c_char {
    intern::intern_static_with_nul(s).c()
}

pub const HFRI_DEFAULT: sys::_header_field_info = sys::_header_field_info {
    name: 0 as _,
    abbrev: 0 as _,
    type_: 0,
    display: 0,
    strings: 0 as _,
    bitmask: 0,
    blurb: 0 as _,
    id: -1,
    parent: 0,
    ref_type: 0,
    same_name_prev_id: -1,
    same_name_next: 0 as _,
};

#[derive(Default)]
struct Counter(u64);

impl Counter {
    fn new() -> Counter {
        Default::default()
    }
    fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl<W> warn::Warn<W> for Counter {
    fn warn(&mut self, _warning: W) {
        self.0 += 1;
    }
}

trait IdentifierEx {
    fn _identifier(&self) -> &Identifier;
    fn isnake(&self) -> Interned {
        intern(&self._identifier().snake())
    }
    fn idesc(&self) -> Interned {
        intern(&self._identifier().desc())
    }
}
impl IdentifierEx for Identifier {
    fn _identifier(&self) -> &Identifier {
        self
    }
}

fn to_guid(uuid: Uuid) -> sys::e_guid_t {
    let (data1, data2, data3, &data4) = uuid.as_fields();
    sys::e_guid_t {
        data1,
        data2,
        data3,
        data4,
    }
}

unsafe extern "C" fn proto_register() {
    tw::proto_register();
    tw7::proto_register();
}

unsafe extern "C" fn proto_reg_handoff() {
    tw::proto_reg_handoff();
    tw7::proto_reg_handoff();
}

#[no_mangle]
pub unsafe extern "C" fn plugin_register() {
    {
        let version = CStr::from_ptr(sys::epan_get_version()).to_bytes();
        if version == b"4.0.4" {
            eprintln!("libtw2: Wireshark 4.0.4 is ABI-incompatible with the 4.0 series.");
            eprintln!("libtw2: Use Wireshark 4.0.3 or Wireshark 4.0.5+ instead.");
            eprintln!("libtw2: https://gitlab.com/wireshark/wireshark/-/issues/18908");
            eprintln!("libtw2: https://github.com/heinrich5991/libtw2/issues/73");
            process::abort();
        }
    }
    sys::proto_register_plugin(&sys::proto_plugin {
        register_protoinfo: Some(proto_register),
        register_handoff: Some(proto_reg_handoff),
    });
}
