#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "ARC Polyester - older";
const THREADS: [ThreadRef; 288] = [
    ThreadRef {
        brand: BRAND,
        code: "10",
        name: "Palest Blue",
        color: &[236, 244, 251],
    },
    ThreadRef {
        brand: BRAND,
        code: "101",
        name: "Chrome",
        color: &[203, 209, 194],
    },
    ThreadRef {
        brand: BRAND,
        code: "102",
        name: "Grey",
        color: &[174, 185, 182],
    },
    ThreadRef {
        brand: BRAND,
        code: "1031",
        name: "Dark Blue",
        color: &[75, 72, 132],
    },
    ThreadRef {
        brand: BRAND,
        code: "104",
        name: "Solar Blue",
        color: &[26, 102, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "107",
        name: "Grey",
        color: &[178, 191, 189],
    },
    ThreadRef {
        brand: BRAND,
        code: "109",
        name: "Sea Green",
        color: &[30, 158, 137],
    },
    ThreadRef {
        brand: BRAND,
        code: "111",
        name: "Moss",
        color: &[155, 169, 165],
    },
    ThreadRef {
        brand: BRAND,
        code: "112",
        name: "Silvery Gray",
        color: &[136, 145, 134],
    },
    ThreadRef {
        brand: BRAND,
        code: "114",
        name: "Grey Blue",
        color: &[108, 124, 126],
    },
    ThreadRef {
        brand: BRAND,
        code: "1140",
        name: "Palest Ivory",
        color: &[237, 237, 210],
    },
    ThreadRef {
        brand: BRAND,
        code: "1141",
        name: "Medium Grey Brown",
        color: &[189, 185, 162],
    },
    ThreadRef {
        brand: BRAND,
        code: "1145",
        name: "Gold",
        color: &[227, 196, 152],
    },
    ThreadRef {
        brand: BRAND,
        code: "1146",
        name: "Beige",
        color: &[211, 200, 165],
    },
    ThreadRef {
        brand: BRAND,
        code: "1147",
        name: "Medium Grey Brown",
        color: &[193, 185, 157],
    },
    ThreadRef {
        brand: BRAND,
        code: "1148",
        name: "Medium Grey Brown",
        color: &[199, 186, 154],
    },
    ThreadRef {
        brand: BRAND,
        code: "1149",
        name: "Medium Grey",
        color: &[163, 162, 142],
    },
    ThreadRef {
        brand: BRAND,
        code: "115",
        name: "Charcoal",
        color: &[115, 127, 127],
    },
    ThreadRef {
        brand: BRAND,
        code: "1152",
        name: "Dark Taupe",
        color: &[95, 78, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "116",
        name: "Dark Gray",
        color: &[86, 94, 90],
    },
    ThreadRef {
        brand: BRAND,
        code: "1160",
        name: "Pale Apricot",
        color: &[253, 219, 185],
    },
    ThreadRef {
        brand: BRAND,
        code: "1163",
        name: "Marine Blue",
        color: &[55, 112, 164],
    },
    ThreadRef {
        brand: BRAND,
        code: "117",
        name: "Dark Grey",
        color: &[81, 82, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "118",
        name: "Charcoal",
        color: &[120, 118, 104],
    },
    ThreadRef {
        brand: BRAND,
        code: "1183",
        name: "Acid Green",
        color: &[127, 217, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "1240",
        name: "Red Brown",
        color: &[167, 59, 65],
    },
    ThreadRef {
        brand: BRAND,
        code: "1241",
        name: "Red Brown",
        color: &[141, 63, 67],
    },
    ThreadRef {
        brand: BRAND,
        code: "1243",
        name: "Sepia",
        color: &[115, 59, 61],
    },
    ThreadRef {
        brand: BRAND,
        code: "1313",
        name: "Grey Blue",
        color: &[130, 98, 139],
    },
    ThreadRef {
        brand: BRAND,
        code: "1323",
        name: "Ducky Mauve",
        color: &[131, 84, 120],
    },
    ThreadRef {
        brand: BRAND,
        code: "1324",
        name: "Silver Blue",
        color: &[129, 116, 159],
    },
    ThreadRef {
        brand: BRAND,
        code: "1331",
        name: "Steel Blue",
        color: &[83, 85, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "134",
        name: "Tangerine",
        color: &[227, 103, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "135",
        name: "Tangerine",
        color: &[243, 89, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "138",
        name: "Sea Green",
        color: &[46, 165, 155],
    },
    ThreadRef {
        brand: BRAND,
        code: "1386",
        name: "Dark Blue",
        color: &[72, 112, 114],
    },
    ThreadRef {
        brand: BRAND,
        code: "142",
        name: "Dark Blue",
        color: &[57, 98, 118],
    },
    ThreadRef {
        brand: BRAND,
        code: "1423",
        name: "Marine Blue",
        color: &[48, 108, 166],
    },
    ThreadRef {
        brand: BRAND,
        code: "146",
        name: "Brown",
        color: &[177, 109, 70],
    },
    ThreadRef {
        brand: BRAND,
        code: "15",
        name: "Palest Blue",
        color: &[240, 247, 248],
    },
    ThreadRef {
        brand: BRAND,
        code: "1520",
        name: "Khaki Green",
        color: &[150, 139, 119],
    },
    ThreadRef {
        brand: BRAND,
        code: "1527",
        name: "Mushroom",
        color: &[113, 77, 61],
    },
    ThreadRef {
        brand: BRAND,
        code: "1545",
        name: "Brown",
        color: &[126, 92, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "1552",
        name: "Burnished Brown",
        color: &[182, 163, 108],
    },
    ThreadRef {
        brand: BRAND,
        code: "1615",
        name: "Slate Green",
        color: &[46, 159, 117],
    },
    ThreadRef {
        brand: BRAND,
        code: "1619",
        name: "Light Leaf Green",
        color: &[152, 193, 115],
    },
    ThreadRef {
        brand: BRAND,
        code: "165",
        name: "Palest Ivory",
        color: &[240, 238, 204],
    },
    ThreadRef {
        brand: BRAND,
        code: "1707",
        name: "Silver Grey",
        color: &[192, 203, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "1708",
        name: "Palm Leaf",
        color: &[181, 191, 184],
    },
    ThreadRef {
        brand: BRAND,
        code: "1710",
        name: "Grey",
        color: &[165, 176, 171],
    },
    ThreadRef {
        brand: BRAND,
        code: "1713",
        name: "Silvery Gray",
        color: &[141, 147, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "1716",
        name: "Pewter",
        color: &[109, 110, 115],
    },
    ThreadRef {
        brand: BRAND,
        code: "187",
        name: "Red Brown",
        color: &[168, 61, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "190",
        name: "Bronze",
        color: &[194, 71, 85],
    },
    ThreadRef {
        brand: BRAND,
        code: "20",
        name: "Charcoal",
        color: &[59, 60, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "2031",
        name: "Dark Blue",
        color: &[55, 73, 124],
    },
    ThreadRef {
        brand: BRAND,
        code: "2093",
        name: "Oceanic Green",
        color: &[0, 128, 168],
    },
    ThreadRef {
        brand: BRAND,
        code: "21",
        name: "Lime",
        color: &[241, 255, 82],
    },
    ThreadRef {
        brand: BRAND,
        code: "213",
        name: "Red Brown",
        color: &[155, 59, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "216",
        name: "Intense Maroon",
        color: &[108, 62, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "2250",
        name: "Brown",
        color: &[131, 68, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "240",
        name: "Dark Grey",
        color: &[76, 91, 65],
    },
    ThreadRef {
        brand: BRAND,
        code: "247",
        name: "Palm Leaf",
        color: &[72, 80, 82],
    },
    ThreadRef {
        brand: BRAND,
        code: "2518",
        name: "Brownish Pink",
        color: &[201, 173, 127],
    },
    ThreadRef {
        brand: BRAND,
        code: "2519",
        name: "Pistachio",
        color: &[186, 156, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "2526",
        name: "Burnished Brown",
        color: &[197, 161, 108],
    },
    ThreadRef {
        brand: BRAND,
        code: "253",
        name: "Light Brown",
        color: &[186, 110, 77],
    },
    ThreadRef {
        brand: BRAND,
        code: "255",
        name: "Sienna",
        color: &[165, 88, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "266",
        name: "Dark Rust",
        color: &[192, 70, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "286",
        name: "Gold Orange",
        color: &[251, 191, 73],
    },
    ThreadRef {
        brand: BRAND,
        code: "3001",
        name: "Copper",
        color: &[234, 129, 76],
    },
    ThreadRef {
        brand: BRAND,
        code: "301",
        name: "Very Pale Pink",
        color: &[252, 225, 209],
    },
    ThreadRef {
        brand: BRAND,
        code: "3014",
        name: "Blush",
        color: &[226, 140, 115],
    },
    ThreadRef {
        brand: BRAND,
        code: "3015",
        name: "Dark Rust",
        color: &[180, 58, 57],
    },
    ThreadRef {
        brand: BRAND,
        code: "3016",
        name: "Burnt Orange",
        color: &[207, 76, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "302",
        name: "Pink",
        color: &[255, 206, 210],
    },
    ThreadRef {
        brand: BRAND,
        code: "303",
        name: "Very Pale Pink",
        color: &[251, 222, 214],
    },
    ThreadRef {
        brand: BRAND,
        code: "304",
        name: "Pale Pink",
        color: &[246, 202, 213],
    },
    ThreadRef {
        brand: BRAND,
        code: "305",
        name: "Rose",
        color: &[242, 172, 179],
    },
    ThreadRef {
        brand: BRAND,
        code: "306",
        name: "Carnation",
        color: &[233, 176, 186],
    },
    ThreadRef {
        brand: BRAND,
        code: "307",
        name: "Dusty Rose",
        color: &[255, 161, 181],
    },
    ThreadRef {
        brand: BRAND,
        code: "309",
        name: "Dusky Rose",
        color: &[231, 127, 157],
    },
    ThreadRef {
        brand: BRAND,
        code: "313",
        name: "Light Cerise",
        color: &[240, 111, 140],
    },
    ThreadRef {
        brand: BRAND,
        code: "3142",
        name: "Brown",
        color: &[143, 95, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "315",
        name: "Peony Purple",
        color: &[227, 92, 120],
    },
    ThreadRef {
        brand: BRAND,
        code: "317",
        name: "Medium Leaf Green",
        color: &[0, 131, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "32",
        name: "Fresh Green",
        color: &[150, 232, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "321",
        name: "Pale Pink",
        color: &[223, 152, 181],
    },
    ThreadRef {
        brand: BRAND,
        code: "324",
        name: "Passion Pink",
        color: &[203, 105, 143],
    },
    ThreadRef {
        brand: BRAND,
        code: "325",
        name: "Dark Rose",
        color: &[183, 82, 122],
    },
    ThreadRef {
        brand: BRAND,
        code: "33",
        name: "Yellow",
        color: &[255, 233, 98],
    },
    ThreadRef {
        brand: BRAND,
        code: "332",
        name: "Dark Rose",
        color: &[189, 83, 121],
    },
    ThreadRef {
        brand: BRAND,
        code: "3325",
        name: "Dark Slate Green",
        color: &[54, 125, 103],
    },
    ThreadRef {
        brand: BRAND,
        code: "333",
        name: "Burgundy Rose",
        color: &[177, 64, 91],
    },
    ThreadRef {
        brand: BRAND,
        code: "343",
        name: "Sterling",
        color: &[169, 152, 178],
    },
    ThreadRef {
        brand: BRAND,
        code: "345",
        name: "Dusky Mauve",
        color: &[194, 147, 173],
    },
    ThreadRef {
        brand: BRAND,
        code: "347",
        name: "Lilac",
        color: &[167, 109, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "348",
        name: "Imperial Purple",
        color: &[103, 64, 103],
    },
    ThreadRef {
        brand: BRAND,
        code: "361",
        name: "Intense Maroon",
        color: &[106, 60, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "362",
        name: "Intense Maroon",
        color: &[89, 65, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "363",
        name: "Brown",
        color: &[117, 71, 79],
    },
    ThreadRef {
        brand: BRAND,
        code: "376",
        name: "Brownish Cream",
        color: &[230, 207, 213],
    },
    ThreadRef {
        brand: BRAND,
        code: "379",
        name: "Pale Sky",
        color: &[168, 190, 213],
    },
    ThreadRef {
        brand: BRAND,
        code: "380",
        name: "Aqua",
        color: &[160, 191, 215],
    },
    ThreadRef {
        brand: BRAND,
        code: "381",
        name: "Powder Blue",
        color: &[144, 166, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "382",
        name: "Rockport Blue",
        color: &[144, 174, 202],
    },
    ThreadRef {
        brand: BRAND,
        code: "383",
        name: "Paris Blue",
        color: &[177, 184, 211],
    },
    ThreadRef {
        brand: BRAND,
        code: "385",
        name: "Slate Blue",
        color: &[67, 111, 157],
    },
    ThreadRef {
        brand: BRAND,
        code: "386",
        name: "Palm Leaf",
        color: &[153, 144, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "387",
        name: "Dover Gray",
        color: &[210, 195, 204],
    },
    ThreadRef {
        brand: BRAND,
        code: "388",
        name: "Pale Violet",
        color: &[170, 166, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "390",
        name: "Mauve",
        color: &[124, 104, 171],
    },
    ThreadRef {
        brand: BRAND,
        code: "392",
        name: "Grey Blue",
        color: &[102, 80, 143],
    },
    ThreadRef {
        brand: BRAND,
        code: "398",
        name: "Imperial Purple",
        color: &[93, 74, 110],
    },
    ThreadRef {
        brand: BRAND,
        code: "4004",
        name: "Aqua",
        color: &[157, 200, 214],
    },
    ThreadRef {
        brand: BRAND,
        code: "402",
        name: "Light Blue",
        color: &[175, 203, 207],
    },
    ThreadRef {
        brand: BRAND,
        code: "403",
        name: "Aqua",
        color: &[150, 200, 217],
    },
    ThreadRef {
        brand: BRAND,
        code: "404",
        name: "Country Blue",
        color: &[125, 171, 187],
    },
    ThreadRef {
        brand: BRAND,
        code: "405",
        name: "Sea Green",
        color: &[96, 153, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "406",
        name: "Powder Blue",
        color: &[127, 174, 202],
    },
    ThreadRef {
        brand: BRAND,
        code: "409",
        name: "Sea Green",
        color: &[57, 129, 163],
    },
    ThreadRef {
        brand: BRAND,
        code: "4117",
        name: "Sunflower",
        color: &[255, 197, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "412",
        name: "Olive Drab",
        color: &[159, 138, 103],
    },
    ThreadRef {
        brand: BRAND,
        code: "413",
        name: "Intense Blue",
        color: &[0, 102, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "414",
        name: "Teal Blue",
        color: &[32, 99, 145],
    },
    ThreadRef {
        brand: BRAND,
        code: "415",
        name: "Dark Blue",
        color: &[51, 76, 105],
    },
    ThreadRef {
        brand: BRAND,
        code: "416",
        name: "Palm Leaf",
        color: &[61, 67, 91],
    },
    ThreadRef {
        brand: BRAND,
        code: "417",
        name: "Slate Blue",
        color: &[62, 105, 156],
    },
    ThreadRef {
        brand: BRAND,
        code: "419",
        name: "Yellow",
        color: &[255, 210, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "42",
        name: "Yellow Ocher",
        color: &[255, 201, 76],
    },
    ThreadRef {
        brand: BRAND,
        code: "422",
        name: "Palm Leaf",
        color: &[58, 65, 79],
    },
    ThreadRef {
        brand: BRAND,
        code: "423",
        name: "Dark Gray",
        color: &[58, 66, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "43",
        name: "Mimosa",
        color: &[255, 185, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "432",
        name: "Gold Orange",
        color: &[250, 176, 68],
    },
    ThreadRef {
        brand: BRAND,
        code: "4371",
        name: "Khaki Green",
        color: &[168, 151, 129],
    },
    ThreadRef {
        brand: BRAND,
        code: "4419",
        name: "Umber",
        color: &[81, 178, 187],
    },
    ThreadRef {
        brand: BRAND,
        code: "442",
        name: "Aquamarine",
        color: &[192, 233, 218],
    },
    ThreadRef {
        brand: BRAND,
        code: "443",
        name: "Cyan",
        color: &[0, 153, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "444",
        name: "Umber",
        color: &[78, 182, 193],
    },
    ThreadRef {
        brand: BRAND,
        code: "445",
        name: "Kingfisher",
        color: &[36, 156, 187],
    },
    ThreadRef {
        brand: BRAND,
        code: "4453",
        name: "Solar Blue",
        color: &[30, 96, 149],
    },
    ThreadRef {
        brand: BRAND,
        code: "446",
        name: "Umber",
        color: &[85, 187, 200],
    },
    ThreadRef {
        brand: BRAND,
        code: "447",
        name: "Emerald",
        color: &[0, 142, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "448",
        name: "Olive",
        color: &[67, 74, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "449",
        name: "Dark Olive Green",
        color: &[0, 115, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "450",
        name: "Emerald",
        color: &[0, 140, 110],
    },
    ThreadRef {
        brand: BRAND,
        code: "451",
        name: "Lido",
        color: &[49, 135, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "455",
        name: "Steel Blue",
        color: &[87, 120, 104],
    },
    ThreadRef {
        brand: BRAND,
        code: "46",
        name: "Dusky Rose",
        color: &[255, 133, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "4627",
        name: "Emerald",
        color: &[0, 135, 122],
    },
    ThreadRef {
        brand: BRAND,
        code: "466",
        name: "Gold",
        color: &[229, 178, 92],
    },
    ThreadRef {
        brand: BRAND,
        code: "47",
        name: "Dark Peach",
        color: &[255, 118, 95],
    },
    ThreadRef {
        brand: BRAND,
        code: "4735",
        name: "Palm Leaf",
        color: &[48, 85, 79],
    },
    ThreadRef {
        brand: BRAND,
        code: "501",
        name: "Opaline",
        color: &[245, 209, 185],
    },
    ThreadRef {
        brand: BRAND,
        code: "502",
        name: "Very Pale Peach",
        color: &[238, 197, 174],
    },
    ThreadRef {
        brand: BRAND,
        code: "503",
        name: "Very Pale Peach",
        color: &[242, 197, 173],
    },
    ThreadRef {
        brand: BRAND,
        code: "504",
        name: "Peach",
        color: &[255, 190, 172],
    },
    ThreadRef {
        brand: BRAND,
        code: "505",
        name: "Rose Pink",
        color: &[253, 174, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "506",
        name: "Salmon",
        color: &[238, 153, 142],
    },
    ThreadRef {
        brand: BRAND,
        code: "508",
        name: "Salmon Pink",
        color: &[250, 166, 129],
    },
    ThreadRef {
        brand: BRAND,
        code: "513",
        name: "Dark Taupe",
        color: &[106, 77, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "520",
        name: "Gold",
        color: &[233, 146, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "525",
        name: "Copper",
        color: &[227, 123, 85],
    },
    ThreadRef {
        brand: BRAND,
        code: "526",
        name: "Marigold",
        color: &[224, 93, 67],
    },
    ThreadRef {
        brand: BRAND,
        code: "527",
        name: "Persimmon",
        color: &[221, 88, 81],
    },
    ThreadRef {
        brand: BRAND,
        code: "528",
        name: "Dark Rust",
        color: &[186, 70, 54],
    },
    ThreadRef {
        brand: BRAND,
        code: "529",
        name: "Dark Rust",
        color: &[182, 70, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "530",
        name: "Burgundy",
        color: &[157, 61, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "531",
        name: "Wine Red",
        color: &[144, 67, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "54",
        name: "Dark Rose",
        color: &[184, 81, 110],
    },
    ThreadRef {
        brand: BRAND,
        code: "541",
        name: "Steel Blue",
        color: &[106, 135, 149],
    },
    ThreadRef {
        brand: BRAND,
        code: "5550",
        name: "Dark Blue",
        color: &[59, 72, 109],
    },
    ThreadRef {
        brand: BRAND,
        code: "5551",
        name: "Dark Blue",
        color: &[55, 77, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "5552",
        name: "Midnight",
        color: &[62, 66, 77],
    },
    ThreadRef {
        brand: BRAND,
        code: "5553",
        name: "Palm Leaf",
        color: &[56, 64, 89],
    },
    ThreadRef {
        brand: BRAND,
        code: "5554",
        name: "Medium Blue",
        color: &[118, 175, 203],
    },
    ThreadRef {
        brand: BRAND,
        code: "5555",
        name: "Emerald",
        color: &[0, 153, 175],
    },
    ThreadRef {
        brand: BRAND,
        code: "5556",
        name: "Steel Blue",
        color: &[67, 90, 106],
    },
    ThreadRef {
        brand: BRAND,
        code: "5557",
        name: "Sage",
        color: &[64, 155, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "5558",
        name: "Medium Brown",
        color: &[119, 104, 82],
    },
    ThreadRef {
        brand: BRAND,
        code: "5559",
        name: "Light Silver Grey",
        color: &[208, 210, 204],
    },
    ThreadRef {
        brand: BRAND,
        code: "571",
        name: "Red Brown",
        color: &[154, 65, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "5829",
        name: "Palm Leaf",
        color: &[198, 200, 186],
    },
    ThreadRef {
        brand: BRAND,
        code: "585",
        name: "Charcoal",
        color: &[118, 119, 124],
    },
    ThreadRef {
        brand: BRAND,
        code: "588",
        name: "Medium Grey",
        color: &[150, 158, 156],
    },
    ThreadRef {
        brand: BRAND,
        code: "589",
        name: "Silvery Gray",
        color: &[126, 137, 142],
    },
    ThreadRef {
        brand: BRAND,
        code: "601",
        name: "Blond",
        color: &[245, 222, 143],
    },
    ThreadRef {
        brand: BRAND,
        code: "602",
        name: "Daffodil",
        color: &[248, 210, 114],
    },
    ThreadRef {
        brand: BRAND,
        code: "604",
        name: "Star Gold",
        color: &[247, 215, 72],
    },
    ThreadRef {
        brand: BRAND,
        code: "605",
        name: "Gold",
        color: &[241, 206, 88],
    },
    ThreadRef {
        brand: BRAND,
        code: "609",
        name: "Gold Orange",
        color: &[226, 180, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "612",
        name: "Old Gold",
        color: &[255, 215, 125],
    },
    ThreadRef {
        brand: BRAND,
        code: "613",
        name: "Pebblestone",
        color: &[238, 217, 134],
    },
    ThreadRef {
        brand: BRAND,
        code: "6137",
        name: "Aqua",
        color: &[168, 202, 217],
    },
    ThreadRef {
        brand: BRAND,
        code: "616",
        name: "Wicker",
        color: &[209, 172, 73],
    },
    ThreadRef {
        brand: BRAND,
        code: "619",
        name: "Wicker",
        color: &[206, 158, 89],
    },
    ThreadRef {
        brand: BRAND,
        code: "620",
        name: "Wicker",
        color: &[191, 155, 84],
    },
    ThreadRef {
        brand: BRAND,
        code: "621",
        name: "Cocoa Mulch",
        color: &[176, 138, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "624",
        name: "Brown",
        color: &[179, 119, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "627",
        name: "Celery",
        color: &[247, 230, 185],
    },
    ThreadRef {
        brand: BRAND,
        code: "628",
        name: "Burnished Brown",
        color: &[185, 163, 129],
    },
    ThreadRef {
        brand: BRAND,
        code: "632",
        name: "Pale Yellow",
        color: &[244, 238, 128],
    },
    ThreadRef {
        brand: BRAND,
        code: "633",
        name: "Bright Yellow",
        color: &[247, 225, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "635",
        name: "Bright Yellow",
        color: &[250, 227, 55],
    },
    ThreadRef {
        brand: BRAND,
        code: "641",
        name: "Gold Orange",
        color: &[246, 188, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "642",
        name: "Penny",
        color: &[244, 178, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "646",
        name: "Orange",
        color: &[252, 163, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "649",
        name: "Orange",
        color: &[248, 149, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "650",
        name: "Tangerine",
        color: &[229, 100, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "651",
        name: "Rust",
        color: &[233, 108, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "652",
        name: "Old Gold",
        color: &[197, 146, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "653",
        name: "Burnished Brown",
        color: &[166, 164, 121],
    },
    ThreadRef {
        brand: BRAND,
        code: "654",
        name: "Old Gold",
        color: &[193, 136, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "655",
        name: "Dark Olive Green",
        color: &[97, 96, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "675",
        name: "Charcoal",
        color: &[117, 126, 120],
    },
    ThreadRef {
        brand: BRAND,
        code: "688",
        name: "Emerald",
        color: &[0, 147, 142],
    },
    ThreadRef {
        brand: BRAND,
        code: "695",
        name: "Dark Teal Blue",
        color: &[41, 95, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "697",
        name: "Oceanic Green",
        color: &[0, 120, 166],
    },
    ThreadRef {
        brand: BRAND,
        code: "700",
        name: "Dark Rust",
        color: &[185, 64, 54],
    },
    ThreadRef {
        brand: BRAND,
        code: "763",
        name: "Sunflower",
        color: &[248, 190, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "777",
        name: "Emerald Green",
        color: &[0, 139, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "8010",
        name: "Silvery Gray",
        color: &[134, 142, 140],
    },
    ThreadRef {
        brand: BRAND,
        code: "806",
        name: "Dark Blue",
        color: &[51, 80, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "809",
        name: "Steel Blue",
        color: &[67, 126, 176],
    },
    ThreadRef {
        brand: BRAND,
        code: "811",
        name: "Palest Ivory",
        color: &[240, 242, 216],
    },
    ThreadRef {
        brand: BRAND,
        code: "812",
        name: "Cream",
        color: &[234, 222, 188],
    },
    ThreadRef {
        brand: BRAND,
        code: "814",
        name: "Camel",
        color: &[220, 204, 168],
    },
    ThreadRef {
        brand: BRAND,
        code: "815",
        name: "Pale Brown",
        color: &[210, 192, 149],
    },
    ThreadRef {
        brand: BRAND,
        code: "818",
        name: "Pale Brown",
        color: &[217, 193, 157],
    },
    ThreadRef {
        brand: BRAND,
        code: "819",
        name: "Cinnamon",
        color: &[177, 153, 131],
    },
    ThreadRef {
        brand: BRAND,
        code: "825",
        name: "Emerald",
        color: &[0, 139, 111],
    },
    ThreadRef {
        brand: BRAND,
        code: "828",
        name: "Light Khaki Green",
        color: &[227, 237, 207],
    },
    ThreadRef {
        brand: BRAND,
        code: "829",
        name: "Palm Leaf",
        color: &[187, 195, 181],
    },
    ThreadRef {
        brand: BRAND,
        code: "830",
        name: "Brownish Pink",
        color: &[205, 171, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "831",
        name: "Cinnamon",
        color: &[216, 162, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "832",
        name: "Cocoa Mulch",
        color: &[190, 135, 109],
    },
    ThreadRef {
        brand: BRAND,
        code: "833",
        name: "Cocoa Mulch",
        color: &[170, 126, 96],
    },
    ThreadRef {
        brand: BRAND,
        code: "836",
        name: "Silvery Gray",
        color: &[148, 135, 117],
    },
    ThreadRef {
        brand: BRAND,
        code: "838",
        name: "Wine Red",
        color: &[148, 76, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "839",
        name: "Brown",
        color: &[156, 87, 68],
    },
    ThreadRef {
        brand: BRAND,
        code: "840",
        name: "Reddish Brown",
        color: &[135, 74, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "841",
        name: "Brown",
        color: &[133, 88, 55],
    },
    ThreadRef {
        brand: BRAND,
        code: "842",
        name: "Pistachio",
        color: &[177, 152, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "843",
        name: "Burnished Brown",
        color: &[176, 148, 91],
    },
    ThreadRef {
        brand: BRAND,
        code: "845",
        name: "Brown",
        color: &[125, 120, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "854",
        name: "Brown",
        color: &[133, 102, 72],
    },
    ThreadRef {
        brand: BRAND,
        code: "857",
        name: "Medium Brown",
        color: &[120, 97, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "858",
        name: "Dark Taupe",
        color: &[104, 78, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "859",
        name: "Dark Taupe",
        color: &[111, 81, 67],
    },
    ThreadRef {
        brand: BRAND,
        code: "862",
        name: "Brownish Pink",
        color: &[197, 155, 147],
    },
    ThreadRef {
        brand: BRAND,
        code: "864",
        name: "Cocoa Mulch",
        color: &[170, 123, 114],
    },
    ThreadRef {
        brand: BRAND,
        code: "867",
        name: "Medium Brown",
        color: &[159, 110, 99],
    },
    ThreadRef {
        brand: BRAND,
        code: "873",
        name: "Willow",
        color: &[138, 123, 107],
    },
    ThreadRef {
        brand: BRAND,
        code: "878",
        name: "Dark Olive Green",
        color: &[115, 92, 72],
    },
    ThreadRef {
        brand: BRAND,
        code: "888",
        name: "Brown",
        color: &[118, 85, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "890",
        name: "Dark Taupe",
        color: &[104, 80, 68],
    },
    ThreadRef {
        brand: BRAND,
        code: "891",
        name: "Dark Grey",
        color: &[81, 73, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "892",
        name: "Dark Grey",
        color: &[84, 74, 68],
    },
    ThreadRef {
        brand: BRAND,
        code: "903",
        name: "Pale Aqua",
        color: &[147, 212, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "904",
        name: "Pale Aqua",
        color: &[148, 211, 187],
    },
    ThreadRef {
        brand: BRAND,
        code: "905",
        name: "Light Brown",
        color: &[174, 133, 72],
    },
    ThreadRef {
        brand: BRAND,
        code: "906",
        name: "Emerald Green",
        color: &[62, 173, 158],
    },
    ThreadRef {
        brand: BRAND,
        code: "907",
        name: "Umber",
        color: &[91, 185, 169],
    },
    ThreadRef {
        brand: BRAND,
        code: "909",
        name: "Emerald Green",
        color: &[82, 179, 151],
    },
    ThreadRef {
        brand: BRAND,
        code: "913",
        name: "Deep Atlantic",
        color: &[30, 97, 106],
    },
    ThreadRef {
        brand: BRAND,
        code: "944",
        name: "Seashell",
        color: &[214, 225, 182],
    },
    ThreadRef {
        brand: BRAND,
        code: "945",
        name: "Opal Green",
        color: &[207, 226, 189],
    },
    ThreadRef {
        brand: BRAND,
        code: "947",
        name: "Light Leaf Green",
        color: &[174, 215, 168],
    },
    ThreadRef {
        brand: BRAND,
        code: "949",
        name: "Emerald Green",
        color: &[83, 174, 121],
    },
    ThreadRef {
        brand: BRAND,
        code: "950",
        name: "Pistachio",
        color: &[159, 170, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "951",
        name: "Khaki Green",
        color: &[155, 153, 85],
    },
    ThreadRef {
        brand: BRAND,
        code: "952",
        name: "Khaki Green",
        color: &[156, 138, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "953",
        name: "Khaki Green",
        color: &[138, 134, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "955",
        name: "Dark Olive Green",
        color: &[97, 101, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "956",
        name: "Brown",
        color: &[117, 113, 68],
    },
    ThreadRef {
        brand: BRAND,
        code: "961",
        name: "Aqua",
        color: &[135, 188, 166],
    },
    ThreadRef {
        brand: BRAND,
        code: "962",
        name: "Palest Green",
        color: &[152, 171, 134],
    },
    ThreadRef {
        brand: BRAND,
        code: "963",
        name: "Grey Blue",
        color: &[122, 142, 104],
    },
    ThreadRef {
        brand: BRAND,
        code: "965",
        name: "Steel Blue",
        color: &[98, 136, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "966",
        name: "Grey Blue",
        color: &[98, 127, 126],
    },
    ThreadRef {
        brand: BRAND,
        code: "982",
        name: "Light Khaki Green",
        color: &[204, 184, 109],
    },
    ThreadRef {
        brand: BRAND,
        code: "983",
        name: "Light Khaki Green",
        color: &[184, 198, 107],
    },
    ThreadRef {
        brand: BRAND,
        code: "984",
        name: "Light Leaf Green",
        color: &[185, 222, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "985",
        name: "Light Khaki Green",
        color: &[192, 232, 124],
    },
    ThreadRef {
        brand: BRAND,
        code: "988",
        name: "Christmas Green",
        color: &[115, 181, 78],
    },
    ThreadRef {
        brand: BRAND,
        code: "990",
        name: "Sage",
        color: &[53, 143, 72],
    },
    ThreadRef {
        brand: BRAND,
        code: "992",
        name: "Jade",
        color: &[37, 117, 72],
    },
    ThreadRef {
        brand: BRAND,
        code: "995",
        name: "Teal",
        color: &[67, 100, 72],
    },
];