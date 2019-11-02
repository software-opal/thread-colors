#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "Premium Select Soft Antique Poly";
const THREADS: [ThreadRef; 93] = [
    ThreadRef {
        brand: BRAND,
        code: "3001",
        name: "Paper White",
        color: &[255, 255, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "3002",
        name: "Black",
        color: &[0, 0, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "3008",
        name: "Red",
        color: &[205, 32, 44],
    },
    ThreadRef {
        brand: BRAND,
        code: "3009",
        name: "Poinsettia",
        color: &[167, 25, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "3010",
        name: "Cranberry",
        color: &[131, 51, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "3012",
        name: "Bordeaux",
        color: &[100, 31, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "3014",
        name: "Emerald",
        color: &[0, 121, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "3015",
        name: "Shamrock",
        color: &[0, 133, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "3016",
        name: "Kelly",
        color: &[0, 122, 77],
    },
    ThreadRef {
        brand: BRAND,
        code: "3017",
        name: "Trellis Green",
        color: &[0, 181, 36],
    },
    ThreadRef {
        brand: BRAND,
        code: "3018",
        name: "Celestial",
        color: &[142, 186, 229],
    },
    ThreadRef {
        brand: BRAND,
        code: "3024",
        name: "Azure Blue",
        color: &[160, 207, 235],
    },
    ThreadRef {
        brand: BRAND,
        code: "3025",
        name: "Laguna",
        color: &[33, 87, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "3027",
        name: "Starlight Blue",
        color: &[83, 98, 171],
    },
    ThreadRef {
        brand: BRAND,
        code: "3031",
        name: "Oxford",
        color: &[152, 198, 234],
    },
    ThreadRef {
        brand: BRAND,
        code: "3033",
        name: "Linen",
        color: &[236, 222, 187],
    },
    ThreadRef {
        brand: BRAND,
        code: "3034",
        name: "Vanilla",
        color: &[247, 214, 165],
    },
    ThreadRef {
        brand: BRAND,
        code: "3035",
        name: "Caramel Cream",
        color: &[189, 138, 94],
    },
    ThreadRef {
        brand: BRAND,
        code: "3040",
        name: "Citrus",
        color: &[252, 217, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "3043",
        name: "Red Pepper",
        color: &[251, 79, 20],
    },
    ThreadRef {
        brand: BRAND,
        code: "3046",
        name: "Pumpkin",
        color: &[255, 121, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "3062",
        name: "Cherry",
        color: &[158, 48, 157],
    },
    ThreadRef {
        brand: BRAND,
        code: "3064",
        name: "Dolphin Grey",
        color: &[142, 144, 143],
    },
    ThreadRef {
        brand: BRAND,
        code: "3068",
        name: "Light Grey",
        color: &[206, 207, 203],
    },
    ThreadRef {
        brand: BRAND,
        code: "3069",
        name: "Grey",
        color: &[129, 138, 143],
    },
    ThreadRef {
        brand: BRAND,
        code: "3073",
        name: "Copenhagen",
        color: &[92, 127, 146],
    },
    ThreadRef {
        brand: BRAND,
        code: "3083",
        name: "Golden Grain",
        color: &[156, 97, 20],
    },
    ThreadRef {
        brand: BRAND,
        code: "3090",
        name: "Rust",
        color: &[131, 56, 32],
    },
    ThreadRef {
        brand: BRAND,
        code: "3091",
        name: "Espresso",
        color: &[93, 79, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "3092",
        name: "Chocolate",
        color: &[81, 43, 27],
    },
    ThreadRef {
        brand: BRAND,
        code: "3097",
        name: "Lavender",
        color: &[190, 165, 202],
    },
    ThreadRef {
        brand: BRAND,
        code: "3099",
        name: "Wild Iris",
        color: &[130, 75, 176],
    },
    ThreadRef {
        brand: BRAND,
        code: "3104",
        name: "Dusty Rose",
        color: &[157, 102, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "3114",
        name: "Raspberry",
        color: &[218, 72, 126],
    },
    ThreadRef {
        brand: BRAND,
        code: "3118",
        name: "Mauve",
        color: &[215, 131, 150],
    },
    ThreadRef {
        brand: BRAND,
        code: "3122",
        name: "Buttercup",
        color: &[242, 214, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "3128",
        name: "Honey Gold",
        color: &[186, 111, 46],
    },
    ThreadRef {
        brand: BRAND,
        code: "3129",
        name: "Nutmeg",
        color: &[95, 51, 22],
    },
    ThreadRef {
        brand: BRAND,
        code: "3136",
        name: "Concord",
        color: &[33, 49, 77],
    },
    ThreadRef {
        brand: BRAND,
        code: "3138",
        name: "Midnight",
        color: &[3, 32, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "3143",
        name: "Marsh",
        color: &[190, 214, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "3161",
        name: "Seafoam",
        color: &[114, 220, 212],
    },
    ThreadRef {
        brand: BRAND,
        code: "3165",
        name: "Wave Blue",
        color: &[0, 169, 224],
    },
    ThreadRef {
        brand: BRAND,
        code: "3169",
        name: "Tropical Blue",
        color: &[0, 102, 161],
    },
    ThreadRef {
        brand: BRAND,
        code: "3174",
        name: "Dark Concord",
        color: &[0, 61, 76],
    },
    ThreadRef {
        brand: BRAND,
        code: "3182",
        name: "Irish Green",
        color: &[0, 105, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "3183",
        name: "Soft Pink",
        color: &[229, 159, 219],
    },
    ThreadRef {
        brand: BRAND,
        code: "3193",
        name: "Lime Drop",
        color: &[105, 190, 40],
    },
    ThreadRef {
        brand: BRAND,
        code: "3194",
        name: "Dark Ink",
        color: &[31, 20, 93],
    },
    ThreadRef {
        brand: BRAND,
        code: "3196",
        name: "Dark Turquoise",
        color: &[0, 116, 122],
    },
    ThreadRef {
        brand: BRAND,
        code: "3197",
        name: "Medium Navy",
        color: &[0, 33, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "3200",
        name: "Royal Blue",
        color: &[33, 36, 146],
    },
    ThreadRef {
        brand: BRAND,
        code: "3201",
        name: "Sapphire",
        color: &[3, 31, 115],
    },
    ThreadRef {
        brand: BRAND,
        code: "3202",
        name: "Purple",
        color: &[90, 36, 90],
    },
    ThreadRef {
        brand: BRAND,
        code: "3203",
        name: "Fire Blue",
        color: &[0, 53, 145],
    },
    ThreadRef {
        brand: BRAND,
        code: "3209",
        name: "Dark Pewter",
        color: &[94, 106, 113],
    },
    ThreadRef {
        brand: BRAND,
        code: "3212",
        name: "Whale Grey",
        color: &[72, 81, 86],
    },
    ThreadRef {
        brand: BRAND,
        code: "3223",
        name: "Petal Pink",
        color: &[243, 201, 211],
    },
    ThreadRef {
        brand: BRAND,
        code: "3227",
        name: "Azalea Pink",
        color: &[243, 173, 222],
    },
    ThreadRef {
        brand: BRAND,
        code: "3242",
        name: "Shrimp",
        color: &[239, 190, 156],
    },
    ThreadRef {
        brand: BRAND,
        code: "3269",
        name: "Dark Shrimp",
        color: &[233, 153, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "3271",
        name: "Canyon Gold",
        color: &[197, 146, 23],
    },
    ThreadRef {
        brand: BRAND,
        code: "3277",
        name: "Stone",
        color: &[198, 188, 137],
    },
    ThreadRef {
        brand: BRAND,
        code: "3311",
        name: "Silver Sage",
        color: &[145, 216, 174],
    },
    ThreadRef {
        brand: BRAND,
        code: "3313",
        name: "Pale Luster",
        color: &[170, 201, 182],
    },
    ThreadRef {
        brand: BRAND,
        code: "3320",
        name: "Medium Turquoise",
        color: &[0, 176, 202],
    },
    ThreadRef {
        brand: BRAND,
        code: "3323",
        name: "Tropical Blue",
        color: &[0, 117, 176],
    },
    ThreadRef {
        brand: BRAND,
        code: "3328",
        name: "Blue",
        color: &[42, 110, 187],
    },
    ThreadRef {
        brand: BRAND,
        code: "3330",
        name: "Dark Blue",
        color: &[0, 44, 119],
    },
    ThreadRef {
        brand: BRAND,
        code: "3337",
        name: "Forest Green",
        color: &[0, 73, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "3338",
        name: "Swamp Green",
        color: &[0, 58, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "3344",
        name: "Evergreen",
        color: &[28, 69, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "3347",
        name: "Spruce",
        color: &[0, 80, 92],
    },
    ThreadRef {
        brand: BRAND,
        code: "3363",
        name: "Gold",
        color: &[234, 171, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "3366",
        name: "Liberty Gold",
        color: &[171, 132, 34],
    },
    ThreadRef {
        brand: BRAND,
        code: "3369",
        name: "Fox Brown",
        color: &[96, 53, 29],
    },
    ThreadRef {
        brand: BRAND,
        code: "3603",
        name: "White",
        color: &[255, 255, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "3608",
        name: "Candy Apple",
        color: &[198, 12, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "3609",
        name: "Country Red",
        color: &[152, 30, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "3612",
        name: "Rio Red",
        color: &[130, 36, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "3613",
        name: "Red Wine",
        color: &[75, 41, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "3616",
        name: "Cherry",
        color: &[167, 25, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "3620",
        name: "Beet Red",
        color: &[110, 39, 61],
    },
    ThreadRef {
        brand: BRAND,
        code: "3635",
        name: "Champagne",
        color: &[179, 153, 93],
    },
    ThreadRef {
        brand: BRAND,
        code: "3711",
        name: "Dark Fuchsia",
        color: &[163, 0, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "3778",
        name: "Reef Blue",
        color: &[75, 146, 219],
    },
    ThreadRef {
        brand: BRAND,
        code: "3804",
        name: "Teaberry",
        color: &[222, 183, 202],
    },
    ThreadRef {
        brand: BRAND,
        code: "3815",
        name: "Dark Emerald",
        color: &[88, 166, 24],
    },
    ThreadRef {
        brand: BRAND,
        code: "3834",
        name: "Chiffon",
        color: &[235, 202, 184],
    },
    ThreadRef {
        brand: BRAND,
        code: "3845",
        name: "Pale Orange",
        color: &[255, 158, 113],
    },
    ThreadRef {
        brand: BRAND,
        code: "3873",
        name: "Sterling",
        color: &[195, 200, 200],
    },
    ThreadRef {
        brand: BRAND,
        code: "3879",
        name: "Blueberry",
        color: &[36, 0, 120],
    },
    ThreadRef {
        brand: BRAND,
        code: "3899",
        name: "Iris",
        color: &[79, 45, 127],
    },
];