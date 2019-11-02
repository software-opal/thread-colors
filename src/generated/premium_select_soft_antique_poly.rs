#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Premium Select Soft Antique Poly";
pub const THREADS: [ThreadRef; 93] = [
    ThreadRef::new(
        BRAND,
        "3001",
        "Paper White",
        &[255, 255, 255],
    ),
    ThreadRef::new(
        BRAND,
        "3002",
        "Black",
        &[0, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "3008",
        "Red",
        &[205, 32, 44],
    ),
    ThreadRef::new(
        BRAND,
        "3009",
        "Poinsettia",
        &[167, 25, 48],
    ),
    ThreadRef::new(
        BRAND,
        "3010",
        "Cranberry",
        &[131, 51, 74],
    ),
    ThreadRef::new(
        BRAND,
        "3012",
        "Bordeaux",
        &[100, 31, 69],
    ),
    ThreadRef::new(
        BRAND,
        "3014",
        "Emerald",
        &[0, 121, 52],
    ),
    ThreadRef::new(
        BRAND,
        "3015",
        "Shamrock",
        &[0, 133, 66],
    ),
    ThreadRef::new(
        BRAND,
        "3016",
        "Kelly",
        &[0, 122, 77],
    ),
    ThreadRef::new(
        BRAND,
        "3017",
        "Trellis Green",
        &[0, 181, 36],
    ),
    ThreadRef::new(
        BRAND,
        "3018",
        "Celestial",
        &[142, 186, 229],
    ),
    ThreadRef::new(
        BRAND,
        "3024",
        "Azure Blue",
        &[160, 207, 235],
    ),
    ThreadRef::new(
        BRAND,
        "3025",
        "Laguna",
        &[33, 87, 138],
    ),
    ThreadRef::new(
        BRAND,
        "3027",
        "Starlight Blue",
        &[83, 98, 171],
    ),
    ThreadRef::new(
        BRAND,
        "3031",
        "Oxford",
        &[152, 198, 234],
    ),
    ThreadRef::new(
        BRAND,
        "3033",
        "Linen",
        &[236, 222, 187],
    ),
    ThreadRef::new(
        BRAND,
        "3034",
        "Vanilla",
        &[247, 214, 165],
    ),
    ThreadRef::new(
        BRAND,
        "3035",
        "Caramel Cream",
        &[189, 138, 94],
    ),
    ThreadRef::new(
        BRAND,
        "3040",
        "Citrus",
        &[252, 217, 0],
    ),
    ThreadRef::new(
        BRAND,
        "3043",
        "Red Pepper",
        &[251, 79, 20],
    ),
    ThreadRef::new(
        BRAND,
        "3046",
        "Pumpkin",
        &[255, 121, 0],
    ),
    ThreadRef::new(
        BRAND,
        "3062",
        "Cherry",
        &[158, 48, 157],
    ),
    ThreadRef::new(
        BRAND,
        "3064",
        "Dolphin Grey",
        &[142, 144, 143],
    ),
    ThreadRef::new(
        BRAND,
        "3068",
        "Light Grey",
        &[206, 207, 203],
    ),
    ThreadRef::new(
        BRAND,
        "3069",
        "Grey",
        &[129, 138, 143],
    ),
    ThreadRef::new(
        BRAND,
        "3073",
        "Copenhagen",
        &[92, 127, 146],
    ),
    ThreadRef::new(
        BRAND,
        "3083",
        "Golden Grain",
        &[156, 97, 20],
    ),
    ThreadRef::new(
        BRAND,
        "3090",
        "Rust",
        &[131, 56, 32],
    ),
    ThreadRef::new(
        BRAND,
        "3091",
        "Espresso",
        &[93, 79, 75],
    ),
    ThreadRef::new(
        BRAND,
        "3092",
        "Chocolate",
        &[81, 43, 27],
    ),
    ThreadRef::new(
        BRAND,
        "3097",
        "Lavender",
        &[190, 165, 202],
    ),
    ThreadRef::new(
        BRAND,
        "3099",
        "Wild Iris",
        &[130, 75, 176],
    ),
    ThreadRef::new(
        BRAND,
        "3104",
        "Dusty Rose",
        &[157, 102, 112],
    ),
    ThreadRef::new(
        BRAND,
        "3114",
        "Raspberry",
        &[218, 72, 126],
    ),
    ThreadRef::new(
        BRAND,
        "3118",
        "Mauve",
        &[215, 131, 150],
    ),
    ThreadRef::new(
        BRAND,
        "3122",
        "Buttercup",
        &[242, 214, 83],
    ),
    ThreadRef::new(
        BRAND,
        "3128",
        "Honey Gold",
        &[186, 111, 46],
    ),
    ThreadRef::new(
        BRAND,
        "3129",
        "Nutmeg",
        &[95, 51, 22],
    ),
    ThreadRef::new(
        BRAND,
        "3136",
        "Concord",
        &[33, 49, 77],
    ),
    ThreadRef::new(
        BRAND,
        "3138",
        "Midnight",
        &[3, 32, 47],
    ),
    ThreadRef::new(
        BRAND,
        "3143",
        "Marsh",
        &[190, 214, 0],
    ),
    ThreadRef::new(
        BRAND,
        "3161",
        "Seafoam",
        &[114, 220, 212],
    ),
    ThreadRef::new(
        BRAND,
        "3165",
        "Wave Blue",
        &[0, 169, 224],
    ),
    ThreadRef::new(
        BRAND,
        "3169",
        "Tropical Blue",
        &[0, 102, 161],
    ),
    ThreadRef::new(
        BRAND,
        "3174",
        "Dark Concord",
        &[0, 61, 76],
    ),
    ThreadRef::new(
        BRAND,
        "3182",
        "Irish Green",
        &[0, 105, 60],
    ),
    ThreadRef::new(
        BRAND,
        "3183",
        "Soft Pink",
        &[229, 159, 219],
    ),
    ThreadRef::new(
        BRAND,
        "3193",
        "Lime Drop",
        &[105, 190, 40],
    ),
    ThreadRef::new(
        BRAND,
        "3194",
        "Dark Ink",
        &[31, 20, 93],
    ),
    ThreadRef::new(
        BRAND,
        "3196",
        "Dark Turquoise",
        &[0, 116, 122],
    ),
    ThreadRef::new(
        BRAND,
        "3197",
        "Medium Navy",
        &[0, 33, 71],
    ),
    ThreadRef::new(
        BRAND,
        "3200",
        "Royal Blue",
        &[33, 36, 146],
    ),
    ThreadRef::new(
        BRAND,
        "3201",
        "Sapphire",
        &[3, 31, 115],
    ),
    ThreadRef::new(
        BRAND,
        "3202",
        "Purple",
        &[90, 36, 90],
    ),
    ThreadRef::new(
        BRAND,
        "3203",
        "Fire Blue",
        &[0, 53, 145],
    ),
    ThreadRef::new(
        BRAND,
        "3209",
        "Dark Pewter",
        &[94, 106, 113],
    ),
    ThreadRef::new(
        BRAND,
        "3212",
        "Whale Grey",
        &[72, 81, 86],
    ),
    ThreadRef::new(
        BRAND,
        "3223",
        "Petal Pink",
        &[243, 201, 211],
    ),
    ThreadRef::new(
        BRAND,
        "3227",
        "Azalea Pink",
        &[243, 173, 222],
    ),
    ThreadRef::new(
        BRAND,
        "3242",
        "Shrimp",
        &[239, 190, 156],
    ),
    ThreadRef::new(
        BRAND,
        "3269",
        "Dark Shrimp",
        &[233, 153, 74],
    ),
    ThreadRef::new(
        BRAND,
        "3271",
        "Canyon Gold",
        &[197, 146, 23],
    ),
    ThreadRef::new(
        BRAND,
        "3277",
        "Stone",
        &[198, 188, 137],
    ),
    ThreadRef::new(
        BRAND,
        "3311",
        "Silver Sage",
        &[145, 216, 174],
    ),
    ThreadRef::new(
        BRAND,
        "3313",
        "Pale Luster",
        &[170, 201, 182],
    ),
    ThreadRef::new(
        BRAND,
        "3320",
        "Medium Turquoise",
        &[0, 176, 202],
    ),
    ThreadRef::new(
        BRAND,
        "3323",
        "Tropical Blue",
        &[0, 117, 176],
    ),
    ThreadRef::new(
        BRAND,
        "3328",
        "Blue",
        &[42, 110, 187],
    ),
    ThreadRef::new(
        BRAND,
        "3330",
        "Dark Blue",
        &[0, 44, 119],
    ),
    ThreadRef::new(
        BRAND,
        "3337",
        "Forest Green",
        &[0, 73, 83],
    ),
    ThreadRef::new(
        BRAND,
        "3338",
        "Swamp Green",
        &[0, 58, 66],
    ),
    ThreadRef::new(
        BRAND,
        "3344",
        "Evergreen",
        &[28, 69, 59],
    ),
    ThreadRef::new(
        BRAND,
        "3347",
        "Spruce",
        &[0, 80, 92],
    ),
    ThreadRef::new(
        BRAND,
        "3363",
        "Gold",
        &[234, 171, 0],
    ),
    ThreadRef::new(
        BRAND,
        "3366",
        "Liberty Gold",
        &[171, 132, 34],
    ),
    ThreadRef::new(
        BRAND,
        "3369",
        "Fox Brown",
        &[96, 53, 29],
    ),
    ThreadRef::new(
        BRAND,
        "3603",
        "White",
        &[255, 255, 255],
    ),
    ThreadRef::new(
        BRAND,
        "3608",
        "Candy Apple",
        &[198, 12, 48],
    ),
    ThreadRef::new(
        BRAND,
        "3609",
        "Country Red",
        &[152, 30, 50],
    ),
    ThreadRef::new(
        BRAND,
        "3612",
        "Rio Red",
        &[130, 36, 51],
    ),
    ThreadRef::new(
        BRAND,
        "3613",
        "Red Wine",
        &[75, 41, 66],
    ),
    ThreadRef::new(
        BRAND,
        "3616",
        "Cherry",
        &[167, 25, 48],
    ),
    ThreadRef::new(
        BRAND,
        "3620",
        "Beet Red",
        &[110, 39, 61],
    ),
    ThreadRef::new(
        BRAND,
        "3635",
        "Champagne",
        &[179, 153, 93],
    ),
    ThreadRef::new(
        BRAND,
        "3711",
        "Dark Fuchsia",
        &[163, 0, 80],
    ),
    ThreadRef::new(
        BRAND,
        "3778",
        "Reef Blue",
        &[75, 146, 219],
    ),
    ThreadRef::new(
        BRAND,
        "3804",
        "Teaberry",
        &[222, 183, 202],
    ),
    ThreadRef::new(
        BRAND,
        "3815",
        "Dark Emerald",
        &[88, 166, 24],
    ),
    ThreadRef::new(
        BRAND,
        "3834",
        "Chiffon",
        &[235, 202, 184],
    ),
    ThreadRef::new(
        BRAND,
        "3845",
        "Pale Orange",
        &[255, 158, 113],
    ),
    ThreadRef::new(
        BRAND,
        "3873",
        "Sterling",
        &[195, 200, 200],
    ),
    ThreadRef::new(
        BRAND,
        "3879",
        "Blueberry",
        &[36, 0, 120],
    ),
    ThreadRef::new(
        BRAND,
        "3899",
        "Iris",
        &[79, 45, 127],
    ),
];