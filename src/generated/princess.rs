#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Princess";
pub const THREADS: [ThreadRef; 120] = [
    ThreadRef::new(
        BRAND,
        "1-1",
        "White",
        &[255, 255, 255],
    ),
    ThreadRef::new(
        BRAND,
        "1-2",
        "Grey",
        &[216, 205, 212],
    ),
    ThreadRef::new(
        BRAND,
        "1-3",
        "Pink",
        &[230, 60, 102],
    ),
    ThreadRef::new(
        BRAND,
        "1-4",
        "Lt. Olive",
        &[109, 132, 72],
    ),
    ThreadRef::new(
        BRAND,
        "1-5",
        "Yellow",
        &[255, 210, 60],
    ),
    ThreadRef::new(
        BRAND,
        "1-6",
        "Black",
        &[0, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1-7",
        "Mid Blue",
        &[94, 140, 176],
    ),
    ThreadRef::new(
        BRAND,
        "1-8",
        "Red",
        &[255, 0, 3],
    ),
    ThreadRef::new(
        BRAND,
        "1-9",
        "Green",
        &[41, 122, 30],
    ),
    ThreadRef::new(
        BRAND,
        "1-10",
        "Brown",
        &[89, 23, 23],
    ),
    ThreadRef::new(
        BRAND,
        "2-1",
        "Cream",
        &[232, 221, 192],
    ),
    ThreadRef::new(
        BRAND,
        "2-2",
        "Gold",
        &[227, 130, 70],
    ),
    ThreadRef::new(
        BRAND,
        "2-3",
        "Light Brown",
        &[180, 114, 83],
    ),
    ThreadRef::new(
        BRAND,
        "2-4",
        "Orange",
        &[180, 45, 48],
    ),
    ThreadRef::new(
        BRAND,
        "2-5",
        "Purple",
        &[161, 63, 125],
    ),
    ThreadRef::new(
        BRAND,
        "2-6",
        "Pewter",
        &[106, 112, 115],
    ),
    ThreadRef::new(
        BRAND,
        "2-7",
        "Cyan",
        &[0, 152, 160],
    ),
    ThreadRef::new(
        BRAND,
        "2-8",
        "Dark Blue",
        &[23, 50, 142],
    ),
    ThreadRef::new(
        BRAND,
        "2-9",
        "Olive",
        &[60, 78, 44],
    ),
    ThreadRef::new(
        BRAND,
        "2-10",
        "Burgundy",
        &[128, 36, 38],
    ),
    ThreadRef::new(
        BRAND,
        "3-1",
        "Palest Ivory",
        &[237, 228, 216],
    ),
    ThreadRef::new(
        BRAND,
        "3-2",
        "Palest Yellow",
        &[232, 210, 168],
    ),
    ThreadRef::new(
        BRAND,
        "3-3",
        "Palest Fawn",
        &[230, 198, 182],
    ),
    ThreadRef::new(
        BRAND,
        "3-4",
        "Palest Peach",
        &[235, 208, 203],
    ),
    ThreadRef::new(
        BRAND,
        "3-5",
        "Palest Pink",
        &[238, 200, 220],
    ),
    ThreadRef::new(
        BRAND,
        "3-6",
        "Palest Grey",
        &[220, 216, 225],
    ),
    ThreadRef::new(
        BRAND,
        "3-7",
        "Palest Blue",
        &[209, 216, 231],
    ),
    ThreadRef::new(
        BRAND,
        "3-8",
        "Palest Aqua",
        &[206, 250, 244],
    ),
    ThreadRef::new(
        BRAND,
        "3-9",
        "Palest Green",
        &[155, 182, 133],
    ),
    ThreadRef::new(
        BRAND,
        "3-10",
        "Palest Mauve",
        &[211, 187, 213],
    ),
    ThreadRef::new(
        BRAND,
        "4-1",
        "Light Blue",
        &[182, 200, 225],
    ),
    ThreadRef::new(
        BRAND,
        "4-2",
        "Powder Blue",
        &[135, 160, 212],
    ),
    ThreadRef::new(
        BRAND,
        "4-3",
        "Sky Blue",
        &[114, 119, 199],
    ),
    ThreadRef::new(
        BRAND,
        "4-4",
        "Slate Blue",
        &[62, 95, 151],
    ),
    ThreadRef::new(
        BRAND,
        "4-5",
        "Storm Blue",
        &[85, 87, 122],
    ),
    ThreadRef::new(
        BRAND,
        "4-6",
        "Denim",
        &[38, 42, 92],
    ),
    ThreadRef::new(
        BRAND,
        "4-7",
        "Petrel Blue",
        &[10, 75, 140],
    ),
    ThreadRef::new(
        BRAND,
        "4-8",
        "Imperial Blue",
        &[17, 27, 94],
    ),
    ThreadRef::new(
        BRAND,
        "4-9",
        "Royal Blue",
        &[14, 34, 114],
    ),
    ThreadRef::new(
        BRAND,
        "4-10",
        "Navy Blue",
        &[18, 17, 59],
    ),
    ThreadRef::new(
        BRAND,
        "5-1",
        "Copper",
        &[230, 120, 80],
    ),
    ThreadRef::new(
        BRAND,
        "5-2",
        "Golden Brown",
        &[215, 107, 60],
    ),
    ThreadRef::new(
        BRAND,
        "5-3",
        "Tan",
        &[110, 40, 25],
    ),
    ThreadRef::new(
        BRAND,
        "5-4",
        "Bark",
        &[116, 37, 20],
    ),
    ThreadRef::new(
        BRAND,
        "5-5",
        "Dark Oak",
        &[75, 27, 18],
    ),
    ThreadRef::new(
        BRAND,
        "5-6",
        "Russet",
        &[130, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "5-7",
        "Mahogany",
        &[80, 20, 18],
    ),
    ThreadRef::new(
        BRAND,
        "5-8",
        "Mushroom",
        &[120, 84, 54],
    ),
    ThreadRef::new(
        BRAND,
        "5-9",
        "Khaki",
        &[91, 60, 17],
    ),
    ThreadRef::new(
        BRAND,
        "5-10",
        "Dark Chocolate",
        &[68, 1, 0],
    ),
    ThreadRef::new(
        BRAND,
        "6-1",
        "Light Salmon",
        &[248, 189, 155],
    ),
    ThreadRef::new(
        BRAND,
        "6-2",
        "Salmon",
        &[243, 138, 103],
    ),
    ThreadRef::new(
        BRAND,
        "6-3",
        "Tangerine",
        &[242, 82, 56],
    ),
    ThreadRef::new(
        BRAND,
        "6-4",
        "Blush",
        &[230, 145, 120],
    ),
    ThreadRef::new(
        BRAND,
        "6-5",
        "Bronze",
        &[184, 90, 60],
    ),
    ThreadRef::new(
        BRAND,
        "6-6",
        "Scarlet",
        &[232, 41, 37],
    ),
    ThreadRef::new(
        BRAND,
        "6-7",
        "Rust",
        &[202, 32, 23],
    ),
    ThreadRef::new(
        BRAND,
        "6-8",
        "Carmine",
        &[210, 4, 20],
    ),
    ThreadRef::new(
        BRAND,
        "6-9",
        "Ruby",
        &[231, 0, 28],
    ),
    ThreadRef::new(
        BRAND,
        "6-10",
        "Antique Red",
        &[224, 47, 52],
    ),
    ThreadRef::new(
        BRAND,
        "7-1",
        "Light Jade",
        &[144, 214, 130],
    ),
    ThreadRef::new(
        BRAND,
        "7-2",
        "Pistachio",
        &[158, 164, 70],
    ),
    ThreadRef::new(
        BRAND,
        "7-3",
        "Fresh Green",
        &[170, 201, 60],
    ),
    ThreadRef::new(
        BRAND,
        "7-4",
        "Sage",
        &[75, 148, 81],
    ),
    ThreadRef::new(
        BRAND,
        "7-5",
        "Teal",
        &[64, 96, 64],
    ),
    ThreadRef::new(
        BRAND,
        "7-6",
        "Bright Green",
        &[54, 143, 52],
    ),
    ThreadRef::new(
        BRAND,
        "7-7",
        "Jade",
        &[41, 118, 66],
    ),
    ThreadRef::new(
        BRAND,
        "7-8",
        "Leaf Green",
        &[58, 130, 60],
    ),
    ThreadRef::new(
        BRAND,
        "7-9",
        "Emerald",
        &[30, 145, 41],
    ),
    ThreadRef::new(
        BRAND,
        "7-10",
        "Dark Green",
        &[11, 96, 32],
    ),
    ThreadRef::new(
        BRAND,
        "8-1",
        "Sugar Pink",
        &[245, 192, 200],
    ),
    ThreadRef::new(
        BRAND,
        "8-2",
        "Petal Pink",
        &[250, 170, 190],
    ),
    ThreadRef::new(
        BRAND,
        "8-3",
        "Rose Pink",
        &[250, 134, 160],
    ),
    ThreadRef::new(
        BRAND,
        "8-4",
        "Passion Pink",
        &[208, 102, 144],
    ),
    ThreadRef::new(
        BRAND,
        "8-5",
        "Dusky Rose",
        &[190, 52, 85],
    ),
    ThreadRef::new(
        BRAND,
        "8-6",
        "Light Cerise",
        &[241, 105, 133],
    ),
    ThreadRef::new(
        BRAND,
        "8-7",
        "Dusky Pink",
        &[239, 91, 111],
    ),
    ThreadRef::new(
        BRAND,
        "8-8",
        "Cerise",
        &[220, 0, 74],
    ),
    ThreadRef::new(
        BRAND,
        "8-9",
        "Fuschia",
        &[255, 0, 118],
    ),
    ThreadRef::new(
        BRAND,
        "8-10",
        "Raspberry",
        &[149, 36, 80],
    ),
    ThreadRef::new(
        BRAND,
        "9-1",
        "Platinum",
        &[244, 241, 236],
    ),
    ThreadRef::new(
        BRAND,
        "9-2",
        "Silver Fox",
        &[185, 164, 183],
    ),
    ThreadRef::new(
        BRAND,
        "9-3",
        "Sleet Grey",
        &[176, 176, 187],
    ),
    ThreadRef::new(
        BRAND,
        "9-4",
        "Silver Blue",
        &[116, 121, 146],
    ),
    ThreadRef::new(
        BRAND,
        "9-5",
        "Silver Moon",
        &[150, 140, 142],
    ),
    ThreadRef::new(
        BRAND,
        "9-6",
        "Willow",
        &[138, 125, 102],
    ),
    ThreadRef::new(
        BRAND,
        "9-7",
        "Dark Grey",
        &[102, 83, 102],
    ),
    ThreadRef::new(
        BRAND,
        "9-8",
        "Steel",
        &[86, 83, 108],
    ),
    ThreadRef::new(
        BRAND,
        "9-9",
        "Midnight",
        &[75, 71, 80],
    ),
    ThreadRef::new(
        BRAND,
        "9-10",
        "Seal",
        &[36, 36, 54],
    ),
    ThreadRef::new(
        BRAND,
        "10-1",
        "Pacific Mist",
        &[162, 251, 230],
    ),
    ThreadRef::new(
        BRAND,
        "10-2",
        "Mint",
        &[184, 231, 202],
    ),
    ThreadRef::new(
        BRAND,
        "10-3",
        "Aqua",
        &[149, 244, 219],
    ),
    ThreadRef::new(
        BRAND,
        "10-4",
        "Capri",
        &[49, 214, 227],
    ),
    ThreadRef::new(
        BRAND,
        "10-5",
        "Lagoon",
        &[70, 140, 145],
    ),
    ThreadRef::new(
        BRAND,
        "10-6",
        "Kingfisher",
        &[21, 151, 174],
    ),
    ThreadRef::new(
        BRAND,
        "10-7",
        "Lido",
        &[51, 142, 103],
    ),
    ThreadRef::new(
        BRAND,
        "10-8",
        "Tropical Green",
        &[40, 111, 68],
    ),
    ThreadRef::new(
        BRAND,
        "10-9",
        "Deep Atlantic",
        &[34, 96, 100],
    ),
    ThreadRef::new(
        BRAND,
        "10-10",
        "Ocean Depths",
        &[40, 67, 70],
    ),
    ThreadRef::new(
        BRAND,
        "11-1",
        "Delicate Mauve",
        &[255, 210, 220],
    ),
    ThreadRef::new(
        BRAND,
        "11-2",
        "Pale Lilac",
        &[225, 186, 233],
    ),
    ThreadRef::new(
        BRAND,
        "11-3",
        "Lilac",
        &[159, 107, 156],
    ),
    ThreadRef::new(
        BRAND,
        "11-4",
        "Lavender",
        &[196, 102, 168],
    ),
    ThreadRef::new(
        BRAND,
        "11-5",
        "Violet",
        &[151, 76, 148],
    ),
    ThreadRef::new(
        BRAND,
        "11-6",
        "Clematis",
        &[189, 75, 151],
    ),
    ThreadRef::new(
        BRAND,
        "11-7",
        "Imperial Purple",
        &[93, 59, 123],
    ),
    ThreadRef::new(
        BRAND,
        "11-8",
        "Amethyst",
        &[142, 95, 196],
    ),
    ThreadRef::new(
        BRAND,
        "11-9",
        "Deep Violet",
        &[129, 58, 126],
    ),
    ThreadRef::new(
        BRAND,
        "11-10",
        "Cassis",
        &[63, 53, 102],
    ),
    ThreadRef::new(
        BRAND,
        "12-1",
        "Pale Lemon",
        &[254, 246, 196],
    ),
    ThreadRef::new(
        BRAND,
        "12-2",
        "Mimosa",
        &[248, 187, 85],
    ),
    ThreadRef::new(
        BRAND,
        "12-3",
        "Lime",
        &[251, 251, 66],
    ),
    ThreadRef::new(
        BRAND,
        "12-4",
        "Daffodil",
        &[249, 210, 113],
    ),
    ThreadRef::new(
        BRAND,
        "12-5",
        "Sunkissed",
        &[215, 155, 96],
    ),
    ThreadRef::new(
        BRAND,
        "12-6",
        "Golden Lights",
        &[245, 180, 34],
    ),
    ThreadRef::new(
        BRAND,
        "12-7",
        "Soleil",
        &[255, 124, 28],
    ),
    ThreadRef::new(
        BRAND,
        "12-8",
        "Golden Glory",
        &[225, 72, 28],
    ),
    ThreadRef::new(
        BRAND,
        "12-9",
        "Sunburst",
        &[211, 131, 71],
    ),
    ThreadRef::new(
        BRAND,
        "12-10",
        "Marigold",
        &[214, 92, 48],
    ),
];