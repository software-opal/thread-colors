#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "MTB-Embroidex";
pub const THREADS: [ThreadRef; 107] = [
    ThreadRef::new(
        BRAND,
        "P101",
        "Black",
        &[0, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "P102",
        "White",
        &[255, 255, 255],
    ),
    ThreadRef::new(
        BRAND,
        "P110",
        "Off White",
        &[255, 241, 215],
    ),
    ThreadRef::new(
        BRAND,
        "P115",
        "Sand",
        &[242, 251, 215],
    ),
    ThreadRef::new(
        BRAND,
        "P120",
        "Lt Beige",
        &[237, 227, 184],
    ),
    ThreadRef::new(
        BRAND,
        "P121",
        "Beige",
        &[198, 174, 157],
    ),
    ThreadRef::new(
        BRAND,
        "P122",
        "SandStone",
        &[196, 187, 157],
    ),
    ThreadRef::new(
        BRAND,
        "P123",
        "Beachcomber",
        &[183, 172, 134],
    ),
    ThreadRef::new(
        BRAND,
        "P125",
        "Dusky Goldenrod",
        &[214, 194, 158],
    ),
    ThreadRef::new(
        BRAND,
        "P126",
        "Twilight Harvest",
        &[191, 172, 106],
    ),
    ThreadRef::new(
        BRAND,
        "P127",
        "Brown",
        &[120, 96, 63],
    ),
    ThreadRef::new(
        BRAND,
        "P130",
        "Sienna",
        &[226, 197, 126],
    ),
    ThreadRef::new(
        BRAND,
        "P131",
        "Sienna Rose",
        &[185, 138, 68],
    ),
    ThreadRef::new(
        BRAND,
        "P135",
        "Sienna Brass",
        &[163, 104, 20],
    ),
    ThreadRef::new(
        BRAND,
        "P140",
        "Light Cocoa",
        &[145, 87, 53],
    ),
    ThreadRef::new(
        BRAND,
        "P143",
        "Burnished Eggplant",
        &[94, 52, 51],
    ),
    ThreadRef::new(
        BRAND,
        "P145",
        "Spanish Roast",
        &[72, 38, 38],
    ),
    ThreadRef::new(
        BRAND,
        "P195",
        "Pearled Gold",
        &[248, 244, 211],
    ),
    ThreadRef::new(
        BRAND,
        "P196",
        "Golden Sand",
        &[242, 236, 182],
    ),
    ThreadRef::new(
        BRAND,
        "P200",
        "Pearled bronze",
        &[247, 244, 201],
    ),
    ThreadRef::new(
        BRAND,
        "P201",
        "Pale Gold",
        &[253, 236, 168],
    ),
    ThreadRef::new(
        BRAND,
        "P205",
        "Buttercup",
        &[251, 242, 89],
    ),
    ThreadRef::new(
        BRAND,
        "P206",
        "Bright gold",
        &[249, 233, 38],
    ),
    ThreadRef::new(
        BRAND,
        "P207",
        "Goldenrod",
        &[230, 196, 77],
    ),
    ThreadRef::new(
        BRAND,
        "P210",
        "Brass",
        &[226, 179, 14],
    ),
    ThreadRef::new(
        BRAND,
        "P211",
        "Brass Rose",
        &[248, 194, 114],
    ),
    ThreadRef::new(
        BRAND,
        "P212",
        "Dusky Brass",
        &[217, 166, 3],
    ),
    ThreadRef::new(
        BRAND,
        "P215",
        "Cayenne",
        &[218, 99, 12],
    ),
    ThreadRef::new(
        BRAND,
        "P220",
        "Cinnamon",
        &[221, 121, 11],
    ),
    ThreadRef::new(
        BRAND,
        "P221",
        "Orange Glory",
        &[162, 47, 3],
    ),
    ThreadRef::new(
        BRAND,
        "P224",
        "Antique Bronze",
        &[190, 168, 12],
    ),
    ThreadRef::new(
        BRAND,
        "P225",
        "Antique Gold",
        &[190, 154, 12],
    ),
    ThreadRef::new(
        BRAND,
        "P226",
        "Pickled Olive",
        &[158, 129, 10],
    ),
    ThreadRef::new(
        BRAND,
        "P300",
        "Cool Mint",
        &[205, 243, 167],
    ),
    ThreadRef::new(
        BRAND,
        "P305",
        "California Lime",
        &[168, 235, 101],
    ),
    ThreadRef::new(
        BRAND,
        "P306",
        "Vivid Mint",
        &[115, 190, 12],
    ),
    ThreadRef::new(
        BRAND,
        "P307",
        "Grey Moss",
        &[194, 218, 112],
    ),
    ThreadRef::new(
        BRAND,
        "P308",
        "Deep Moss",
        &[96, 115, 30],
    ),
    ThreadRef::new(
        BRAND,
        "P310",
        "New Pine",
        &[80, 149, 38],
    ),
    ThreadRef::new(
        BRAND,
        "P312",
        "Lilypad Green",
        &[79, 91, 26],
    ),
    ThreadRef::new(
        BRAND,
        "P313",
        "Palmetto",
        &[73, 96, 26],
    ),
    ThreadRef::new(
        BRAND,
        "P314",
        "Black Pine",
        &[50, 66, 17],
    ),
    ThreadRef::new(
        BRAND,
        "P315",
        "Spring Fern",
        &[196, 210, 87],
    ),
    ThreadRef::new(
        BRAND,
        "P316",
        "Olive",
        &[156, 139, 41],
    ),
    ThreadRef::new(
        BRAND,
        "P320",
        "Parsley",
        &[135, 120, 35],
    ),
    ThreadRef::new(
        BRAND,
        "P325",
        "Golden Pine",
        &[98, 104, 26],
    ),
    ThreadRef::new(
        BRAND,
        "P400",
        "Frosted Wintergreen",
        &[225, 249, 204],
    ),
    ThreadRef::new(
        BRAND,
        "P401",
        "Pastoral Green",
        &[172, 199, 116],
    ),
    ThreadRef::new(
        BRAND,
        "P405",
        "Deep Wintergreen",
        &[86, 203, 128],
    ),
    ThreadRef::new(
        BRAND,
        "P406",
        "Parrot Green",
        &[108, 173, 20],
    ),
    ThreadRef::new(
        BRAND,
        "P407",
        "Christmas Fir",
        &[51, 89, 9],
    ),
    ThreadRef::new(
        BRAND,
        "P408",
        "Morning Cyan",
        &[209, 250, 230],
    ),
    ThreadRef::new(
        BRAND,
        "P409",
        "Dusky Cyan",
        &[131, 194, 233],
    ),
    ThreadRef::new(
        BRAND,
        "P410",
        "Mountain Lake",
        &[52, 139, 113],
    ),
    ThreadRef::new(
        BRAND,
        "P411",
        "Lake Shadows",
        &[38, 111, 102],
    ),
    ThreadRef::new(
        BRAND,
        "P415",
        "Lake Forest",
        &[48, 129, 104],
    ),
    ThreadRef::new(
        BRAND,
        "P416",
        "Midnight Lake",
        &[22, 75, 69],
    ),
    ThreadRef::new(
        BRAND,
        "P420",
        "Winter Morning",
        &[225, 247, 244],
    ),
    ThreadRef::new(
        BRAND,
        "P421",
        "Pearled Coneflower",
        &[208, 240, 253],
    ),
    ThreadRef::new(
        BRAND,
        "P422",
        "winter Haze",
        &[202, 213, 221],
    ),
    ThreadRef::new(
        BRAND,
        "P423",
        "Russina Blue",
        &[117, 120, 134],
    ),
    ThreadRef::new(
        BRAND,
        "P424",
        "Charcoal Blue",
        &[104, 107, 119],
    ),
    ThreadRef::new(
        BRAND,
        "P425",
        "Summer morning",
        &[172, 211, 223],
    ),
    ThreadRef::new(
        BRAND,
        "P426",
        "Harbor",
        &[48, 105, 122],
    ),
    ThreadRef::new(
        BRAND,
        "P427",
        "Cornflower",
        &[131, 153, 186],
    ),
    ThreadRef::new(
        BRAND,
        "P428",
        "Dusky Harbor",
        &[92, 151, 194],
    ),
    ThreadRef::new(
        BRAND,
        "P429",
        "Harbor Twilight",
        &[39, 65, 92],
    ),
    ThreadRef::new(
        BRAND,
        "P430",
        "Royal Blue",
        &[46, 65, 114],
    ),
    ThreadRef::new(
        BRAND,
        "P431",
        "Suskty Navy",
        &[69, 81, 101],
    ),
    ThreadRef::new(
        BRAND,
        "P435",
        "Deep Navy",
        &[63, 60, 96],
    ),
    ThreadRef::new(
        BRAND,
        "P500",
        "Frosted Rose",
        &[253, 232, 236],
    ),
    ThreadRef::new(
        BRAND,
        "P501",
        "Rose",
        &[250, 190, 191],
    ),
    ThreadRef::new(
        BRAND,
        "P505",
        "Pearled Rose",
        &[252, 216, 216],
    ),
    ThreadRef::new(
        BRAND,
        "P506",
        "Tea Rose",
        &[247, 155, 197],
    ),
    ThreadRef::new(
        BRAND,
        "P507",
        "Vivid Rose",
        &[241, 139, 221],
    ),
    ThreadRef::new(
        BRAND,
        "P510",
        "Salmon Rose",
        &[230, 159, 155],
    ),
    ThreadRef::new(
        BRAND,
        "P515",
        "Dusty Fuschia",
        &[177, 37, 151],
    ),
    ThreadRef::new(
        BRAND,
        "P520",
        "Light Rose",
        &[193, 13, 113],
    ),
    ThreadRef::new(
        BRAND,
        "P525",
        "Paprika Red",
        &[213, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "P526",
        "Cherrywood Red",
        &[155, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "P530",
        "Dusty Burgundy",
        &[126, 10, 65],
    ),
    ThreadRef::new(
        BRAND,
        "P531",
        "Burgandy",
        &[111, 9, 57],
    ),
    ThreadRef::new(
        BRAND,
        "P535",
        "Pearld Burgandy",
        &[188, 14, 97],
    ),
    ThreadRef::new(
        BRAND,
        "P540",
        "Syrah",
        &[86, 7, 65],
    ),
    ThreadRef::new(
        BRAND,
        "P600",
        "Blonde",
        &[252, 228, 173],
    ),
    ThreadRef::new(
        BRAND,
        "P605",
        "Platinum Rose",
        &[254, 218, 188],
    ),
    ThreadRef::new(
        BRAND,
        "P606",
        "Dusty Salmon",
        &[253, 201, 166],
    ),
    ThreadRef::new(
        BRAND,
        "P607",
        "Pearled Salmon",
        &[250, 183, 150],
    ),
    ThreadRef::new(
        BRAND,
        "P610",
        "Salmon",
        &[248, 162, 120],
    ),
    ThreadRef::new(
        BRAND,
        "P612",
        "Vivid Salmon",
        &[255, 128, 128],
    ),
    ThreadRef::new(
        BRAND,
        "P613",
        "Light Mauve",
        &[244, 187, 196],
    ),
    ThreadRef::new(
        BRAND,
        "P695",
        "Light Lilac",
        &[184, 161, 169],
    ),
    ThreadRef::new(
        BRAND,
        "P696",
        "Antique Lilac",
        &[141, 117, 143],
    ),
    ThreadRef::new(
        BRAND,
        "P697",
        "Silver Lilac",
        &[155, 136, 139],
    ),
    ThreadRef::new(
        BRAND,
        "P700",
        "Lilac",
        &[207, 154, 179],
    ),
    ThreadRef::new(
        BRAND,
        "P701",
        "Twilight Lilac",
        &[150, 102, 168],
    ),
    ThreadRef::new(
        BRAND,
        "P702",
        "Lupine",
        &[125, 94, 168],
    ),
    ThreadRef::new(
        BRAND,
        "P703",
        "Deep Violet",
        &[132, 70, 122],
    ),
    ThreadRef::new(
        BRAND,
        "P704",
        "Royal Eggplant",
        &[99, 60, 104],
    ),
    ThreadRef::new(
        BRAND,
        "P705",
        "Twilight Lupine",
        &[91, 67, 124],
    ),
    ThreadRef::new(
        BRAND,
        "P800",
        "Platinum",
        &[220, 220, 219],
    ),
    ThreadRef::new(
        BRAND,
        "P801",
        "Quartz",
        &[217, 232, 215],
    ),
    ThreadRef::new(
        BRAND,
        "P802",
        "Titanium",
        &[198, 187, 185],
    ),
    ThreadRef::new(
        BRAND,
        "P803",
        "Blue Titanium",
        &[187, 187, 187],
    ),
    ThreadRef::new(
        BRAND,
        "P805",
        "Anitque Platinum",
        &[156, 156, 146],
    ),
    ThreadRef::new(
        BRAND,
        "P810",
        "Hematite",
        &[133, 133, 120],
    ),
    ThreadRef::new(
        BRAND,
        "P815",
        "Twilight",
        &[87, 80, 79],
    ),
];