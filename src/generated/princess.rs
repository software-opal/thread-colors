#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "Princess";
const THREADS: [ThreadRef; 120] = [
    ThreadRef {
        brand: BRAND,
        code: "1-1",
        name: "White",
        color: &[255, 255, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "1-10",
        name: "Brown",
        color: &[89, 23, 23],
    },
    ThreadRef {
        brand: BRAND,
        code: "1-2",
        name: "Grey",
        color: &[216, 205, 212],
    },
    ThreadRef {
        brand: BRAND,
        code: "1-3",
        name: "Pink",
        color: &[230, 60, 102],
    },
    ThreadRef {
        brand: BRAND,
        code: "1-4",
        name: "Lt. Olive",
        color: &[109, 132, 72],
    },
    ThreadRef {
        brand: BRAND,
        code: "1-5",
        name: "Yellow",
        color: &[255, 210, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "1-6",
        name: "Black",
        color: &[0, 0, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "1-7",
        name: "Mid Blue",
        color: &[94, 140, 176],
    },
    ThreadRef {
        brand: BRAND,
        code: "1-8",
        name: "Red",
        color: &[255, 0, 3],
    },
    ThreadRef {
        brand: BRAND,
        code: "1-9",
        name: "Green",
        color: &[41, 122, 30],
    },
    ThreadRef {
        brand: BRAND,
        code: "10-1",
        name: "Pacific Mist",
        color: &[162, 251, 230],
    },
    ThreadRef {
        brand: BRAND,
        code: "10-10",
        name: "Ocean Depths",
        color: &[40, 67, 70],
    },
    ThreadRef {
        brand: BRAND,
        code: "10-2",
        name: "Mint",
        color: &[184, 231, 202],
    },
    ThreadRef {
        brand: BRAND,
        code: "10-3",
        name: "Aqua",
        color: &[149, 244, 219],
    },
    ThreadRef {
        brand: BRAND,
        code: "10-4",
        name: "Capri",
        color: &[49, 214, 227],
    },
    ThreadRef {
        brand: BRAND,
        code: "10-5",
        name: "Lagoon",
        color: &[70, 140, 145],
    },
    ThreadRef {
        brand: BRAND,
        code: "10-6",
        name: "Kingfisher",
        color: &[21, 151, 174],
    },
    ThreadRef {
        brand: BRAND,
        code: "10-7",
        name: "Lido",
        color: &[51, 142, 103],
    },
    ThreadRef {
        brand: BRAND,
        code: "10-8",
        name: "Tropical Green",
        color: &[40, 111, 68],
    },
    ThreadRef {
        brand: BRAND,
        code: "10-9",
        name: "Deep Atlantic",
        color: &[34, 96, 100],
    },
    ThreadRef {
        brand: BRAND,
        code: "11-1",
        name: "Delicate Mauve",
        color: &[255, 210, 220],
    },
    ThreadRef {
        brand: BRAND,
        code: "11-10",
        name: "Cassis",
        color: &[63, 53, 102],
    },
    ThreadRef {
        brand: BRAND,
        code: "11-2",
        name: "Pale Lilac",
        color: &[225, 186, 233],
    },
    ThreadRef {
        brand: BRAND,
        code: "11-3",
        name: "Lilac",
        color: &[159, 107, 156],
    },
    ThreadRef {
        brand: BRAND,
        code: "11-4",
        name: "Lavender",
        color: &[196, 102, 168],
    },
    ThreadRef {
        brand: BRAND,
        code: "11-5",
        name: "Violet",
        color: &[151, 76, 148],
    },
    ThreadRef {
        brand: BRAND,
        code: "11-6",
        name: "Clematis",
        color: &[189, 75, 151],
    },
    ThreadRef {
        brand: BRAND,
        code: "11-7",
        name: "Imperial Purple",
        color: &[93, 59, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "11-8",
        name: "Amethyst",
        color: &[142, 95, 196],
    },
    ThreadRef {
        brand: BRAND,
        code: "11-9",
        name: "Deep Violet",
        color: &[129, 58, 126],
    },
    ThreadRef {
        brand: BRAND,
        code: "12-1",
        name: "Pale Lemon",
        color: &[254, 246, 196],
    },
    ThreadRef {
        brand: BRAND,
        code: "12-10",
        name: "Marigold",
        color: &[214, 92, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "12-2",
        name: "Mimosa",
        color: &[248, 187, 85],
    },
    ThreadRef {
        brand: BRAND,
        code: "12-3",
        name: "Lime",
        color: &[251, 251, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "12-4",
        name: "Daffodil",
        color: &[249, 210, 113],
    },
    ThreadRef {
        brand: BRAND,
        code: "12-5",
        name: "Sunkissed",
        color: &[215, 155, 96],
    },
    ThreadRef {
        brand: BRAND,
        code: "12-6",
        name: "Golden Lights",
        color: &[245, 180, 34],
    },
    ThreadRef {
        brand: BRAND,
        code: "12-7",
        name: "Soleil",
        color: &[255, 124, 28],
    },
    ThreadRef {
        brand: BRAND,
        code: "12-8",
        name: "Golden Glory",
        color: &[225, 72, 28],
    },
    ThreadRef {
        brand: BRAND,
        code: "12-9",
        name: "Sunburst",
        color: &[211, 131, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "2-1",
        name: "Cream",
        color: &[232, 221, 192],
    },
    ThreadRef {
        brand: BRAND,
        code: "2-10",
        name: "Burgundy",
        color: &[128, 36, 38],
    },
    ThreadRef {
        brand: BRAND,
        code: "2-2",
        name: "Gold",
        color: &[227, 130, 70],
    },
    ThreadRef {
        brand: BRAND,
        code: "2-3",
        name: "Light Brown",
        color: &[180, 114, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "2-4",
        name: "Orange",
        color: &[180, 45, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "2-5",
        name: "Purple",
        color: &[161, 63, 125],
    },
    ThreadRef {
        brand: BRAND,
        code: "2-6",
        name: "Pewter",
        color: &[106, 112, 115],
    },
    ThreadRef {
        brand: BRAND,
        code: "2-7",
        name: "Cyan",
        color: &[0, 152, 160],
    },
    ThreadRef {
        brand: BRAND,
        code: "2-8",
        name: "Dark Blue",
        color: &[23, 50, 142],
    },
    ThreadRef {
        brand: BRAND,
        code: "2-9",
        name: "Olive",
        color: &[60, 78, 44],
    },
    ThreadRef {
        brand: BRAND,
        code: "3-1",
        name: "Palest Ivory",
        color: &[237, 228, 216],
    },
    ThreadRef {
        brand: BRAND,
        code: "3-10",
        name: "Palest Mauve",
        color: &[211, 187, 213],
    },
    ThreadRef {
        brand: BRAND,
        code: "3-2",
        name: "Palest Yellow",
        color: &[232, 210, 168],
    },
    ThreadRef {
        brand: BRAND,
        code: "3-3",
        name: "Palest Fawn",
        color: &[230, 198, 182],
    },
    ThreadRef {
        brand: BRAND,
        code: "3-4",
        name: "Palest Peach",
        color: &[235, 208, 203],
    },
    ThreadRef {
        brand: BRAND,
        code: "3-5",
        name: "Palest Pink",
        color: &[238, 200, 220],
    },
    ThreadRef {
        brand: BRAND,
        code: "3-6",
        name: "Palest Grey",
        color: &[220, 216, 225],
    },
    ThreadRef {
        brand: BRAND,
        code: "3-7",
        name: "Palest Blue",
        color: &[209, 216, 231],
    },
    ThreadRef {
        brand: BRAND,
        code: "3-8",
        name: "Palest Aqua",
        color: &[206, 250, 244],
    },
    ThreadRef {
        brand: BRAND,
        code: "3-9",
        name: "Palest Green",
        color: &[155, 182, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "4-1",
        name: "Light Blue",
        color: &[182, 200, 225],
    },
    ThreadRef {
        brand: BRAND,
        code: "4-10",
        name: "Navy Blue",
        color: &[18, 17, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "4-2",
        name: "Powder Blue",
        color: &[135, 160, 212],
    },
    ThreadRef {
        brand: BRAND,
        code: "4-3",
        name: "Sky Blue",
        color: &[114, 119, 199],
    },
    ThreadRef {
        brand: BRAND,
        code: "4-4",
        name: "Slate Blue",
        color: &[62, 95, 151],
    },
    ThreadRef {
        brand: BRAND,
        code: "4-5",
        name: "Storm Blue",
        color: &[85, 87, 122],
    },
    ThreadRef {
        brand: BRAND,
        code: "4-6",
        name: "Denim",
        color: &[38, 42, 92],
    },
    ThreadRef {
        brand: BRAND,
        code: "4-7",
        name: "Petrel Blue",
        color: &[10, 75, 140],
    },
    ThreadRef {
        brand: BRAND,
        code: "4-8",
        name: "Imperial Blue",
        color: &[17, 27, 94],
    },
    ThreadRef {
        brand: BRAND,
        code: "4-9",
        name: "Royal Blue",
        color: &[14, 34, 114],
    },
    ThreadRef {
        brand: BRAND,
        code: "5-1",
        name: "Copper",
        color: &[230, 120, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "5-10",
        name: "Dark Chocolate",
        color: &[68, 1, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "5-2",
        name: "Golden Brown",
        color: &[215, 107, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "5-3",
        name: "Tan",
        color: &[110, 40, 25],
    },
    ThreadRef {
        brand: BRAND,
        code: "5-4",
        name: "Bark",
        color: &[116, 37, 20],
    },
    ThreadRef {
        brand: BRAND,
        code: "5-5",
        name: "Dark Oak",
        color: &[75, 27, 18],
    },
    ThreadRef {
        brand: BRAND,
        code: "5-6",
        name: "Russet",
        color: &[130, 0, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "5-7",
        name: "Mahogany",
        color: &[80, 20, 18],
    },
    ThreadRef {
        brand: BRAND,
        code: "5-8",
        name: "Mushroom",
        color: &[120, 84, 54],
    },
    ThreadRef {
        brand: BRAND,
        code: "5-9",
        name: "Khaki",
        color: &[91, 60, 17],
    },
    ThreadRef {
        brand: BRAND,
        code: "6-1",
        name: "Light Salmon",
        color: &[248, 189, 155],
    },
    ThreadRef {
        brand: BRAND,
        code: "6-10",
        name: "Antique Red",
        color: &[224, 47, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "6-2",
        name: "Salmon",
        color: &[243, 138, 103],
    },
    ThreadRef {
        brand: BRAND,
        code: "6-3",
        name: "Tangerine",
        color: &[242, 82, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "6-4",
        name: "Blush",
        color: &[230, 145, 120],
    },
    ThreadRef {
        brand: BRAND,
        code: "6-5",
        name: "Bronze",
        color: &[184, 90, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "6-6",
        name: "Scarlet",
        color: &[232, 41, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "6-7",
        name: "Rust",
        color: &[202, 32, 23],
    },
    ThreadRef {
        brand: BRAND,
        code: "6-8",
        name: "Carmine",
        color: &[210, 4, 20],
    },
    ThreadRef {
        brand: BRAND,
        code: "6-9",
        name: "Ruby",
        color: &[231, 0, 28],
    },
    ThreadRef {
        brand: BRAND,
        code: "7-1",
        name: "Light Jade",
        color: &[144, 214, 130],
    },
    ThreadRef {
        brand: BRAND,
        code: "7-10",
        name: "Dark Green",
        color: &[11, 96, 32],
    },
    ThreadRef {
        brand: BRAND,
        code: "7-2",
        name: "Pistachio",
        color: &[158, 164, 70],
    },
    ThreadRef {
        brand: BRAND,
        code: "7-3",
        name: "Fresh Green",
        color: &[170, 201, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "7-4",
        name: "Sage",
        color: &[75, 148, 81],
    },
    ThreadRef {
        brand: BRAND,
        code: "7-5",
        name: "Teal",
        color: &[64, 96, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "7-6",
        name: "Bright Green",
        color: &[54, 143, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "7-7",
        name: "Jade",
        color: &[41, 118, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "7-8",
        name: "Leaf Green",
        color: &[58, 130, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "7-9",
        name: "Emerald",
        color: &[30, 145, 41],
    },
    ThreadRef {
        brand: BRAND,
        code: "8-1",
        name: "Sugar Pink",
        color: &[245, 192, 200],
    },
    ThreadRef {
        brand: BRAND,
        code: "8-10",
        name: "Raspberry",
        color: &[149, 36, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "8-2",
        name: "Petal Pink",
        color: &[250, 170, 190],
    },
    ThreadRef {
        brand: BRAND,
        code: "8-3",
        name: "Rose Pink",
        color: &[250, 134, 160],
    },
    ThreadRef {
        brand: BRAND,
        code: "8-4",
        name: "Passion Pink",
        color: &[208, 102, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "8-5",
        name: "Dusky Rose",
        color: &[190, 52, 85],
    },
    ThreadRef {
        brand: BRAND,
        code: "8-6",
        name: "Light Cerise",
        color: &[241, 105, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "8-7",
        name: "Dusky Pink",
        color: &[239, 91, 111],
    },
    ThreadRef {
        brand: BRAND,
        code: "8-8",
        name: "Cerise",
        color: &[220, 0, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "8-9",
        name: "Fuschia",
        color: &[255, 0, 118],
    },
    ThreadRef {
        brand: BRAND,
        code: "9-1",
        name: "Platinum",
        color: &[244, 241, 236],
    },
    ThreadRef {
        brand: BRAND,
        code: "9-10",
        name: "Seal",
        color: &[36, 36, 54],
    },
    ThreadRef {
        brand: BRAND,
        code: "9-2",
        name: "Silver Fox",
        color: &[185, 164, 183],
    },
    ThreadRef {
        brand: BRAND,
        code: "9-3",
        name: "Sleet Grey",
        color: &[176, 176, 187],
    },
    ThreadRef {
        brand: BRAND,
        code: "9-4",
        name: "Silver Blue",
        color: &[116, 121, 146],
    },
    ThreadRef {
        brand: BRAND,
        code: "9-5",
        name: "Silver Moon",
        color: &[150, 140, 142],
    },
    ThreadRef {
        brand: BRAND,
        code: "9-6",
        name: "Willow",
        color: &[138, 125, 102],
    },
    ThreadRef {
        brand: BRAND,
        code: "9-7",
        name: "Dark Grey",
        color: &[102, 83, 102],
    },
    ThreadRef {
        brand: BRAND,
        code: "9-8",
        name: "Steel",
        color: &[86, 83, 108],
    },
    ThreadRef {
        brand: BRAND,
        code: "9-9",
        name: "Midnight",
        color: &[75, 71, 80],
    },
];