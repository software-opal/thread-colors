#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Madeira Fire Fighter";
pub const THREADS: [ThreadRef; 24] = [
    ThreadRef::new(
        BRAND,
        "N1845",
        "Neptune Green",
        &[109, 195, 159],
    ),
    ThreadRef::new(
        BRAND,
        "N1851",
        "Fleece Green",
        &[0, 94, 56],
    ),
    ThreadRef::new(
        BRAND,
        "N1651",
        "Kelly",
        &[0, 132, 69],
    ),
    ThreadRef::new(
        BRAND,
        "N1965",
        "Orange",
        &[249, 103, 34],
    ),
    ThreadRef::new(
        BRAND,
        "N1838",
        "Foxy Red",
        &[180, 34, 50],
    ),
    ThreadRef::new(
        BRAND,
        "N1747",
        "Very Red",
        &[164, 30, 53],
    ),
    ThreadRef::new(
        BRAND,
        "N1784",
        "Wine",
        &[117, 44, 58],
    ),
    ThreadRef::new(
        BRAND,
        "N1815",
        "Pink",
        &[252, 190, 210],
    ),
    ThreadRef::new(
        BRAND,
        "N1874",
        "Baby Blue",
        &[144, 179, 216],
    ),
    ThreadRef::new(
        BRAND,
        "N1977",
        "Dark Turquoise Blue",
        &[0, 129, 175],
    ),
    ThreadRef::new(
        BRAND,
        "N1842",
        "Fire Blue",
        &[0, 85, 149],
    ),
    ThreadRef::new(
        BRAND,
        "N1643",
        "Navy",
        &[45, 53, 65],
    ),
    ThreadRef::new(
        BRAND,
        "N1924",
        "Moonbeam",
        &[251, 212, 0],
    ),
    ThreadRef::new(
        BRAND,
        "N1624",
        "Gold",
        &[244, 167, 49],
    ),
    ThreadRef::new(
        BRAND,
        "N1725",
        "Old Gold",
        &[202, 140, 63],
    ),
    ThreadRef::new(
        BRAND,
        "N1957",
        "Best Brown",
        &[74, 64, 50],
    ),
    ThreadRef::new(
        BRAND,
        "N1859",
        "Brown Mule",
        &[75, 63, 62],
    ),
    ThreadRef::new(
        BRAND,
        "N1884",
        "Rattan",
        &[197, 169, 141],
    ),
    ThreadRef::new(
        BRAND,
        "N1682",
        "Taupe",
        &[205, 191, 172],
    ),
    ThreadRef::new(
        BRAND,
        "N1803",
        "Lily White",
        &[237, 236, 223],
    ),
    ThreadRef::new(
        BRAND,
        "N1610",
        "Celestial Blue",
        &[183, 195, 197],
    ),
    ThreadRef::new(
        BRAND,
        "N1918",
        "Limestone",
        &[138, 139, 142],
    ),
    ThreadRef::new(
        BRAND,
        "N1640",
        "Twilight",
        &[94, 97, 98],
    ),
    ThreadRef::new(
        BRAND,
        "N1800",
        "Black",
        &[47, 48, 50],
    ),
];