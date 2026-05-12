#![allow(clippy::unwrap_used, clippy::expect_used, clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::must_use_candidate, clippy::uninlined_format_args, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_lossless, clippy::redundant_field_names, clippy::trivially_copy_pass_by_ref, clippy::needless_pass_by_value, clippy::needless_lifetimes, clippy::elidable_lifetime_names, clippy::items_after_statements, clippy::unreadable_literal, clippy::single_match_else, clippy::manual_range_contains, clippy::many_single_char_names, clippy::ref_patterns, clippy::map_unwrap_or, clippy::extra_unused_lifetimes, clippy::collapsible_match, clippy::collapsible_else_if, clippy::cloned_instead_of_copied, clippy::if_not_else, clippy::nonminimal_bool, clippy::default_trait_access, clippy::len_zero, clippy::let_underscore_future, clippy::enum_glob_use, clippy::wildcard_imports, clippy::match_same_arms, clippy::too_many_lines, clippy::large_stack_arrays, clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_wrap, clippy::bool_to_int_with_if, clippy::redundant_closure_for_method_calls, clippy::redundant_closure, clippy::used_underscore_binding, clippy::manual_let_else, clippy::explicit_iter_loop, clippy::explicit_into_iter_loop, clippy::match_bool, clippy::useless_conversion, clippy::semicolon_if_nothing_returned, clippy::print_literal, clippy::println_empty_string, clippy::needless_borrow, clippy::collapsible_if, clippy::range_plus_one, clippy::return_self_not_must_use, clippy::needless_late_init, clippy::unnecessary_wraps, clippy::redundant_static_lifetimes)]
extern crate libtw2_gamenet_teeworlds_0_6 as libtw2_gamenet;

use arrayvec::ArrayVec;
use hexdump::hexdump;
use libtw2_buffer::ReadBuffer;
use libtw2_gamenet::msg;
use libtw2_gamenet::msg::Connless;
use libtw2_net::protocol::ChunksIter;
use libtw2_net::protocol::ConnectedPacketType;
use libtw2_net::protocol::Packet;
use libtw2_packer::Unpacker;
use libtw2_tools::unhexdump::Unhexdump;
use libtw2_tools::warn_stdout::Stdout;
use std::io;

fn main() {
    let mut un = Unhexdump::new();
    let mut buf: ArrayVec<[u8; 4096]> = ArrayVec::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    while {
        buf.clear();
        stdin.read_buffer(&mut buf).unwrap().len() != 0
    } {
        un.feed(&buf).unwrap();
    }

    let bytes = un.into_inner().unwrap();

    println!("packet");
    hexdump(&bytes);
    let p = match Packet::read(&mut Stdout, &bytes, None, &mut buf) {
        Err(e) => {
            println!("ERROR: {:?}", e);
            return;
        }
        Ok(p) => p,
    };

    let cp = match p {
        Packet::Connless(data) => {
            println!("connless");
            let msg = match Connless::decode(&mut Stdout, &mut Unpacker::new(data)) {
                Err(e) => {
                    println!("ERROR: {:?}", e);
                    return;
                }
                Ok(m) => m,
            };
            println!("{:?}", msg);
            return;
        }
        Packet::Connected(cp) => cp,
    };

    if let Some(token) = cp.token {
        println!("token={}", token);
    }

    let (request_resend, num_chunks, payload) = match cp.type_ {
        ConnectedPacketType::Control(control) => {
            println!("control ack={}", cp.ack);
            println!("{:?}", control);
            return;
        }
        ConnectedPacketType::Chunks(r, n, p) => (r, n, p),
    };
    println!(
        "chunks ack={} request_resend={} num_chunks={}",
        cp.ack, request_resend, num_chunks
    );
    hexdump(payload);
    let mut i = 0;
    let mut chunks_iter = ChunksIter::new(payload, num_chunks);
    loop {
        if chunks_iter.clone().next().is_some() {
            println!("chunk {}", i);
        }
        let chunk = if let Some(chunk) = chunks_iter.next_warn(&mut Stdout) {
            i += 1;
            chunk
        } else {
            break;
        };

        match chunk.vital {
            Some((sequence, resend)) => {
                println!("vital=true sequence={} resend={}", sequence, resend)
            }
            None => println!("vital=false"),
        }
        hexdump(chunk.data);

        let msg = match msg::decode(&mut Stdout, &mut Unpacker::new(chunk.data)) {
            Err(e) => {
                println!("ERROR: {:?}", e);
                continue;
            }
            Ok(m) => m,
        };

        println!("{:?}", msg);
    }
}
