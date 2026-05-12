#![allow(clippy::unwrap_used, clippy::expect_used, clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::must_use_candidate, clippy::uninlined_format_args, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_lossless, clippy::redundant_field_names, clippy::trivially_copy_pass_by_ref, clippy::needless_pass_by_value, clippy::needless_lifetimes, clippy::elidable_lifetime_names, clippy::items_after_statements, clippy::unreadable_literal, clippy::single_match_else, clippy::manual_range_contains, clippy::many_single_char_names, clippy::ref_patterns, clippy::map_unwrap_or, clippy::extra_unused_lifetimes, clippy::collapsible_match, clippy::collapsible_else_if, clippy::cloned_instead_of_copied, clippy::if_not_else, clippy::nonminimal_bool, clippy::default_trait_access, clippy::len_zero, clippy::let_underscore_future, clippy::enum_glob_use, clippy::wildcard_imports, clippy::match_same_arms, clippy::too_many_lines, clippy::large_stack_arrays, clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_wrap, clippy::bool_to_int_with_if, clippy::redundant_closure_for_method_calls, clippy::redundant_closure, clippy::used_underscore_binding, clippy::manual_let_else, clippy::explicit_iter_loop, clippy::explicit_into_iter_loop, clippy::match_bool, clippy::useless_conversion, clippy::semicolon_if_nothing_returned, clippy::print_literal, clippy::println_empty_string, clippy::needless_borrow, clippy::collapsible_if, clippy::range_plus_one, clippy::return_self_not_must_use, clippy::needless_late_init, clippy::unnecessary_wraps, clippy::redundant_static_lifetimes)]
#![cfg(not(test))]

#[macro_use]
extern crate log;

use libtw2_serverbrowse::protocol as browse_protocol;
use libtw2_serverbrowse::protocol::Count7Response;
use libtw2_serverbrowse::protocol::Response;
use libtw2_serverbrowse::protocol::Token7;
use libtw2_serverbrowse::protocol::Token7Response;
use std::net::SocketAddr;
use std::net::UdpSocket;

const BUFSIZE: usize = 2048;

fn do_(socket: UdpSocket, addr: SocketAddr) {
    let mut buf = [0; BUFSIZE];

    socket
        .send_to(&browse_protocol::request_token_7(Token7([0; 4])), addr)
        .unwrap();

    loop {
        let (len, from) = socket.recv_from(&mut buf).unwrap();
        if from != addr {
            error!(
                "received response from non-peer, wanted={} got={}",
                addr, from
            );
            continue;
        }
        match browse_protocol::parse_response(&buf[..len]) {
            Some(Response::Token7(Token7Response(Token7([0, 0, 0, 0]), their_token))) => {
                info!("token={}", their_token);
                socket
                    .send_to(
                        &browse_protocol::request_count_7(Token7([0; 4]), their_token),
                        addr,
                    )
                    .unwrap();
                break;
            }
            _ => {
                error!("received non-token response from peer");
            }
        }
    }
    loop {
        let (len, from) = socket.recv_from(&mut buf).unwrap();
        if from != addr {
            error!(
                "received response from non-peer, wanted={} got={}",
                addr, from
            );
            continue;
        }
        match browse_protocol::parse_response(&buf[..len]) {
            Some(Response::Count7(Count7Response(_, _, x))) => {
                println!("{}", x);
                break;
            }
            _ => {
                error!("received non-count response from peer");
            }
        }
    }
}

fn main() {
    libtw2_tools::client::client(do_);
}
