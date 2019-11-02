#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "Madeira Fire Fighter";
const THREADS: [ThreadRef; 24] = [
    ThreadRef {
        brand: BRAND,
        code: "N1610",
        name: "Celestial Blue",
        color: &[183, 195, 197],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1624",
        name: "Gold",
        color: &[244, 167, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1640",
        name: "Twilight",
        color: &[94, 97, 98],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1643",
        name: "Navy",
        color: &[45, 53, 65],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1651",
        name: "Kelly",
        color: &[0, 132, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1682",
        name: "Taupe",
        color: &[205, 191, 172],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1725",
        name: "Old Gold",
        color: &[202, 140, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1747",
        name: "Very Red",
        color: &[164, 30, 53],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1784",
        name: "Wine",
        color: &[117, 44, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1800",
        name: "Black",
        color: &[47, 48, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1803",
        name: "Lily White",
        color: &[237, 236, 223],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1815",
        name: "Pink",
        color: &[252, 190, 210],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1838",
        name: "Foxy Red",
        color: &[180, 34, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1842",
        name: "Fire Blue",
        color: &[0, 85, 149],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1845",
        name: "Neptune Green",
        color: &[109, 195, 159],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1851",
        name: "Fleece Green",
        color: &[0, 94, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1859",
        name: "Brown Mule",
        color: &[75, 63, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1874",
        name: "Baby Blue",
        color: &[144, 179, 216],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1884",
        name: "Rattan",
        color: &[197, 169, 141],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1918",
        name: "Limestone",
        color: &[138, 139, 142],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1924",
        name: "Moonbeam",
        color: &[251, 212, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1957",
        name: "Best Brown",
        color: &[74, 64, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1965",
        name: "Orange",
        color: &[249, 103, 34],
    },
    ThreadRef {
        brand: BRAND,
        code: "N1977",
        name: "Dark Turquoise Blue",
        color: &[0, 129, 175],
    },
];