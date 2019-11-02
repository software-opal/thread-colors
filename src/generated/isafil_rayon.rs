#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "Isafil Rayon";
const THREADS: [ThreadRef; 286] = [
    ThreadRef {
        brand: BRAND,
        code: "0010",
        name: "White",
        color: &[255, 255, 250],
    },
    ThreadRef {
        brand: BRAND,
        code: "0015",
        name: "White",
        color: &[255, 255, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "0020",
        name: "Black",
        color: &[0, 0, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "0104",
        name: "Vapor Blue",
        color: &[193, 189, 195],
    },
    ThreadRef {
        brand: BRAND,
        code: "0107",
        name: "GS Gray",
        color: &[149, 145, 142],
    },
    ThreadRef {
        brand: BRAND,
        code: "0108",
        name: "Medium Grey",
        color: &[117, 122, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "0109",
        name: "Medium Silver",
        color: &[205, 205, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "0111",
        name: "Dark Gray",
        color: &[84, 97, 94],
    },
    ThreadRef {
        brand: BRAND,
        code: "0124",
        name: "Natural",
        color: &[216, 218, 215],
    },
    ThreadRef {
        brand: BRAND,
        code: "0141",
        name: "Jadeite",
        color: &[149, 164, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "0142",
        name: "Lilac Hint",
        color: &[212, 213, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "0145",
        name: "Cream",
        color: &[220, 227, 235],
    },
    ThreadRef {
        brand: BRAND,
        code: "0150",
        name: "Natural White",
        color: &[240, 240, 225],
    },
    ThreadRef {
        brand: BRAND,
        code: "0152",
        name: "Gray Ridge",
        color: &[130, 120, 135],
    },
    ThreadRef {
        brand: BRAND,
        code: "0156",
        name: "Light Grey/Green",
        color: &[183, 182, 183],
    },
    ThreadRef {
        brand: BRAND,
        code: "0221",
        name: "Pale Yellow",
        color: &[221, 224, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "0232",
        name: "Charlock",
        color: &[221, 215, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "0241",
        name: "Bright Yellow",
        color: &[232, 241, 170],
    },
    ThreadRef {
        brand: BRAND,
        code: "0251",
        name: "Pale Yellow",
        color: &[253, 248, 188],
    },
    ThreadRef {
        brand: BRAND,
        code: "0260",
        name: "Pale Lemon",
        color: &[255, 255, 201],
    },
    ThreadRef {
        brand: BRAND,
        code: "0270",
        name: "Ivory White",
        color: &[255, 255, 236],
    },
    ThreadRef {
        brand: BRAND,
        code: "0345",
        name: "Pistachio",
        color: &[150, 161, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "0352",
        name: "Leek Green",
        color: &[191, 193, 116],
    },
    ThreadRef {
        brand: BRAND,
        code: "0442",
        name: "Lime",
        color: &[173, 184, 84],
    },
    ThreadRef {
        brand: BRAND,
        code: "0453",
        name: "Sage Green",
        color: &[167, 165, 125],
    },
    ThreadRef {
        brand: BRAND,
        code: "0454",
        name: "Medium Olive Green",
        color: &[139, 147, 89],
    },
    ThreadRef {
        brand: BRAND,
        code: "0465",
        name: "Medium Brown",
        color: &[128, 109, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "0505",
        name: "Pale Lemon",
        color: &[255, 255, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "0506",
        name: "Yellow",
        color: &[255, 215, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "0510",
        name: "Ivory White",
        color: &[255, 255, 233],
    },
    ThreadRef {
        brand: BRAND,
        code: "0520",
        name: "Pale Lemon",
        color: &[255, 255, 203],
    },
    ThreadRef {
        brand: BRAND,
        code: "0532",
        name: "Cocoon",
        color: &[215, 191, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "0542",
        name: "Pale Beige",
        color: &[219, 203, 44],
    },
    ThreadRef {
        brand: BRAND,
        code: "0545",
        name: "Medium Yellow",
        color: &[201, 184, 44],
    },
    ThreadRef {
        brand: BRAND,
        code: "0551",
        name: "Gold/Yellow",
        color: &[169, 124, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "0580",
        name: "Pale Yellow",
        color: &[249, 243, 105],
    },
    ThreadRef {
        brand: BRAND,
        code: "0590",
        name: "Lemon",
        color: &[255, 238, 67],
    },
    ThreadRef {
        brand: BRAND,
        code: "0600",
        name: "Yellow",
        color: &[255, 224, 61],
    },
    ThreadRef {
        brand: BRAND,
        code: "0605",
        name: "Canary Yellow",
        color: &[255, 238, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "0610",
        name: "Buttercup",
        color: &[255, 203, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "0615",
        name: "Star Gold",
        color: &[254, 227, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "0620",
        name: "Pale Yellow",
        color: &[255, 249, 111],
    },
    ThreadRef {
        brand: BRAND,
        code: "0625",
        name: "Pale Yellow",
        color: &[252, 250, 107],
    },
    ThreadRef {
        brand: BRAND,
        code: "0630",
        name: "Pale Yellow",
        color: &[255, 240, 137],
    },
    ThreadRef {
        brand: BRAND,
        code: "0635",
        name: "Pale Lemon",
        color: &[255, 247, 201],
    },
    ThreadRef {
        brand: BRAND,
        code: "0640",
        name: "Star Gold",
        color: &[249, 226, 81],
    },
    ThreadRef {
        brand: BRAND,
        code: "0660",
        name: "Ivory White",
        color: &[255, 255, 217],
    },
    ThreadRef {
        brand: BRAND,
        code: "0670",
        name: "White",
        color: &[255, 255, 245],
    },
    ThreadRef {
        brand: BRAND,
        code: "0672",
        name: "Stainless Steel",
        color: &[207, 195, 180],
    },
    ThreadRef {
        brand: BRAND,
        code: "0702",
        name: "Gold/Orange",
        color: &[242, 155, 57],
    },
    ThreadRef {
        brand: BRAND,
        code: "0704",
        name: "Medium Yellow",
        color: &[212, 173, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "0713",
        name: "Canary",
        color: &[250, 215, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "0726",
        name: "Medium Green/Yellow",
        color: &[168, 154, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "0732",
        name: "Pale Green/Grey",
        color: &[161, 160, 134],
    },
    ThreadRef {
        brand: BRAND,
        code: "0747",
        name: "Green/Yellow",
        color: &[79, 79, 46],
    },
    ThreadRef {
        brand: BRAND,
        code: "0761",
        name: "Pale Peach",
        color: &[221, 201, 165],
    },
    ThreadRef {
        brand: BRAND,
        code: "0800",
        name: "Light Orange",
        color: &[249, 172, 27],
    },
    ThreadRef {
        brand: BRAND,
        code: "0805",
        name: "Buttercup",
        color: &[255, 212, 42],
    },
    ThreadRef {
        brand: BRAND,
        code: "0811",
        name: "Mustard Yellow",
        color: &[226, 175, 35],
    },
    ThreadRef {
        brand: BRAND,
        code: "0821",
        name: "Goldenrod",
        color: &[217, 149, 23],
    },
    ThreadRef {
        brand: BRAND,
        code: "0822",
        name: "Pale Yellow",
        color: &[240, 199, 115],
    },
    ThreadRef {
        brand: BRAND,
        code: "0830",
        name: "Sunshine",
        color: &[255, 226, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "0842",
        name: "Gold",
        color: &[172, 129, 81],
    },
    ThreadRef {
        brand: BRAND,
        code: "0851",
        name: "Maize",
        color: &[231, 201, 155],
    },
    ThreadRef {
        brand: BRAND,
        code: "0853",
        name: "Camel",
        color: &[170, 139, 109],
    },
    ThreadRef {
        brand: BRAND,
        code: "0866",
        name: "Dark Brown",
        color: &[74, 58, 42],
    },
    ThreadRef {
        brand: BRAND,
        code: "0870",
        name: "White",
        color: &[255, 255, 240],
    },
    ThreadRef {
        brand: BRAND,
        code: "0873",
        name: "Sand",
        color: &[198, 186, 165],
    },
    ThreadRef {
        brand: BRAND,
        code: "0900",
        name: "Gold/Orange",
        color: &[255, 168, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "0932",
        name: "Cocoa Brown",
        color: &[174, 110, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "0933",
        name: "Light Cocoa",
        color: &[137, 91, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "0936",
        name: "Dark Green/Yellow",
        color: &[102, 93, 30],
    },
    ThreadRef {
        brand: BRAND,
        code: "0940",
        name: "Gold",
        color: &[187, 142, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "0945",
        name: "Bagby Green",
        color: &[120, 95, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "0961",
        name: "Ivory White",
        color: &[255, 250, 227],
    },
    ThreadRef {
        brand: BRAND,
        code: "0970",
        name: "Eggshell",
        color: &[255, 244, 214],
    },
    ThreadRef {
        brand: BRAND,
        code: "1041",
        name: "Apricot Nectar",
        color: &[230, 166, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "1055",
        name: "Mushroom",
        color: &[121, 81, 53],
    },
    ThreadRef {
        brand: BRAND,
        code: "1060",
        name: "Wheat",
        color: &[255, 225, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "1061",
        name: "Woodrose",
        color: &[181, 148, 139],
    },
    ThreadRef {
        brand: BRAND,
        code: "1099",
        name: "Soleil",
        color: &[255, 140, 35],
    },
    ThreadRef {
        brand: BRAND,
        code: "1100",
        name: "Orange",
        color: &[255, 152, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "1101",
        name: "Mango",
        color: &[255, 168, 30],
    },
    ThreadRef {
        brand: BRAND,
        code: "1113",
        name: "Bronze",
        color: &[185, 81, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "1140",
        name: "Maize",
        color: &[231, 202, 157],
    },
    ThreadRef {
        brand: BRAND,
        code: "1172",
        name: "Almond Oil",
        color: &[252, 225, 190],
    },
    ThreadRef {
        brand: BRAND,
        code: "1211",
        name: "Brown",
        color: &[201, 116, 89],
    },
    ThreadRef {
        brand: BRAND,
        code: "1276",
        name: "Dark Brown",
        color: &[55, 41, 35],
    },
    ThreadRef {
        brand: BRAND,
        code: "1300",
        name: "Soleil",
        color: &[252, 121, 28],
    },
    ThreadRef {
        brand: BRAND,
        code: "1302",
        name: "Orange",
        color: &[217, 74, 9],
    },
    ThreadRef {
        brand: BRAND,
        code: "1304",
        name: "Sunkist",
        color: &[253, 71, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "1305",
        name: "Sunkist",
        color: &[253, 79, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "1311",
        name: "Saffron",
        color: &[192, 109, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "1313",
        name: "Dark Orange/ Brown",
        color: &[185, 70, 55],
    },
    ThreadRef {
        brand: BRAND,
        code: "1320",
        name: "Fluo Orange",
        color: &[255, 112, 28],
    },
    ThreadRef {
        brand: BRAND,
        code: "1324",
        name: "Medium Brown",
        color: &[149, 74, 53],
    },
    ThreadRef {
        brand: BRAND,
        code: "1342",
        name: "Unaka Sand",
        color: &[145, 93, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "1346",
        name: "Clayrust",
        color: &[105, 74, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "1348",
        name: "Dark Brown",
        color: &[131, 70, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "1351",
        name: "Peach",
        color: &[255, 181, 125],
    },
    ThreadRef {
        brand: BRAND,
        code: "1352",
        name: "Chamois",
        color: &[255, 166, 92],
    },
    ThreadRef {
        brand: BRAND,
        code: "1355",
        name: "Dark Brown",
        color: &[130, 88, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "1361",
        name: "Alesan",
        color: &[255, 212, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "1362",
        name: "Alesan",
        color: &[255, 220, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "1366",
        name: "Dark Brown",
        color: &[51, 37, 21],
    },
    ThreadRef {
        brand: BRAND,
        code: "1376",
        name: "Brown",
        color: &[84, 58, 30],
    },
    ThreadRef {
        brand: BRAND,
        code: "1421",
        name: "Crabapple",
        color: &[215, 114, 95],
    },
    ThreadRef {
        brand: BRAND,
        code: "1430",
        name: "Medium Orange",
        color: &[250, 133, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "1573",
        name: "Dark Grey/Green",
        color: &[94, 76, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "1600",
        name: "Copper",
        color: &[227, 116, 79],
    },
    ThreadRef {
        brand: BRAND,
        code: "1630",
        name: "Bone",
        color: &[255, 229, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "1701",
        name: "Orange/Red",
        color: &[232, 51, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "1703",
        name: "Paprika",
        color: &[255, 56, 33],
    },
    ThreadRef {
        brand: BRAND,
        code: "1705",
        name: "Golden Glory",
        color: &[235, 79, 25],
    },
    ThreadRef {
        brand: BRAND,
        code: "1750",
        name: "Tea Rose",
        color: &[220, 117, 120],
    },
    ThreadRef {
        brand: BRAND,
        code: "1761",
        name: "Heather",
        color: &[219, 201, 191],
    },
    ThreadRef {
        brand: BRAND,
        code: "1800",
        name: "Honeysuckle",
        color: &[226, 53, 39],
    },
    ThreadRef {
        brand: BRAND,
        code: "1802",
        name: "Medium Red/Orange",
        color: &[231, 81, 88],
    },
    ThreadRef {
        brand: BRAND,
        code: "1805",
        name: "Dark Salmon Pink",
        color: &[196, 63, 79],
    },
    ThreadRef {
        brand: BRAND,
        code: "1840",
        name: "Light Orange",
        color: &[236, 152, 152],
    },
    ThreadRef {
        brand: BRAND,
        code: "1842",
        name: "Dark Brown",
        color: &[134, 69, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "1849",
        name: "Shell Pink",
        color: &[241, 135, 130],
    },
    ThreadRef {
        brand: BRAND,
        code: "1850",
        name: "Strawberry Cream",
        color: &[255, 198, 188],
    },
    ThreadRef {
        brand: BRAND,
        code: "1855",
        name: "Delicate Mauve",
        color: &[255, 219, 222],
    },
    ThreadRef {
        brand: BRAND,
        code: "1860",
        name: "Light Pink",
        color: &[240, 215, 212],
    },
    ThreadRef {
        brand: BRAND,
        code: "1870",
        name: "Star White",
        color: &[255, 236, 228],
    },
    ThreadRef {
        brand: BRAND,
        code: "1876",
        name: "Dark Brown",
        color: &[70, 46, 28],
    },
    ThreadRef {
        brand: BRAND,
        code: "1900",
        name: "Dark Salmon Pink",
        color: &[205, 50, 76],
    },
    ThreadRef {
        brand: BRAND,
        code: "1902",
        name: "Red",
        color: &[191, 30, 30],
    },
    ThreadRef {
        brand: BRAND,
        code: "1905",
        name: "Dark Orange/ Brown",
        color: &[169, 65, 43],
    },
    ThreadRef {
        brand: BRAND,
        code: "1912",
        name: "Dark Rust Red",
        color: &[145, 53, 61],
    },
    ThreadRef {
        brand: BRAND,
        code: "1913",
        name: "Terra Cotta",
        color: &[160, 56, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "1914",
        name: "Dark Red/Orange",
        color: &[159, 74, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "1921",
        name: "Baroque Rose",
        color: &[185, 97, 105],
    },
    ThreadRef {
        brand: BRAND,
        code: "1960",
        name: "Medium Pink",
        color: &[228, 200, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "1972",
        name: "Light Grey/Green",
        color: &[181, 181, 181],
    },
    ThreadRef {
        brand: BRAND,
        code: "2051",
        name: "Light Purple",
        color: &[168, 128, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "2053",
        name: "Deauville Mauve",
        color: &[193, 149, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "2100",
        name: "Dark Orange/Red",
        color: &[150, 51, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "2123",
        name: "Dark Orange/Red",
        color: &[145, 53, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "2152",
        name: "Cashmere Rose",
        color: &[205, 138, 156],
    },
    ThreadRef {
        brand: BRAND,
        code: "2165",
        name: "Star White",
        color: &[255, 245, 238],
    },
    ThreadRef {
        brand: BRAND,
        code: "2168",
        name: "Oyster",
        color: &[238, 229, 229],
    },
    ThreadRef {
        brand: BRAND,
        code: "2170",
        name: "Peach Skin",
        color: &[238, 187, 180],
    },
    ThreadRef {
        brand: BRAND,
        code: "2180",
        name: "Eggshell",
        color: &[255, 240, 215],
    },
    ThreadRef {
        brand: BRAND,
        code: "2220",
        name: "Pale Pink",
        color: &[255, 145, 183],
    },
    ThreadRef {
        brand: BRAND,
        code: "2230",
        name: "Pink",
        color: &[255, 180, 204],
    },
    ThreadRef {
        brand: BRAND,
        code: "2241",
        name: "Heather Rose",
        color: &[178, 108, 122],
    },
    ThreadRef {
        brand: BRAND,
        code: "2250",
        name: "Rosewater",
        color: &[255, 224, 224],
    },
    ThreadRef {
        brand: BRAND,
        code: "2300",
        name: "Shocking Pink",
        color: &[217, 52, 101],
    },
    ThreadRef {
        brand: BRAND,
        code: "2310",
        name: "Azalea",
        color: &[231, 70, 127],
    },
    ThreadRef {
        brand: BRAND,
        code: "2332",
        name: "Dark Brown",
        color: &[139, 74, 94],
    },
    ThreadRef {
        brand: BRAND,
        code: "2333",
        name: "Dark Purple",
        color: &[107, 54, 73],
    },
    ThreadRef {
        brand: BRAND,
        code: "2360",
        name: "Orchid Ice",
        color: &[231, 210, 216],
    },
    ThreadRef {
        brand: BRAND,
        code: "2500",
        name: "Dusky Pink",
        color: &[169, 85, 117],
    },
    ThreadRef {
        brand: BRAND,
        code: "2502",
        name: "Dark Maroon",
        color: &[95, 30, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "2505",
        name: "Medium Purple",
        color: &[166, 58, 109],
    },
    ThreadRef {
        brand: BRAND,
        code: "2513",
        name: "Dark Pink",
        color: &[189, 84, 116],
    },
    ThreadRef {
        brand: BRAND,
        code: "2520",
        name: "Light Cerise",
        color: &[237, 106, 146],
    },
    ThreadRef {
        brand: BRAND,
        code: "2524",
        name: "Purple Haze",
        color: &[147, 115, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "2550",
        name: "Medium Pink",
        color: &[241, 147, 174],
    },
    ThreadRef {
        brand: BRAND,
        code: "2555",
        name: "Orchid Pink",
        color: &[255, 198, 229],
    },
    ThreadRef {
        brand: BRAND,
        code: "2600",
        name: "Medium Berry",
        color: &[154, 85, 119],
    },
    ThreadRef {
        brand: BRAND,
        code: "2611",
        name: "Medium Purple",
        color: &[123, 58, 100],
    },
    ThreadRef {
        brand: BRAND,
        code: "2620",
        name: "Light Pink",
        color: &[229, 161, 203],
    },
    ThreadRef {
        brand: BRAND,
        code: "2640",
        name: "Pale Pink",
        color: &[243, 192, 233],
    },
    ThreadRef {
        brand: BRAND,
        code: "2645",
        name: "Oyster",
        color: &[250, 219, 238],
    },
    ThreadRef {
        brand: BRAND,
        code: "2650",
        name: "Medium Pink",
        color: &[231, 201, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "2675",
        name: "Medium Grey",
        color: &[124, 120, 122],
    },
    ThreadRef {
        brand: BRAND,
        code: "2712",
        name: "Violet",
        color: &[147, 84, 152],
    },
    ThreadRef {
        brand: BRAND,
        code: "2715",
        name: "Dark Maroon",
        color: &[71, 0, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "2743",
        name: "Dark Purple",
        color: &[44, 13, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "2810",
        name: "Medium Purple",
        color: &[143, 93, 143],
    },
    ThreadRef {
        brand: BRAND,
        code: "2820",
        name: "Purple",
        color: &[201, 121, 196],
    },
    ThreadRef {
        brand: BRAND,
        code: "2830",
        name: "Lavender",
        color: &[185, 143, 195],
    },
    ThreadRef {
        brand: BRAND,
        code: "3040",
        name: "Tulip",
        color: &[199, 168, 248],
    },
    ThreadRef {
        brand: BRAND,
        code: "3045",
        name: "Light Pink",
        color: &[233, 194, 229],
    },
    ThreadRef {
        brand: BRAND,
        code: "3102",
        name: "Medium Purple",
        color: &[83, 78, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "3103",
        name: "Purple",
        color: &[100, 72, 134],
    },
    ThreadRef {
        brand: BRAND,
        code: "3110",
        name: "Dark Purple",
        color: &[78, 57, 114],
    },
    ThreadRef {
        brand: BRAND,
        code: "3200",
        name: "Violet",
        color: &[108, 110, 191],
    },
    ThreadRef {
        brand: BRAND,
        code: "3211",
        name: "Imperial Purple",
        color: &[91, 73, 127],
    },
    ThreadRef {
        brand: BRAND,
        code: "3213",
        name: "Pale Purple",
        color: &[149, 123, 188],
    },
    ThreadRef {
        brand: BRAND,
        code: "3222",
        name: "Petrol Blue",
        color: &[76, 66, 111],
    },
    ThreadRef {
        brand: BRAND,
        code: "3230",
        name: "Violet Tulip",
        color: &[149, 138, 192],
    },
    ThreadRef {
        brand: BRAND,
        code: "3241",
        name: "Pastel Lilac",
        color: &[189, 176, 207],
    },
    ThreadRef {
        brand: BRAND,
        code: "3262",
        name: "Pale Lilac",
        color: &[208, 197, 220],
    },
    ThreadRef {
        brand: BRAND,
        code: "3301",
        name: "Bright Iris",
        color: &[105, 98, 165],
    },
    ThreadRef {
        brand: BRAND,
        code: "3313",
        name: "Blackberry",
        color: &[70, 45, 79],
    },
    ThreadRef {
        brand: BRAND,
        code: "3321",
        name: "Medium Purple",
        color: &[123, 146, 215],
    },
    ThreadRef {
        brand: BRAND,
        code: "3322",
        name: "Grey/Blue",
        color: &[75, 81, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "3330",
        name: "Purple",
        color: &[68, 55, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "3340",
        name: "Pale Lilac",
        color: &[183, 191, 229],
    },
    ThreadRef {
        brand: BRAND,
        code: "3344",
        name: "Dark Maroon",
        color: &[59, 45, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "3353",
        name: "Concord",
        color: &[61, 50, 88],
    },
    ThreadRef {
        brand: BRAND,
        code: "3355",
        name: "Navy Blue",
        color: &[0, 7, 79],
    },
    ThreadRef {
        brand: BRAND,
        code: "3420",
        name: "Pale Lilac",
        color: &[163, 164, 201],
    },
    ThreadRef {
        brand: BRAND,
        code: "3430",
        name: "Blue",
        color: &[129, 147, 189],
    },
    ThreadRef {
        brand: BRAND,
        code: "3443",
        name: "Steel",
        color: &[84, 79, 108],
    },
    ThreadRef {
        brand: BRAND,
        code: "3540",
        name: "Violet",
        color: &[100, 83, 170],
    },
    ThreadRef {
        brand: BRAND,
        code: "3543",
        name: "Purple",
        color: &[74, 85, 151],
    },
    ThreadRef {
        brand: BRAND,
        code: "3600",
        name: "Soldier Blue",
        color: &[86, 102, 172],
    },
    ThreadRef {
        brand: BRAND,
        code: "3611",
        name: "Purple",
        color: &[70, 82, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "3631",
        name: "Medium Purple",
        color: &[116, 140, 193],
    },
    ThreadRef {
        brand: BRAND,
        code: "3661",
        name: "White",
        color: &[229, 233, 249],
    },
    ThreadRef {
        brand: BRAND,
        code: "3743",
        name: "Pale Green",
        color: &[100, 222, 131],
    },
    ThreadRef {
        brand: BRAND,
        code: "3752",
        name: "Pastel Blue",
        color: &[195, 206, 214],
    },
    ThreadRef {
        brand: BRAND,
        code: "3761",
        name: "Ice Blue",
        color: &[186, 209, 241],
    },
    ThreadRef {
        brand: BRAND,
        code: "3810",
        name: "Slate Blue",
        color: &[110, 129, 169],
    },
    ThreadRef {
        brand: BRAND,
        code: "3820",
        name: "Light Blue",
        color: &[143, 176, 207],
    },
    ThreadRef {
        brand: BRAND,
        code: "3822",
        name: "Dark Blue",
        color: &[29, 60, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "3840",
        name: "Light Blue/Green",
        color: &[175, 198, 214],
    },
    ThreadRef {
        brand: BRAND,
        code: "3845",
        name: "White",
        color: &[215, 229, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "3851",
        name: "Lake Blue",
        color: &[171, 194, 219],
    },
    ThreadRef {
        brand: BRAND,
        code: "3901",
        name: "Mid Windsor",
        color: &[20, 161, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "3910",
        name: "Powder Blue",
        color: &[171, 233, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "3952",
        name: "Faded Denim",
        color: &[126, 143, 170],
    },
    ThreadRef {
        brand: BRAND,
        code: "3961",
        name: "Cadet Blue",
        color: &[192, 223, 239],
    },
    ThreadRef {
        brand: BRAND,
        code: "3971",
        name: "Lilac Hint",
        color: &[212, 213, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "4022",
        name: "Dark Green/Blue",
        color: &[44, 55, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "4030",
        name: "Light Blue",
        color: &[77, 166, 212],
    },
    ThreadRef {
        brand: BRAND,
        code: "4032",
        name: "Smoke Blue",
        color: &[104, 121, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "4040",
        name: "Chicory",
        color: &[141, 167, 171],
    },
    ThreadRef {
        brand: BRAND,
        code: "4071",
        name: "White",
        color: &[236, 240, 239],
    },
    ThreadRef {
        brand: BRAND,
        code: "4100",
        name: "Medium Blue",
        color: &[28, 105, 125],
    },
    ThreadRef {
        brand: BRAND,
        code: "4105",
        name: "Light Blue",
        color: &[72, 180, 229],
    },
    ThreadRef {
        brand: BRAND,
        code: "4111",
        name: "Grizzly Turquoise",
        color: &[39, 159, 166],
    },
    ThreadRef {
        brand: BRAND,
        code: "4121",
        name: "Bright Blue",
        color: &[89, 166, 175],
    },
    ThreadRef {
        brand: BRAND,
        code: "4133",
        name: "Dark Blue/Green",
        color: &[60, 86, 99],
    },
    ThreadRef {
        brand: BRAND,
        code: "4175",
        name: "Dark Grey",
        color: &[86, 85, 86],
    },
    ThreadRef {
        brand: BRAND,
        code: "4231",
        name: "Bright Blue",
        color: &[93, 173, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "4232",
        name: "Dark Blue",
        color: &[24, 83, 106],
    },
    ThreadRef {
        brand: BRAND,
        code: "4250",
        name: "Aqualine",
        color: &[173, 206, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "4400",
        name: "Medium Green/Blue",
        color: &[56, 117, 126],
    },
    ThreadRef {
        brand: BRAND,
        code: "4420",
        name: "Aqua",
        color: &[101, 169, 179],
    },
    ThreadRef {
        brand: BRAND,
        code: "4421",
        name: "Slate Grey",
        color: &[80, 125, 146],
    },
    ThreadRef {
        brand: BRAND,
        code: "4430",
        name: "Light Clear Blue",
        color: &[116, 171, 174],
    },
    ThreadRef {
        brand: BRAND,
        code: "4432",
        name: "Dark Navy",
        color: &[54, 69, 78],
    },
    ThreadRef {
        brand: BRAND,
        code: "4440",
        name: "Tropical Wave",
        color: &[138, 205, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "4500",
        name: "Dark Bottle Green",
        color: &[44, 84, 86],
    },
    ThreadRef {
        brand: BRAND,
        code: "4513",
        name: "Bristol Blue",
        color: &[77, 130, 131],
    },
    ThreadRef {
        brand: BRAND,
        code: "4516",
        name: "Dark Green",
        color: &[48, 63, 46],
    },
    ThreadRef {
        brand: BRAND,
        code: "4610",
        name: "Teal",
        color: &[97, 150, 147],
    },
    ThreadRef {
        brand: BRAND,
        code: "4620",
        name: "Porcelain",
        color: &[92, 169, 163],
    },
    ThreadRef {
        brand: BRAND,
        code: "4643",
        name: "Gobelin Blue",
        color: &[88, 107, 111],
    },
    ThreadRef {
        brand: BRAND,
        code: "4644",
        name: "Enchanted Sea",
        color: &[58, 86, 82],
    },
    ThreadRef {
        brand: BRAND,
        code: "4840",
        name: "Pale Mint Green",
        color: &[152, 217, 199],
    },
    ThreadRef {
        brand: BRAND,
        code: "5000",
        name: "Gotcha Green",
        color: &[37, 156, 114],
    },
    ThreadRef {
        brand: BRAND,
        code: "5020",
        name: "Pale Green",
        color: &[126, 217, 185],
    },
    ThreadRef {
        brand: BRAND,
        code: "5040",
        name: "White",
        color: &[245, 252, 251],
    },
    ThreadRef {
        brand: BRAND,
        code: "5050",
        name: "Mint",
        color: &[180, 226, 203],
    },
    ThreadRef {
        brand: BRAND,
        code: "5111",
        name: "Pale Jade",
        color: &[30, 175, 116],
    },
    ThreadRef {
        brand: BRAND,
        code: "5210",
        name: "Neon Green",
        color: &[95, 203, 119],
    },
    ThreadRef {
        brand: BRAND,
        code: "5230",
        name: "Ocean Wave",
        color: &[145, 203, 166],
    },
    ThreadRef {
        brand: BRAND,
        code: "5240",
        name: "Seafoam",
        color: &[121, 212, 161],
    },
    ThreadRef {
        brand: BRAND,
        code: "5255",
        name: "Dark Jade Green",
        color: &[55, 98, 81],
    },
    ThreadRef {
        brand: BRAND,
        code: "5326",
        name: "Green",
        color: &[0, 88, 39],
    },
    ThreadRef {
        brand: BRAND,
        code: "5375",
        name: "Dark Green/Blue",
        color: &[42, 63, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "5422",
        name: "Green",
        color: &[11, 124, 42],
    },
    ThreadRef {
        brand: BRAND,
        code: "5425",
        name: "Green",
        color: &[2, 102, 39],
    },
    ThreadRef {
        brand: BRAND,
        code: "5431",
        name: "Medium Green",
        color: &[77, 180, 91],
    },
    ThreadRef {
        brand: BRAND,
        code: "5460",
        name: "Mint",
        color: &[184, 229, 187],
    },
    ThreadRef {
        brand: BRAND,
        code: "5470",
        name: "Dark Olive Green",
        color: &[194, 254, 168],
    },
    ThreadRef {
        brand: BRAND,
        code: "5515",
        name: "Vibrant Green",
        color: &[2, 154, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "5531",
        name: "Lime",
        color: &[152, 210, 98],
    },
    ThreadRef {
        brand: BRAND,
        code: "5542",
        name: "Shale Green",
        color: &[109, 145, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "5600",
        name: "Green",
        color: &[98, 203, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "5610",
        name: "Lime",
        color: &[175, 238, 116],
    },
    ThreadRef {
        brand: BRAND,
        code: "5611",
        name: "Medium Green",
        color: &[121, 196, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "5620",
        name: "Harvest Green",
        color: &[60, 189, 65],
    },
    ThreadRef {
        brand: BRAND,
        code: "5633",
        name: "Moss Green",
        color: &[84, 138, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "5650",
        name: "Brilliant Lime",
        color: &[205, 255, 135],
    },
    ThreadRef {
        brand: BRAND,
        code: "5765",
        name: "Dark Green",
        color: &[37, 84, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "5766",
        name: "Dark Green",
        color: &[58, 93, 53],
    },
    ThreadRef {
        brand: BRAND,
        code: "5776",
        name: "Olive",
        color: &[58, 74, 42],
    },
    ThreadRef {
        brand: BRAND,
        code: "5832",
        name: "Spruce",
        color: &[161, 224, 91],
    },
    ThreadRef {
        brand: BRAND,
        code: "5833",
        name: "Green/Yellow",
        color: &[117, 154, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "5840",
        name: "Sharp Green",
        color: &[191, 240, 109],
    },
    ThreadRef {
        brand: BRAND,
        code: "5933",
        name: "Meadow",
        color: &[79, 140, 25],
    },
    ThreadRef {
        brand: BRAND,
        code: "5934",
        name: "Green",
        color: &[72, 126, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "5944",
        name: "Lime",
        color: &[53, 98, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "6051",
        name: "Lime",
        color: &[187, 229, 121],
    },
    ThreadRef {
        brand: BRAND,
        code: "6065",
        name: "Pale Green",
        color: &[88, 100, 46],
    },
    ThreadRef {
        brand: BRAND,
        code: "6071",
        name: "Lime Cream",
        color: &[210, 229, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "6133",
        name: "Pistachio",
        color: &[170, 166, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "6156",
        name: "Dark Grey/Green",
        color: &[93, 85, 67],
    },
];