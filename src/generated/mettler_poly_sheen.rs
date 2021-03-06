#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Mettler Poly Sheen";
pub const THREADS: [ThreadRef; 351] = [
    ThreadRef::new(
        BRAND,
        "0015",
        "White",
        &[255, 255, 255],
    ),
    ThreadRef::new(
        BRAND,
        "0020",
        "Black",
        &[35, 31, 32],
    ),
    ThreadRef::new(
        BRAND,
        "0010",
        "Silky White",
        &[255, 255, 255],
    ),
    ThreadRef::new(
        BRAND,
        "0101",
        "Eggshell",
        &[255, 254, 244],
    ),
    ThreadRef::new(
        BRAND,
        "0870",
        "Muslin",
        &[246, 239, 226],
    ),
    ThreadRef::new(
        BRAND,
        "0672",
        "Baguette",
        &[205, 203, 179],
    ),
    ThreadRef::new(
        BRAND,
        "0873",
        "Stone",
        &[162, 155, 134],
    ),
    ThreadRef::new(
        BRAND,
        "0874",
        "Gravel",
        &[173, 158, 148],
    ),
    ThreadRef::new(
        BRAND,
        "1061",
        "Taupe",
        &[174, 141, 127],
    ),
    ThreadRef::new(
        BRAND,
        "0722",
        "Khaki",
        &[153, 137, 128],
    ),
    ThreadRef::new(
        BRAND,
        "0853",
        "Pecan",
        &[147, 113, 95],
    ),
    ThreadRef::new(
        BRAND,
        "0934",
        "Fawn",
        &[201, 162, 131],
    ),
    ThreadRef::new(
        BRAND,
        "1172",
        "Ivory",
        &[224, 204, 170],
    ),
    ThreadRef::new(
        BRAND,
        "0651",
        "Cornsilk",
        &[229, 190, 146],
    ),
    ThreadRef::new(
        BRAND,
        "1140",
        "Meringue",
        &[254, 215, 170],
    ),
    ThreadRef::new(
        BRAND,
        "1141",
        "Tan",
        &[206, 157, 115],
    ),
    ThreadRef::new(
        BRAND,
        "0842",
        "Toffee",
        &[206, 157, 115],
    ),
    ThreadRef::new(
        BRAND,
        "0832",
        "Sisal",
        &[176, 133, 81],
    ),
    ThreadRef::new(
        BRAND,
        "0940",
        "Autumn Leaf",
        &[186, 120, 49],
    ),
    ThreadRef::new(
        BRAND,
        "0922",
        "Ashley Gold",
        &[177, 119, 52],
    ),
    ThreadRef::new(
        BRAND,
        "0932",
        "Nutmeg",
        &[185, 104, 49],
    ),
    ThreadRef::new(
        BRAND,
        "1134",
        "Light Cocoa",
        &[149, 92, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1342",
        "Rust",
        &[132, 75, 55],
    ),
    ThreadRef::new(
        BRAND,
        "0933",
        "Redwood",
        &[113, 81, 58],
    ),
    ThreadRef::new(
        BRAND,
        "1355",
        "Fox",
        &[115, 66, 57],
    ),
    ThreadRef::new(
        BRAND,
        "1346",
        "Cinnamon",
        &[95, 66, 57],
    ),
    ThreadRef::new(
        BRAND,
        "1876",
        "Chocolate",
        &[90, 81, 78],
    ),
    ThreadRef::new(
        BRAND,
        "0945",
        "Pine Bark",
        &[103, 81, 59],
    ),
    ThreadRef::new(
        BRAND,
        "1055",
        "Bark",
        &[113, 81, 58],
    ),
    ThreadRef::new(
        BRAND,
        "1565",
        "Espresso",
        &[104, 73, 58],
    ),
    ThreadRef::new(
        BRAND,
        "0776",
        "Sage",
        &[110, 89, 91],
    ),
    ThreadRef::new(
        BRAND,
        "0941",
        "Golden Grain",
        &[157, 115, 55],
    ),
    ThreadRef::new(
        BRAND,
        "0822",
        "Palomino",
        &[177, 133, 53],
    ),
    ThreadRef::new(
        BRAND,
        "0552",
        "Flax",
        &[196, 162, 118],
    ),
    ThreadRef::new(
        BRAND,
        "0532",
        "Champagne",
        &[206, 183, 139],
    ),
    ThreadRef::new(
        BRAND,
        "0761",
        "Oat",
        &[218, 206, 171],
    ),
    ThreadRef::new(
        BRAND,
        "0670",
        "Cream",
        &[255, 253, 233],
    ),
    ThreadRef::new(
        BRAND,
        "0250",
        "Lemon Frost",
        &[242, 245, 194],
    ),
    ThreadRef::new(
        BRAND,
        "0600",
        "Citrus",
        &[255, 242, 0],
    ),
    ThreadRef::new(
        BRAND,
        "0311",
        "Canary",
        &[255, 203, 8],
    ),
    ThreadRef::new(
        BRAND,
        "0310",
        "Yellow",
        &[255, 243, 43],
    ),
    ThreadRef::new(
        BRAND,
        "0520",
        "Daffodil",
        &[250, 236, 180],
    ),
    ThreadRef::new(
        BRAND,
        "0970",
        "Linen",
        &[255, 249, 231],
    ),
    ThreadRef::new(
        BRAND,
        "0660",
        "Vanilla",
        &[255, 244, 201],
    ),
    ThreadRef::new(
        BRAND,
        "0640",
        "Parchment",
        &[255, 222, 136],
    ),
    ThreadRef::new(
        BRAND,
        "0811",
        "Candlelight",
        &[239, 163, 31],
    ),
    ThreadRef::new(
        BRAND,
        "0702",
        "Papaya",
        &[252, 175, 23],
    ),
    ThreadRef::new(
        BRAND,
        "0630",
        "Buttercup",
        &[255, 224, 106],
    ),
    ThreadRef::new(
        BRAND,
        "0713",
        "Lemon",
        &[255, 210, 78],
    ),
    ThreadRef::new(
        BRAND,
        "0700",
        "Bright Yellow",
        &[253, 184, 19],
    ),
    ThreadRef::new(
        BRAND,
        "0704",
        "Gold",
        &[228, 160, 36],
    ),
    ThreadRef::new(
        BRAND,
        "0800",
        "Golden Rod",
        &[249, 157, 28],
    ),
    ThreadRef::new(
        BRAND,
        "1102",
        "Pumpkin",
        &[244, 121, 32],
    ),
    ThreadRef::new(
        BRAND,
        "1300",
        "Tangerine",
        &[242, 101, 49],
    ),
    ThreadRef::new(
        BRAND,
        "1304",
        "Red Pepper",
        &[241, 89, 34],
    ),
    ThreadRef::new(
        BRAND,
        "1305",
        "Fox Fire",
        &[239, 64, 35],
    ),
    ThreadRef::new(
        BRAND,
        "1703",
        "Poppy",
        &[228, 30, 38],
    ),
    ThreadRef::new(
        BRAND,
        "1902",
        "Poinsettia",
        &[183, 41, 60],
    ),
    ThreadRef::new(
        BRAND,
        "1903",
        "Lipstick",
        &[201, 37, 53],
    ),
    ThreadRef::new(
        BRAND,
        "1725",
        "Terra Cotta",
        &[183, 41, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1335",
        "Dark Rust",
        &[176, 66, 49],
    ),
    ThreadRef::new(
        BRAND,
        "1334",
        "Spice",
        &[158, 75, 52],
    ),
    ThreadRef::new(
        BRAND,
        "1760",
        "Twine",
        &[239, 196, 171],
    ),
    ThreadRef::new(
        BRAND,
        "1060",
        "Shrimp Pink",
        &[254, 219, 179],
    ),
    ThreadRef::new(
        BRAND,
        "1362",
        "Shrimp",
        &[251, 179, 131],
    ),
    ThreadRef::new(
        BRAND,
        "1220",
        "Apricot",
        &[244, 127, 59],
    ),
    ThreadRef::new(
        BRAND,
        "1114",
        "Clay",
        &[222, 110, 69],
    ),
    ThreadRef::new(
        BRAND,
        "1352",
        "Salmon",
        &[247, 150, 107],
    ),
    ThreadRef::new(
        BRAND,
        "1351",
        "Starfish",
        &[249, 165, 126],
    ),
    ThreadRef::new(
        BRAND,
        "1551",
        "Pink Clay",
        &[248, 170, 143],
    ),
    ThreadRef::new(
        BRAND,
        "1532",
        "Coral",
        &[246, 141, 118],
    ),
    ThreadRef::new(
        BRAND,
        "1430",
        "Melon",
        &[234, 130, 101],
    ),
    ThreadRef::new(
        BRAND,
        "1521",
        "Flamingo",
        &[241, 90, 78],
    ),
    ThreadRef::new(
        BRAND,
        "1600",
        "Spanish Tile",
        &[230, 77, 64],
    ),
    ThreadRef::new(
        BRAND,
        "1805",
        "Strawberry",
        &[210, 33, 62],
    ),
    ThreadRef::new(
        BRAND,
        "1900",
        "Geranium",
        &[201, 36, 74],
    ),
    ThreadRef::new(
        BRAND,
        "1911",
        "Foliage Rose",
        &[166, 43, 67],
    ),
    ThreadRef::new(
        BRAND,
        "1912",
        "Winterberry",
        &[132, 48, 70],
    ),
    ThreadRef::new(
        BRAND,
        "2222",
        "Burgundy",
        &[115, 49, 71],
    ),
    ThreadRef::new(
        BRAND,
        "2241",
        "Mauve",
        &[164, 85, 106],
    ),
    ThreadRef::new(
        BRAND,
        "2051",
        "Teaberry",
        &[182, 105, 123],
    ),
    ThreadRef::new(
        BRAND,
        "1761",
        "Tea Rose",
        &[215, 162, 151],
    ),
    ThreadRef::new(
        BRAND,
        "2170",
        "Chiffon",
        &[239, 217, 215],
    ),
    ThreadRef::new(
        BRAND,
        "2160",
        "Iced Pink",
        &[243, 190, 208],
    ),
    ThreadRef::new(
        BRAND,
        "1755",
        "Hyacinth",
        &[224, 164, 166],
    ),
    ThreadRef::new(
        BRAND,
        "2152",
        "Heather Pink",
        &[211, 126, 148],
    ),
    ThreadRef::new(
        BRAND,
        "1860",
        "Shell",
        &[237, 187, 199],
    ),
    ThreadRef::new(
        BRAND,
        "2171",
        "Blush",
        &[252, 229, 238],
    ),
    ThreadRef::new(
        BRAND,
        "1840",
        "Corsage",
        &[243, 133, 141],
    ),
    ThreadRef::new(
        BRAND,
        "1921",
        "Blossom",
        &[183, 65, 90],
    ),
    ThreadRef::new(
        BRAND,
        "2113",
        "Cranberry",
        &[132, 48, 70],
    ),
    ThreadRef::new(
        BRAND,
        "2123",
        "Bordeaux",
        &[116, 50, 66],
    ),
    ThreadRef::new(
        BRAND,
        "1366",
        "Mahogany",
        &[73, 67, 66],
    ),
    ThreadRef::new(
        BRAND,
        "3331",
        "Cadet Blue",
        &[87, 122, 188],
    ),
    ThreadRef::new(
        BRAND,
        "3211",
        "Twilight",
        &[64, 86, 166],
    ),
    ThreadRef::new(
        BRAND,
        "3210",
        "Blueberry",
        &[59, 71, 157],
    ),
    ThreadRef::new(
        BRAND,
        "3541",
        "Venetian Blue",
        &[73, 71, 157],
    ),
    ThreadRef::new(
        BRAND,
        "3110",
        "Dark Ink",
        &[37, 36, 123],
    ),
    ThreadRef::new(
        BRAND,
        "2900",
        "Deep Purple",
        &[71, 47, 145],
    ),
    ThreadRef::new(
        BRAND,
        "3536",
        "Heraldic",
        &[9, 25, 93],
    ),
    ThreadRef::new(
        BRAND,
        "2715",
        "Pansy",
        &[85, 51, 100],
    ),
    ThreadRef::new(
        BRAND,
        "2810",
        "Orchid",
        &[111, 65, 123],
    ),
    ThreadRef::new(
        BRAND,
        "2920",
        "Purple",
        &[104, 80, 162],
    ),
    ThreadRef::new(
        BRAND,
        "2830",
        "Wild Iris",
        &[131, 98, 171],
    ),
    ThreadRef::new(
        BRAND,
        "2650",
        "Impatient",
        &[222, 176, 209],
    ),
    ThreadRef::new(
        BRAND,
        "2655",
        "Aura",
        &[223, 204, 227],
    ),
    ThreadRef::new(
        BRAND,
        "2764",
        "Violet",
        &[171, 135, 162],
    ),
    ThreadRef::new(
        BRAND,
        "2640",
        "Frosted Plum",
        &[189, 132, 186],
    ),
    ThreadRef::new(
        BRAND,
        "2600",
        "Dusty Grape",
        &[122, 45, 115],
    ),
    ThreadRef::new(
        BRAND,
        "2504",
        "Plum",
        &[153, 63, 152],
    ),
    ThreadRef::new(
        BRAND,
        "2506",
        "Cerise",
        &[131, 46, 98],
    ),
    ThreadRef::new(
        BRAND,
        "2300",
        "Bright Ruby",
        &[182, 34, 107],
    ),
    ThreadRef::new(
        BRAND,
        "2520",
        "Garden Rose",
        &[238, 56, 151],
    ),
    ThreadRef::new(
        BRAND,
        "2550",
        "Soft Pink",
        &[230, 112, 172],
    ),
    ThreadRef::new(
        BRAND,
        "2560",
        "Azalea Pink",
        &[231, 132, 183],
    ),
    ThreadRef::new(
        BRAND,
        "2363",
        "Carnation",
        &[250, 213, 229],
    ),
    ThreadRef::new(
        BRAND,
        "2250",
        "Petal Pink",
        &[247, 183, 202],
    ),
    ThreadRef::new(
        BRAND,
        "2155",
        "Pink Tulip",
        &[247, 172, 188],
    ),
    ThreadRef::new(
        BRAND,
        "2220",
        "Tropicana",
        &[238, 76, 148],
    ),
    ThreadRef::new(
        BRAND,
        "2521",
        "Fuchsia",
        &[200, 33, 93],
    ),
    ThreadRef::new(
        BRAND,
        "2320",
        "Raspberry",
        &[182, 38, 94],
    ),
    ThreadRef::new(
        BRAND,
        "2500",
        "Boysenberry",
        &[148, 44, 97],
    ),
    ThreadRef::new(
        BRAND,
        "2711",
        "Dark Current",
        &[97, 18, 109],
    ),
    ThreadRef::new(
        BRAND,
        "2336",
        "Maroon",
        &[77, 53, 64],
    ),
    ThreadRef::new(
        BRAND,
        "3040",
        "Lavender",
        &[176, 155, 202],
    ),
    ThreadRef::new(
        BRAND,
        "3241",
        "Amethyst Frost",
        &[122, 112, 143],
    ),
    ThreadRef::new(
        BRAND,
        "3062",
        "Cinder",
        &[141, 126, 148],
    ),
    ThreadRef::new(
        BRAND,
        "0112",
        "Leadville",
        &[120, 125, 138],
    ),
    ThreadRef::new(
        BRAND,
        "0132",
        "Dark Pewter",
        &[64, 80, 100],
    ),
    ThreadRef::new(
        BRAND,
        "2674",
        "Steel",
        &[119, 132, 148],
    ),
    ThreadRef::new(
        BRAND,
        "0131",
        "Smoke",
        &[160, 157, 162],
    ),
    ThreadRef::new(
        BRAND,
        "0151",
        "Cloud",
        &[176, 173, 161],
    ),
    ThreadRef::new(
        BRAND,
        "0182",
        "Saturn Grey",
        &[214, 204, 203],
    ),
    ThreadRef::new(
        BRAND,
        "0150",
        "Mystic Grey",
        &[193, 190, 183],
    ),
    ThreadRef::new(
        BRAND,
        "0124",
        "Fieldstone",
        &[191, 199, 196],
    ),
    ThreadRef::new(
        BRAND,
        "3971",
        "Silver",
        &[190, 200, 205],
    ),
    ThreadRef::new(
        BRAND,
        "0142",
        "Sterling",
        &[157, 172, 178],
    ),
    ThreadRef::new(
        BRAND,
        "1972",
        "Silvery Grey",
        &[150, 150, 154],
    ),
    ThreadRef::new(
        BRAND,
        "0108",
        "Cobblestone",
        &[119, 132, 141],
    ),
    ThreadRef::new(
        BRAND,
        "0152",
        "Dolphin",
        &[121, 125, 131],
    ),
    ThreadRef::new(
        BRAND,
        "0111",
        "Whale",
        &[92, 107, 120],
    ),
    ThreadRef::new(
        BRAND,
        "4174",
        "Charcoal",
        &[36, 40, 63],
    ),
    ThreadRef::new(
        BRAND,
        "1375",
        "Dark Charcoal",
        &[85, 94, 93],
    ),
    ThreadRef::new(
        BRAND,
        "6156",
        "Olive",
        &[71, 73, 60],
    ),
    ThreadRef::new(
        BRAND,
        "5866",
        "Herb Green",
        &[48, 67, 59],
    ),
    ThreadRef::new(
        BRAND,
        "0465",
        "Umber",
        &[113, 81, 58],
    ),
    ThreadRef::new(
        BRAND,
        "0747",
        "Golden Brown",
        &[111, 95, 60],
    ),
    ThreadRef::new(
        BRAND,
        "0345",
        "Moss",
        &[120, 102, 60],
    ),
    ThreadRef::new(
        BRAND,
        "0542",
        "Ochre",
        &[178, 147, 53],
    ),
    ThreadRef::new(
        BRAND,
        "0221",
        "Light Brass",
        &[235, 212, 23],
    ),
    ThreadRef::new(
        BRAND,
        "0232",
        "Seaweed",
        &[177, 172, 89],
    ),
    ThreadRef::new(
        BRAND,
        "0352",
        "Marsh",
        &[177, 180, 91],
    ),
    ThreadRef::new(
        BRAND,
        "6133",
        "Caper",
        &[138, 132, 61],
    ),
    ThreadRef::new(
        BRAND,
        "5934",
        "Moss Green",
        &[106, 120, 64],
    ),
    ThreadRef::new(
        BRAND,
        "5944",
        "Backyard Green",
        &[39, 79, 62],
    ),
    ThreadRef::new(
        BRAND,
        "5664",
        "Willow",
        &[96, 133, 119],
    ),
    ThreadRef::new(
        BRAND,
        "5552",
        "Palm Leaf",
        &[129, 148, 134],
    ),
    ThreadRef::new(
        BRAND,
        "0453",
        "Army Drab",
        &[152, 159, 123],
    ),
    ThreadRef::new(
        BRAND,
        "5822",
        "Kiwi",
        &[137, 184, 116],
    ),
    ThreadRef::new(
        BRAND,
        "6051",
        "Jalepeno",
        &[162, 192, 132],
    ),
    ThreadRef::new(
        BRAND,
        "5650",
        "Spring Frost",
        &[181, 219, 173],
    ),
    ThreadRef::new(
        BRAND,
        "5531",
        "Pear",
        &[93, 158, 69],
    ),
    ThreadRef::new(
        BRAND,
        "5610",
        "Bright Mint",
        &[114, 177, 100],
    ),
    ThreadRef::new(
        BRAND,
        "5912",
        "Erin Green",
        &[113, 191, 68],
    ),
    ThreadRef::new(
        BRAND,
        "5833",
        "Limabean",
        &[103, 132, 66],
    ),
    ThreadRef::new(
        BRAND,
        "5633",
        "Lime",
        &[58, 123, 68],
    ),
    ThreadRef::new(
        BRAND,
        "5643",
        "Green Dust",
        &[53, 103, 65],
    ),
    ThreadRef::new(
        BRAND,
        "5513",
        "Ming",
        &[0, 159, 77],
    ),
    ThreadRef::new(
        BRAND,
        "5510",
        "Emerald",
        &[41, 164, 74],
    ),
    ThreadRef::new(
        BRAND,
        "5613",
        "Light Kelly",
        &[0, 166, 81],
    ),
    ThreadRef::new(
        BRAND,
        "5515",
        "Kelly",
        &[0, 144, 88],
    ),
    ThreadRef::new(
        BRAND,
        "5415",
        "Irish Green",
        &[0, 133, 74],
    ),
    ThreadRef::new(
        BRAND,
        "5422",
        "Swiss Ivy",
        &[0, 167, 109],
    ),
    ThreadRef::new(
        BRAND,
        "5324",
        "Bright Green",
        &[6, 101, 67],
    ),
    ThreadRef::new(
        BRAND,
        "5555",
        "Deep Green",
        &[34, 85, 63],
    ),
    ThreadRef::new(
        BRAND,
        "5326",
        "Evergreen",
        &[32, 85, 72],
    ),
    ThreadRef::new(
        BRAND,
        "5100",
        "Green",
        &[0, 122, 101],
    ),
    ThreadRef::new(
        BRAND,
        "5210",
        "Trellis Green",
        &[0, 160, 120],
    ),
    ThreadRef::new(
        BRAND,
        "5230",
        "Bottle Green",
        &[74, 190, 157],
    ),
    ThreadRef::new(
        BRAND,
        "5220",
        "Silver Sage",
        &[126, 203, 174],
    ),
    ThreadRef::new(
        BRAND,
        "4250",
        "Snomoon",
        &[202, 222, 215],
    ),
    ThreadRef::new(
        BRAND,
        "5050",
        "Luster",
        &[190, 218, 215],
    ),
    ThreadRef::new(
        BRAND,
        "5115",
        "Baccarat Green",
        &[5, 181, 157],
    ),
    ThreadRef::new(
        BRAND,
        "5101",
        "Dark Jade",
        &[0, 140, 128],
    ),
    ThreadRef::new(
        BRAND,
        "5005",
        "Rain Forest",
        &[0, 107, 101],
    ),
    ThreadRef::new(
        BRAND,
        "5374",
        "Forest Green",
        &[36, 79, 80],
    ),
    ThreadRef::new(
        BRAND,
        "4071",
        "Glacier Green",
        &[188, 208, 224],
    ),
    ThreadRef::new(
        BRAND,
        "3650",
        "Ice Cap",
        &[175, 197, 221],
    ),
    ThreadRef::new(
        BRAND,
        "3761",
        "Wintersky",
        &[148, 189, 229],
    ),
    ThreadRef::new(
        BRAND,
        "3840",
        "Oxford",
        &[126, 167, 208],
    ),
    ThreadRef::new(
        BRAND,
        "3951",
        "Azure Blue",
        &[118, 157, 195],
    ),
    ThreadRef::new(
        BRAND,
        "3820",
        "Celestial",
        &[101, 154, 210],
    ),
    ThreadRef::new(
        BRAND,
        "3810",
        "Laguna",
        &[53, 118, 174],
    ),
    ThreadRef::new(
        BRAND,
        "3953",
        "Ocean Blue",
        &[96, 122, 150],
    ),
    ThreadRef::new(
        BRAND,
        "3842",
        "Copenhagen",
        &[57, 124, 148],
    ),
    ThreadRef::new(
        BRAND,
        "4644",
        "Mallard",
        &[12, 91, 116],
    ),
    ThreadRef::new(
        BRAND,
        "4032",
        "Teal",
        &[28, 115, 150],
    ),
    ThreadRef::new(
        BRAND,
        "3901",
        "Tropical Blue",
        &[0, 113, 174],
    ),
    ThreadRef::new(
        BRAND,
        "3900",
        "Cerulean",
        &[0, 114, 188],
    ),
    ThreadRef::new(
        BRAND,
        "3910",
        "Crystal Blue",
        &[1, 174, 230],
    ),
    ThreadRef::new(
        BRAND,
        "3962",
        "River Mist",
        &[112, 198, 240],
    ),
    ThreadRef::new(
        BRAND,
        "4240",
        "Spearmint",
        &[80, 198, 216],
    ),
    ThreadRef::new(
        BRAND,
        "4230",
        "Aqua",
        &[76, 198, 224],
    ),
    ThreadRef::new(
        BRAND,
        "4111",
        "Turquoise",
        &[14, 163, 193],
    ),
    ThreadRef::new(
        BRAND,
        "4103",
        "California Blue",
        &[0, 149, 218],
    ),
    ThreadRef::new(
        BRAND,
        "4101",
        "Wave Blue",
        &[0, 159, 207],
    ),
    ThreadRef::new(
        BRAND,
        "4010",
        "Caribbean Blue",
        &[0, 136, 178],
    ),
    ThreadRef::new(
        BRAND,
        "4116",
        "Dark Teal",
        &[0, 113, 138],
    ),
    ThreadRef::new(
        BRAND,
        "4033",
        "Tartan Blue",
        &[22, 85, 108],
    ),
    ThreadRef::new(
        BRAND,
        "4442",
        "Deep Sea Blue",
        &[4, 96, 119],
    ),
    ThreadRef::new(
        BRAND,
        "4531",
        "Caribbean",
        &[0, 124, 145],
    ),
    ThreadRef::new(
        BRAND,
        "4423",
        "Marine Aqua",
        &[0, 141, 155],
    ),
    ThreadRef::new(
        BRAND,
        "4220",
        "Island Green",
        &[0, 176, 181],
    ),
    ThreadRef::new(
        BRAND,
        "4620",
        "Jade",
        &[37, 189, 173],
    ),
    ThreadRef::new(
        BRAND,
        "4610",
        "Deep Aqua",
        &[0, 156, 147],
    ),
    ThreadRef::new(
        BRAND,
        "4643",
        "Amazon",
        &[0, 102, 121],
    ),
    ThreadRef::new(
        BRAND,
        "4515",
        "Spruce",
        &[0, 101, 99],
    ),
    ThreadRef::new(
        BRAND,
        "3770",
        "Oyster",
        &[191, 202, 198],
    ),
    ThreadRef::new(
        BRAND,
        "0145",
        "Skylight",
        &[178, 187, 199],
    ),
    ThreadRef::new(
        BRAND,
        "3750",
        "Winter Frost",
        &[154, 180, 198],
    ),
    ThreadRef::new(
        BRAND,
        "3640",
        "Lake Blue",
        &[149, 181, 224],
    ),
    ThreadRef::new(
        BRAND,
        "3641",
        "Wedgewood",
        &[85, 129, 186],
    ),
    ThreadRef::new(
        BRAND,
        "3600",
        "Nordic Blue",
        &[12, 77, 162],
    ),
    ThreadRef::new(
        BRAND,
        "3522",
        "Blue",
        &[13, 78, 150],
    ),
    ThreadRef::new(
        BRAND,
        "3622",
        "Imperial Blue",
        &[32, 71, 127],
    ),
    ThreadRef::new(
        BRAND,
        "3543",
        "Royal Blue",
        &[32, 64, 154],
    ),
    ThreadRef::new(
        BRAND,
        "3544",
        "Sapphire",
        &[23, 70, 158],
    ),
    ThreadRef::new(
        BRAND,
        "3333",
        "Fire Blue",
        &[32, 64, 154],
    ),
    ThreadRef::new(
        BRAND,
        "3102",
        "Provence",
        &[22, 43, 117],
    ),
    ThreadRef::new(
        BRAND,
        "3323",
        "Delft",
        &[2, 37, 98],
    ),
    ThreadRef::new(
        BRAND,
        "3355",
        "Dark Indigo",
        &[0, 38, 89],
    ),
    ThreadRef::new(
        BRAND,
        "3444",
        "Concord",
        &[0, 44, 84],
    ),
    ThreadRef::new(
        BRAND,
        "3732",
        "Slate Blue",
        &[32, 71, 127],
    ),
    ThreadRef::new(
        BRAND,
        "3743",
        "Harbor",
        &[24, 79, 121],
    ),
    ThreadRef::new(
        BRAND,
        "5500",
        "Limedrop",
        &[108, 192, 103],
    ),
    ThreadRef::new(
        BRAND,
        "5940",
        "Sour Apple",
        &[226, 228, 23],
    ),
    ThreadRef::new(
        BRAND,
        "0501",
        "Sun",
        &[244, 240, 110],
    ),
    ThreadRef::new(
        BRAND,
        "0706",
        "Sunflower",
        &[255, 200, 11],
    ),
    ThreadRef::new(
        BRAND,
        "1120",
        "Sunset",
        &[252, 175, 23],
    ),
    ThreadRef::new(
        BRAND,
        "1106",
        "Orange",
        &[247, 148, 30],
    ),
    ThreadRef::new(
        BRAND,
        "1306",
        "Devil Red",
        &[239, 64, 35],
    ),
    ThreadRef::new(
        BRAND,
        "1940",
        "Chrysanthemum",
        &[242, 113, 146],
    ),
    ThreadRef::new(
        BRAND,
        "1950",
        "Tropical Pink",
        &[239, 58, 128],
    ),
    ThreadRef::new(
        BRAND,
        "0862",
        "Wild Rice",
        &[190, 175, 152],
    ),
    ThreadRef::new(
        BRAND,
        "0170",
        "Sea Shell",
        &[226, 219, 204],
    ),
    ThreadRef::new(
        BRAND,
        "0861",
        "Tantone",
        &[220, 209, 189],
    ),
    ThreadRef::new(
        BRAND,
        "1874",
        "Pewter",
        &[117, 116, 109],
    ),
    ThreadRef::new(
        BRAND,
        "1344",
        "Coffee Bean",
        &[127, 62, 56],
    ),
    ThreadRef::new(
        BRAND,
        "1154",
        "Penny",
        &[164, 116, 77],
    ),
    ThreadRef::new(
        BRAND,
        "0771",
        "Rattan",
        &[228, 209, 174],
    ),
    ThreadRef::new(
        BRAND,
        "0821",
        "Honey Gold",
        &[224, 181, 83],
    ),
    ThreadRef::new(
        BRAND,
        "0504",
        "Mimosa",
        &[239, 210, 36],
    ),
    ThreadRef::new(
        BRAND,
        "0622",
        "Star Gold",
        &[231, 207, 94],
    ),
    ThreadRef::new(
        BRAND,
        "0824",
        "Liberty Gold",
        &[231, 183, 0],
    ),
    ThreadRef::new(
        BRAND,
        "0851",
        "Old Gold",
        &[224, 199, 150],
    ),
    ThreadRef::new(
        BRAND,
        "1123",
        "Carmel Cream",
        &[224, 196, 156],
    ),
    ThreadRef::new(
        BRAND,
        "1252",
        "Dark Tan",
        &[203, 163, 138],
    ),
    ThreadRef::new(
        BRAND,
        "1161",
        "Straw",
        &[224, 196, 166],
    ),
    ThreadRef::new(
        BRAND,
        "0931",
        "Honey",
        &[214, 160, 78],
    ),
    ThreadRef::new(
        BRAND,
        "1115",
        "Copper",
        &[219, 145, 74],
    ),
    ThreadRef::new(
        BRAND,
        "1311",
        "Date",
        &[201, 113, 59],
    ),
    ThreadRef::new(
        BRAND,
        "1312",
        "Burnt Orange",
        &[206, 108, 52],
    ),
    ThreadRef::new(
        BRAND,
        "1514",
        "Brick",
        &[182, 64, 49],
    ),
    ThreadRef::new(
        BRAND,
        "0270",
        "Buttercream",
        &[255, 253, 239],
    ),
    ThreadRef::new(
        BRAND,
        "0605",
        "Daisy",
        &[246, 222, 0],
    ),
    ThreadRef::new(
        BRAND,
        "0608",
        "Sunshine",
        &[249, 228, 59],
    ),
    ThreadRef::new(
        BRAND,
        "0904",
        "Spanish Gold",
        &[240, 170, 0],
    ),
    ThreadRef::new(
        BRAND,
        "0506",
        "Yellow Bird",
        &[250, 222, 94],
    ),
    ThreadRef::new(
        BRAND,
        "1010",
        "Toast",
        &[228, 171, 86],
    ),
    ThreadRef::new(
        BRAND,
        "1332",
        "Harvest",
        &[227, 157, 83],
    ),
    ThreadRef::new(
        BRAND,
        "1301",
        "Paprika",
        &[221, 73, 32],
    ),
    ThreadRef::new(
        BRAND,
        "1913",
        "Cherry",
        &[171, 0, 51],
    ),
    ThreadRef::new(
        BRAND,
        "2011",
        "Fire Engine",
        &[171, 7, 64],
    ),
    ThreadRef::new(
        BRAND,
        "1730",
        "Persimmon",
        &[230, 72, 90],
    ),
    ThreadRef::new(
        BRAND,
        "1906",
        "Tulip",
        &[199, 0, 78],
    ),
    ThreadRef::new(
        BRAND,
        "2224",
        "Claret",
        &[133, 25, 74],
    ),
    ThreadRef::new(
        BRAND,
        "2022",
        "Rio Red",
        &[173, 10, 73],
    ),
    ThreadRef::new(
        BRAND,
        "1704",
        "Candy Apple",
        &[210, 0, 43],
    ),
    ThreadRef::new(
        BRAND,
        "2101",
        "Country Red",
        &[191, 0, 61],
    ),
    ThreadRef::new(
        BRAND,
        "1904",
        "Cardinal",
        &[197, 0, 50],
    ),
    ThreadRef::new(
        BRAND,
        "1800",
        "Wildfire",
        &[196, 6, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1701",
        "Red Berry",
        &[225, 90, 77],
    ),
    ThreadRef::new(
        BRAND,
        "2530",
        "Rose",
        &[245, 165, 187],
    ),
    ThreadRef::new(
        BRAND,
        "2153",
        "Dusty Mauve",
        &[214, 144, 156],
    ),
    ThreadRef::new(
        BRAND,
        "2166",
        "Flesh",
        &[236, 218, 212],
    ),
    ThreadRef::new(
        BRAND,
        "2115",
        "Beet Red",
        &[117, 31, 66],
    ),
    ThreadRef::new(
        BRAND,
        "2333",
        "Wine",
        &[157, 13, 80],
    ),
    ThreadRef::new(
        BRAND,
        "3150",
        "Stainless",
        &[210, 200, 221],
    ),
    ThreadRef::new(
        BRAND,
        "3151",
        "Blue Dawn",
        &[194, 194, 224],
    ),
    ThreadRef::new(
        BRAND,
        "3114",
        "Purple Twist",
        &[94, 39, 132],
    ),
    ThreadRef::new(
        BRAND,
        "2905",
        "Iris Blue",
        &[127, 93, 161],
    ),
    ThreadRef::new(
        BRAND,
        "2910",
        "Grape",
        &[170, 131, 182],
    ),
    ThreadRef::new(
        BRAND,
        "3045",
        "Cachet",
        &[224, 194, 217],
    ),
    ThreadRef::new(
        BRAND,
        "2832",
        "Easter Purple",
        &[136, 72, 147],
    ),
    ThreadRef::new(
        BRAND,
        "2761",
        "Dessert",
        &[218, 188, 197],
    ),
    ThreadRef::new(
        BRAND,
        "0180",
        "Whitewash",
        &[255, 246, 240],
    ),
    ThreadRef::new(
        BRAND,
        "2510",
        "Roseate",
        &[194, 91, 153],
    ),
    ThreadRef::new(
        BRAND,
        "2721",
        "Very Berry",
        &[178, 77, 146],
    ),
    ThreadRef::new(
        BRAND,
        "2720",
        "Sangria",
        &[156, 59, 135],
    ),
    ThreadRef::new(
        BRAND,
        "2912",
        "Sugar Plum",
        &[164, 111, 168],
    ),
    ThreadRef::new(
        BRAND,
        "0463",
        "Cypress",
        &[155, 166, 131],
    ),
    ThreadRef::new(
        BRAND,
        "0017",
        "Paper White",
        &[255, 255, 255],
    ),
    ThreadRef::new(
        BRAND,
        "0184",
        "Pearl",
        &[247, 247, 240],
    ),
    ThreadRef::new(
        BRAND,
        "0176",
        "Fog",
        &[229, 228, 221],
    ),
    ThreadRef::new(
        BRAND,
        "4073",
        "Metal",
        &[156, 160, 162],
    ),
    ThreadRef::new(
        BRAND,
        "2576",
        "Greyhound",
        &[131, 119, 142],
    ),
    ThreadRef::new(
        BRAND,
        "3251",
        "Haze",
        &[201, 182, 201],
    ),
    ThreadRef::new(
        BRAND,
        "3353",
        "Light Midnight",
        &[39, 48, 117],
    ),
    ThreadRef::new(
        BRAND,
        "0546",
        "Ginger",
        &[206, 188, 95],
    ),
    ThreadRef::new(
        BRAND,
        "0442",
        "Tarnished Gold",
        &[181, 179, 93],
    ),
    ThreadRef::new(
        BRAND,
        "6141",
        "Spring Green",
        &[178, 207, 128],
    ),
    ThreadRef::new(
        BRAND,
        "5730",
        "Apple Green",
        &[165, 211, 109],
    ),
    ThreadRef::new(
        BRAND,
        "5722",
        "Green Grass",
        &[51, 145, 52],
    ),
    ThreadRef::new(
        BRAND,
        "5933",
        "Grasshopper",
        &[93, 140, 65],
    ),
    ThreadRef::new(
        BRAND,
        "5740",
        "Mint",
        &[201, 226, 172],
    ),
    ThreadRef::new(
        BRAND,
        "6011",
        "Tamarack",
        &[192, 221, 106],
    ),
    ThreadRef::new(
        BRAND,
        "5411",
        "Shamrock",
        &[0, 152, 100],
    ),
    ThreadRef::new(
        BRAND,
        "5400",
        "Scrub Green",
        &[0, 169, 87],
    ),
    ThreadRef::new(
        BRAND,
        "5770",
        "Spanish Moss",
        &[221, 235, 208],
    ),
    ThreadRef::new(
        BRAND,
        "5335",
        "Swamp",
        &[0, 109, 86],
    ),
    ThreadRef::new(
        BRAND,
        "5233",
        "Field Green",
        &[0, 124, 115],
    ),
    ThreadRef::new(
        BRAND,
        "4625",
        "Seagreen",
        &[0, 144, 141],
    ),
    ThreadRef::new(
        BRAND,
        "5010",
        "Scotty Green",
        &[0, 178, 162],
    ),
    ThreadRef::new(
        BRAND,
        "4410",
        "Aqua Velva",
        &[0, 165, 183],
    ),
    ThreadRef::new(
        BRAND,
        "3906",
        "Pacific Blue",
        &[0, 174, 234],
    ),
    ThreadRef::new(
        BRAND,
        "4114",
        "Danish Teal",
        &[0, 176, 212],
    ),
    ThreadRef::new(
        BRAND,
        "4122",
        "Peacock",
        &[72, 188, 227],
    ),
    ThreadRef::new(
        BRAND,
        "3920",
        "Chicory",
        &[120, 200, 242],
    ),
    ThreadRef::new(
        BRAND,
        "4113",
        "Alexis Blue",
        &[0, 176, 219],
    ),
    ThreadRef::new(
        BRAND,
        "4421",
        "Light Mallard",
        &[0, 141, 166],
    ),
    ThreadRef::new(
        BRAND,
        "4430",
        "Island Waters",
        &[171, 216, 220],
    ),
    ThreadRef::new(
        BRAND,
        "4452",
        "Truly Teal",
        &[0, 156, 176],
    ),
    ThreadRef::new(
        BRAND,
        "4425",
        "Dark Aqua",
        &[0, 161, 180],
    ),
    ThreadRef::new(
        BRAND,
        "4133",
        "Deep Ocean",
        &[21, 98, 143],
    ),
    ThreadRef::new(
        BRAND,
        "4332",
        "Rough Sea",
        &[115, 159, 174],
    ),
    ThreadRef::new(
        BRAND,
        "3853",
        "Ash Blue",
        &[126, 150, 175],
    ),
    ThreadRef::new(
        BRAND,
        "3902",
        "Colonial Blue",
        &[0, 121, 192],
    ),
    ThreadRef::new(
        BRAND,
        "3815",
        "Reef Blue",
        &[112, 179, 227],
    ),
    ThreadRef::new(
        BRAND,
        "3762",
        "Country Blue",
        &[162, 188, 213],
    ),
    ThreadRef::new(
        BRAND,
        "3620",
        "Marine Blue",
        &[67, 112, 180],
    ),
    ThreadRef::new(
        BRAND,
        "3722",
        "Empire Blue",
        &[56, 127, 193],
    ),
    ThreadRef::new(
        BRAND,
        "3710",
        "Blue Bird",
        &[69, 129, 193],
    ),
    ThreadRef::new(
        BRAND,
        "3652",
        "Baby Blue",
        &[197, 220, 242],
    ),
    ThreadRef::new(
        BRAND,
        "3711",
        "Dolphin Blue",
        &[73, 137, 199],
    ),
    ThreadRef::new(
        BRAND,
        "3611",
        "Blue Ribbon",
        &[39, 85, 164],
    ),
    ThreadRef::new(
        BRAND,
        "3612",
        "Starlight Blue",
        &[59, 94, 168],
    ),
    ThreadRef::new(
        BRAND,
        "3335",
        "Flag Blue",
        &[52, 62, 146],
    ),
    ThreadRef::new(
        BRAND,
        "3554",
        "Navy",
        &[30, 36, 93],
    ),
    ThreadRef::new(
        BRAND,
        "3344",
        "Midnight",
        &[24, 31, 80],
    ),
    ThreadRef::new(
        BRAND,
        "6010",
        "Mountain Dew",
        &[202, 225, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1501",
        "Watermelon",
        &[235, 105, 104],
    ),
];