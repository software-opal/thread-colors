#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "BFC Poly";
pub const THREADS: [ThreadRef; 320] = [
    ThreadRef::new(
        BRAND,
        "2168",
        "White",
        &[247, 247, 247],
    ),
    ThreadRef::new(
        BRAND,
        "2169",
        "Bright White",
        &[240, 240, 240],
    ),
    ThreadRef::new(
        BRAND,
        "2170",
        "Black",
        &[0, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2172",
        "Antique White",
        &[235, 231, 219],
    ),
    ThreadRef::new(
        BRAND,
        "2181",
        "PALE Salmon Pink",
        &[237, 201, 212],
    ),
    ThreadRef::new(
        BRAND,
        "2182",
        "LT Salmon Pink",
        &[252, 187, 209],
    ),
    ThreadRef::new(
        BRAND,
        "2187",
        "MD Salmon Pink",
        &[248, 141, 172],
    ),
    ThreadRef::new(
        BRAND,
        "2189",
        "PALE Pink",
        &[230, 207, 217],
    ),
    ThreadRef::new(
        BRAND,
        "2193",
        "LT Pink",
        &[255, 188, 224],
    ),
    ThreadRef::new(
        BRAND,
        "2198",
        "Pink",
        &[253, 165, 203],
    ),
    ThreadRef::new(
        BRAND,
        "2199",
        "MD Pink",
        &[229, 86, 109],
    ),
    ThreadRef::new(
        BRAND,
        "2202",
        "DK Pink",
        &[237, 114, 170],
    ),
    ThreadRef::new(
        BRAND,
        "2208",
        "PALE Pink Ice",
        &[235, 225, 255],
    ),
    ThreadRef::new(
        BRAND,
        "2218",
        "Pink Lady",
        &[234, 15, 107],
    ),
    ThreadRef::new(
        BRAND,
        "2219",
        "Fushia",
        &[196, 43, 112],
    ),
    ThreadRef::new(
        BRAND,
        "2220",
        "MD Fushia",
        &[216, 69, 138],
    ),
    ThreadRef::new(
        BRAND,
        "2222",
        "LT Cranberry",
        &[215, 61, 99],
    ),
    ThreadRef::new(
        BRAND,
        "2223",
        "MD Cranberry",
        &[165, 42, 70],
    ),
    ThreadRef::new(
        BRAND,
        "2226",
        "DK Cranberry",
        &[166, 23, 85],
    ),
    ThreadRef::new(
        BRAND,
        "2228",
        "MD Bright Pink",
        &[225, 140, 190],
    ),
    ThreadRef::new(
        BRAND,
        "2235",
        "MD Desert Rose",
        &[209, 161, 212],
    ),
    ThreadRef::new(
        BRAND,
        "2241",
        "MD Bright Plum",
        &[168, 50, 97],
    ),
    ThreadRef::new(
        BRAND,
        "2243",
        "DK Bright Plum",
        &[133, 28, 86],
    ),
    ThreadRef::new(
        BRAND,
        "2245",
        "DK Fushia",
        &[181, 66, 133],
    ),
    ThreadRef::new(
        BRAND,
        "2247",
        "DK Burgandy",
        &[102, 11, 41],
    ),
    ThreadRef::new(
        BRAND,
        "2250",
        "PLST Peach",
        &[238, 209, 202],
    ),
    ThreadRef::new(
        BRAND,
        "2253",
        "LT Peach",
        &[233, 172, 165],
    ),
    ThreadRef::new(
        BRAND,
        "2256",
        "MD Peach",
        &[255, 138, 142],
    ),
    ThreadRef::new(
        BRAND,
        "2260",
        "DK Peach",
        &[255, 105, 109],
    ),
    ThreadRef::new(
        BRAND,
        "2270",
        "DK Salmon Pink",
        &[242, 124, 138],
    ),
    ThreadRef::new(
        BRAND,
        "2271",
        "MD Pink Carnation",
        &[253, 152, 180],
    ),
    ThreadRef::new(
        BRAND,
        "2275",
        "DK Pink Carnation",
        &[244, 71, 91],
    ),
    ThreadRef::new(
        BRAND,
        "2278",
        "Ruby Red",
        &[168, 54, 81],
    ),
    ThreadRef::new(
        BRAND,
        "2283",
        "DK Red Rust",
        &[201, 80, 66],
    ),
    ThreadRef::new(
        BRAND,
        "2287",
        "DKST Burgandy",
        &[97, 37, 51],
    ),
    ThreadRef::new(
        BRAND,
        "2291",
        "Bright Red",
        &[214, 58, 50],
    ),
    ThreadRef::new(
        BRAND,
        "2295",
        "Cherry Red",
        &[184, 20, 42],
    ),
    ThreadRef::new(
        BRAND,
        "2297",
        "Satin Red",
        &[161, 25, 25],
    ),
    ThreadRef::new(
        BRAND,
        "2303",
        "DK Satin Red",
        &[133, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2306",
        "DKST Satin Red",
        &[116, 5, 11],
    ),
    ThreadRef::new(
        BRAND,
        "2307",
        "Red Wine",
        &[138, 9, 13],
    ),
    ThreadRef::new(
        BRAND,
        "2315",
        "Brandy",
        &[196, 72, 107],
    ),
    ThreadRef::new(
        BRAND,
        "2317",
        "MD Burgandy",
        &[148, 55, 81],
    ),
    ThreadRef::new(
        BRAND,
        "2320",
        "Desert Rose",
        &[194, 122, 161],
    ),
    ThreadRef::new(
        BRAND,
        "2331",
        "LT Fushia",
        &[199, 95, 138],
    ),
    ThreadRef::new(
        BRAND,
        "2342",
        "Black Wine",
        &[74, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2368",
        "PALE Yellow",
        &[247, 247, 183],
    ),
    ThreadRef::new(
        BRAND,
        "2371",
        "LT Yellow",
        &[247, 241, 153],
    ),
    ThreadRef::new(
        BRAND,
        "2376",
        "Yellow",
        &[255, 239, 11],
    ),
    ThreadRef::new(
        BRAND,
        "2379",
        "Buttered Popcorn",
        &[255, 227, 87],
    ),
    ThreadRef::new(
        BRAND,
        "2380",
        "Buttercup",
        &[245, 200, 95],
    ),
    ThreadRef::new(
        BRAND,
        "2385",
        "Bright Yellow",
        &[233, 202, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2388",
        "PALE Mellow Yellow",
        &[240, 238, 183],
    ),
    ThreadRef::new(
        BRAND,
        "2390",
        "LT Buttered Popcorn",
        &[250, 233, 160],
    ),
    ThreadRef::new(
        BRAND,
        "2403",
        "Bright Popcorn",
        &[255, 230, 100],
    ),
    ThreadRef::new(
        BRAND,
        "2410",
        "LT Mock Orange",
        &[255, 196, 133],
    ),
    ThreadRef::new(
        BRAND,
        "2411",
        "LT Apricot",
        &[255, 197, 61],
    ),
    ThreadRef::new(
        BRAND,
        "2412",
        "MD Apricot",
        &[255, 191, 71],
    ),
    ThreadRef::new(
        BRAND,
        "2413",
        "Apricot",
        &[255, 203, 101],
    ),
    ThreadRef::new(
        BRAND,
        "2414",
        "DK Apricot",
        &[247, 182, 52],
    ),
    ThreadRef::new(
        BRAND,
        "2416",
        "MD Mock  Orange",
        &[230, 145, 34],
    ),
    ThreadRef::new(
        BRAND,
        "2426",
        "DKST Apricot",
        &[255, 146, 23],
    ),
    ThreadRef::new(
        BRAND,
        "2428",
        "PALE Peach Fuzz",
        &[255, 201, 159],
    ),
    ThreadRef::new(
        BRAND,
        "2430",
        "MD Peach Fuzz",
        &[255, 183, 151],
    ),
    ThreadRef::new(
        BRAND,
        "2433",
        "LT Melon",
        &[253, 205, 195],
    ),
    ThreadRef::new(
        BRAND,
        "2435",
        "Melon",
        &[252, 169, 152],
    ),
    ThreadRef::new(
        BRAND,
        "2437",
        "DK Melon",
        &[255, 136, 101],
    ),
    ThreadRef::new(
        BRAND,
        "2451",
        "Bright orange",
        &[227, 114, 20],
    ),
    ThreadRef::new(
        BRAND,
        "2454",
        "Orange",
        &[210, 58, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2455",
        "Sunset Orange",
        &[255, 121, 84],
    ),
    ThreadRef::new(
        BRAND,
        "2458",
        "Red Orange",
        &[255, 102, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2465",
        "DK Red Orange",
        &[205, 46, 14],
    ),
    ThreadRef::new(
        BRAND,
        "2468",
        "PLST Salmon",
        &[250, 206, 195],
    ),
    ThreadRef::new(
        BRAND,
        "2473",
        "PALE Salmon",
        &[255, 200, 191],
    ),
    ThreadRef::new(
        BRAND,
        "2484",
        "Terra Cotta",
        &[186, 82, 47],
    ),
    ThreadRef::new(
        BRAND,
        "2490",
        "Beige",
        &[220, 210, 189],
    ),
    ThreadRef::new(
        BRAND,
        "2499",
        "LT Rose Taupe",
        &[189, 147, 138],
    ),
    ThreadRef::new(
        BRAND,
        "2504",
        "LT Terra Cotta",
        &[201, 129, 66],
    ),
    ThreadRef::new(
        BRAND,
        "2508",
        "PLST Mink",
        &[232, 216, 202],
    ),
    ThreadRef::new(
        BRAND,
        "2509",
        "LT Mink",
        &[217, 202, 190],
    ),
    ThreadRef::new(
        BRAND,
        "2513",
        "LT Salmon",
        &[255, 188, 182],
    ),
    ThreadRef::new(
        BRAND,
        "2514",
        "MD Salmon",
        &[255, 175, 139],
    ),
    ThreadRef::new(
        BRAND,
        "2515",
        "LT Rose",
        &[243, 170, 186],
    ),
    ThreadRef::new(
        BRAND,
        "2521",
        "MD Ash Rose",
        &[158, 106, 116],
    ),
    ThreadRef::new(
        BRAND,
        "2545",
        "LT Ash Rose",
        &[173, 117, 128],
    ),
    ThreadRef::new(
        BRAND,
        "2546",
        "DK Ash Rose",
        &[125, 56, 68],
    ),
    ThreadRef::new(
        BRAND,
        "2552",
        "PALE Cameo",
        &[219, 191, 170],
    ),
    ThreadRef::new(
        BRAND,
        "2553",
        "Bisque",
        &[233, 213, 186],
    ),
    ThreadRef::new(
        BRAND,
        "2567",
        "DK Brick",
        &[117, 37, 38],
    ),
    ThreadRef::new(
        BRAND,
        "2572",
        "PALE Mauve Gray",
        &[191, 177, 168],
    ),
    ThreadRef::new(
        BRAND,
        "2575",
        "PALE Gray",
        &[209, 204, 201],
    ),
    ThreadRef::new(
        BRAND,
        "2581",
        "MD Mauve",
        &[124, 97, 116],
    ),
    ThreadRef::new(
        BRAND,
        "2586",
        "Mahogany",
        &[107, 52, 58],
    ),
    ThreadRef::new(
        BRAND,
        "2602",
        "Eggplant",
        &[109, 79, 69],
    ),
    ThreadRef::new(
        BRAND,
        "2606",
        "DK Mauve",
        &[64, 24, 24],
    ),
    ThreadRef::new(
        BRAND,
        "2611",
        "Nutmeg",
        &[128, 125, 95],
    ),
    ThreadRef::new(
        BRAND,
        "2630",
        "LT Sandstone",
        &[242, 205, 176],
    ),
    ThreadRef::new(
        BRAND,
        "2642",
        "Chestnut",
        &[133, 82, 39],
    ),
    ThreadRef::new(
        BRAND,
        "2656",
        "Warm Sand",
        &[215, 184, 127],
    ),
    ThreadRef::new(
        BRAND,
        "2657",
        "Mustard Gold",
        &[209, 162, 92],
    ),
    ThreadRef::new(
        BRAND,
        "2675",
        "PALE Rust",
        &[253, 201, 164],
    ),
    ThreadRef::new(
        BRAND,
        "2689",
        "Ecru",
        &[236, 216, 183],
    ),
    ThreadRef::new(
        BRAND,
        "2690",
        "PALE Ecru",
        &[245, 229, 206],
    ),
    ThreadRef::new(
        BRAND,
        "2693",
        "LT Ecru",
        &[237, 217, 184],
    ),
    ThreadRef::new(
        BRAND,
        "2695",
        "DK Honey",
        &[202, 140, 103],
    ),
    ThreadRef::new(
        BRAND,
        "2709",
        "LT Putty",
        &[246, 214, 167],
    ),
    ThreadRef::new(
        BRAND,
        "2721",
        "Bright Rust",
        &[189, 97, 20],
    ),
    ThreadRef::new(
        BRAND,
        "2729",
        "MD Putty",
        &[222, 194, 160],
    ),
    ThreadRef::new(
        BRAND,
        "2731",
        "Hemp",
        &[204, 187, 158],
    ),
    ThreadRef::new(
        BRAND,
        "2745",
        "MD Chestnut",
        &[120, 73, 63],
    ),
    ThreadRef::new(
        BRAND,
        "2747",
        "DK Chestnut",
        &[99, 68, 54],
    ),
    ThreadRef::new(
        BRAND,
        "2748",
        "Taupe",
        &[173, 157, 132],
    ),
    ThreadRef::new(
        BRAND,
        "2751",
        "PALE Gold",
        &[219, 215, 153],
    ),
    ThreadRef::new(
        BRAND,
        "2756",
        "DK Mustard Gold",
        &[168, 129, 81],
    ),
    ThreadRef::new(
        BRAND,
        "2758",
        "MD Rust",
        &[169, 115, 69],
    ),
    ThreadRef::new(
        BRAND,
        "2759",
        "DK Taupe",
        &[135, 120, 96],
    ),
    ThreadRef::new(
        BRAND,
        "2760",
        "MD Taupe",
        &[149, 136, 104],
    ),
    ThreadRef::new(
        BRAND,
        "2765",
        "DKST Camel",
        &[140, 104, 82],
    ),
    ThreadRef::new(
        BRAND,
        "2768",
        "Camel",
        &[191, 151, 115],
    ),
    ThreadRef::new(
        BRAND,
        "2772",
        "LT Golden Brown",
        &[166, 120, 90],
    ),
    ThreadRef::new(
        BRAND,
        "2781",
        "Golden Mink",
        &[156, 135, 122],
    ),
    ThreadRef::new(
        BRAND,
        "2783",
        "DK Earth",
        &[84, 53, 41],
    ),
    ThreadRef::new(
        BRAND,
        "2784",
        "DKST Earth",
        &[38, 14, 14],
    ),
    ThreadRef::new(
        BRAND,
        "2793",
        "PALE Mink",
        &[191, 175, 154],
    ),
    ThreadRef::new(
        BRAND,
        "2802",
        "Golden Brown",
        &[106, 70, 46],
    ),
    ThreadRef::new(
        BRAND,
        "2808",
        "Mink",
        &[156, 146, 142],
    ),
    ThreadRef::new(
        BRAND,
        "2809",
        "MD Mink",
        &[144, 126, 122],
    ),
    ThreadRef::new(
        BRAND,
        "2817",
        "DK Mink",
        &[124, 106, 104],
    ),
    ThreadRef::new(
        BRAND,
        "2818",
        "DKST Mink",
        &[114, 99, 94],
    ),
    ThreadRef::new(
        BRAND,
        "2819",
        "DKST Reddish Mink",
        &[82, 40, 42],
    ),
    ThreadRef::new(
        BRAND,
        "2821",
        "DK Reddish Mink",
        &[71, 35, 37],
    ),
    ThreadRef::new(
        BRAND,
        "2828",
        "Wheat",
        &[191, 187, 162],
    ),
    ThreadRef::new(
        BRAND,
        "2829",
        "Yellow wheat",
        &[240, 226, 175],
    ),
    ThreadRef::new(
        BRAND,
        "2830",
        "LT Amber",
        &[255, 203, 130],
    ),
    ThreadRef::new(
        BRAND,
        "2831",
        "MD Amber",
        &[255, 201, 121],
    ),
    ThreadRef::new(
        BRAND,
        "2832",
        "Golden Amber",
        &[255, 201, 63],
    ),
    ThreadRef::new(
        BRAND,
        "2833",
        "DK Golden Amber",
        &[222, 167, 71],
    ),
    ThreadRef::new(
        BRAND,
        "2834",
        "Harvest Gold",
        &[222, 167, 57],
    ),
    ThreadRef::new(
        BRAND,
        "2852",
        "LT Green Amber",
        &[221, 194, 93],
    ),
    ThreadRef::new(
        BRAND,
        "2862",
        "Bronze Brown",
        &[132, 88, 59],
    ),
    ThreadRef::new(
        BRAND,
        "2870",
        "PALE Olive",
        &[245, 221, 81],
    ),
    ThreadRef::new(
        BRAND,
        "2873",
        "LT Olive",
        &[181, 163, 60],
    ),
    ThreadRef::new(
        BRAND,
        "2881",
        "MD Olive",
        &[201, 174, 87],
    ),
    ThreadRef::new(
        BRAND,
        "2899",
        "DK Olive",
        &[127, 107, 36],
    ),
    ThreadRef::new(
        BRAND,
        "2906",
        "Loden Green",
        &[69, 57, 43],
    ),
    ThreadRef::new(
        BRAND,
        "2911",
        "PLST Aloe",
        &[255, 243, 201],
    ),
    ThreadRef::new(
        BRAND,
        "2913",
        "PALE Aloe",
        &[220, 221, 185],
    ),
    ThreadRef::new(
        BRAND,
        "2924",
        "DK Moss",
        &[63, 44, 40],
    ),
    ThreadRef::new(
        BRAND,
        "2926",
        "DKST Green Gray",
        &[45, 44, 42],
    ),
    ThreadRef::new(
        BRAND,
        "2928",
        "PALE Sage",
        &[209, 204, 181],
    ),
    ThreadRef::new(
        BRAND,
        "2934",
        "LT Sage",
        &[161, 149, 123],
    ),
    ThreadRef::new(
        BRAND,
        "2943",
        "DK Yellow Green",
        &[99, 96, 63],
    ),
    ThreadRef::new(
        BRAND,
        "2945",
        "DKST Sage",
        &[70, 71, 52],
    ),
    ThreadRef::new(
        BRAND,
        "2949",
        "Sage Tint White",
        &[227, 229, 216],
    ),
    ThreadRef::new(
        BRAND,
        "2970",
        "Basil",
        &[118, 141, 115],
    ),
    ThreadRef::new(
        BRAND,
        "2972",
        "Gray Basil",
        &[139, 142, 140],
    ),
    ThreadRef::new(
        BRAND,
        "3008",
        "LT Smoke Green",
        &[170, 191, 183],
    ),
    ThreadRef::new(
        BRAND,
        "3009",
        "MD Smoke Green",
        &[126, 172, 135],
    ),
    ThreadRef::new(
        BRAND,
        "3013",
        "DK Smoke Green",
        &[66, 97, 63],
    ),
    ThreadRef::new(
        BRAND,
        "3014",
        "DKST Smoke Green",
        &[61, 91, 63],
    ),
    ThreadRef::new(
        BRAND,
        "3023",
        "DKST Pine Green",
        &[61, 66, 24],
    ),
    ThreadRef::new(
        BRAND,
        "3028",
        "PLST Smoke green",
        &[240, 254, 238],
    ),
    ThreadRef::new(
        BRAND,
        "3032",
        "PALE Smoke Green",
        &[183, 207, 180],
    ),
    ThreadRef::new(
        BRAND,
        "3034",
        "LT Spruce",
        &[107, 145, 110],
    ),
    ThreadRef::new(
        BRAND,
        "3035",
        "MD Spruce",
        &[40, 87, 32],
    ),
    ThreadRef::new(
        BRAND,
        "3038",
        "DK Spruce",
        &[33, 71, 17],
    ),
    ThreadRef::new(
        BRAND,
        "3049",
        "LT Jade",
        &[162, 212, 178],
    ),
    ThreadRef::new(
        BRAND,
        "3050",
        "MD Aqua Green",
        &[90, 158, 144],
    ),
    ThreadRef::new(
        BRAND,
        "3059",
        "DKST Aqua Green",
        &[16, 74, 34],
    ),
    ThreadRef::new(
        BRAND,
        "3060",
        "LT Aqua Green",
        &[101, 212, 203],
    ),
    ThreadRef::new(
        BRAND,
        "3069",
        "MD Jade",
        &[81, 192, 162],
    ),
    ThreadRef::new(
        BRAND,
        "3072",
        "DK Jade",
        &[48, 166, 72],
    ),
    ThreadRef::new(
        BRAND,
        "3075",
        "DK Aqua Green",
        &[38, 158, 130],
    ),
    ThreadRef::new(
        BRAND,
        "3080",
        "DK Emerald",
        &[8, 93, 34],
    ),
    ThreadRef::new(
        BRAND,
        "3082",
        "Bright Aqua",
        &[41, 176, 185],
    ),
    ThreadRef::new(
        BRAND,
        "3092",
        "LT Moss",
        &[165, 196, 157],
    ),
    ThreadRef::new(
        BRAND,
        "3096",
        "PALE Jade",
        &[188, 245, 196],
    ),
    ThreadRef::new(
        BRAND,
        "3105",
        "Pine Green",
        &[17, 124, 46],
    ),
    ThreadRef::new(
        BRAND,
        "3110",
        "PLST Moss",
        &[200, 255, 189],
    ),
    ThreadRef::new(
        BRAND,
        "3113",
        "Moss",
        &[127, 147, 75],
    ),
    ThreadRef::new(
        BRAND,
        "3116",
        "MD Moss",
        &[143, 177, 101],
    ),
    ThreadRef::new(
        BRAND,
        "3130",
        "PALE Mint",
        &[188, 245, 204],
    ),
    ThreadRef::new(
        BRAND,
        "3133",
        "LT Mint",
        &[168, 219, 183],
    ),
    ThreadRef::new(
        BRAND,
        "3136",
        "Spearmint",
        &[94, 125, 65],
    ),
    ThreadRef::new(
        BRAND,
        "3142",
        "DKST Grass Green",
        &[80, 115, 60],
    ),
    ThreadRef::new(
        BRAND,
        "3145",
        "DKST Moss",
        &[35, 51, 25],
    ),
    ThreadRef::new(
        BRAND,
        "3148",
        "PLST Lime",
        &[239, 242, 199],
    ),
    ThreadRef::new(
        BRAND,
        "3159",
        "PALE Spring Green",
        &[202, 245, 137],
    ),
    ThreadRef::new(
        BRAND,
        "3160",
        "MD Spring Green",
        &[155, 219, 99],
    ),
    ThreadRef::new(
        BRAND,
        "3161",
        "Spring Green",
        &[126, 204, 0],
    ),
    ThreadRef::new(
        BRAND,
        "3166",
        "DK Spring Green",
        &[105, 140, 71],
    ),
    ThreadRef::new(
        BRAND,
        "3167",
        "DKST Spring Green",
        &[15, 115, 0],
    ),
    ThreadRef::new(
        BRAND,
        "3174",
        "LT Lime",
        &[155, 165, 40],
    ),
    ThreadRef::new(
        BRAND,
        "3176",
        "MD Lime",
        &[148, 158, 46],
    ),
    ThreadRef::new(
        BRAND,
        "3178",
        "PALE Grass Green",
        &[173, 186, 55],
    ),
    ThreadRef::new(
        BRAND,
        "3185",
        "MD Grass Green",
        &[100, 113, 5],
    ),
    ThreadRef::new(
        BRAND,
        "3186",
        "Dk Lime",
        &[115, 109, 0],
    ),
    ThreadRef::new(
        BRAND,
        "3194",
        "Blue Green",
        &[97, 130, 62],
    ),
    ThreadRef::new(
        BRAND,
        "3200",
        "MD Mint",
        &[146, 224, 130],
    ),
    ThreadRef::new(
        BRAND,
        "3201",
        "DK Mint",
        &[102, 203, 101],
    ),
    ThreadRef::new(
        BRAND,
        "3204",
        "MD Shamrock Green",
        &[40, 141, 35],
    ),
    ThreadRef::new(
        BRAND,
        "3206",
        "DK Shamrock Green",
        &[23, 118, 16],
    ),
    ThreadRef::new(
        BRAND,
        "3211",
        "MD Yellow Green",
        &[122, 161, 98],
    ),
    ThreadRef::new(
        BRAND,
        "3218",
        "Md Loden Green",
        &[113, 115, 0],
    ),
    ThreadRef::new(
        BRAND,
        "3225",
        "DK Loden Green",
        &[46, 52, 26],
    ),
    ThreadRef::new(
        BRAND,
        "3235",
        "LT Grass Green",
        &[128, 143, 58],
    ),
    ThreadRef::new(
        BRAND,
        "3238",
        "Dk Grass Green",
        &[91, 102, 42],
    ),
    ThreadRef::new(
        BRAND,
        "3250",
        "PLST Silver",
        &[218, 229, 225],
    ),
    ThreadRef::new(
        BRAND,
        "3254",
        "MD Sage",
        &[119, 138, 117],
    ),
    ThreadRef::new(
        BRAND,
        "3263",
        "DKST Blue Gray",
        &[55, 230, 195],
    ),
    ThreadRef::new(
        BRAND,
        "3295",
        "PLST Sea Green",
        &[199, 222, 213],
    ),
    ThreadRef::new(
        BRAND,
        "3299",
        "Blue Sea Green",
        &[143, 183, 195],
    ),
    ThreadRef::new(
        BRAND,
        "3321",
        "MD Sea Green",
        &[122, 151, 133],
    ),
    ThreadRef::new(
        BRAND,
        "3324",
        "DKST Blue Granite",
        &[15, 29, 32],
    ),
    ThreadRef::new(
        BRAND,
        "3340",
        "MD Slate",
        &[106, 119, 127],
    ),
    ThreadRef::new(
        BRAND,
        "3345",
        "DK Slate",
        &[77, 82, 88],
    ),
    ThreadRef::new(
        BRAND,
        "3368",
        "Silver Tint White",
        &[226, 226, 228],
    ),
    ThreadRef::new(
        BRAND,
        "3369",
        "Gray Pearl",
        &[221, 221, 223],
    ),
    ThreadRef::new(
        BRAND,
        "3370",
        "PALE Mauve Pearl",
        &[172, 165, 159],
    ),
    ThreadRef::new(
        BRAND,
        "3371",
        "PALE Cloud Gray",
        &[214, 214, 224],
    ),
    ThreadRef::new(
        BRAND,
        "3376",
        "LT Gray",
        &[129, 131, 128],
    ),
    ThreadRef::new(
        BRAND,
        "3379",
        "Blue Gray Granite",
        &[95, 102, 118],
    ),
    ThreadRef::new(
        BRAND,
        "3380",
        "Steel Gray",
        &[67, 82, 94],
    ),
    ThreadRef::new(
        BRAND,
        "3393",
        "LT Pigeon Gray",
        &[155, 146, 147],
    ),
    ThreadRef::new(
        BRAND,
        "3398",
        "DK Pigeon Gray",
        &[41, 36, 30],
    ),
    ThreadRef::new(
        BRAND,
        "3408",
        "PLST Gray",
        &[207, 206, 204],
    ),
    ThreadRef::new(
        BRAND,
        "3409",
        "Silver",
        &[209, 208, 217],
    ),
    ThreadRef::new(
        BRAND,
        "3412",
        "MD Pigeon Gray",
        &[118, 113, 109],
    ),
    ThreadRef::new(
        BRAND,
        "3413",
        "MD Gray",
        &[137, 137, 145],
    ),
    ThreadRef::new(
        BRAND,
        "3415",
        "DK Gray",
        &[97, 82, 90],
    ),
    ThreadRef::new(
        BRAND,
        "3416",
        "LT Pewter",
        &[82, 79, 98],
    ),
    ThreadRef::new(
        BRAND,
        "3417",
        "MD Pewter",
        &[99, 101, 122],
    ),
    ThreadRef::new(
        BRAND,
        "3418",
        "DK Pewter",
        &[84, 86, 101],
    ),
    ThreadRef::new(
        BRAND,
        "3421",
        "Purple Ash",
        &[110, 97, 125],
    ),
    ThreadRef::new(
        BRAND,
        "3427",
        "DKST Eggplant",
        &[15, 8, 29],
    ),
    ThreadRef::new(
        BRAND,
        "3429",
        "PLST Blue",
        &[233, 243, 254],
    ),
    ThreadRef::new(
        BRAND,
        "3431",
        "PALE Blue",
        &[179, 197, 221],
    ),
    ThreadRef::new(
        BRAND,
        "3437",
        "Light Denim",
        &[128, 159, 188],
    ),
    ThreadRef::new(
        BRAND,
        "3468",
        "Light Turquoise",
        &[70, 159, 175],
    ),
    ThreadRef::new(
        BRAND,
        "3471",
        "Turquoise",
        &[33, 113, 138],
    ),
    ThreadRef::new(
        BRAND,
        "3472",
        "LT Peacock",
        &[85, 177, 228],
    ),
    ThreadRef::new(
        BRAND,
        "3474",
        "Peacock",
        &[36, 125, 157],
    ),
    ThreadRef::new(
        BRAND,
        "3479",
        "Deep Lake",
        &[32, 102, 89],
    ),
    ThreadRef::new(
        BRAND,
        "3486",
        "Green Blue Storm",
        &[42, 77, 99],
    ),
    ThreadRef::new(
        BRAND,
        "3487",
        "DK Lagoon",
        &[51, 86, 121],
    ),
    ThreadRef::new(
        BRAND,
        "3507",
        "DKST Lagoon",
        &[58, 92, 75],
    ),
    ThreadRef::new(
        BRAND,
        "3511",
        "PALE Teal",
        &[155, 228, 198],
    ),
    ThreadRef::new(
        BRAND,
        "3513",
        "PALE Teal Blue",
        &[140, 229, 201],
    ),
    ThreadRef::new(
        BRAND,
        "3514",
        "LT Teal",
        &[140, 234, 220],
    ),
    ThreadRef::new(
        BRAND,
        "3520",
        "MD Teal",
        &[112, 209, 178],
    ),
    ThreadRef::new(
        BRAND,
        "3531",
        "PALE Aqua",
        &[218, 255, 255],
    ),
    ThreadRef::new(
        BRAND,
        "3532",
        "LT Aqua",
        &[198, 255, 255],
    ),
    ThreadRef::new(
        BRAND,
        "3542",
        "DK Teal",
        &[42, 143, 130],
    ),
    ThreadRef::new(
        BRAND,
        "3545",
        "DKST Teal",
        &[29, 99, 90],
    ),
    ThreadRef::new(
        BRAND,
        "3550",
        "Aqua Tint White",
        &[218, 243, 237],
    ),
    ThreadRef::new(
        BRAND,
        "3559",
        "MD Turqoise",
        &[67, 177, 192],
    ),
    ThreadRef::new(
        BRAND,
        "3560",
        "LT Mediterranean Blue",
        &[76, 147, 158],
    ),
    ThreadRef::new(
        BRAND,
        "3563",
        "Dk Turquoise",
        &[28, 149, 132],
    ),
    ThreadRef::new(
        BRAND,
        "3587",
        "Navy",
        &[24, 30, 64],
    ),
    ThreadRef::new(
        BRAND,
        "3597",
        "Light Blue Aster",
        &[98, 142, 204],
    ),
    ThreadRef::new(
        BRAND,
        "3600",
        "MD Blue Aster",
        &[20, 102, 164],
    ),
    ThreadRef::new(
        BRAND,
        "3601",
        "DK Bright Aster",
        &[53, 87, 161],
    ),
    ThreadRef::new(
        BRAND,
        "3604",
        "MD Mediterranean Blue",
        &[92, 128, 154],
    ),
    ThreadRef::new(
        BRAND,
        "3606",
        "DK Mediterranean Blue",
        &[45, 87, 120],
    ),
    ThreadRef::new(
        BRAND,
        "3620",
        "French Blue",
        &[37, 130, 196],
    ),
    ThreadRef::new(
        BRAND,
        "3621",
        "DK Marine Blue",
        &[29, 53, 115],
    ),
    ThreadRef::new(
        BRAND,
        "3636",
        "LT Clear Blue",
        &[174, 212, 255],
    ),
    ThreadRef::new(
        BRAND,
        "3639",
        "Dusk Blue",
        &[151, 179, 200],
    ),
    ThreadRef::new(
        BRAND,
        "3645",
        "Marine Blue",
        &[13, 46, 100],
    ),
    ThreadRef::new(
        BRAND,
        "3646",
        "DKST Navy",
        &[38, 42, 69],
    ),
    ThreadRef::new(
        BRAND,
        "3649",
        "LT Bright Blue",
        &[165, 201, 253],
    ),
    ThreadRef::new(
        BRAND,
        "3655",
        "MD Bright Blue",
        &[55, 119, 197],
    ),
    ThreadRef::new(
        BRAND,
        "3660",
        "DK Bright Blue",
        &[3, 33, 168],
    ),
    ThreadRef::new(
        BRAND,
        "3668",
        "PALE Clear Blue",
        &[217, 235, 255],
    ),
    ThreadRef::new(
        BRAND,
        "3670",
        "LT  Blue",
        &[177, 214, 252],
    ),
    ThreadRef::new(
        BRAND,
        "3674",
        "PALE Aster Blue",
        &[144, 175, 232],
    ),
    ThreadRef::new(
        BRAND,
        "3675",
        "MD Clear Blue",
        &[94, 154, 208],
    ),
    ThreadRef::new(
        BRAND,
        "3679",
        "DK Clear Blue",
        &[84, 122, 193],
    ),
    ThreadRef::new(
        BRAND,
        "3688",
        "Violet Tint White",
        &[222, 229, 245],
    ),
    ThreadRef::new(
        BRAND,
        "3690",
        "Light Blue Violet",
        &[185, 202, 246],
    ),
    ThreadRef::new(
        BRAND,
        "3692",
        "MD Blue Violet",
        &[110, 139, 209],
    ),
    ThreadRef::new(
        BRAND,
        "3694",
        "DK Blue Violet",
        &[99, 109, 180],
    ),
    ThreadRef::new(
        BRAND,
        "3704",
        "DK Heliotrope",
        &[32, 42, 89],
    ),
    ThreadRef::new(
        BRAND,
        "3713",
        "PALE Eggplant",
        &[217, 200, 205],
    ),
    ThreadRef::new(
        BRAND,
        "3714",
        "MD Eggplant",
        &[147, 129, 145],
    ),
    ThreadRef::new(
        BRAND,
        "3721",
        "LT Purple Sage",
        &[135, 122, 152],
    ),
    ThreadRef::new(
        BRAND,
        "3722",
        "MD Purple Sage",
        &[102, 88, 121],
    ),
    ThreadRef::new(
        BRAND,
        "3727",
        "DK Purple Sage",
        &[87, 77, 85],
    ),
    ThreadRef::new(
        BRAND,
        "3728",
        "Lilac Tint White",
        &[234, 236, 233],
    ),
    ThreadRef::new(
        BRAND,
        "3730",
        "PALE Heather",
        &[223, 226, 230],
    ),
    ThreadRef::new(
        BRAND,
        "3731",
        "LT Lilac",
        &[219, 216, 255],
    ),
    ThreadRef::new(
        BRAND,
        "3733",
        "MD Lilac",
        &[201, 187, 240],
    ),
    ThreadRef::new(
        BRAND,
        "3735",
        "Heather",
        &[180, 180, 214],
    ),
    ThreadRef::new(
        BRAND,
        "3740",
        "MD Heather",
        &[139, 122, 202],
    ),
    ThreadRef::new(
        BRAND,
        "3749",
        "PLST Periwinkle",
        &[224, 236, 250],
    ),
    ThreadRef::new(
        BRAND,
        "3751",
        "LT Periwinkle",
        &[136, 146, 207],
    ),
    ThreadRef::new(
        BRAND,
        "3752",
        "DK Periwinkle",
        &[117, 113, 172],
    ),
    ThreadRef::new(
        BRAND,
        "3753",
        "MD Periwinkle",
        &[128, 132, 201],
    ),
    ThreadRef::new(
        BRAND,
        "3756",
        "DKST Periwinkle",
        &[48, 55, 125],
    ),
    ThreadRef::new(
        BRAND,
        "3758",
        "DK Heather",
        &[76, 80, 143],
    ),
    ThreadRef::new(
        BRAND,
        "3764",
        "DK Blue Purple",
        &[50, 37, 109],
    ),
    ThreadRef::new(
        BRAND,
        "3770",
        "PALE Lavender",
        &[214, 186, 205],
    ),
    ThreadRef::new(
        BRAND,
        "3772",
        "LT Lavender",
        &[208, 198, 227],
    ),
    ThreadRef::new(
        BRAND,
        "3777",
        "MD Lavender",
        &[132, 95, 164],
    ),
    ThreadRef::new(
        BRAND,
        "3778",
        "Blue Purple",
        &[68, 40, 117],
    ),
    ThreadRef::new(
        BRAND,
        "3783",
        "Red Purple",
        &[63, 14, 93],
    ),
    ThreadRef::new(
        BRAND,
        "3793",
        "LT African Violet",
        &[160, 129, 163],
    ),
    ThreadRef::new(
        BRAND,
        "3794",
        "African Violet",
        &[147, 117, 148],
    ),
    ThreadRef::new(
        BRAND,
        "3796",
        "MD African Violet",
        &[185, 106, 197],
    ),
    ThreadRef::new(
        BRAND,
        "3803",
        "DK African Violet",
        &[116, 56, 153],
    ),
    ThreadRef::new(
        BRAND,
        "3805",
        "DKST African Violet",
        &[86, 33, 120],
    ),
    ThreadRef::new(
        BRAND,
        "3812",
        "PALE Orchid",
        &[255, 195, 243],
    ),
    ThreadRef::new(
        BRAND,
        "3821",
        "MD Orchid",
        &[208, 107, 170],
    ),
    ThreadRef::new(
        BRAND,
        "3822",
        "DK Orchid",
        &[167, 73, 161],
    ),
    ThreadRef::new(
        BRAND,
        "3828",
        "PLST Orchid",
        &[239, 229, 255],
    ),
    ThreadRef::new(
        BRAND,
        "3830",
        "PLST Mauve",
        &[224, 218, 232],
    ),
    ThreadRef::new(
        BRAND,
        "3833",
        "LT Orchid",
        &[210, 160, 189],
    ),
    ThreadRef::new(
        BRAND,
        "3834",
        "LT Pink Orchid",
        &[247, 178, 222],
    ),
    ThreadRef::new(
        BRAND,
        "3841",
        "Purple",
        &[110, 59, 130],
    ),
    ThreadRef::new(
        BRAND,
        "3846",
        "DKST wine",
        &[74, 6, 74],
    ),
];