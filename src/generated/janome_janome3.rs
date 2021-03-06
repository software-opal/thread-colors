#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Janome";
pub const THREADS: [ThreadRef; 78] = [
    ThreadRef::new(
        BRAND,
        "001",
        "White",
        &[240, 240, 240],
    ),
    ThreadRef::new(
        BRAND,
        "002",
        "Black",
        &[0, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "003",
        "Gold",
        &[228, 195, 93],
    ),
    ThreadRef::new(
        BRAND,
        "201",
        "Pink",
        &[246, 105, 160],
    ),
    ThreadRef::new(
        BRAND,
        "225",
        "Red",
        &[255, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "203",
        "Orange",
        &[255, 102, 0],
    ),
    ThreadRef::new(
        BRAND,
        "204",
        "Yellow",
        &[255, 255, 23],
    ),
    ThreadRef::new(
        BRAND,
        "205",
        "Dark Brown",
        &[72, 26, 5],
    ),
    ThreadRef::new(
        BRAND,
        "226",
        "Green",
        &[35, 115, 54],
    ),
    ThreadRef::new(
        BRAND,
        "207",
        "Blue",
        &[11, 47, 132],
    ),
    ThreadRef::new(
        BRAND,
        "208",
        "Purple",
        &[171, 90, 150],
    ),
    ThreadRef::new(
        BRAND,
        "209",
        "Pale Violet",
        &[172, 156, 199],
    ),
    ThreadRef::new(
        BRAND,
        "210",
        "Pale Yellow",
        &[253, 245, 181],
    ),
    ThreadRef::new(
        BRAND,
        "211",
        "Pale Pink",
        &[249, 153, 183],
    ),
    ThreadRef::new(
        BRAND,
        "212",
        "Peach",
        &[250, 179, 129],
    ),
    ThreadRef::new(
        BRAND,
        "213",
        "Beige",
        &[215, 189, 164],
    ),
    ThreadRef::new(
        BRAND,
        "214",
        "Brown",
        &[156, 100, 69],
    ),
    ThreadRef::new(
        BRAND,
        "215",
        "Wine Red",
        &[151, 5, 51],
    ),
    ThreadRef::new(
        BRAND,
        "216",
        "Pale Sky",
        &[160, 184, 204],
    ),
    ThreadRef::new(
        BRAND,
        "217",
        "Sky",
        &[101, 194, 200],
    ),
    ThreadRef::new(
        BRAND,
        "218",
        "Yellow Green",
        &[127, 194, 28],
    ),
    ThreadRef::new(
        BRAND,
        "219",
        "Olive Green",
        &[47, 89, 51],
    ),
    ThreadRef::new(
        BRAND,
        "220",
        "Silver Gray",
        &[229, 229, 229],
    ),
    ThreadRef::new(
        BRAND,
        "221",
        "Gray",
        &[136, 155, 155],
    ),
    ThreadRef::new(
        BRAND,
        "227",
        "Pale Aqua",
        &[152, 214, 189],
    ),
    ThreadRef::new(
        BRAND,
        "228",
        "Baby Blue",
        &[178, 225, 227],
    ),
    ThreadRef::new(
        BRAND,
        "229",
        "Powder Blue",
        &[152, 243, 254],
    ),
    ThreadRef::new(
        BRAND,
        "230",
        "Bright Blue",
        &[112, 169, 226],
    ),
    ThreadRef::new(
        BRAND,
        "231",
        "Slate Blue",
        &[29, 84, 120],
    ),
    ThreadRef::new(
        BRAND,
        "232",
        "Navy Blue",
        &[7, 22, 80],
    ),
    ThreadRef::new(
        BRAND,
        "233",
        "Salmon Pink",
        &[255, 187, 187],
    ),
    ThreadRef::new(
        BRAND,
        "234",
        "Coral",
        &[255, 96, 72],
    ),
    ThreadRef::new(
        BRAND,
        "235",
        "Burnt Orange",
        &[255, 90, 39],
    ),
    ThreadRef::new(
        BRAND,
        "236",
        "Cinnamon",
        &[226, 161, 136],
    ),
    ThreadRef::new(
        BRAND,
        "237",
        "Umber",
        &[181, 148, 116],
    ),
    ThreadRef::new(
        BRAND,
        "238",
        "Blond",
        &[245, 219, 139],
    ),
    ThreadRef::new(
        BRAND,
        "239",
        "Sunflower",
        &[255, 204, 0],
    ),
    ThreadRef::new(
        BRAND,
        "240",
        "Orchid Pink",
        &[255, 189, 227],
    ),
    ThreadRef::new(
        BRAND,
        "241",
        "Peony Purple",
        &[195, 0, 126],
    ),
    ThreadRef::new(
        BRAND,
        "242",
        "Burgundy",
        &[168, 0, 67],
    ),
    ThreadRef::new(
        BRAND,
        "243",
        "Royal Purple",
        &[84, 5, 113],
    ),
    ThreadRef::new(
        BRAND,
        "244",
        "Cardinal Red",
        &[255, 9, 39],
    ),
    ThreadRef::new(
        BRAND,
        "245",
        "Opal Green",
        &[198, 238, 203],
    ),
    ThreadRef::new(
        BRAND,
        "246",
        "Moss Green",
        &[96, 133, 65],
    ),
    ThreadRef::new(
        BRAND,
        "247",
        "Meadow Green",
        &[96, 148, 24],
    ),
    ThreadRef::new(
        BRAND,
        "248",
        "Dark Green",
        &[6, 72, 13],
    ),
    ThreadRef::new(
        BRAND,
        "249",
        "Aquamarine",
        &[91, 210, 181],
    ),
    ThreadRef::new(
        BRAND,
        "250",
        "Emerald Green",
        &[76, 181, 143],
    ),
    ThreadRef::new(
        BRAND,
        "251",
        "Peacock Green",
        &[4, 145, 123],
    ),
    ThreadRef::new(
        BRAND,
        "252",
        "Dark Gray",
        &[89, 91, 97],
    ),
    ThreadRef::new(
        BRAND,
        "253",
        "Ivory White",
        &[255, 255, 220],
    ),
    ThreadRef::new(
        BRAND,
        "202",
        "Vermilion",
        &[255, 71, 32],
    ),
    ThreadRef::new(
        BRAND,
        "206",
        "Bright Green",
        &[0, 181, 82],
    ),
    ThreadRef::new(
        BRAND,
        "222",
        "Ocean Blue",
        &[2, 87, 181],
    ),
    ThreadRef::new(
        BRAND,
        "223",
        "Beige Gray",
        &[208, 186, 176],
    ),
    ThreadRef::new(
        BRAND,
        "224",
        "Bamboo",
        &[227, 190, 129],
    ),
    ThreadRef::new(
        BRAND,
        "254",
        "Hazel",
        &[230, 101, 30],
    ),
    ThreadRef::new(
        BRAND,
        "255",
        "Toast",
        &[230, 150, 90],
    ),
    ThreadRef::new(
        BRAND,
        "256",
        "Salmon",
        &[240, 156, 150],
    ),
    ThreadRef::new(
        BRAND,
        "257",
        "Cocoa Brown",
        &[167, 108, 61],
    ),
    ThreadRef::new(
        BRAND,
        "258",
        "Sienna",
        &[180, 90, 48],
    ),
    ThreadRef::new(
        BRAND,
        "259",
        "Sepia",
        &[110, 57, 55],
    ),
    ThreadRef::new(
        BRAND,
        "260",
        "Dark Sepia",
        &[92, 38, 37],
    ),
    ThreadRef::new(
        BRAND,
        "261",
        "Violet Blue",
        &[98, 49, 189],
    ),
    ThreadRef::new(
        BRAND,
        "262",
        "Blue Ink",
        &[20, 50, 156],
    ),
    ThreadRef::new(
        BRAND,
        "263",
        "Solar Blue",
        &[22, 95, 167],
    ),
    ThreadRef::new(
        BRAND,
        "264",
        "Green Dust",
        &[196, 227, 157],
    ),
    ThreadRef::new(
        BRAND,
        "265",
        "Crimson",
        &[253, 51, 163],
    ),
    ThreadRef::new(
        BRAND,
        "266",
        "Floral Pink",
        &[238, 113, 175],
    ),
    ThreadRef::new(
        BRAND,
        "267",
        "Wine",
        &[132, 49, 84],
    ),
    ThreadRef::new(
        BRAND,
        "268",
        "Olive Drab",
        &[163, 145, 102],
    ),
    ThreadRef::new(
        BRAND,
        "269",
        "Meadow",
        &[12, 137, 24],
    ),
    ThreadRef::new(
        BRAND,
        "270",
        "Mustard",
        &[247, 242, 151],
    ),
    ThreadRef::new(
        BRAND,
        "271",
        "Yellow Ocher",
        &[204, 153, 0],
    ),
    ThreadRef::new(
        BRAND,
        "272",
        "Old Gold",
        &[199, 151, 50],
    ),
    ThreadRef::new(
        BRAND,
        "273",
        "Honey Dew",
        &[255, 157, 0],
    ),
    ThreadRef::new(
        BRAND,
        "274",
        "Tangerine",
        &[255, 186, 94],
    ),
    ThreadRef::new(
        BRAND,
        "275",
        "Canary Yellow",
        &[252, 241, 33],
    ),
];