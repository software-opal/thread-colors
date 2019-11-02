#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "Brildor AC";
const THREADS: [ThreadRef; 120] = [
    ThreadRef {
        brand: BRAND,
        code: "5101",
        name: "Snow White",
        color: &[255, 255, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "5125",
        name: "Black Eyed Susie",
        color: &[184, 143, 43],
    },
    ThreadRef {
        brand: BRAND,
        code: "5142",
        name: "Dark Navy",
        color: &[46, 67, 106],
    },
    ThreadRef {
        brand: BRAND,
        code: "5143",
        name: "Dk. Navy",
        color: &[24, 8, 25],
    },
    ThreadRef {
        brand: BRAND,
        code: "5145",
        name: "Pale Green",
        color: &[129, 223, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "5146",
        name: "Pacific Mist",
        color: &[169, 255, 229],
    },
    ThreadRef {
        brand: BRAND,
        code: "5152",
        name: "Green Forest",
        color: &[38, 105, 90],
    },
    ThreadRef {
        brand: BRAND,
        code: "5160",
        name: "Victorian Rose",
        color: &[199, 177, 165],
    },
    ThreadRef {
        brand: BRAND,
        code: "5186",
        name: "Lt. Putty",
        color: &[223, 221, 221],
    },
    ThreadRef {
        brand: BRAND,
        code: "5197",
        name: "Medium Aqua",
        color: &[110, 214, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "5198",
        name: "Darkest Jade",
        color: &[11, 80, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "5199",
        name: "Ivy",
        color: &[14, 46, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "5207",
        name: "Azalea",
        color: &[255, 139, 173],
    },
    ThreadRef {
        brand: BRAND,
        code: "5209",
        name: "Medium Pink",
        color: &[226, 108, 152],
    },
    ThreadRef {
        brand: BRAND,
        code: "5211",
        name: "Arden Lavender",
        color: &[255, 218, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "5215",
        name: "Oyster",
        color: &[255, 227, 233],
    },
    ThreadRef {
        brand: BRAND,
        code: "5216",
        name: "Deep Mocha",
        color: &[202, 123, 127],
    },
    ThreadRef {
        brand: BRAND,
        code: "5217",
        name: "Cocoa",
        color: &[57, 24, 29],
    },
    ThreadRef {
        brand: BRAND,
        code: "5223",
        name: "PaleYellow",
        color: &[239, 242, 161],
    },
    ThreadRef {
        brand: BRAND,
        code: "5224",
        name: "Neon Yellow",
        color: &[255, 232, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "5235",
        name: "Stainless Steel",
        color: &[191, 184, 184],
    },
    ThreadRef {
        brand: BRAND,
        code: "5236",
        name: "Dark Grey",
        color: &[107, 92, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "5237",
        name: "Best Brown",
        color: &[77, 66, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "5252",
        name: "Coral Reef",
        color: &[253, 143, 120],
    },
    ThreadRef {
        brand: BRAND,
        code: "5255",
        name: "Toast",
        color: &[233, 172, 89],
    },
    ThreadRef {
        brand: BRAND,
        code: "5256",
        name: "Burnt Orange",
        color: &[204, 110, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "5281",
        name: "Cherry Red",
        color: &[172, 26, 38],
    },
    ThreadRef {
        brand: BRAND,
        code: "5283",
        name: "Burgundy",
        color: &[157, 9, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "5285",
        name: "Green",
        color: &[2, 88, 82],
    },
    ThreadRef {
        brand: BRAND,
        code: "5291",
        name: "Medium Green/Yellow",
        color: &[156, 125, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "5300",
        name: "Warm Red",
        color: &[227, 48, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "5302",
        name: "Dark Red/Orange",
        color: &[212, 37, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "5303",
        name: "Devil Red",
        color: &[236, 65, 39],
    },
    ThreadRef {
        brand: BRAND,
        code: "5304",
        name: "Wild Fire",
        color: &[200, 17, 17],
    },
    ThreadRef {
        brand: BRAND,
        code: "5305",
        name: "Tan",
        color: &[114, 26, 34],
    },
    ThreadRef {
        brand: BRAND,
        code: "5308",
        name: "Dark Brown",
        color: &[116, 54, 32],
    },
    ThreadRef {
        brand: BRAND,
        code: "5310",
        name: "Royal Blue",
        color: &[28, 45, 132],
    },
    ThreadRef {
        brand: BRAND,
        code: "5311",
        name: "Light Navy",
        color: &[16, 25, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "5320",
        name: "Ivy",
        color: &[3, 33, 33],
    },
    ThreadRef {
        brand: BRAND,
        code: "5321",
        name: "Olive Green",
        color: &[7, 38, 25],
    },
    ThreadRef {
        brand: BRAND,
        code: "5322",
        name: "Olive Green",
        color: &[12, 44, 26],
    },
    ThreadRef {
        brand: BRAND,
        code: "5323",
        name: "Festive Green",
        color: &[19, 110, 72],
    },
    ThreadRef {
        brand: BRAND,
        code: "5325",
        name: "Deep Forest Green",
        color: &[11, 69, 45],
    },
    ThreadRef {
        brand: BRAND,
        code: "5326",
        name: "Lt Jade",
        color: &[22, 165, 54],
    },
    ThreadRef {
        brand: BRAND,
        code: "5327",
        name: "Green",
        color: &[13, 110, 43],
    },
    ThreadRef {
        brand: BRAND,
        code: "5329",
        name: "Dark Salmon Pink",
        color: &[202, 62, 81],
    },
    ThreadRef {
        brand: BRAND,
        code: "5330",
        name: "Espresso",
        color: &[12, 11, 7],
    },
    ThreadRef {
        brand: BRAND,
        code: "5331",
        name: "Espresso",
        color: &[24, 23, 14],
    },
    ThreadRef {
        brand: BRAND,
        code: "5332",
        name: "Dark Brown",
        color: &[61, 43, 28],
    },
    ThreadRef {
        brand: BRAND,
        code: "5333",
        name: "Bright Mulberry",
        color: &[104, 50, 119],
    },
    ThreadRef {
        brand: BRAND,
        code: "5334",
        name: "Baton Rouge",
        color: &[114, 27, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "5337",
        name: "Grey Suede",
        color: &[72, 74, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "5338",
        name: "Black",
        color: &[52, 58, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "5339",
        name: "Eventide Green",
        color: &[156, 191, 93],
    },
    ThreadRef {
        brand: BRAND,
        code: "5340",
        name: "Light brown",
        color: &[179, 128, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "5341",
        name: "Light Orange",
        color: &[206, 174, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "5342",
        name: "Primedor",
        color: &[216, 203, 145],
    },
    ThreadRef {
        brand: BRAND,
        code: "5343",
        name: "Dark Brown",
        color: &[110, 56, 40],
    },
    ThreadRef {
        brand: BRAND,
        code: "5344",
        name: "Light Brown",
        color: &[183, 114, 86],
    },
    ThreadRef {
        brand: BRAND,
        code: "5345",
        name: "Light Yellow/Orange",
        color: &[230, 226, 167],
    },
    ThreadRef {
        brand: BRAND,
        code: "5346",
        name: "Medium Blue",
        color: &[64, 117, 173],
    },
    ThreadRef {
        brand: BRAND,
        code: "5348",
        name: "Dark Brown",
        color: &[56, 21, 21],
    },
    ThreadRef {
        brand: BRAND,
        code: "5350",
        name: "Manila",
        color: &[252, 202, 16],
    },
    ThreadRef {
        brand: BRAND,
        code: "5352",
        name: "Hunter Orange",
        color: &[254, 78, 26],
    },
    ThreadRef {
        brand: BRAND,
        code: "5353",
        name: "Gold",
        color: &[239, 238, 28],
    },
    ThreadRef {
        brand: BRAND,
        code: "5354",
        name: "Gold/Yellow",
        color: &[240, 253, 82],
    },
    ThreadRef {
        brand: BRAND,
        code: "5355",
        name: "Bright Gold",
        color: &[210, 138, 45],
    },
    ThreadRef {
        brand: BRAND,
        code: "5358",
        name: "Wisteria",
        color: &[171, 160, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "5360",
        name: "Pale Pink",
        color: &[243, 166, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "5361",
        name: "Bright Navy Blue",
        color: &[48, 26, 81],
    },
    ThreadRef {
        brand: BRAND,
        code: "5363",
        name: "Brown",
        color: &[102, 18, 33],
    },
    ThreadRef {
        brand: BRAND,
        code: "5364",
        name: "Dark Brown",
        color: &[54, 13, 21],
    },
    ThreadRef {
        brand: BRAND,
        code: "5371",
        name: "Teal Haze",
        color: &[5, 80, 101],
    },
    ThreadRef {
        brand: BRAND,
        code: "5372",
        name: "Dark Teal",
        color: &[21, 47, 77],
    },
    ThreadRef {
        brand: BRAND,
        code: "5373",
        name: "Dark Royal Blue",
        color: &[42, 66, 165],
    },
    ThreadRef {
        brand: BRAND,
        code: "5375",
        name: "Light Blue/Green",
        color: &[90, 219, 217],
    },
    ThreadRef {
        brand: BRAND,
        code: "5376",
        name: "Turquoise Medium Bright",
        color: &[47, 183, 227],
    },
    ThreadRef {
        brand: BRAND,
        code: "5380",
        name: "Purple",
        color: &[180, 131, 189],
    },
    ThreadRef {
        brand: BRAND,
        code: "5381",
        name: "Med. Dk. Gray",
        color: &[135, 128, 128],
    },
    ThreadRef {
        brand: BRAND,
        code: "5383",
        name: "Dk. Royal Blue",
        color: &[29, 42, 109],
    },
    ThreadRef {
        brand: BRAND,
        code: "5384",
        name: "Oriental Blue",
        color: &[92, 138, 192],
    },
    ThreadRef {
        brand: BRAND,
        code: "5387",
        name: "Chipmunk",
        color: &[115, 81, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "5388",
        name: "Med. Ecru",
        color: &[253, 236, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "5389",
        name: "Tiger Eye",
        color: &[170, 112, 43],
    },
    ThreadRef {
        brand: BRAND,
        code: "5390",
        name: "Ivory White",
        color: &[255, 255, 226],
    },
    ThreadRef {
        brand: BRAND,
        code: "5391",
        name: "Pale Blue",
        color: &[240, 244, 239],
    },
    ThreadRef {
        brand: BRAND,
        code: "5392",
        name: "Black",
        color: &[0, 0, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "5393",
        name: "Light Jade",
        color: &[142, 217, 120],
    },
    ThreadRef {
        brand: BRAND,
        code: "5397",
        name: "Warm Wine",
        color: &[106, 29, 40],
    },
    ThreadRef {
        brand: BRAND,
        code: "5398",
        name: "Lt. Olive",
        color: &[105, 130, 73],
    },
    ThreadRef {
        brand: BRAND,
        code: "5399",
        name: "Parrot Green Very Light",
        color: &[246, 255, 195],
    },
    ThreadRef {
        brand: BRAND,
        code: "5400",
        name: "Wild Fuschia",
        color: &[177, 13, 85],
    },
    ThreadRef {
        brand: BRAND,
        code: "5403",
        name: "Alpine Green",
        color: &[57, 70, 26],
    },
    ThreadRef {
        brand: BRAND,
        code: "5404",
        name: "Sea Green Medium Dark",
        color: &[92, 167, 124],
    },
    ThreadRef {
        brand: BRAND,
        code: "5405",
        name: "Limerick",
        color: &[177, 188, 106],
    },
    ThreadRef {
        brand: BRAND,
        code: "5406",
        name: "Harbor Green",
        color: &[56, 84, 46],
    },
    ThreadRef {
        brand: BRAND,
        code: "5408",
        name: "Green",
        color: &[81, 197, 43],
    },
    ThreadRef {
        brand: BRAND,
        code: "5409",
        name: "Lake Blue",
        color: &[150, 194, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "5410",
        name: "Pale Blue",
        color: &[155, 210, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "5411",
        name: "Pastal Blue",
        color: &[186, 207, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "5412",
        name: "Canary",
        color: &[255, 255, 68],
    },
    ThreadRef {
        brand: BRAND,
        code: "5415",
        name: "Deep Ocean",
        color: &[22, 96, 142],
    },
    ThreadRef {
        brand: BRAND,
        code: "5416",
        name: "Ivy",
        color: &[10, 50, 39],
    },
    ThreadRef {
        brand: BRAND,
        code: "5419",
        name: "China Rose Medium",
        color: &[255, 205, 209],
    },
    ThreadRef {
        brand: BRAND,
        code: "5423",
        name: "Taupe",
        color: &[208, 190, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "5431",
        name: "Pear",
        color: &[83, 163, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "5435",
        name: "Bright Pink",
        color: &[192, 66, 126],
    },
    ThreadRef {
        brand: BRAND,
        code: "5440",
        name: "Kelly",
        color: &[60, 135, 44],
    },
    ThreadRef {
        brand: BRAND,
        code: "5441",
        name: "Dark Green",
        color: &[46, 113, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "5442",
        name: "Lavender",
        color: &[153, 127, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "5443",
        name: "Twilight Lupine",
        color: &[90, 64, 127],
    },
    ThreadRef {
        brand: BRAND,
        code: "5444",
        name: "Cristy Blue",
        color: &[185, 179, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "5445",
        name: "Medium Purple",
        color: &[98, 112, 180],
    },
    ThreadRef {
        brand: BRAND,
        code: "5446",
        name: "Dark Royal Blue",
        color: &[41, 54, 129],
    },
    ThreadRef {
        brand: BRAND,
        code: "5464",
        name: "Applegreen",
        color: &[234, 253, 136],
    },
    ThreadRef {
        brand: BRAND,
        code: "5480",
        name: "Meadow Green",
        color: &[119, 143, 23],
    },
    ThreadRef {
        brand: BRAND,
        code: "5489",
        name: "Blossem Pink Dark",
        color: &[234, 28, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "5490",
        name: "Atom Red",
        color: &[170, 16, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "5496",
        name: "Aqua",
        color: &[38, 171, 156],
    },
    ThreadRef {
        brand: BRAND,
        code: "5497",
        name: "Dark Purple/Navy",
        color: &[51, 18, 48],
    },
];