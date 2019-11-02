#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Gunold Silk";
pub const THREADS: [ThreadRef; 12] = [
    ThreadRef::new(
        BRAND,
        "91002",
        "Soft White",
        &[249, 249, 244],
    ),
    ThreadRef::new(
        BRAND,
        "91005",
        "Black",
        &[0, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "91011",
        "Steel Gray",
        &[183, 169, 172],
    ),
    ThreadRef::new(
        BRAND,
        "91019",
        "Peach",
        &[236, 160, 130],
    ),
    ThreadRef::new(
        BRAND,
        "91029",
        "Med. Blue",
        &[160, 195, 235],
    ),
    ThreadRef::new(
        BRAND,
        "91044",
        "Midnight Blue",
        &[29, 6, 47],
    ),
    ThreadRef::new(
        BRAND,
        "91071",
        "Off White",
        &[249, 249, 234],
    ),
    ThreadRef::new(
        BRAND,
        "91101",
        "True Green",
        &[9, 133, 49],
    ),
    ThreadRef::new(
        BRAND,
        "91135",
        "Pastel Yellow",
        &[255, 240, 114],
    ),
    ThreadRef::new(
        BRAND,
        "91170",
        "Lt. Brown",
        &[151, 95, 47],
    ),
    ThreadRef::new(
        BRAND,
        "91225",
        "Pastel Pink",
        &[250, 203, 203],
    ),
    ThreadRef::new(
        BRAND,
        "91317",
        "Blaze Medium",
        &[255, 0, 0],
    ),
];