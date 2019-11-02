#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Viking Palette";
pub const THREADS: [ThreadRef; 29] = [
    ThreadRef::new(
        BRAND,
        "1",
        "Black",
        &[0, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2",
        "Blue",
        &[0, 0, 255],
    ),
    ThreadRef::new(
        BRAND,
        "3",
        "Light Green",
        &[0, 255, 0],
    ),
    ThreadRef::new(
        BRAND,
        "4",
        "Red",
        &[255, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "5",
        "Purple",
        &[255, 0, 255],
    ),
    ThreadRef::new(
        BRAND,
        "6",
        "Yellow",
        &[255, 255, 0],
    ),
    ThreadRef::new(
        BRAND,
        "7",
        "Gray",
        &[132, 130, 132],
    ),
    ThreadRef::new(
        BRAND,
        "8",
        "Light Blue",
        &[0, 130, 255],
    ),
    ThreadRef::new(
        BRAND,
        "9",
        "Green",
        &[0, 255, 132],
    ),
    ThreadRef::new(
        BRAND,
        "10",
        "Orange",
        &[255, 130, 0],
    ),
    ThreadRef::new(
        BRAND,
        "11",
        "Pink",
        &[255, 162, 181],
    ),
    ThreadRef::new(
        BRAND,
        "12",
        "Brown",
        &[198, 65, 0],
    ),
    ThreadRef::new(
        BRAND,
        "13",
        "White",
        &[255, 255, 255],
    ),
    ThreadRef::new(
        BRAND,
        "14",
        "Dark Blue",
        &[0, 0, 132],
    ),
    ThreadRef::new(
        BRAND,
        "15",
        "Dark Green",
        &[0, 130, 0],
    ),
    ThreadRef::new(
        BRAND,
        "16",
        "Dark Red",
        &[165, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "17",
        "Light Red",
        &[255, 121, 123],
    ),
    ThreadRef::new(
        BRAND,
        "18",
        "Dark Purple",
        &[132, 0, 132],
    ),
    ThreadRef::new(
        BRAND,
        "19",
        "Light Purple",
        &[255, 130, 255],
    ),
    ThreadRef::new(
        BRAND,
        "20",
        "Dark Yellow",
        &[198, 195, 0],
    ),
    ThreadRef::new(
        BRAND,
        "21",
        "Light Yellow",
        &[255, 255, 165],
    ),
    ThreadRef::new(
        BRAND,
        "22",
        "Dark Gray",
        &[66, 65, 66],
    ),
    ThreadRef::new(
        BRAND,
        "23",
        "Light Gray",
        &[198, 195, 198],
    ),
    ThreadRef::new(
        BRAND,
        "24",
        "Dark Orange",
        &[231, 65, 0],
    ),
    ThreadRef::new(
        BRAND,
        "25",
        "Light Orange",
        &[255, 174, 66],
    ),
    ThreadRef::new(
        BRAND,
        "26",
        "Dark Pink",
        &[255, 89, 123],
    ),
    ThreadRef::new(
        BRAND,
        "27",
        "Light Pink",
        &[255, 211, 214],
    ),
    ThreadRef::new(
        BRAND,
        "28",
        "Dark Brown",
        &[132, 32, 0],
    ),
    ThreadRef::new(
        BRAND,
        "29",
        "Light Brown",
        &[231, 97, 33],
    ),
];