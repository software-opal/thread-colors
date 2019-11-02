#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Isafil Rayon - older";
pub const THREADS: [ThreadRef; 288] = [
    ThreadRef::new(
        BRAND,
        "0270",
        "Ivory White",
        &[254, 253, 224],
    ),
    ThreadRef::new(
        BRAND,
        "0505",
        "Cream",
        &[254, 253, 202],
    ),
    ThreadRef::new(
        BRAND,
        "0580",
        "Pale Yellow",
        &[254, 251, 134],
    ),
    ThreadRef::new(
        BRAND,
        "0590",
        "Yellow",
        &[254, 249, 93],
    ),
    ThreadRef::new(
        BRAND,
        "0600",
        "Yellow",
        &[254, 243, 78],
    ),
    ThreadRef::new(
        BRAND,
        "0605",
        "Bright Yellow",
        &[253, 234, 63],
    ),
    ThreadRef::new(
        BRAND,
        "0610",
        "Bright Yellow",
        &[252, 220, 65],
    ),
    ThreadRef::new(
        BRAND,
        "0713",
        "Daffodil",
        &[252, 222, 109],
    ),
    ThreadRef::new(
        BRAND,
        "0260",
        "Pale Lemon",
        &[250, 245, 193],
    ),
    ThreadRef::new(
        BRAND,
        "0510",
        "Cream",
        &[253, 251, 210],
    ),
    ThreadRef::new(
        BRAND,
        "0520",
        "Pale Yellow",
        &[251, 245, 171],
    ),
    ThreadRef::new(
        BRAND,
        "0620",
        "Sunflower",
        &[248, 235, 109],
    ),
    ThreadRef::new(
        BRAND,
        "0615",
        "Manila",
        &[239, 216, 75],
    ),
    ThreadRef::new(
        BRAND,
        "0625",
        "Moonbeam",
        &[242, 220, 93],
    ),
    ThreadRef::new(
        BRAND,
        "0221",
        "Moonbeam",
        &[225, 217, 95],
    ),
    ThreadRef::new(
        BRAND,
        "0542",
        "Old Gold",
        &[185, 161, 62],
    ),
    ThreadRef::new(
        BRAND,
        "0545",
        "Pistachio",
        &[165, 143, 62],
    ),
    ThreadRef::new(
        BRAND,
        "0726",
        "Brown",
        &[132, 106, 52],
    ),
    ThreadRef::new(
        BRAND,
        "0660",
        "Ivory White",
        &[254, 249, 219],
    ),
    ThreadRef::new(
        BRAND,
        "0640",
        "Visor Gold",
        &[250, 226, 147],
    ),
    ThreadRef::new(
        BRAND,
        "0630",
        "Orangy Cream",
        &[254, 240, 132],
    ),
    ThreadRef::new(
        BRAND,
        "0506",
        "Marigold",
        &[251, 211, 88],
    ),
    ThreadRef::new(
        BRAND,
        "0702",
        "Goldenrod",
        &[248, 194, 71],
    ),
    ThreadRef::new(
        BRAND,
        "0704",
        "Mustard",
        &[230, 181, 70],
    ),
    ThreadRef::new(
        BRAND,
        "0805",
        "Manila",
        &[253, 195, 81],
    ),
    ThreadRef::new(
        BRAND,
        "0800",
        "Gold Orange",
        &[247, 179, 56],
    ),
    ThreadRef::new(
        BRAND,
        "0811",
        "Gold",
        &[237, 178, 89],
    ),
    ThreadRef::new(
        BRAND,
        "0821",
        "Sun Gold",
        &[213, 159, 75],
    ),
    ThreadRef::new(
        BRAND,
        "0635",
        "Pale Yellow",
        &[254, 238, 178],
    ),
    ThreadRef::new(
        BRAND,
        "0830",
        "Cornsilk",
        &[250, 198, 106],
    ),
    ThreadRef::new(
        BRAND,
        "0900",
        "Orange",
        &[250, 157, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1100",
        "Gold",
        &[235, 140, 58],
    ),
    ThreadRef::new(
        BRAND,
        "0940",
        "Brown",
        &[174, 117, 44],
    ),
    ThreadRef::new(
        BRAND,
        "1101",
        "Orange",
        &[252, 150, 57],
    ),
    ThreadRef::new(
        BRAND,
        "1099",
        "Rust",
        &[245, 127, 52],
    ),
    ThreadRef::new(
        BRAND,
        "1300",
        "Rust",
        &[247, 115, 54],
    ),
    ThreadRef::new(
        BRAND,
        "1060",
        "Cottage Beige",
        &[253, 232, 199],
    ),
    ThreadRef::new(
        BRAND,
        "1362",
        "Salmon",
        &[251, 205, 160],
    ),
    ThreadRef::new(
        BRAND,
        "1113",
        "Gold",
        &[228, 127, 73],
    ),
    ThreadRef::new(
        BRAND,
        "1211",
        "Wicker",
        &[215, 137, 95],
    ),
    ThreadRef::new(
        BRAND,
        "1320",
        "Golden Poppy",
        &[250, 131, 80],
    ),
    ThreadRef::new(
        BRAND,
        "1304",
        "Rust",
        &[248, 110, 54],
    ),
    ThreadRef::new(
        BRAND,
        "1305",
        "Tangerine",
        &[247, 101, 50],
    ),
    ThreadRef::new(
        BRAND,
        "1302",
        "Rust",
        &[241, 108, 54],
    ),
    ThreadRef::new(
        BRAND,
        "2180",
        "Flesh",
        &[253, 232, 215],
    ),
    ThreadRef::new(
        BRAND,
        "1351",
        "Light Salmon",
        &[253, 189, 157],
    ),
    ThreadRef::new(
        BRAND,
        "1430",
        "Salmon",
        &[234, 138, 107],
    ),
    ThreadRef::new(
        BRAND,
        "1421",
        "Bronze",
        &[189, 97, 70],
    ),
    ThreadRef::new(
        BRAND,
        "1905",
        "Red Brown",
        &[160, 73, 45],
    ),
    ThreadRef::new(
        BRAND,
        "1352",
        "Salmon Pink",
        &[251, 164, 123],
    ),
    ThreadRef::new(
        BRAND,
        "1313",
        "Golden Brown",
        &[208, 110, 71],
    ),
    ThreadRef::new(
        BRAND,
        "1311",
        "Bronze",
        &[176, 96, 59],
    ),
    ThreadRef::new(
        BRAND,
        "1324",
        "Brown",
        &[131, 74, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1348",
        "Mushroom",
        &[114, 70, 54],
    ),
    ThreadRef::new(
        BRAND,
        "2165",
        "Snow White",
        &[254, 248, 247],
    ),
    ThreadRef::new(
        BRAND,
        "1870",
        "Bisque",
        &[253, 224, 209],
    ),
    ThreadRef::new(
        BRAND,
        "1630",
        "Illusion",
        &[253, 194, 177],
    ),
    ThreadRef::new(
        BRAND,
        "1850",
        "Melon",
        &[248, 162, 145],
    ),
    ThreadRef::new(
        BRAND,
        "1849",
        "Melonade",
        &[251, 155, 132],
    ),
    ThreadRef::new(
        BRAND,
        "1600",
        "Honeysuckle",
        &[232, 115, 87],
    ),
    ThreadRef::new(
        BRAND,
        "1701",
        "Coral",
        &[242, 102, 67],
    ),
    ThreadRef::new(
        BRAND,
        "1705",
        "Paprika",
        &[239, 89, 51],
    ),
    ThreadRef::new(
        BRAND,
        "1703",
        "Tuxedo Red",
        &[225, 79, 48],
    ),
    ThreadRef::new(
        BRAND,
        "1800",
        "Paprika",
        &[207, 74, 48],
    ),
    ThreadRef::new(
        BRAND,
        "1902",
        "Red Brown",
        &[177, 65, 40],
    ),
    ThreadRef::new(
        BRAND,
        "1802",
        "Tangerine",
        &[226, 86, 70],
    ),
    ThreadRef::new(
        BRAND,
        "1805",
        "Marigold",
        &[218, 85, 69],
    ),
    ThreadRef::new(
        BRAND,
        "1900",
        "Saffron",
        &[208, 78, 68],
    ),
    ThreadRef::new(
        BRAND,
        "1913",
        "Terra Cotta",
        &[152, 64, 45],
    ),
    ThreadRef::new(
        BRAND,
        "2100",
        "Red Brown",
        &[152, 62, 40],
    ),
    ThreadRef::new(
        BRAND,
        "1914",
        "Carolina Red",
        &[133, 61, 42],
    ),
    ThreadRef::new(
        BRAND,
        "2123",
        "Sepia",
        &[113, 55, 41],
    ),
    ThreadRef::new(
        BRAND,
        "2170",
        "Flesh",
        &[253, 232, 222],
    ),
    ThreadRef::new(
        BRAND,
        "2168",
        "Ivory",
        &[253, 238, 234],
    ),
    ThreadRef::new(
        BRAND,
        "1860",
        "Bisque",
        &[251, 216, 212],
    ),
    ThreadRef::new(
        BRAND,
        "1855",
        "Desert Bloom",
        &[253, 217, 213],
    ),
    ThreadRef::new(
        BRAND,
        "1840",
        "Shrimp",
        &[251, 161, 159],
    ),
    ThreadRef::new(
        BRAND,
        "1750",
        "Dusty Rose",
        &[236, 123, 119],
    ),
    ThreadRef::new(
        BRAND,
        "1921",
        "Bronze",
        &[185, 90, 86],
    ),
    ThreadRef::new(
        BRAND,
        "1912",
        "Burgundy",
        &[144, 63, 50],
    ),
    ThreadRef::new(
        BRAND,
        "1761",
        "Beige",
        &[221, 183, 172],
    ),
    ThreadRef::new(
        BRAND,
        "2053",
        "Brownish Pink",
        &[200, 148, 128],
    ),
    ThreadRef::new(
        BRAND,
        "1842",
        "Cocoa Mulch",
        &[163, 111, 96],
    ),
    ThreadRef::new(
        BRAND,
        "2051",
        "Grape",
        &[205, 165, 163],
    ),
    ThreadRef::new(
        BRAND,
        "2360",
        "Light Pink",
        &[253, 238, 243],
    ),
    ThreadRef::new(
        BRAND,
        "1960",
        "Pale Pink",
        &[253, 223, 231],
    ),
    ThreadRef::new(
        BRAND,
        "2152",
        "Rose Pink",
        &[236, 158, 165],
    ),
    ThreadRef::new(
        BRAND,
        "2241",
        "Ducky Mauve",
        &[165, 102, 107],
    ),
    ThreadRef::new(
        BRAND,
        "2332",
        "Brown",
        &[132, 74, 80],
    ),
    ThreadRef::new(
        BRAND,
        "2333",
        "Russet",
        &[105, 54, 52],
    ),
    ThreadRef::new(
        BRAND,
        "2250",
        "Delicate Mauve",
        &[253, 209, 220],
    ),
    ThreadRef::new(
        BRAND,
        "2230",
        "Shrimp",
        &[248, 154, 171],
    ),
    ThreadRef::new(
        BRAND,
        "2220",
        "Bashful Pink",
        &[247, 128, 152],
    ),
    ThreadRef::new(
        BRAND,
        "2300",
        "Strawberry",
        &[201, 71, 89],
    ),
    ThreadRef::new(
        BRAND,
        "2555",
        "Pale Orchid",
        &[252, 188, 219],
    ),
    ThreadRef::new(
        BRAND,
        "2520",
        "Light Cerise",
        &[235, 111, 141],
    ),
    ThreadRef::new(
        BRAND,
        "2310",
        "Dark Pink",
        &[202, 74, 105],
    ),
    ThreadRef::new(
        BRAND,
        "2550",
        "Rose",
        &[249, 174, 209],
    ),
    ThreadRef::new(
        BRAND,
        "2524",
        "Rose Pink",
        &[245, 131, 166],
    ),
    ThreadRef::new(
        BRAND,
        "2513",
        "Passion Pink",
        &[213, 101, 139],
    ),
    ThreadRef::new(
        BRAND,
        "2505",
        "Hot Peony",
        &[179, 78, 110],
    ),
    ThreadRef::new(
        BRAND,
        "2500",
        "Wine Red",
        &[143, 70, 88],
    ),
    ThreadRef::new(
        BRAND,
        "2502",
        "Sepia",
        &[112, 53, 62],
    ),
    ThreadRef::new(
        BRAND,
        "2650",
        "Palest Pink",
        &[239, 200, 225],
    ),
    ThreadRef::new(
        BRAND,
        "2640",
        "Dusky Mauve",
        &[205, 148, 191],
    ),
    ThreadRef::new(
        BRAND,
        "2620",
        "Dark Rose",
        &[188, 125, 160],
    ),
    ThreadRef::new(
        BRAND,
        "2600",
        "Plum",
        &[129, 79, 110],
    ),
    ThreadRef::new(
        BRAND,
        "2611",
        "Brushed Burgundy",
        &[109, 64, 81],
    ),
    ThreadRef::new(
        BRAND,
        "2645",
        "Memphis Belle",
        &[242, 211, 234],
    ),
    ThreadRef::new(
        BRAND,
        "3045",
        "Purple",
        &[202, 166, 209],
    ),
    ThreadRef::new(
        BRAND,
        "2830",
        "Cachet",
        &[157, 125, 165],
    ),
    ThreadRef::new(
        BRAND,
        "2810",
        "Raspberry",
        &[125, 79, 124],
    ),
    ThreadRef::new(
        BRAND,
        "2715",
        "Dark Grey",
        &[99, 54, 85],
    ),
    ThreadRef::new(
        BRAND,
        "3040",
        "Palest Mauve",
        &[204, 181, 216],
    ),
    ThreadRef::new(
        BRAND,
        "2820",
        "Palm Leaf",
        &[173, 137, 186],
    ),
    ThreadRef::new(
        BRAND,
        "2712",
        "Iris",
        &[133, 95, 152],
    ),
    ThreadRef::new(
        BRAND,
        "3213",
        "Mauve",
        &[139, 112, 172],
    ),
    ThreadRef::new(
        BRAND,
        "3103",
        "Imperial Purple",
        &[89, 69, 128],
    ),
    ThreadRef::new(
        BRAND,
        "3262",
        "Banner Gray",
        &[173, 164, 179],
    ),
    ThreadRef::new(
        BRAND,
        "3241",
        "Palm Leaf",
        &[155, 145, 173],
    ),
    ThreadRef::new(
        BRAND,
        "3340",
        "Paris Blue",
        &[192, 191, 230],
    ),
    ThreadRef::new(
        BRAND,
        "3230",
        "Mauve",
        &[143, 135, 187],
    ),
    ThreadRef::new(
        BRAND,
        "3200",
        "Steel Blue",
        &[105, 99, 165],
    ),
    ThreadRef::new(
        BRAND,
        "3540",
        "Empire Blue",
        &[77, 68, 130],
    ),
    ThreadRef::new(
        BRAND,
        "3110",
        "Cassis",
        &[57, 51, 97],
    ),
    ThreadRef::new(
        BRAND,
        "2743",
        "PN Navy (24)",
        &[51, 46, 74],
    ),
    ThreadRef::new(
        BRAND,
        "3661",
        "White",
        &[235, 238, 248],
    ),
    ThreadRef::new(
        BRAND,
        "3845",
        "Ice Blue",
        &[213, 220, 244],
    ),
    ThreadRef::new(
        BRAND,
        "3430",
        "Tropic Blue",
        &[136, 147, 192],
    ),
    ThreadRef::new(
        BRAND,
        "3321",
        "Steel Blue",
        &[114, 127, 185],
    ),
    ThreadRef::new(
        BRAND,
        "3420",
        "Grey",
        &[159, 166, 213],
    ),
    ThreadRef::new(
        BRAND,
        "3301",
        "Steel Blue",
        &[96, 97, 153],
    ),
    ThreadRef::new(
        BRAND,
        "3330",
        "Dark Blue",
        &[68, 68, 126],
    ),
    ThreadRef::new(
        BRAND,
        "3102",
        "Purple Twist",
        &[52, 53, 101],
    ),
    ThreadRef::new(
        BRAND,
        "3211",
        "Storm Blue",
        &[97, 93, 138],
    ),
    ThreadRef::new(
        BRAND,
        "3222",
        "Palm Leaf",
        &[67, 65, 101],
    ),
    ThreadRef::new(
        BRAND,
        "3133",
        "Blue Ink",
        &[58, 55, 75],
    ),
    ThreadRef::new(
        BRAND,
        "3761",
        "Light Blue",
        &[181, 197, 228],
    ),
    ThreadRef::new(
        BRAND,
        "3631",
        "Copen",
        &[110, 127, 180],
    ),
    ThreadRef::new(
        BRAND,
        "3611",
        "Slate Blue",
        &[70, 88, 159],
    ),
    ThreadRef::new(
        BRAND,
        "3543",
        "Blue",
        &[70, 81, 149],
    ),
    ThreadRef::new(
        BRAND,
        "3322",
        "Fire Blue",
        &[60, 65, 114],
    ),
    ThreadRef::new(
        BRAND,
        "3353",
        "Blue Ink",
        &[52, 54, 86],
    ),
    ThreadRef::new(
        BRAND,
        "3355",
        "Light Navy",
        &[47, 46, 59],
    ),
    ThreadRef::new(
        BRAND,
        "4071",
        "Chrome",
        &[233, 237, 242],
    ),
    ThreadRef::new(
        BRAND,
        "3752",
        "Pale Blue",
        &[227, 235, 245],
    ),
    ThreadRef::new(
        BRAND,
        "3840",
        "Bridgeport Blue",
        &[181, 203, 234],
    ),
    ThreadRef::new(
        BRAND,
        "3820",
        "Powder Blue",
        &[145, 171, 225],
    ),
    ThreadRef::new(
        BRAND,
        "3810",
        "Copen",
        &[104, 127, 175],
    ),
    ThreadRef::new(
        BRAND,
        "3600",
        "Jay Blue",
        &[76, 96, 164],
    ),
    ThreadRef::new(
        BRAND,
        "3961",
        "Sky Blue",
        &[192, 222, 243],
    ),
    ThreadRef::new(
        BRAND,
        "3910",
        "Indian Ocean Blue",
        &[129, 184, 233],
    ),
    ThreadRef::new(
        BRAND,
        "3901",
        "California Blue",
        &[73, 127, 192],
    ),
    ThreadRef::new(
        BRAND,
        "4030",
        "Aqua",
        &[146, 186, 223],
    ),
    ThreadRef::new(
        BRAND,
        "4105",
        "Medium Blue",
        &[77, 143, 195],
    ),
    ThreadRef::new(
        BRAND,
        "4032",
        "Mallard Blue",
        &[89, 114, 139],
    ),
    ThreadRef::new(
        BRAND,
        "4133",
        "Dark Teal",
        &[54, 69, 87],
    ),
    ThreadRef::new(
        BRAND,
        "3851",
        "Pale Sky",
        &[165, 185, 208],
    ),
    ThreadRef::new(
        BRAND,
        "3952",
        "Slate Blue",
        &[105, 120, 143],
    ),
    ThreadRef::new(
        BRAND,
        "3743",
        "Steel Blue",
        &[68, 83, 104],
    ),
    ThreadRef::new(
        BRAND,
        "3443",
        "Deep Windsor",
        &[58, 63, 77],
    ),
    ThreadRef::new(
        BRAND,
        "3344",
        "Smokey",
        &[56, 55, 58],
    ),
    ThreadRef::new(
        BRAND,
        "4040",
        "Very Pale Blue",
        &[205, 247, 250],
    ),
    ThreadRef::new(
        BRAND,
        "4121",
        "Blue Frost",
        &[148, 219, 240],
    ),
    ThreadRef::new(
        BRAND,
        "4111",
        "Umber",
        &[89, 180, 221],
    ),
    ThreadRef::new(
        BRAND,
        "4100",
        "Medium Blue",
        &[65, 148, 190],
    ),
    ThreadRef::new(
        BRAND,
        "3822",
        "Teal Blue",
        &[52, 103, 142],
    ),
    ThreadRef::new(
        BRAND,
        "4232",
        "Dark Teal Blue",
        &[46, 81, 109],
    ),
    ThreadRef::new(
        BRAND,
        "4022",
        "Steel Blue",
        &[69, 87, 105],
    ),
    ThreadRef::new(
        BRAND,
        "4440",
        "Mint Julep",
        &[171, 227, 235],
    ),
    ThreadRef::new(
        BRAND,
        "4231",
        "Sky",
        &[114, 184, 207],
    ),
    ThreadRef::new(
        BRAND,
        "4500",
        "Dark Blue",
        &[62, 101, 122],
    ),
    ThreadRef::new(
        BRAND,
        "4432",
        "Garden Green",
        &[54, 80, 91],
    ),
    ThreadRef::new(
        BRAND,
        "4430",
        "Green Pearl",
        &[151, 203, 216],
    ),
    ThreadRef::new(
        BRAND,
        "4420",
        "Sky",
        &[111, 196, 208],
    ),
    ThreadRef::new(
        BRAND,
        "4421",
        "Oceanic Green",
        &[72, 126, 147],
    ),
    ThreadRef::new(
        BRAND,
        "4400",
        "Dark Sea Green",
        &[57, 112, 129],
    ),
    ThreadRef::new(
        BRAND,
        "4643",
        "Jade",
        &[78, 102, 105],
    ),
    ThreadRef::new(
        BRAND,
        "4644",
        "Green Forest",
        &[63, 85, 90],
    ),
    ThreadRef::new(
        BRAND,
        "4516",
        "Blue Spruce",
        &[51, 69, 72],
    ),
    ThreadRef::new(
        BRAND,
        "5040",
        "White",
        &[237, 245, 240],
    ),
    ThreadRef::new(
        BRAND,
        "4250",
        "Pale Green",
        &[208, 230, 228],
    ),
    ThreadRef::new(
        BRAND,
        "4840",
        "Aqua",
        &[161, 222, 213],
    ),
    ThreadRef::new(
        BRAND,
        "4620",
        "Country Blue",
        &[115, 175, 179],
    ),
    ThreadRef::new(
        BRAND,
        "4610",
        "Sea Green",
        &[92, 145, 146],
    ),
    ThreadRef::new(
        BRAND,
        "4513",
        "Dark Sea Green",
        &[73, 128, 133],
    ),
    ThreadRef::new(
        BRAND,
        "5050",
        "Light Blue",
        &[197, 228, 223],
    ),
    ThreadRef::new(
        BRAND,
        "5020",
        "Aqua",
        &[124, 191, 183],
    ),
    ThreadRef::new(
        BRAND,
        "5111",
        "Bluestone",
        &[86, 166, 151],
    ),
    ThreadRef::new(
        BRAND,
        "5000",
        "Lagoon",
        &[76, 144, 132],
    ),
    ThreadRef::new(
        BRAND,
        "5240",
        "Aqua",
        &[163, 226, 200],
    ),
    ThreadRef::new(
        BRAND,
        "5230",
        "Seafoam",
        &[147, 203, 177],
    ),
    ThreadRef::new(
        BRAND,
        "5210",
        "Emerald Green",
        &[81, 161, 121],
    ),
    ThreadRef::new(
        BRAND,
        "5431",
        "Sage",
        &[70, 148, 111],
    ),
    ThreadRef::new(
        BRAND,
        "5460",
        "Opal Green",
        &[209, 231, 215],
    ),
    ThreadRef::new(
        BRAND,
        "5542",
        "Silvery Gray",
        &[127, 149, 127],
    ),
    ThreadRef::new(
        BRAND,
        "5255",
        "Evergreen",
        &[61, 85, 77],
    ),
    ThreadRef::new(
        BRAND,
        "5375",
        "Dark Army Green",
        &[44, 54, 43],
    ),
    ThreadRef::new(
        BRAND,
        "5470",
        "Green Oak",
        &[177, 236, 180],
    ),
    ThreadRef::new(
        BRAND,
        "5620",
        "Blue Green",
        &[110, 192, 137],
    ),
    ThreadRef::new(
        BRAND,
        "5515",
        "Green Grass",
        &[53, 126, 77],
    ),
    ThreadRef::new(
        BRAND,
        "5422",
        "Kelly",
        &[77, 121, 87],
    ),
    ThreadRef::new(
        BRAND,
        "5425",
        "Olive Green",
        &[40, 93, 53],
    ),
    ThreadRef::new(
        BRAND,
        "5326",
        "Green Petal",
        &[45, 74, 51],
    ),
    ThreadRef::new(
        BRAND,
        "5611",
        "Christmas Green",
        &[105, 179, 90],
    ),
    ThreadRef::new(
        BRAND,
        "5600",
        "Erin Green",
        &[129, 178, 90],
    ),
    ThreadRef::new(
        BRAND,
        "5650",
        "Aquamarine",
        &[203, 232, 177],
    ),
    ThreadRef::new(
        BRAND,
        "5840",
        "Green Dust",
        &[191, 218, 145],
    ),
    ThreadRef::new(
        BRAND,
        "5610",
        "Light Leaf Green",
        &[157, 196, 120],
    ),
    ThreadRef::new(
        BRAND,
        "5531",
        "Green Dust",
        &[140, 164, 108],
    ),
    ThreadRef::new(
        BRAND,
        "5633",
        "Dark Olive Green",
        &[77, 118, 53],
    ),
    ThreadRef::new(
        BRAND,
        "6051",
        "Green Dust",
        &[202, 217, 152],
    ),
    ThreadRef::new(
        BRAND,
        "5832",
        "Peapod",
        &[159, 190, 113],
    ),
    ThreadRef::new(
        BRAND,
        "5833",
        "Ming",
        &[123, 142, 67],
    ),
    ThreadRef::new(
        BRAND,
        "5933",
        "Dark Olive Green",
        &[83, 110, 46],
    ),
    ThreadRef::new(
        BRAND,
        "5934",
        "Dark Olive Green",
        &[79, 104, 41],
    ),
    ThreadRef::new(
        BRAND,
        "0970",
        "Celery",
        &[252, 250, 227],
    ),
    ThreadRef::new(
        BRAND,
        "0251",
        "Pebblestone",
        &[227, 219, 159],
    ),
    ThreadRef::new(
        BRAND,
        "0232",
        "Light Khaki Green",
        &[189, 189, 98],
    ),
    ThreadRef::new(
        BRAND,
        "0442",
        "Pistachio",
        &[159, 152, 68],
    ),
    ThreadRef::new(
        BRAND,
        "0345",
        "Dark Olive Green",
        &[117, 111, 46],
    ),
    ThreadRef::new(
        BRAND,
        "0241",
        "Pebblestone",
        &[227, 225, 147],
    ),
    ThreadRef::new(
        BRAND,
        "0352",
        "Burnished Brown",
        &[185, 186, 100],
    ),
    ThreadRef::new(
        BRAND,
        "6133",
        "Autumn Green",
        &[126, 123, 51],
    ),
    ThreadRef::new(
        BRAND,
        "0465",
        "Dark Olive Green",
        &[87, 81, 47],
    ),
    ThreadRef::new(
        BRAND,
        "6156",
        "Hedge",
        &[81, 76, 51],
    ),
    ThreadRef::new(
        BRAND,
        "6071",
        "Flite Green",
        &[210, 210, 172],
    ),
    ThreadRef::new(
        BRAND,
        "0453",
        "Khaki Green",
        &[155, 154, 112],
    ),
    ThreadRef::new(
        BRAND,
        "0454",
        "Brown",
        &[117, 115, 70],
    ),
    ThreadRef::new(
        BRAND,
        "6065",
        "Hedge",
        &[74, 77, 47],
    ),
    ThreadRef::new(
        BRAND,
        "5765",
        "Olive",
        &[58, 74, 46],
    ),
    ThreadRef::new(
        BRAND,
        "5944",
        "Olive",
        &[56, 76, 42],
    ),
    ThreadRef::new(
        BRAND,
        "5766",
        "Dark Gray",
        &[59, 69, 41],
    ),
    ThreadRef::new(
        BRAND,
        "5776",
        "Mahogany",
        &[43, 52, 35],
    ),
    ThreadRef::new(
        BRAND,
        "0761",
        "Palest Yellow",
        &[222, 208, 172],
    ),
    ThreadRef::new(
        BRAND,
        "0532",
        "Mocha Cream",
        &[219, 199, 139],
    ),
    ThreadRef::new(
        BRAND,
        "0822",
        "Old Gold",
        &[186, 153, 80],
    ),
    ThreadRef::new(
        BRAND,
        "0551",
        "Bullion",
        &[161, 132, 64],
    ),
    ThreadRef::new(
        BRAND,
        "0936",
        "Chocolate",
        &[121, 101, 52],
    ),
    ThreadRef::new(
        BRAND,
        "0747",
        "Dark Olive Green",
        &[98, 82, 44],
    ),
    ThreadRef::new(
        BRAND,
        "0851",
        "14 Kt. Gold",
        &[211, 183, 128],
    ),
    ThreadRef::new(
        BRAND,
        "0842",
        "Gold Brown",
        &[196, 157, 104],
    ),
    ThreadRef::new(
        BRAND,
        "0961",
        "Creamy Beige",
        &[245, 229, 192],
    ),
    ThreadRef::new(
        BRAND,
        "1140",
        "New Gold",
        &[242, 215, 169],
    ),
    ThreadRef::new(
        BRAND,
        "1361",
        "Tawny",
        &[243, 194, 152],
    ),
    ThreadRef::new(
        BRAND,
        "1041",
        "Gold Brown",
        &[204, 148, 95],
    ),
    ThreadRef::new(
        BRAND,
        "0932",
        "Brown",
        &[163, 108, 52],
    ),
    ThreadRef::new(
        BRAND,
        "0933",
        "Sienna",
        &[114, 82, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1342",
        "Chocolate",
        &[127, 84, 48],
    ),
    ThreadRef::new(
        BRAND,
        "1355",
        "Dark Brown",
        &[99, 67, 42],
    ),
    ThreadRef::new(
        BRAND,
        "1346",
        "Brown",
        &[91, 64, 44],
    ),
    ThreadRef::new(
        BRAND,
        "1376",
        "Best Brown",
        &[65, 53, 38],
    ),
    ThreadRef::new(
        BRAND,
        "0670",
        "Off White",
        &[253, 253, 242],
    ),
    ThreadRef::new(
        BRAND,
        "0870",
        "Ivory",
        &[249, 244, 226],
    ),
    ThreadRef::new(
        BRAND,
        "1172",
        "Beige",
        &[216, 195, 162],
    ),
    ThreadRef::new(
        BRAND,
        "0853",
        "Medium Brown",
        &[158, 132, 93],
    ),
    ThreadRef::new(
        BRAND,
        "0945",
        "Dark Taupe",
        &[107, 87, 59],
    ),
    ThreadRef::new(
        BRAND,
        "0866",
        "Espresso",
        &[79, 69, 48],
    ),
    ThreadRef::new(
        BRAND,
        "1061",
        "Beige",
        &[176, 152, 128],
    ),
    ThreadRef::new(
        BRAND,
        "1055",
        "Light Cocoa",
        &[112, 88, 63],
    ),
    ThreadRef::new(
        BRAND,
        "1876",
        "Brown",
        &[81, 66, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1366",
        "Egyptian Brown",
        &[66, 56, 39],
    ),
    ThreadRef::new(
        BRAND,
        "1276",
        "Dark Army Green",
        &[57, 53, 41],
    ),
    ThreadRef::new(
        BRAND,
        "0672",
        "Stainless Steel",
        &[210, 200, 177],
    ),
    ThreadRef::new(
        BRAND,
        "0873",
        "Cloud",
        &[169, 161, 141],
    ),
    ThreadRef::new(
        BRAND,
        "0732",
        "Pewter",
        &[122, 116, 93],
    ),
    ThreadRef::new(
        BRAND,
        "1573",
        "Dark Brown",
        &[91, 80, 58],
    ),
    ThreadRef::new(
        BRAND,
        "0109",
        "Banner Gray",
        &[167, 166, 169],
    ),
    ThreadRef::new(
        BRAND,
        "0112",
        "Metal",
        &[116, 117, 116],
    ),
    ThreadRef::new(
        BRAND,
        "2578",
        "Twilight",
        &[99, 95, 95],
    ),
    ThreadRef::new(
        BRAND,
        "0015",
        "Snow White",
        &[253, 252, 252],
    ),
    ThreadRef::new(
        BRAND,
        "0010",
        "White",
        &[254, 253, 253],
    ),
    ThreadRef::new(
        BRAND,
        "0150",
        "Stainless Steel",
        &[209, 203, 199],
    ),
    ThreadRef::new(
        BRAND,
        "3971",
        "Silver Steel",
        &[195, 197, 200],
    ),
    ThreadRef::new(
        BRAND,
        "0124",
        "Palm Leaf",
        &[203, 201, 192],
    ),
    ThreadRef::new(
        BRAND,
        "0142",
        "Grey",
        &[180, 186, 187],
    ),
    ThreadRef::new(
        BRAND,
        "0104",
        "Banner Gray",
        &[171, 171, 176],
    ),
    ThreadRef::new(
        BRAND,
        "0145",
        "Paris Blue",
        &[207, 208, 216],
    ),
    ThreadRef::new(
        BRAND,
        "0156",
        "Medium Grey",
        &[160, 157, 159],
    ),
    ThreadRef::new(
        BRAND,
        "1972",
        "Medium Grey",
        &[162, 156, 158],
    ),
    ThreadRef::new(
        BRAND,
        "0141",
        "Silvery Gray",
        &[137, 134, 134],
    ),
    ThreadRef::new(
        BRAND,
        "0108",
        "Willow",
        &[130, 130, 126],
    ),
    ThreadRef::new(
        BRAND,
        "0152",
        "Willow",
        &[134, 133, 129],
    ),
    ThreadRef::new(
        BRAND,
        "0111",
        "Twilight",
        &[102, 103, 100],
    ),
    ThreadRef::new(
        BRAND,
        "0107",
        "Pewter",
        &[109, 114, 116],
    ),
    ThreadRef::new(
        BRAND,
        "2675",
        "Charcoal",
        &[113, 111, 110],
    ),
    ThreadRef::new(
        BRAND,
        "4175",
        "Field Green",
        &[63, 65, 61],
    ),
    ThreadRef::new(
        BRAND,
        "0020",
        "Mahogany",
        &[48, 47, 38],
    ),
];