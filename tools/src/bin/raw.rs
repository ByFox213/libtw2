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
#![cfg(not(test))]

use libtw2_map::format::*;
use std::fmt::Debug;
use std::path::Path;

fn process(_: &Path, dfr: libtw2_datafile::Reader, _: &mut ()) -> Result<(), libtw2_map::Error> {
    let mut env_version = None;

    for item in dfr.items() {
        let item: libtw2_datafile::ItemView = item;
        fn print_map_item<T: MapItem + Debug>(slice: &[i32]) {
            if let Ok(Some(mi)) = T::from_slice(slice) {
                print!(" {:?}", mi);
            }
        }

        print!("{} {} {}", item.type_id, item.id, item.data.len());
        match item.type_id {
            MAP_ITEMTYPE_VERSION => {
                print_map_item::<MapItemCommonV0>(item.data);
                //print_map_item::<MapItemVersionV1>(item.data);
            }
            MAP_ITEMTYPE_INFO => {
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemInfoV1>(item.data);
            }
            MAP_ITEMTYPE_IMAGE => {
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemImageV1>(item.data);
                print_map_item::<MapItemImageV2>(item.data);
            }
            MAP_ITEMTYPE_ENVELOPE => {
                print_map_item::<MapItemCommonV0>(item.data);
                if let Ok(Some(c)) = MapItemCommonV0::from_slice(item.data) {
                    match env_version {
                        None => env_version = Some(c.version),
                        Some(v) if v == c.version => {}
                        Some(v) => panic!(
                            "differing versions for envpoints, v1={} v2={}",
                            v, c.version
                        ),
                    }
                }
                print_map_item::<MapItemEnvelopeV1>(item.data);
                print_map_item::<MapItemEnvelopeV2>(item.data);
            }
            MAP_ITEMTYPE_GROUP => {
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemGroupV1>(item.data);
                print_map_item::<MapItemGroupV2>(item.data);
            }
            MAP_ITEMTYPE_LAYER => {
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemLayerV1>(item.data);
                if let Ok(Some((layer, rest))) = MapItemLayerV1::from_slice_rest(item.data) {
                    print!(" {} {} {}", layer.type_, layer.flags, rest.len());
                    match layer.type_ {
                        MAP_ITEMTYPE_LAYER_V1_TILEMAP => {
                            print_map_item::<MapItemLayerV1CommonV0>(rest);
                            //print_map_item::<MapItemLayerV1TilemapV1>(rest);
                            print_map_item::<MapItemLayerV1TilemapV2>(rest);
                            print_map_item::<MapItemLayerV1TilemapV3>(rest);
                        }
                        MAP_ITEMTYPE_LAYER_V1_QUADS => {
                            print_map_item::<MapItemLayerV1CommonV0>(rest);
                            print_map_item::<MapItemLayerV1QuadsV1>(rest);
                            print_map_item::<MapItemLayerV1QuadsV2>(rest);
                        }
                        MAP_ITEMTYPE_LAYER_V1_DDRACE_SOUNDS
                        | MAP_ITEMTYPE_LAYER_V1_DDRACE_SOUNDS_LEGACY => {
                            print_map_item::<MapItemLayerV1CommonV0>(rest);
                            print_map_item::<MapItemLayerV1DdraceSoundsV1>(rest);
                            //print_map_item::<MapItemLayerV1DdraceSoundsV2>(rest);
                        }
                        _ => panic!("unknown layer type {}", layer.type_),
                    }
                }
            }
            MAP_ITEMTYPE_ENVPOINTS => {
                let env_version =
                    env_version.unwrap_or_else(|| panic!("envpoints but no envelope"));
                fn print_envpoints<E: Envpoint + Debug>(slice: &[i32], env_version: i32) {
                    if let Some(e) = E::from_slice(slice, env_version) {
                        print!(" {:?}", e);
                    }
                }
                print_envpoints::<MapItemEnvpointV1>(item.data, env_version);
                print_envpoints::<MapItemEnvpointV2>(item.data, env_version);
            }
            MAP_ITEMTYPE_DDRACE_SOUND => {
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemDdraceSoundV1>(item.data);
            }
            _ => {
                print_map_item::<MapItemCommonV0>(item.data);
                panic!("unknown datafile item type {}", item.type_id);
            }
        }
        println!("");
    }
    Ok(())
}

fn nothing(_: &()) {}

fn main() {
    libtw2_tools::map_stats::stats(process, nothing);
}
