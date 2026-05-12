#![allow(clippy::unwrap_used, clippy::expect_used, clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::must_use_candidate, clippy::uninlined_format_args, clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_lossless, clippy::redundant_field_names, clippy::trivially_copy_pass_by_ref, clippy::needless_pass_by_value, clippy::needless_lifetimes, clippy::elidable_lifetime_names, clippy::items_after_statements, clippy::unreadable_literal, clippy::single_match_else, clippy::manual_range_contains, clippy::many_single_char_names, clippy::ref_patterns, clippy::map_unwrap_or, clippy::extra_unused_lifetimes, clippy::collapsible_match, clippy::collapsible_else_if, clippy::cloned_instead_of_copied, clippy::if_not_else, clippy::nonminimal_bool, clippy::default_trait_access, clippy::len_zero, clippy::let_underscore_future, clippy::enum_glob_use, clippy::wildcard_imports, clippy::match_same_arms, clippy::too_many_lines, clippy::large_stack_arrays, clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_wrap, clippy::bool_to_int_with_if, clippy::redundant_closure_for_method_calls, clippy::redundant_closure, clippy::used_underscore_binding, clippy::manual_let_else, clippy::explicit_iter_loop, clippy::explicit_into_iter_loop, clippy::match_bool, clippy::useless_conversion, clippy::semicolon_if_nothing_returned, clippy::print_literal, clippy::println_empty_string, clippy::needless_borrow, clippy::collapsible_if, clippy::range_plus_one, clippy::return_self_not_must_use, clippy::needless_late_init, clippy::unnecessary_wraps, clippy::redundant_static_lifetimes)]
extern crate libtw2_gamenet_teeworlds_0_6 as libtw2_gamenet;

use arrayvec::ArrayVec;
use libtw2_common::num::Cast;
use libtw2_common::pretty;
use libtw2_gamenet::msg::game;
use libtw2_gamenet::msg::Game;
use libtw2_packer::Unpacker;
use libtw2_teehistorian::format::item::INPUT_LEN;
use libtw2_teehistorian::Buffer;
use libtw2_teehistorian::Error;
use libtw2_teehistorian::Input;
use libtw2_teehistorian::Item;
use libtw2_teehistorian::Reader;
use libtw2_warn::Ignore;
use std::path::Path;
use std::process;
use vec_map::VecMap;

struct Info {
    name: ArrayVec<[u8; 4 * 4 - 1]>,
}

impl<'a> From<game::ClChangeInfo<'a>> for Info {
    fn from(m: game::ClChangeInfo) -> Info {
        Info {
            name: m.name.iter().cloned().collect(),
        }
    }
}

impl<'a> From<game::ClStartInfo<'a>> for Info {
    fn from(m: game::ClStartInfo) -> Info {
        Info {
            name: m.name.iter().cloned().collect(),
        }
    }
}

struct PrevInput {
    tick: i32,
    input: [i32; INPUT_LEN],
}

const ODD: i32 = 10;
const FIRE: usize = 4;
const INPUT_STATE_MASK: i32 = 0x3f;
const TICKS_PER_SECOND: i32 = 50;

fn process(path: &Path) -> Result<(), Error> {
    let mut buffer = Buffer::new();
    let (_, mut reader) = Reader::open(path, &mut buffer)?;
    let mut tick = None;
    let mut inputs: VecMap<PrevInput> = VecMap::new();
    let mut infos: VecMap<Info> = VecMap::new();
    while let Some(item) = reader.read(&mut buffer)? {
        match item {
            Item::TickStart(t) => {
                assert!(tick.is_none());
                tick = Some(t);
            }
            Item::TickEnd(t) => {
                assert_eq!(tick, Some(t));
                tick = None;
            }
            Item::Input(Input { cid, input }) => {
                let name = pretty::AlmostString::new(&infos[cid.assert_usize()].name);
                let tick = tick.expect("in tick");
                if let Some(prev_input) = inputs.get(cid.assert_usize()) {
                    let prev_fire = prev_input.input[FIRE] & INPUT_STATE_MASK;
                    let fire = input[FIRE] & INPUT_STATE_MASK;
                    let df = (fire + INPUT_STATE_MASK + 1 - prev_fire) & INPUT_STATE_MASK;
                    if df > ODD {
                        let clicks = df / 2 * TICKS_PER_SECOND * 10;
                        let dt = tick - prev_input.tick;
                        if dt != 0 {
                            let cps = clicks / dt;
                            println!(
                                "name={:?} dt={} df={} cps={}.{}",
                                name,
                                dt,
                                df,
                                cps / 10,
                                cps % 10
                            );
                        } else {
                            println!("name={:?} dt={} df={} cps=nan", name, dt, df);
                        }
                    }
                    if input[FIRE] > INPUT_STATE_MASK {
                        //println!("weird fire name={:?} t={} f={}", name, tick, input[FIRE]);
                    }
                }
                inputs.insert(
                    cid.assert_usize(),
                    PrevInput {
                        tick: tick,
                        input: input,
                    },
                );
            }
            Item::Message(msg) => {
                let mut p = Unpacker::new(msg.msg);
                if let Ok(m) = Game::decode(&mut Ignore, &mut p) {
                    match m {
                        Game::ClStartInfo(i) => {
                            infos.insert(msg.cid.assert_usize(), i.into());
                        }
                        Game::ClChangeInfo(i) => {
                            infos.insert(msg.cid.assert_usize(), i.into());
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    assert!(tick.is_none());
    Ok(())
}

fn main() {
    use clap::App;
    use clap::Arg;

    libtw2_logger::init();

    let matches = App::new("Teehistorian odd input checker")
        .about("Reads teehistorian file and checks for odd inputs")
        .arg(
            Arg::with_name("TEEHISTORIAN")
                .help("Sets the teehistorian file to search")
                .required(true),
        )
        .get_matches();

    let path = Path::new(matches.value_of_os("TEEHISTORIAN").unwrap());

    match process(path) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}: {:?}", path.display(), err);
            process::exit(1);
        }
    }
}
