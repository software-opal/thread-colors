#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Simthread";
pub const THREADS: [ThreadRef; 107] = [
    ThreadRef::new(
        BRAND,
        "S101",
        "Jet Black",
        &[0, 8, 11],
    ),
    ThreadRef::new(
        BRAND,
        "S102",
        "Powder Blue Tint",
        &[227, 226, 232],
    ),
    ThreadRef::new(
        BRAND,
        "S110",
        "Ecru Very Light",
        &[244, 240, 219],
    ),
    ThreadRef::new(
        BRAND,
        "S115",
        "Sheer Ivory",
        &[238, 233, 195],
    ),
    ThreadRef::new(
        BRAND,
        "S120",
        "Pink Sand",
        &[225, 188, 143],
    ),
    ThreadRef::new(
        BRAND,
        "S121",
        "Pale Tan",
        &[186, 148, 129],
    ),
    ThreadRef::new(
        BRAND,
        "S122",
        "Stone Grey",
        &[172, 165, 173],
    ),
    ThreadRef::new(
        BRAND,
        "S123",
        "Pale Grey",
        &[168, 163, 157],
    ),
    ThreadRef::new(
        BRAND,
        "S125",
        "Roebuck",
        &[182, 142, 117],
    ),
    ThreadRef::new(
        BRAND,
        "S126",
        "Woodsmoke",
        &[152, 124, 102],
    ),
    ThreadRef::new(
        BRAND,
        "S127",
        "Midnight",
        &[76, 66, 77],
    ),
    ThreadRef::new(
        BRAND,
        "S130",
        "Purple",
        &[195, 119, 187],
    ),
    ThreadRef::new(
        BRAND,
        "S131",
        "Rust",
        &[204, 87, 44],
    ),
    ThreadRef::new(
        BRAND,
        "S135",
        "Cranberry",
        &[111, 38, 19],
    ),
    ThreadRef::new(
        BRAND,
        "S140",
        "Cherrywood",
        &[92, 52, 44],
    ),
    ThreadRef::new(
        BRAND,
        "S143",
        "Dark Purple/Navy",
        &[43, 18, 50],
    ),
    ThreadRef::new(
        BRAND,
        "S145",
        "Midnight Blue",
        &[28, 26, 27],
    ),
    ThreadRef::new(
        BRAND,
        "S195",
        "Celery",
        &[240, 236, 188],
    ),
    ThreadRef::new(
        BRAND,
        "S196",
        "Linen",
        &[255, 227, 162],
    ),
    ThreadRef::new(
        BRAND,
        "S200",
        "Medium Yellow",
        &[232, 230, 121],
    ),
    ThreadRef::new(
        BRAND,
        "S201",
        "Cream Yellow",
        &[255, 233, 131],
    ),
    ThreadRef::new(
        BRAND,
        "S205",
        "Liberty Gold",
        &[222, 194, 0],
    ),
    ThreadRef::new(
        BRAND,
        "S206",
        "Cornsilk",
        &[255, 208, 30],
    ),
    ThreadRef::new(
        BRAND,
        "S207",
        "Gold/Orange",
        &[230, 160, 24],
    ),
    ThreadRef::new(
        BRAND,
        "S210",
        "Soleil",
        &[242, 128, 24],
    ),
    ThreadRef::new(
        BRAND,
        "S211",
        "Orange Spice medium",
        &[244, 123, 68],
    ),
    ThreadRef::new(
        BRAND,
        "S212",
        "Inca Gold",
        &[221, 121, 33],
    ),
    ThreadRef::new(
        BRAND,
        "S215",
        "Red",
        &[230, 17, 9],
    ),
    ThreadRef::new(
        BRAND,
        "S220",
        "Paprika",
        &[255, 55, 27],
    ),
    ThreadRef::new(
        BRAND,
        "S221",
        "Scarlet",
        &[171, 19, 18],
    ),
    ThreadRef::new(
        BRAND,
        "S224",
        "Autumn Green",
        &[163, 122, 2],
    ),
    ThreadRef::new(
        BRAND,
        "S225",
        "Tan Brown",
        &[173, 107, 33],
    ),
    ThreadRef::new(
        BRAND,
        "S226",
        "Hazel",
        &[103, 70, 3],
    ),
    ThreadRef::new(
        BRAND,
        "S300",
        "Lt. Green",
        &[148, 236, 97],
    ),
    ThreadRef::new(
        BRAND,
        "S305",
        "Fluorescent Green",
        &[81, 213, 6],
    ),
    ThreadRef::new(
        BRAND,
        "S306",
        "Emerald",
        &[43, 184, 0],
    ),
    ThreadRef::new(
        BRAND,
        "S307",
        "Springtime green",
        &[128, 187, 97],
    ),
    ThreadRef::new(
        BRAND,
        "S308",
        "Green Forest",
        &[28, 93, 0],
    ),
    ThreadRef::new(
        BRAND,
        "S310",
        "Meadow",
        &[18, 163, 0],
    ),
    ThreadRef::new(
        BRAND,
        "S312",
        "Dark Green",
        &[30, 70, 44],
    ),
    ThreadRef::new(
        BRAND,
        "S313",
        "Evergreen",
        &[2, 64, 0],
    ),
    ThreadRef::new(
        BRAND,
        "S314",
        "Olive Green",
        &[13, 37, 37],
    ),
    ThreadRef::new(
        BRAND,
        "S315",
        "Sharp Green",
        &[194, 233, 90],
    ),
    ThreadRef::new(
        BRAND,
        "S316",
        "Sienna",
        &[72, 58, 23],
    ),
    ThreadRef::new(
        BRAND,
        "S320",
        "Khaki",
        &[73, 69, 8],
    ),
    ThreadRef::new(
        BRAND,
        "S325",
        "Blue Spruce",
        &[41, 73, 23],
    ),
    ThreadRef::new(
        BRAND,
        "S400",
        "Slightly Green",
        &[184, 248, 196],
    ),
    ThreadRef::new(
        BRAND,
        "S401",
        "Green",
        &[103, 206, 71],
    ),
    ThreadRef::new(
        BRAND,
        "S405",
        "Green",
        &[54, 224, 92],
    ),
    ThreadRef::new(
        BRAND,
        "S406",
        "Meadow",
        &[19, 144, 24],
    ),
    ThreadRef::new(
        BRAND,
        "S407",
        "Dark Green",
        &[0, 123, 0],
    ),
    ThreadRef::new(
        BRAND,
        "S408",
        "Powder Blue",
        &[137, 240, 247],
    ),
    ThreadRef::new(
        BRAND,
        "S409",
        "Medium Ocean Blue",
        &[37, 152, 209],
    ),
    ThreadRef::new(
        BRAND,
        "S410",
        "Turquoise",
        &[26, 150, 114],
    ),
    ThreadRef::new(
        BRAND,
        "S411",
        "Green/Blue",
        &[17, 134, 126],
    ),
    ThreadRef::new(
        BRAND,
        "S415",
        "Emerald Green",
        &[26, 147, 93],
    ),
    ThreadRef::new(
        BRAND,
        "S416",
        "Dark Green",
        &[5, 41, 15],
    ),
    ThreadRef::new(
        BRAND,
        "S420",
        "Crystal Blue",
        &[167, 207, 215],
    ),
    ThreadRef::new(
        BRAND,
        "S421",
        "Lake Blue",
        &[150, 199, 255],
    ),
    ThreadRef::new(
        BRAND,
        "S422",
        "Medium Blue",
        &[144, 171, 238],
    ),
    ThreadRef::new(
        BRAND,
        "S423",
        "Dark Royal Blue",
        &[42, 60, 136],
    ),
    ThreadRef::new(
        BRAND,
        "S424",
        "Dk. Royal Blue",
        &[27, 37, 108],
    ),
    ThreadRef::new(
        BRAND,
        "S425",
        "Sky Blue",
        &[91, 192, 246],
    ),
    ThreadRef::new(
        BRAND,
        "S426",
        "California Blue",
        &[0, 109, 207],
    ),
    ThreadRef::new(
        BRAND,
        "S427",
        "Dolphin Blue",
        &[48, 93, 238],
    ),
    ThreadRef::new(
        BRAND,
        "S428",
        "Dark Blue",
        &[28, 98, 220],
    ),
    ThreadRef::new(
        BRAND,
        "S429",
        "Brilliant Blue",
        &[6, 38, 141],
    ),
    ThreadRef::new(
        BRAND,
        "S430",
        "Ultra Marine",
        &[0, 11, 228],
    ),
    ThreadRef::new(
        BRAND,
        "S431",
        "Dark Blue",
        &[24, 42, 140],
    ),
    ThreadRef::new(
        BRAND,
        "S435",
        "Dark Purple",
        &[0, 0, 146],
    ),
    ThreadRef::new(
        BRAND,
        "S500",
        "Light Pink",
        &[238, 205, 232],
    ),
    ThreadRef::new(
        BRAND,
        "S501",
        "Medium Pink",
        &[232, 100, 209],
    ),
    ThreadRef::new(
        BRAND,
        "S505",
        "Poker Primrose",
        &[205, 114, 253],
    ),
    ThreadRef::new(
        BRAND,
        "S506",
        "Amethyst",
        &[189, 30, 236],
    ),
    ThreadRef::new(
        BRAND,
        "S507",
        "Light Denim",
        &[162, 2, 248],
    ),
    ThreadRef::new(
        BRAND,
        "S510",
        "Tropicana",
        &[239, 73, 159],
    ),
    ThreadRef::new(
        BRAND,
        "S515",
        "Purple",
        &[119, 15, 236],
    ),
    ThreadRef::new(
        BRAND,
        "S520",
        "Jacaranda",
        &[149, 6, 182],
    ),
    ThreadRef::new(
        BRAND,
        "S525",
        "Maroon",
        &[227, 7, 45],
    ),
    ThreadRef::new(
        BRAND,
        "S526",
        "Dark Red",
        &[142, 3, 42],
    ),
    ThreadRef::new(
        BRAND,
        "S530",
        "Violet Very Dark",
        &[94, 11, 101],
    ),
    ThreadRef::new(
        BRAND,
        "S531",
        "Royal Purple",
        &[72, 1, 95],
    ),
    ThreadRef::new(
        BRAND,
        "S535",
        "Dark Fuschia",
        &[131, 6, 106],
    ),
    ThreadRef::new(
        BRAND,
        "S540",
        "Midnight Navy",
        &[24, 0, 71],
    ),
    ThreadRef::new(
        BRAND,
        "S600",
        "Light Brass",
        &[242, 177, 135],
    ),
    ThreadRef::new(
        BRAND,
        "S605",
        "Orchid Pink",
        &[238, 177, 184],
    ),
    ThreadRef::new(
        BRAND,
        "S606",
        "Pink Petunia",
        &[239, 133, 181],
    ),
    ThreadRef::new(
        BRAND,
        "S607",
        "Lavender",
        &[209, 99, 185],
    ),
    ThreadRef::new(
        BRAND,
        "S610",
        "Garden Rose",
        &[228, 58, 151],
    ),
    ThreadRef::new(
        BRAND,
        "S612",
        "Begonia",
        &[255, 79, 133],
    ),
    ThreadRef::new(
        BRAND,
        "S613",
        "Violet",
        &[206, 57, 187],
    ),
    ThreadRef::new(
        BRAND,
        "S695",
        "Jacaranda",
        &[154, 97, 238],
    ),
    ThreadRef::new(
        BRAND,
        "S696",
        "Violet Blue",
        &[98, 58, 207],
    ),
    ThreadRef::new(
        BRAND,
        "S697",
        "Corsican Blue",
        &[112, 93, 157],
    ),
    ThreadRef::new(
        BRAND,
        "S700",
        "Cindy Purple",
        &[121, 41, 254],
    ),
    ThreadRef::new(
        BRAND,
        "S701",
        "Sky Blue",
        &[96, 35, 252],
    ),
    ThreadRef::new(
        BRAND,
        "S702",
        "Sapphire",
        &[83, 20, 248],
    ),
    ThreadRef::new(
        BRAND,
        "S703",
        "Marine Blue",
        &[54, 3, 192],
    ),
    ThreadRef::new(
        BRAND,
        "S704",
        "Bright Blue",
        &[16, 0, 185],
    ),
    ThreadRef::new(
        BRAND,
        "S705",
        "Purple",
        &[108, 66, 145],
    ),
    ThreadRef::new(
        BRAND,
        "S800",
        "Pale Lilac",
        &[196, 182, 217],
    ),
    ThreadRef::new(
        BRAND,
        "S801",
        "Light Blue",
        &[206, 236, 224],
    ),
    ThreadRef::new(
        BRAND,
        "S802",
        "Geisha",
        &[200, 196, 231],
    ),
    ThreadRef::new(
        BRAND,
        "S803",
        "Oriental Blue",
        &[124, 136, 186],
    ),
    ThreadRef::new(
        BRAND,
        "S805",
        "Silver Blue",
        &[100, 115, 144],
    ),
    ThreadRef::new(
        BRAND,
        "S810",
        "Ash",
        &[62, 69, 113],
    ),
    ThreadRef::new(
        BRAND,
        "S815",
        "Night Sky",
        &[43, 50, 42],
    ),
];