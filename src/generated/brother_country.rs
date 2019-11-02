#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "Brother Country";
const THREADS: [ThreadRef; 61] = [
    ThreadRef {
        brand: BRAND,
        code: "000",
        name: "White",
        color: &[240, 240, 240],
    },
    ThreadRef {
        brand: BRAND,
        code: "003",
        name: "Wistaria Violet",
        color: &[156, 190, 219],
    },
    ThreadRef {
        brand: BRAND,
        code: "012",
        name: "Beige",
        color: &[240, 213, 179],
    },
    ThreadRef {
        brand: BRAND,
        code: "015",
        name: "Corn Flower Blue",
        color: &[85, 127, 186],
    },
    ThreadRef {
        brand: BRAND,
        code: "020",
        name: "Silver",
        color: &[224, 202, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "024",
        name: "Deep Rose",
        color: &[204, 40, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "025",
        name: "Linen",
        color: &[252, 219, 161],
    },
    ThreadRef {
        brand: BRAND,
        code: "043",
        name: "Yellow",
        color: &[251, 251, 32],
    },
    ThreadRef {
        brand: BRAND,
        code: "057",
        name: "Peacock Blue",
        color: &[5, 96, 96],
    },
    ThreadRef {
        brand: BRAND,
        code: "100",
        name: "Black",
        color: &[0, 0, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "122",
        name: "Salmon Pink",
        color: &[241, 159, 188],
    },
    ThreadRef {
        brand: BRAND,
        code: "126",
        name: "Dark Fuchsia",
        color: &[160, 7, 78],
    },
    ThreadRef {
        brand: BRAND,
        code: "133",
        name: "Light Lilac",
        color: &[243, 111, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "148",
        name: "Vermilion",
        color: &[255, 66, 9],
    },
    ThreadRef {
        brand: BRAND,
        code: "149",
        name: "Red",
        color: &[231, 2, 104],
    },
    ThreadRef {
        brand: BRAND,
        code: "150",
        name: "Light Blue",
        color: &[213, 236, 235],
    },
    ThreadRef {
        brand: BRAND,
        code: "152",
        name: "Flesh Pink",
        color: &[252, 141, 175],
    },
    ThreadRef {
        brand: BRAND,
        code: "155",
        name: "Pink",
        color: &[253, 86, 110],
    },
    ThreadRef {
        brand: BRAND,
        code: "158",
        name: "Carmine",
        color: &[229, 0, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "212",
        name: "Amber Red",
        color: &[233, 76, 100],
    },
    ThreadRef {
        brand: BRAND,
        code: "224",
        name: "Clay Brown",
        color: &[184, 116, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "242",
        name: "Khaki",
        color: &[228, 152, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "255",
        name: "Light Brown",
        color: &[104, 36, 8],
    },
    ThreadRef {
        brand: BRAND,
        code: "264",
        name: "Reddish Brown",
        color: &[254, 71, 12],
    },
    ThreadRef {
        brand: BRAND,
        code: "322",
        name: "Pumpkin",
        color: &[250, 173, 78],
    },
    ThreadRef {
        brand: BRAND,
        code: "331",
        name: "Cream Brown",
        color: &[255, 239, 168],
    },
    ThreadRef {
        brand: BRAND,
        code: "334",
        name: "Harvest Gold",
        color: &[251, 233, 106],
    },
    ThreadRef {
        brand: BRAND,
        code: "335",
        name: "Orange",
        color: &[250, 182, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "336",
        name: "Tangerine",
        color: &[250, 183, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "342",
        name: "Lemon Yellow",
        color: &[254, 254, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "354",
        name: "Deep Gold",
        color: &[242, 153, 14],
    },
    ThreadRef {
        brand: BRAND,
        code: "370",
        name: "Cream Yellow",
        color: &[255, 235, 132],
    },
    ThreadRef {
        brand: BRAND,
        code: "404",
        name: "Brass",
        color: &[238, 204, 12],
    },
    ThreadRef {
        brand: BRAND,
        code: "414",
        name: "Russet Brown",
        color: &[186, 163, 8],
    },
    ThreadRef {
        brand: BRAND,
        code: "442",
        name: "Fresh Green",
        color: &[203, 233, 162],
    },
    ThreadRef {
        brand: BRAND,
        code: "444",
        name: "Lime Green",
        color: &[182, 221, 17],
    },
    ThreadRef {
        brand: BRAND,
        code: "446",
        name: "Moss Green",
        color: &[81, 137, 33],
    },
    ThreadRef {
        brand: BRAND,
        code: "461",
        name: "Mint Green",
        color: &[200, 220, 200],
    },
    ThreadRef {
        brand: BRAND,
        code: "463",
        name: "Leaf Green",
        color: &[119, 196, 135],
    },
    ThreadRef {
        brand: BRAND,
        code: "467",
        name: "Deep Green",
        color: &[0, 48, 30],
    },
    ThreadRef {
        brand: BRAND,
        code: "473",
        name: "Dark Olive",
        color: &[176, 207, 120],
    },
    ThreadRef {
        brand: BRAND,
        code: "476",
        name: "Olive Green",
        color: &[13, 42, 7],
    },
    ThreadRef {
        brand: BRAND,
        code: "483",
        name: "Teal Green",
        color: &[120, 199, 159],
    },
    ThreadRef {
        brand: BRAND,
        code: "485",
        name: "Emerald Green",
        color: &[0, 81, 39],
    },
    ThreadRef {
        brand: BRAND,
        code: "505",
        name: "Seacrest",
        color: &[116, 198, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "512",
        name: "Sky Blue",
        color: &[177, 226, 223],
    },
    ThreadRef {
        brand: BRAND,
        code: "564",
        name: "Electric Blue",
        color: &[8, 90, 111],
    },
    ThreadRef {
        brand: BRAND,
        code: "575",
        name: "Ultramarine",
        color: &[10, 89, 163],
    },
    ThreadRef {
        brand: BRAND,
        code: "586",
        name: "Blue",
        color: &[0, 99, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "588",
        name: "Prussian Blue",
        color: &[6, 41, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "604",
        name: "Lavender",
        color: &[146, 133, 195],
    },
    ThreadRef {
        brand: BRAND,
        code: "623",
        name: "Lilac",
        color: &[196, 160, 207],
    },
    ThreadRef {
        brand: BRAND,
        code: "624",
        name: "Violet",
        color: &[180, 128, 196],
    },
    ThreadRef {
        brand: BRAND,
        code: "625",
        name: "Magenta",
        color: &[103, 0, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "635",
        name: "Purple",
        color: &[105, 21, 136],
    },
    ThreadRef {
        brand: BRAND,
        code: "636",
        name: "Royal Purple",
        color: &[69, 4, 81],
    },
    ThreadRef {
        brand: BRAND,
        code: "706",
        name: "Warm Gray",
        color: &[172, 154, 97],
    },
    ThreadRef {
        brand: BRAND,
        code: "717",
        name: "Dark Brown",
        color: &[29, 24, 3],
    },
    ThreadRef {
        brand: BRAND,
        code: "734",
        name: "Gray",
        color: &[110, 100, 82],
    },
    ThreadRef {
        brand: BRAND,
        code: "745",
        name: "Pewter",
        color: &[168, 171, 173],
    },
    ThreadRef {
        brand: BRAND,
        code: "747",
        name: "Dark Gray",
        color: &[16, 20, 12],
    },
];