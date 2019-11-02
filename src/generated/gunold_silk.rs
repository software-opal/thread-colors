#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "Gunold Silk";
const THREADS: [ThreadRef; 12] = [
    ThreadRef {
        brand: BRAND,
        code: "91002",
        name: "Soft White",
        color: &[249, 249, 244],
    },
    ThreadRef {
        brand: BRAND,
        code: "91005",
        name: "Black",
        color: &[0, 0, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "91011",
        name: "Steel Gray",
        color: &[183, 169, 172],
    },
    ThreadRef {
        brand: BRAND,
        code: "91019",
        name: "Peach",
        color: &[236, 160, 130],
    },
    ThreadRef {
        brand: BRAND,
        code: "91029",
        name: "Med. Blue",
        color: &[160, 195, 235],
    },
    ThreadRef {
        brand: BRAND,
        code: "91044",
        name: "Midnight Blue",
        color: &[29, 6, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "91071",
        name: "Off White",
        color: &[249, 249, 234],
    },
    ThreadRef {
        brand: BRAND,
        code: "91101",
        name: "True Green",
        color: &[9, 133, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "91135",
        name: "Pastel Yellow",
        color: &[255, 240, 114],
    },
    ThreadRef {
        brand: BRAND,
        code: "91170",
        name: "Lt. Brown",
        color: &[151, 95, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "91225",
        name: "Pastel Pink",
        color: &[250, 203, 203],
    },
    ThreadRef {
        brand: BRAND,
        code: "91317",
        name: "Blaze Medium",
        color: &[255, 0, 0],
    },
];