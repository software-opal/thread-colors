#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Isacord Polyester - older";
pub const THREADS: [ThreadRef; 304] = [
    ThreadRef::new(
        BRAND,
        "0017",
        "White",
        &[242, 240, 245],
    ),
    ThreadRef::new(
        BRAND,
        "0015",
        "Snow White",
        &[245, 245, 247],
    ),
    ThreadRef::new(
        BRAND,
        "0010",
        "Oyster",
        &[245, 244, 245],
    ),
    ThreadRef::new(
        BRAND,
        "0101",
        "Platinum",
        &[243, 242, 237],
    ),
    ThreadRef::new(
        BRAND,
        "0270",
        "Ivory",
        &[245, 242, 220],
    ),
    ThreadRef::new(
        BRAND,
        "0250",
        "Chinese Yellow",
        &[242, 240, 196],
    ),
    ThreadRef::new(
        BRAND,
        "0520",
        "Wheat",
        &[243, 235, 162],
    ),
    ThreadRef::new(
        BRAND,
        "0310",
        "Canary",
        &[242, 233, 102],
    ),
    ThreadRef::new(
        BRAND,
        "0600",
        "Bright Yellow",
        &[242, 225, 75],
    ),
    ThreadRef::new(
        BRAND,
        "0311",
        "Bright Yellow",
        &[245, 211, 62],
    ),
    ThreadRef::new(
        BRAND,
        "0608",
        "Manila",
        &[243, 210, 79],
    ),
    ThreadRef::new(
        BRAND,
        "0605",
        "Bright Yellow",
        &[240, 216, 68],
    ),
    ThreadRef::new(
        BRAND,
        "0630",
        "Orangy Cream",
        &[243, 224, 125],
    ),
    ThreadRef::new(
        BRAND,
        "0713",
        "Cornsilk",
        &[241, 209, 109],
    ),
    ThreadRef::new(
        BRAND,
        "0506",
        "Nectar",
        &[238, 195, 88],
    ),
    ThreadRef::new(
        BRAND,
        "0700",
        "Penny",
        &[242, 190, 72],
    ),
    ThreadRef::new(
        BRAND,
        "0702",
        "Mustard",
        &[237, 182, 74],
    ),
    ThreadRef::new(
        BRAND,
        "0660",
        "Cottage Beige",
        &[240, 233, 202],
    ),
    ThreadRef::new(
        BRAND,
        "0640",
        "Blond",
        &[239, 214, 144],
    ),
    ThreadRef::new(
        BRAND,
        "0704",
        "Pollen Gold",
        &[213, 157, 64],
    ),
    ThreadRef::new(
        BRAND,
        "0824",
        "24 Kt. Gold",
        &[195, 132, 57],
    ),
    ThreadRef::new(
        BRAND,
        "0800",
        "Merit Gold",
        &[235, 158, 55],
    ),
    ThreadRef::new(
        BRAND,
        "0904",
        "Gold",
        &[225, 132, 56],
    ),
    ThreadRef::new(
        BRAND,
        "1102",
        "Rust",
        &[231, 116, 52],
    ),
    ThreadRef::new(
        BRAND,
        "1300",
        "Orange",
        &[243, 108, 61],
    ),
    ThreadRef::new(
        BRAND,
        "1304",
        "Sunkist",
        &[231, 87, 49],
    ),
    ThreadRef::new(
        BRAND,
        "1305",
        "Tangerine",
        &[251, 87, 58],
    ),
    ThreadRef::new(
        BRAND,
        "1301",
        "Orangeade",
        &[217, 83, 53],
    ),
    ThreadRef::new(
        BRAND,
        "1220",
        "Copper",
        &[231, 134, 87],
    ),
    ThreadRef::new(
        BRAND,
        "1060",
        "Opaline",
        &[243, 218, 189],
    ),
    ThreadRef::new(
        BRAND,
        "1362",
        "Tawny",
        &[243, 193, 154],
    ),
    ThreadRef::new(
        BRAND,
        "1351",
        "Rose Pink",
        &[246, 175, 153],
    ),
    ThreadRef::new(
        BRAND,
        "1352",
        "Melon",
        &[247, 161, 130],
    ),
    ThreadRef::new(
        BRAND,
        "1332",
        "Rust",
        &[200, 108, 76],
    ),
    ThreadRef::new(
        BRAND,
        "1114",
        "Golden Brown",
        &[212, 102, 68],
    ),
    ThreadRef::new(
        BRAND,
        "1335",
        "Red",
        &[174, 66, 51],
    ),
    ThreadRef::new(
        BRAND,
        "1551",
        "Peach",
        &[240, 172, 158],
    ),
    ThreadRef::new(
        BRAND,
        "1532",
        "Salmon",
        &[246, 162, 152],
    ),
    ThreadRef::new(
        BRAND,
        "1430",
        "Blush",
        &[226, 125, 105],
    ),
    ThreadRef::new(
        BRAND,
        "1521",
        "Rosewood",
        &[219, 91, 76],
    ),
    ThreadRef::new(
        BRAND,
        "1600",
        "Rosewood",
        &[217, 92, 80],
    ),
    ThreadRef::new(
        BRAND,
        "1725",
        "Jockey Red",
        &[175, 66, 63],
    ),
    ThreadRef::new(
        BRAND,
        "1701",
        "Marigold",
        &[225, 83, 67],
    ),
    ThreadRef::new(
        BRAND,
        "1703",
        "Jockey Red",
        &[196, 62, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1704",
        "Dark Rust",
        &[195, 58, 48],
    ),
    ThreadRef::new(
        BRAND,
        "1800",
        "Candy Apple Red",
        &[184, 57, 49],
    ),
    ThreadRef::new(
        BRAND,
        "1903",
        "Candy Apple Red",
        &[184, 56, 52],
    ),
    ThreadRef::new(
        BRAND,
        "1904",
        "Candy Apple Red",
        &[181, 55, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1902",
        "Red Brown",
        &[168, 52, 44],
    ),
    ThreadRef::new(
        BRAND,
        "1805",
        "Dark Rust",
        &[193, 63, 63],
    ),
    ThreadRef::new(
        BRAND,
        "1900",
        "Dark Rust",
        &[189, 59, 62],
    ),
    ThreadRef::new(
        BRAND,
        "1906",
        "Red Brown",
        &[155, 51, 55],
    ),
    ThreadRef::new(
        BRAND,
        "2101",
        "Red Brown",
        &[164, 51, 48],
    ),
    ThreadRef::new(
        BRAND,
        "1911",
        "Red Brown",
        &[143, 49, 46],
    ),
    ThreadRef::new(
        BRAND,
        "1913",
        "Red Brown",
        &[150, 54, 49],
    ),
    ThreadRef::new(
        BRAND,
        "2011",
        "Red Brown",
        &[147, 56, 60],
    ),
    ThreadRef::new(
        BRAND,
        "2022",
        "Red Brown",
        &[134, 55, 56],
    ),
    ThreadRef::new(
        BRAND,
        "1912",
        "Reddish Brown",
        &[130, 52, 52],
    ),
    ThreadRef::new(
        BRAND,
        "1840",
        "Dark Rose",
        &[232, 138, 144],
    ),
    ThreadRef::new(
        BRAND,
        "1921",
        "Cherrystone",
        &[177, 76, 83],
    ),
    ThreadRef::new(
        BRAND,
        "2113",
        "Reddish Brown",
        &[116, 48, 50],
    ),
    ThreadRef::new(
        BRAND,
        "2123",
        "Very Dark Brown",
        &[100, 45, 43],
    ),
    ThreadRef::new(
        BRAND,
        "2115",
        "Dark Sepia",
        &[85, 42, 44],
    ),
    ThreadRef::new(
        BRAND,
        "2363",
        "Pink",
        &[243, 217, 231],
    ),
    ThreadRef::new(
        BRAND,
        "2250",
        "Brownish Cream",
        &[241, 197, 211],
    ),
    ThreadRef::new(
        BRAND,
        "2560",
        "Carnation",
        &[234, 159, 198],
    ),
    ThreadRef::new(
        BRAND,
        "2550",
        "Rose",
        &[229, 148, 189],
    ),
    ThreadRef::new(
        BRAND,
        "2160",
        "Brownish Cream",
        &[235, 193, 207],
    ),
    ThreadRef::new(
        BRAND,
        "1860",
        "Desert Bloom",
        &[238, 196, 199],
    ),
    ThreadRef::new(
        BRAND,
        "1755",
        "Dusty Rose",
        &[229, 177, 180],
    ),
    ThreadRef::new(
        BRAND,
        "2171",
        "Silver Gray",
        &[240, 224, 226],
    ),
    ThreadRef::new(
        BRAND,
        "2170",
        "Palest Peach",
        &[236, 212, 210],
    ),
    ThreadRef::new(
        BRAND,
        "1760",
        "Beige",
        &[220, 181, 168],
    ),
    ThreadRef::new(
        BRAND,
        "2166",
        "Orchid",
        &[224, 189, 195],
    ),
    ThreadRef::new(
        BRAND,
        "1761",
        "Grape",
        &[215, 172, 169],
    ),
    ThreadRef::new(
        BRAND,
        "2051",
        "Brownish Pink",
        &[193, 144, 148],
    ),
    ThreadRef::new(
        BRAND,
        "2155",
        "Sugar Pink",
        &[239, 189, 204],
    ),
    ThreadRef::new(
        BRAND,
        "2152",
        "Dark Rose",
        &[211, 128, 147],
    ),
    ThreadRef::new(
        BRAND,
        "2153",
        "Dark Rose",
        &[194, 119, 137],
    ),
    ThreadRef::new(
        BRAND,
        "2241",
        "Passion Rose",
        &[167, 96, 108],
    ),
    ThreadRef::new(
        BRAND,
        "2222",
        "Sepia",
        &[110, 50, 55],
    ),
    ThreadRef::new(
        BRAND,
        "2333",
        "Very Dark Brown",
        &[100, 45, 53],
    ),
    ThreadRef::new(
        BRAND,
        "2224",
        "Warm Wine",
        &[115, 56, 61],
    ),
    ThreadRef::new(
        BRAND,
        "2336",
        "Intense Maroon",
        &[81, 50, 57],
    ),
    ThreadRef::new(
        BRAND,
        "2920",
        "Iris",
        &[132, 98, 153],
    ),
    ThreadRef::new(
        BRAND,
        "2910",
        "Lilac",
        &[152, 98, 167],
    ),
    ThreadRef::new(
        BRAND,
        "2905",
        "Raspberry",
        &[128, 77, 138],
    ),
    ThreadRef::new(
        BRAND,
        "2900",
        "Imperial Purple",
        &[92, 55, 116],
    ),
    ThreadRef::new(
        BRAND,
        "3114",
        "Purple Accent",
        &[78, 50, 92],
    ),
    ThreadRef::new(
        BRAND,
        "3241",
        "Cachet",
        &[149, 126, 166],
    ),
    ThreadRef::new(
        BRAND,
        "3211",
        "Storm Blue",
        &[92, 75, 131],
    ),
    ThreadRef::new(
        BRAND,
        "3210",
        "Blue",
        &[90, 75, 159],
    ),
    ThreadRef::new(
        BRAND,
        "3541",
        "Empire Blue",
        &[84, 61, 145],
    ),
    ThreadRef::new(
        BRAND,
        "3110",
        "Purple Accent",
        &[80, 54, 104],
    ),
    ThreadRef::new(
        BRAND,
        "3102",
        "Cassis",
        &[66, 51, 100],
    ),
    ThreadRef::new(
        BRAND,
        "3331",
        "Cristy Blue",
        &[158, 150, 211],
    ),
    ThreadRef::new(
        BRAND,
        "3640",
        "Lake Blue",
        &[173, 175, 227],
    ),
    ThreadRef::new(
        BRAND,
        "3600",
        "Blue",
        &[70, 77, 149],
    ),
    ThreadRef::new(
        BRAND,
        "3622",
        "Dark Blue",
        &[81, 73, 123],
    ),
    ThreadRef::new(
        BRAND,
        "3612",
        "Blue",
        &[94, 85, 160],
    ),
    ThreadRef::new(
        BRAND,
        "3544",
        "Jamie Blue",
        &[73, 64, 132],
    ),
    ThreadRef::new(
        BRAND,
        "3611",
        "Slate Blue",
        &[66, 74, 149],
    ),
    ThreadRef::new(
        BRAND,
        "3543",
        "Storm Blue",
        &[84, 72, 142],
    ),
    ThreadRef::new(
        BRAND,
        "3335",
        "Purple Maze",
        &[72, 60, 124],
    ),
    ThreadRef::new(
        BRAND,
        "3333",
        "Cassis",
        &[64, 51, 108],
    ),
    ThreadRef::new(
        BRAND,
        "4071",
        "Saturn Gray",
        &[217, 219, 230],
    ),
    ThreadRef::new(
        BRAND,
        "0145",
        "Pearl Gray",
        &[214, 206, 219],
    ),
    ThreadRef::new(
        BRAND,
        "3750",
        "Traditional Gray",
        &[185, 182, 208],
    ),
    ThreadRef::new(
        BRAND,
        "3842",
        "Silver Blue",
        &[118, 116, 141],
    ),
    ThreadRef::new(
        BRAND,
        "4032",
        "Slate Blue",
        &[101, 104, 139],
    ),
    ThreadRef::new(
        BRAND,
        "4033",
        "Ash",
        &[83, 73, 95],
    ),
    ThreadRef::new(
        BRAND,
        "4133",
        "Midnight",
        &[73, 64, 90],
    ),
    ThreadRef::new(
        BRAND,
        "3652",
        "Cristy Blue",
        &[185, 187, 229],
    ),
    ThreadRef::new(
        BRAND,
        "3641",
        "Medium Blue",
        &[140, 144, 198],
    ),
    ThreadRef::new(
        BRAND,
        "3810",
        "Mauve",
        &[122, 121, 170],
    ),
    ThreadRef::new(
        BRAND,
        "3732",
        "Cassis",
        &[77, 67, 100],
    ),
    ThreadRef::new(
        BRAND,
        "3761",
        "Paris Blue",
        &[191, 195, 228],
    ),
    ThreadRef::new(
        BRAND,
        "3951",
        "Baby Blue",
        &[163, 168, 203],
    ),
    ThreadRef::new(
        BRAND,
        "3953",
        "Grey Blue",
        &[110, 104, 137],
    ),
    ThreadRef::new(
        BRAND,
        "3743",
        "Midnight",
        &[77, 66, 96],
    ),
    ThreadRef::new(
        BRAND,
        "3444",
        "Light Midnight",
        &[68, 57, 78],
    ),
    ThreadRef::new(
        BRAND,
        "3344",
        "Smokey",
        &[55, 42, 53],
    ),
    ThreadRef::new(
        BRAND,
        "3353",
        "Purple Accent",
        &[76, 57, 98],
    ),
    ThreadRef::new(
        BRAND,
        "3323",
        "Blue Ink",
        &[63, 49, 82],
    ),
    ThreadRef::new(
        BRAND,
        "3355",
        "Midnight Navy",
        &[47, 37, 56],
    ),
    ThreadRef::new(
        BRAND,
        "3554",
        "Midnight Navy",
        &[46, 37, 51],
    ),
    ThreadRef::new(
        BRAND,
        "3650",
        "Silver Gray",
        &[221, 226, 237],
    ),
    ThreadRef::new(
        BRAND,
        "3840",
        "Light Blue",
        &[183, 202, 228],
    ),
    ThreadRef::new(
        BRAND,
        "3962",
        "Mint Julep",
        &[174, 214, 237],
    ),
    ThreadRef::new(
        BRAND,
        "3910",
        "Bright Blue",
        &[115, 172, 218],
    ),
    ThreadRef::new(
        BRAND,
        "3901",
        "Cerulean",
        &[59, 117, 173],
    ),
    ThreadRef::new(
        BRAND,
        "3902",
        "Slate Blue",
        &[57, 94, 140],
    ),
    ThreadRef::new(
        BRAND,
        "3522",
        "Dark Blue",
        &[69, 82, 136],
    ),
    ThreadRef::new(
        BRAND,
        "3900",
        "Teal Blue",
        &[46, 95, 159],
    ),
    ThreadRef::new(
        BRAND,
        "3815",
        "Surf Blue",
        &[86, 133, 191],
    ),
    ThreadRef::new(
        BRAND,
        "3820",
        "Powder Blue",
        &[137, 164, 209],
    ),
    ThreadRef::new(
        BRAND,
        "4230",
        "Aqua",
        &[142, 197, 220],
    ),
    ThreadRef::new(
        BRAND,
        "4111",
        "Medium Blue",
        &[98, 163, 194],
    ),
    ThreadRef::new(
        BRAND,
        "4113",
        "Medium Blue",
        &[88, 154, 191],
    ),
    ThreadRef::new(
        BRAND,
        "4101",
        "Marine Aqua",
        &[69, 135, 172],
    ),
    ThreadRef::new(
        BRAND,
        "4010",
        "Medium Teal Blue",
        &[67, 131, 160],
    ),
    ThreadRef::new(
        BRAND,
        "4116",
        "Dark Teal Blue",
        &[42, 90, 113],
    ),
    ThreadRef::new(
        BRAND,
        "4103",
        "Marine Aqua",
        &[66, 124, 175],
    ),
    ThreadRef::new(
        BRAND,
        "4240",
        "Aqua",
        &[162, 213, 226],
    ),
    ThreadRef::new(
        BRAND,
        "4220",
        "Country Blue",
        &[103, 169, 184],
    ),
    ThreadRef::new(
        BRAND,
        "4421",
        "Dark Sea Green",
        &[58, 116, 126],
    ),
    ThreadRef::new(
        BRAND,
        "4531",
        "Dark Aqua",
        &[53, 110, 122],
    ),
    ThreadRef::new(
        BRAND,
        "4250",
        "Pale Grey",
        &[209, 226, 223],
    ),
    ThreadRef::new(
        BRAND,
        "4430",
        "Aqua",
        &[140, 199, 213],
    ),
    ThreadRef::new(
        BRAND,
        "4620",
        "Country Blue",
        &[116, 171, 173],
    ),
    ThreadRef::new(
        BRAND,
        "4610",
        "M. D. Green",
        &[85, 144, 138],
    ),
    ThreadRef::new(
        BRAND,
        "4410",
        "Greenstone",
        &[52, 109, 109],
    ),
    ThreadRef::new(
        BRAND,
        "4452",
        "Peppermint",
        &[70, 123, 129],
    ),
    ThreadRef::new(
        BRAND,
        "4423",
        "Peacock Green",
        &[6, 126, 142],
    ),
    ThreadRef::new(
        BRAND,
        "4425",
        "Dark Sea Green",
        &[47, 100, 104],
    ),
    ThreadRef::new(
        BRAND,
        "4442",
        "Garden Green",
        &[54, 82, 91],
    ),
    ThreadRef::new(
        BRAND,
        "4644",
        "Steel Blue",
        &[84, 93, 84],
    ),
    ThreadRef::new(
        BRAND,
        "4643",
        "Green Bay",
        &[77, 99, 93],
    ),
    ThreadRef::new(
        BRAND,
        "4515",
        "Blue Spruce",
        &[52, 69, 69],
    ),
    ThreadRef::new(
        BRAND,
        "5005",
        "Irish Green",
        &[48, 89, 76],
    ),
    ThreadRef::new(
        BRAND,
        "4625",
        "Green Forest",
        &[65, 104, 94],
    ),
    ThreadRef::new(
        BRAND,
        "5101",
        "Jungle Green",
        &[67, 130, 112],
    ),
    ThreadRef::new(
        BRAND,
        "5010",
        "Isle Green",
        &[71, 137, 124],
    ),
    ThreadRef::new(
        BRAND,
        "5115",
        "Blue Green",
        &[103, 171, 153],
    ),
    ThreadRef::new(
        BRAND,
        "5050",
        "Sprite",
        &[190, 223, 217],
    ),
    ThreadRef::new(
        BRAND,
        "5220",
        "Seafoam",
        &[151, 204, 174],
    ),
    ThreadRef::new(
        BRAND,
        "5230",
        "Aqua",
        &[135, 191, 163],
    ),
    ThreadRef::new(
        BRAND,
        "5210",
        "Light Kelly",
        &[76, 139, 101],
    ),
    ThreadRef::new(
        BRAND,
        "5822",
        "Palest Green",
        &[151, 181, 124],
    ),
    ThreadRef::new(
        BRAND,
        "5833",
        "Foliage Green",
        &[127, 133, 70],
    ),
    ThreadRef::new(
        BRAND,
        "5933",
        "Desert Cactus",
        &[100, 103, 56],
    ),
    ThreadRef::new(
        BRAND,
        "5944",
        "Field Green",
        &[53, 67, 48],
    ),
    ThreadRef::new(
        BRAND,
        "5555",
        "D. H. Green",
        &[70, 69, 55],
    ),
    ThreadRef::new(
        BRAND,
        "5324",
        "Blue Spruce",
        &[50, 79, 66],
    ),
    ThreadRef::new(
        BRAND,
        "5643",
        "Deep Green",
        &[91, 97, 72],
    ),
    ThreadRef::new(
        BRAND,
        "5633",
        "Green",
        &[71, 103, 59],
    ),
    ThreadRef::new(
        BRAND,
        "0352",
        "Peapod",
        &[179, 167, 97],
    ),
    ThreadRef::new(
        BRAND,
        "0232",
        "Burnished Brown",
        &[187, 177, 96],
    ),
    ThreadRef::new(
        BRAND,
        "0442",
        "Light Brown",
        &[160, 136, 72],
    ),
    ThreadRef::new(
        BRAND,
        "6133",
        "Autumn Green",
        &[136, 127, 66],
    ),
    ThreadRef::new(
        BRAND,
        "5934",
        "Holly",
        &[75, 95, 48],
    ),
    ThreadRef::new(
        BRAND,
        "0465",
        "Dark Brown",
        &[93, 79, 58],
    ),
    ThreadRef::new(
        BRAND,
        "6156",
        "Espresso",
        &[83, 71, 57],
    ),
    ThreadRef::new(
        BRAND,
        "5866",
        "Field Green",
        &[60, 61, 49],
    ),
    ThreadRef::new(
        BRAND,
        "0221",
        "Gold",
        &[218, 201, 89],
    ),
    ThreadRef::new(
        BRAND,
        "0542",
        "Pistachio",
        &[171, 146, 59],
    ),
    ThreadRef::new(
        BRAND,
        "0546",
        "Bullion",
        &[158, 135, 61],
    ),
    ThreadRef::new(
        BRAND,
        "0345",
        "Brown",
        &[129, 104, 56],
    ),
    ThreadRef::new(
        BRAND,
        "0747",
        "Sienna",
        &[104, 80, 52],
    ),
    ThreadRef::new(
        BRAND,
        "0970",
        "Ivory",
        &[245, 242, 223],
    ),
    ThreadRef::new(
        BRAND,
        "0532",
        "Pale Brown",
        &[215, 187, 134],
    ),
    ThreadRef::new(
        BRAND,
        "0552",
        "Burnished Brown",
        &[191, 159, 113],
    ),
    ThreadRef::new(
        BRAND,
        "1172",
        "Beige",
        &[217, 193, 163],
    ),
    ThreadRef::new(
        BRAND,
        "1123",
        "Umber",
        &[181, 151, 120],
    ),
    ThreadRef::new(
        BRAND,
        "0934",
        "Umber",
        &[182, 152, 121],
    ),
    ThreadRef::new(
        BRAND,
        "0853",
        "Medium Brown",
        &[157, 122, 93],
    ),
    ThreadRef::new(
        BRAND,
        "1565",
        "Dark Taupe",
        &[105, 87, 77],
    ),
    ThreadRef::new(
        BRAND,
        "0761",
        "Camel",
        &[221, 203, 171],
    ),
    ThreadRef::new(
        BRAND,
        "0651",
        "Mocha Cream",
        &[219, 192, 143],
    ),
    ThreadRef::new(
        BRAND,
        "0851",
        "Pale Brown",
        &[213, 177, 132],
    ),
    ThreadRef::new(
        BRAND,
        "0832",
        "Burnished Brown",
        &[191, 149, 98],
    ),
    ThreadRef::new(
        BRAND,
        "0822",
        "Old Gold",
        &[187, 143, 81],
    ),
    ThreadRef::new(
        BRAND,
        "0941",
        "Brown",
        &[156, 112, 66],
    ),
    ThreadRef::new(
        BRAND,
        "1134",
        "Date",
        &[137, 88, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1140",
        "Gold",
        &[234, 200, 159],
    ),
    ThreadRef::new(
        BRAND,
        "1141",
        "Cinnamon",
        &[212, 165, 131],
    ),
    ThreadRef::new(
        BRAND,
        "0842",
        "Wicker",
        &[194, 144, 94],
    ),
    ThreadRef::new(
        BRAND,
        "1154",
        "Brown",
        &[125, 83, 59],
    ),
    ThreadRef::new(
        BRAND,
        "0933",
        "Mushroom",
        &[114, 76, 54],
    ),
    ThreadRef::new(
        BRAND,
        "1055",
        "Sienna",
        &[107, 74, 58],
    ),
    ThreadRef::new(
        BRAND,
        "0870",
        "Palest Ivory",
        &[238, 235, 225],
    ),
    ThreadRef::new(
        BRAND,
        "1061",
        "Cocoa Mulch",
        &[173, 141, 124],
    ),
    ThreadRef::new(
        BRAND,
        "0874",
        "Khaki Green",
        &[164, 146, 135],
    ),
    ThreadRef::new(
        BRAND,
        "0722",
        "Willow",
        &[140, 121, 105],
    ),
    ThreadRef::new(
        BRAND,
        "0945",
        "Light Cocoa",
        &[117, 86, 68],
    ),
    ThreadRef::new(
        BRAND,
        "1876",
        "Very Dark Brown",
        &[83, 60, 52],
    ),
    ThreadRef::new(
        BRAND,
        "1366",
        "Best Brown",
        &[62, 48, 44],
    ),
    ThreadRef::new(
        BRAND,
        "0811",
        "Gold",
        &[231, 170, 90],
    ),
    ThreadRef::new(
        BRAND,
        "0821",
        "Sun Gold",
        &[212, 151, 76],
    ),
    ThreadRef::new(
        BRAND,
        "0922",
        "Toast",
        &[174, 116, 58],
    ),
    ThreadRef::new(
        BRAND,
        "0940",
        "Brown",
        &[174, 114, 51],
    ),
    ThreadRef::new(
        BRAND,
        "0931",
        "Cocoa Brown",
        &[173, 104, 54],
    ),
    ThreadRef::new(
        BRAND,
        "0932",
        "Cocoa Brown",
        &[165, 99, 56],
    ),
    ThreadRef::new(
        BRAND,
        "1115",
        "Brown",
        &[159, 92, 58],
    ),
    ThreadRef::new(
        BRAND,
        "1342",
        "Date",
        &[137, 82, 58],
    ),
    ThreadRef::new(
        BRAND,
        "1355",
        "Brown",
        &[98, 62, 48],
    ),
    ThreadRef::new(
        BRAND,
        "1346",
        "Coffee Bean",
        &[94, 63, 51],
    ),
    ThreadRef::new(
        BRAND,
        "1312",
        "Sienna",
        &[167, 78, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1311",
        "Dark Rust",
        &[167, 91, 60],
    ),
    ThreadRef::new(
        BRAND,
        "1334",
        "Terra Cotta",
        &[152, 71, 46],
    ),
    ThreadRef::new(
        BRAND,
        "1514",
        "Terra Cotta",
        &[150, 66, 47],
    ),
    ThreadRef::new(
        BRAND,
        "0670",
        "Platinum",
        &[245, 244, 234],
    ),
    ThreadRef::new(
        BRAND,
        "0672",
        "Platinum",
        &[201, 188, 168],
    ),
    ThreadRef::new(
        BRAND,
        "0861",
        "Grape",
        &[204, 185, 163],
    ),
    ThreadRef::new(
        BRAND,
        "0873",
        "Khaki Green",
        &[167, 155, 139],
    ),
    ThreadRef::new(
        BRAND,
        "0862",
        "Grayrod",
        &[151, 136, 119],
    ),
    ThreadRef::new(
        BRAND,
        "0776",
        "Olive Drab",
        &[117, 108, 93],
    ),
    ThreadRef::new(
        BRAND,
        "1874",
        "Olive Drab",
        &[112, 95, 85],
    ),
    ThreadRef::new(
        BRAND,
        "1375",
        "Dogwood",
        &[94, 79, 72],
    ),
    ThreadRef::new(
        BRAND,
        "0184",
        "Light Silver Grey",
        &[222, 217, 220],
    ),
    ThreadRef::new(
        BRAND,
        "0151",
        "Saturn Gray",
        &[205, 191, 186],
    ),
    ThreadRef::new(
        BRAND,
        "0124",
        "Saturn Gray",
        &[201, 195, 192],
    ),
    ThreadRef::new(
        BRAND,
        "0152",
        "Silvery Gray",
        &[137, 126, 124],
    ),
    ThreadRef::new(
        BRAND,
        "0180",
        "Steel",
        &[230, 222, 225],
    ),
    ThreadRef::new(
        BRAND,
        "3251",
        "Steel Gray",
        &[155, 133, 158],
    ),
    ThreadRef::new(
        BRAND,
        "3062",
        "Silvery Gray",
        &[135, 120, 133],
    ),
    ThreadRef::new(
        BRAND,
        "0182",
        "Pearl Gray",
        &[214, 211, 218],
    ),
    ThreadRef::new(
        BRAND,
        "0150",
        "Stainless Steel",
        &[210, 202, 201],
    ),
    ThreadRef::new(
        BRAND,
        "0131",
        "Storm Gray",
        &[162, 157, 164],
    ),
    ThreadRef::new(
        BRAND,
        "2674",
        "Silvery Gray",
        &[136, 127, 139],
    ),
    ThreadRef::new(
        BRAND,
        "0112",
        "Charcoal",
        &[127, 114, 119],
    ),
    ThreadRef::new(
        BRAND,
        "2576",
        "Ducky Mauve",
        &[110, 94, 99],
    ),
    ThreadRef::new(
        BRAND,
        "2220",
        "Primrose",
        &[243, 117, 151],
    ),
    ThreadRef::new(
        BRAND,
        "2520",
        "Azalea",
        &[229, 99, 141],
    ),
    ThreadRef::new(
        BRAND,
        "2320",
        "Ruby Glint",
        &[200, 69, 91],
    ),
    ThreadRef::new(
        BRAND,
        "2300",
        "Dark Pink",
        &[196, 62, 90],
    ),
    ThreadRef::new(
        BRAND,
        "2521",
        "Dusky Rose",
        &[184, 60, 84],
    ),
    ThreadRef::new(
        BRAND,
        "2655",
        "Brownish Cream",
        &[236, 212, 233],
    ),
    ThreadRef::new(
        BRAND,
        "2650",
        "Orchid",
        &[230, 188, 219],
    ),
    ThreadRef::new(
        BRAND,
        "2764",
        "Satin Wine",
        &[177, 132, 160],
    ),
    ThreadRef::new(
        BRAND,
        "2500",
        "Maroon",
        &[146, 63, 93],
    ),
    ThreadRef::new(
        BRAND,
        "2506",
        "Wine",
        &[142, 57, 88],
    ),
    ThreadRef::new(
        BRAND,
        "3045",
        "Dusky Mauve",
        &[198, 159, 204],
    ),
    ThreadRef::new(
        BRAND,
        "2640",
        "Mid Lilac",
        &[196, 141, 191],
    ),
    ThreadRef::new(
        BRAND,
        "2504",
        "Purple",
        &[153, 74, 125],
    ),
    ThreadRef::new(
        BRAND,
        "2720",
        "Maroon",
        &[127, 64, 105],
    ),
    ThreadRef::new(
        BRAND,
        "2600",
        "Maroon",
        &[125, 66, 103],
    ),
    ThreadRef::new(
        BRAND,
        "2711",
        "Intense Maroon",
        &[103, 51, 84],
    ),
    ThreadRef::new(
        BRAND,
        "3040",
        "Purple",
        &[197, 165, 211],
    ),
    ThreadRef::new(
        BRAND,
        "2830",
        "Lilac",
        &[155, 108, 167],
    ),
    ThreadRef::new(
        BRAND,
        "2810",
        "Raspberry",
        &[126, 73, 125],
    ),
    ThreadRef::new(
        BRAND,
        "2715",
        "Intense Maroon",
        &[94, 47, 81],
    ),
    ThreadRef::new(
        BRAND,
        "3536",
        "Dark Gray",
        &[71, 46, 70],
    ),
    ThreadRef::new(
        BRAND,
        "5650",
        "Aquamarine",
        &[204, 228, 175],
    ),
    ThreadRef::new(
        BRAND,
        "5610",
        "Peapod",
        &[148, 187, 106],
    ),
    ThreadRef::new(
        BRAND,
        "5531",
        "Erin Green",
        &[122, 166, 94],
    ),
    ThreadRef::new(
        BRAND,
        "5513",
        "Meadow",
        &[89, 136, 54],
    ),
    ThreadRef::new(
        BRAND,
        "6051",
        "Green Dust",
        &[196, 209, 141],
    ),
    ThreadRef::new(
        BRAND,
        "5912",
        "Medium Olive Green",
        &[138, 186, 62],
    ),
    ThreadRef::new(
        BRAND,
        "5510",
        "Sage",
        &[92, 152, 73],
    ),
    ThreadRef::new(
        BRAND,
        "5613",
        "Sage",
        &[101, 159, 74],
    ),
    ThreadRef::new(
        BRAND,
        "5411",
        "Light Kelly",
        &[76, 131, 73],
    ),
    ThreadRef::new(
        BRAND,
        "5515",
        "Green Grass",
        &[67, 132, 76],
    ),
    ThreadRef::new(
        BRAND,
        "5415",
        "Dark Emerald",
        &[54, 113, 59],
    ),
    ThreadRef::new(
        BRAND,
        "5422",
        "Dark Emerald",
        &[56, 109, 67],
    ),
    ThreadRef::new(
        BRAND,
        "5100",
        "Steel Blue",
        &[54, 113, 88],
    ),
    ThreadRef::new(
        BRAND,
        "5233",
        "Deep Green",
        &[88, 104, 78],
    ),
    ThreadRef::new(
        BRAND,
        "5335",
        "Ivy",
        &[69, 77, 56],
    ),
    ThreadRef::new(
        BRAND,
        "5326",
        "Holly",
        &[68, 80, 54],
    ),
    ThreadRef::new(
        BRAND,
        "5374",
        "Olive",
        &[60, 67, 49],
    ),
    ThreadRef::new(
        BRAND,
        "5664",
        "Olive",
        &[118, 130, 99],
    ),
    ThreadRef::new(
        BRAND,
        "5552",
        "Grayrod",
        &[146, 149, 120],
    ),
    ThreadRef::new(
        BRAND,
        "0463",
        "Willow",
        &[138, 137, 99],
    ),
    ThreadRef::new(
        BRAND,
        "0453",
        "Peapod",
        &[164, 161, 108],
    ),
    ThreadRef::new(
        BRAND,
        "3770",
        "Light Silver Grey",
        &[217, 218, 219],
    ),
    ThreadRef::new(
        BRAND,
        "3971",
        "Light Grey",
        &[203, 201, 206],
    ),
    ThreadRef::new(
        BRAND,
        "0142",
        "Grey",
        &[186, 181, 184],
    ),
    ThreadRef::new(
        BRAND,
        "1972",
        "Storm Gray",
        &[168, 154, 160],
    ),
    ThreadRef::new(
        BRAND,
        "4073",
        "GS Gray",
        &[146, 142, 144],
    ),
    ThreadRef::new(
        BRAND,
        "0108",
        "Silvery Gray",
        &[141, 129, 133],
    ),
    ThreadRef::new(
        BRAND,
        "0111",
        "Twilight",
        &[115, 105, 104],
    ),
    ThreadRef::new(
        BRAND,
        "0132",
        "Palm Leaf",
        &[84, 78, 84],
    ),
    ThreadRef::new(
        BRAND,
        "4174",
        "Intense Maroon",
        &[77, 67, 66],
    ),
    ThreadRef::new(
        BRAND,
        "0020",
        "Mahogany",
        &[51, 48, 46],
    ),
];