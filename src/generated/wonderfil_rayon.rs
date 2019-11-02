#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Wonderfil Rayon";
pub const THREADS: [ThreadRef; 342] = [
    ThreadRef::new(
        BRAND,
        "1101",
        "Light Pink",
        &[248, 232, 232],
    ),
    ThreadRef::new(
        BRAND,
        "1102",
        "Soft Pink",
        &[248, 219, 223],
    ),
    ThreadRef::new(
        BRAND,
        "1103",
        "Strawberry Cream",
        &[253, 160, 171],
    ),
    ThreadRef::new(
        BRAND,
        "1104",
        "Pink Mist",
        &[254, 134, 161],
    ),
    ThreadRef::new(
        BRAND,
        "1105",
        "Med Rose",
        &[238, 103, 136],
    ),
    ThreadRef::new(
        BRAND,
        "1106",
        "Pansy Pink",
        &[220, 65, 97],
    ),
    ThreadRef::new(
        BRAND,
        "1107",
        "Dusty Rose",
        &[218, 144, 146],
    ),
    ThreadRef::new(
        BRAND,
        "1108",
        "Dark Rose",
        &[184, 87, 94],
    ),
    ThreadRef::new(
        BRAND,
        "1109",
        "Bright Rose",
        &[164, 74, 84],
    ),
    ThreadRef::new(
        BRAND,
        "1110",
        "Deep Rose",
        &[148, 56, 67],
    ),
    ThreadRef::new(
        BRAND,
        "1111",
        "Soft Fuschia",
        &[255, 194, 199],
    ),
    ThreadRef::new(
        BRAND,
        "1112",
        "Light Fuschia",
        &[211, 142, 161],
    ),
    ThreadRef::new(
        BRAND,
        "1113",
        "Ice Pink",
        &[151, 251, 189],
    ),
    ThreadRef::new(
        BRAND,
        "1114",
        "Hot Pink",
        &[234, 49, 116],
    ),
    ThreadRef::new(
        BRAND,
        "1115",
        "Wild Pink",
        &[218, 80, 132],
    ),
    ThreadRef::new(
        BRAND,
        "1116",
        "Deep Pink",
        &[179, 39, 131],
    ),
    ThreadRef::new(
        BRAND,
        "1117",
        "Dark Fuschia",
        &[159, 7, 96],
    ),
    ThreadRef::new(
        BRAND,
        "1118",
        "Soft Mauve",
        &[230, 180, 201],
    ),
    ThreadRef::new(
        BRAND,
        "1119",
        "Light Mauve",
        &[229, 151, 199],
    ),
    ThreadRef::new(
        BRAND,
        "1120",
        "Med Mauve",
        &[197, 106, 147],
    ),
    ThreadRef::new(
        BRAND,
        "1121",
        "Deep Mauve",
        &[207, 102, 164],
    ),
    ThreadRef::new(
        BRAND,
        "1122",
        "Deep Passion",
        &[161, 26, 128],
    ),
    ThreadRef::new(
        BRAND,
        "1123",
        "Light Burgundy",
        &[128, 1, 88],
    ),
    ThreadRef::new(
        BRAND,
        "1124",
        "Dark Burgundy",
        &[110, 0, 69],
    ),
    ThreadRef::new(
        BRAND,
        "1125",
        "Light Peach",
        &[239, 232, 220],
    ),
    ThreadRef::new(
        BRAND,
        "1126",
        "Opal Mist",
        &[253, 208, 179],
    ),
    ThreadRef::new(
        BRAND,
        "1127",
        "Flesh Pink",
        &[250, 159, 141],
    ),
    ThreadRef::new(
        BRAND,
        "1128",
        "Bashful Pink",
        &[219, 94, 100],
    ),
    ThreadRef::new(
        BRAND,
        "1129",
        "Rust Pink",
        &[184, 89, 64],
    ),
    ThreadRef::new(
        BRAND,
        "1130",
        "Watermelon",
        &[246, 62, 92],
    ),
    ThreadRef::new(
        BRAND,
        "1131",
        "Soft Mocha",
        &[219, 178, 165],
    ),
    ThreadRef::new(
        BRAND,
        "1132",
        "Med Mocha",
        &[209, 143, 146],
    ),
    ThreadRef::new(
        BRAND,
        "1133",
        "Deep Mocha",
        &[197, 118, 121],
    ),
    ThreadRef::new(
        BRAND,
        "1134",
        "Pink Snow",
        &[255, 204, 176],
    ),
    ThreadRef::new(
        BRAND,
        "1135",
        "Sugar Plum",
        &[249, 161, 121],
    ),
    ThreadRef::new(
        BRAND,
        "1136",
        "Charmed Peach",
        &[247, 135, 105],
    ),
    ThreadRef::new(
        BRAND,
        "1137",
        "Light Tangerine",
        &[255, 133, 100],
    ),
    ThreadRef::new(
        BRAND,
        "1138",
        "Soft Tangerine",
        &[254, 132, 78],
    ),
    ThreadRef::new(
        BRAND,
        "1139",
        "Medium Tangerine",
        &[255, 122, 57],
    ),
    ThreadRef::new(
        BRAND,
        "1140",
        "Dark Tangerine",
        &[241, 103, 15],
    ),
    ThreadRef::new(
        BRAND,
        "1141",
        "Deep Tangerine",
        &[251, 68, 12],
    ),
    ThreadRef::new(
        BRAND,
        "1142",
        "Light Coral",
        &[253, 207, 185],
    ),
    ThreadRef::new(
        BRAND,
        "1143",
        "Soft Coral",
        &[241, 179, 166],
    ),
    ThreadRef::new(
        BRAND,
        "1144",
        "Bright Coral",
        &[155, 129, 129],
    ),
    ThreadRef::new(
        BRAND,
        "1145",
        "Dark Coral",
        &[224, 98, 64],
    ),
    ThreadRef::new(
        BRAND,
        "1146",
        "Bright Red",
        &[197, 37, 16],
    ),
    ThreadRef::new(
        BRAND,
        "1147",
        "Satin Red",
        &[194, 17, 42],
    ),
    ThreadRef::new(
        BRAND,
        "1148",
        "Deep Red",
        &[192, 2, 60],
    ),
    ThreadRef::new(
        BRAND,
        "1149",
        "Plush Velvet",
        &[110, 20, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1150",
        "Evening Brandy",
        &[79, 17, 38],
    ),
    ThreadRef::new(
        BRAND,
        "1151",
        "Silky Pink",
        &[241, 207, 208],
    ),
    ThreadRef::new(
        BRAND,
        "1152",
        "Cotton Candy",
        &[240, 169, 184],
    ),
    ThreadRef::new(
        BRAND,
        "1153",
        "Angora Rayon",
        &[250, 187, 178],
    ),
    ThreadRef::new(
        BRAND,
        "1154",
        "Pink Currant",
        &[249, 121, 128],
    ),
    ThreadRef::new(
        BRAND,
        "1155",
        "Carnation Pink",
        &[246, 119, 144],
    ),
    ThreadRef::new(
        BRAND,
        "1156",
        "Soft Plum",
        &[226, 160, 178],
    ),
    ThreadRef::new(
        BRAND,
        "1157",
        "Med Plum",
        &[198, 108, 130],
    ),
    ThreadRef::new(
        BRAND,
        "1158",
        "Dark Plum",
        &[108, 2, 64],
    ),
    ThreadRef::new(
        BRAND,
        "1159",
        "Foxy Red",
        &[216, 19, 19],
    ),
    ThreadRef::new(
        BRAND,
        "1160",
        "Poppy Red",
        &[212, 7, 26],
    ),
    ThreadRef::new(
        BRAND,
        "1161",
        "Crystal Pink",
        &[254, 229, 228],
    ),
    ThreadRef::new(
        BRAND,
        "1162",
        "Pink Crystal",
        &[255, 203, 203],
    ),
    ThreadRef::new(
        BRAND,
        "1163",
        "Rose Wine",
        &[253, 178, 185],
    ),
    ThreadRef::new(
        BRAND,
        "1164",
        "Light Punch",
        &[254, 119, 126],
    ),
    ThreadRef::new(
        BRAND,
        "1165",
        "Medium Punch",
        &[213, 83, 83],
    ),
    ThreadRef::new(
        BRAND,
        "1166",
        "Dark Punch",
        &[203, 57, 57],
    ),
    ThreadRef::new(
        BRAND,
        "1167",
        "Christmas Red",
        &[168, 16, 29],
    ),
    ThreadRef::new(
        BRAND,
        "1168",
        "Wild Fuschia",
        &[181, 5, 88],
    ),
    ThreadRef::new(
        BRAND,
        "1169",
        "Scarlet Red",
        &[210, 28, 25],
    ),
    ThreadRef::new(
        BRAND,
        "1170",
        "Sangria",
        &[198, 23, 40],
    ),
    ThreadRef::new(
        BRAND,
        "1171",
        "Divine Wine",
        &[139, 6, 28],
    ),
    ThreadRef::new(
        BRAND,
        "1172",
        "Barely Rose",
        &[213, 99, 123],
    ),
    ThreadRef::new(
        BRAND,
        "1173",
        "Pearl",
        &[253, 247, 240],
    ),
    ThreadRef::new(
        BRAND,
        "1174",
        "Milky Way",
        &[252, 230, 216],
    ),
    ThreadRef::new(
        BRAND,
        "1175",
        "Apricot",
        &[247, 179, 146],
    ),
    ThreadRef::new(
        BRAND,
        "1176",
        "Light Orange",
        &[250, 157, 101],
    ),
    ThreadRef::new(
        BRAND,
        "1177",
        "Medium Orange",
        &[250, 123, 52],
    ),
    ThreadRef::new(
        BRAND,
        "1178",
        "Deep Orange",
        &[236, 64, 8],
    ),
    ThreadRef::new(
        BRAND,
        "1179",
        "Dark Orange",
        &[221, 2, 2],
    ),
    ThreadRef::new(
        BRAND,
        "1180",
        "Fuschia",
        &[216, 0, 107],
    ),
    ThreadRef::new(
        BRAND,
        "1181",
        "Classic Wine",
        &[236, 149, 163],
    ),
    ThreadRef::new(
        BRAND,
        "1182",
        "Calypso Red",
        &[184, 9, 42],
    ),
    ThreadRef::new(
        BRAND,
        "1183",
        "Berry Red",
        &[194, 82, 102],
    ),
    ThreadRef::new(
        BRAND,
        "1184",
        "Midnight Red",
        &[193, 8, 43],
    ),
    ThreadRef::new(
        BRAND,
        "2101",
        "Cream Silk",
        &[253, 254, 226],
    ),
    ThreadRef::new(
        BRAND,
        "2102",
        "Snow Pearl",
        &[254, 239, 178],
    ),
    ThreadRef::new(
        BRAND,
        "2103",
        "Light Yellow",
        &[255, 204, 124],
    ),
    ThreadRef::new(
        BRAND,
        "2104",
        "Medium Yellow",
        &[253, 190, 91],
    ),
    ThreadRef::new(
        BRAND,
        "2105",
        "Golden Honey",
        &[255, 176, 63],
    ),
    ThreadRef::new(
        BRAND,
        "2106",
        "Honeydew",
        &[255, 154, 56],
    ),
    ThreadRef::new(
        BRAND,
        "2107",
        "Mango",
        &[255, 140, 56],
    ),
    ThreadRef::new(
        BRAND,
        "2108",
        "Pumpkin",
        &[255, 121, 41],
    ),
    ThreadRef::new(
        BRAND,
        "2109",
        "Vanilla",
        &[244, 241, 200],
    ),
    ThreadRef::new(
        BRAND,
        "2110",
        "Light Lemon",
        &[252, 249, 179],
    ),
    ThreadRef::new(
        BRAND,
        "2111",
        "Pineapple",
        &[252, 248, 164],
    ),
    ThreadRef::new(
        BRAND,
        "2112",
        "Sunburst",
        &[252, 238, 94],
    ),
    ThreadRef::new(
        BRAND,
        "2113",
        "Medium Lemon",
        &[255, 227, 129],
    ),
    ThreadRef::new(
        BRAND,
        "2114",
        "Dark Lemon",
        &[248, 202, 78],
    ),
    ThreadRef::new(
        BRAND,
        "2115",
        "Deep Lemon",
        &[246, 186, 64],
    ),
    ThreadRef::new(
        BRAND,
        "2116",
        "Hawaiian Sunrise",
        &[247, 234, 74],
    ),
    ThreadRef::new(
        BRAND,
        "2117",
        "Canary Yellow",
        &[255, 218, 44],
    ),
    ThreadRef::new(
        BRAND,
        "2118",
        "Sunset",
        &[255, 196, 38],
    ),
    ThreadRef::new(
        BRAND,
        "2119",
        "Light Gold",
        &[201, 172, 70],
    ),
    ThreadRef::new(
        BRAND,
        "2120",
        "Medium Gold",
        &[207, 178, 75],
    ),
    ThreadRef::new(
        BRAND,
        "2121",
        "Dark Gold",
        &[202, 146, 37],
    ),
    ThreadRef::new(
        BRAND,
        "2122",
        "Burnished Gold",
        &[207, 140, 61],
    ),
    ThreadRef::new(
        BRAND,
        "2123",
        "Dark Burnished Gold",
        &[203, 128, 62],
    ),
    ThreadRef::new(
        BRAND,
        "2124",
        "Light Ginger",
        &[176, 149, 54],
    ),
    ThreadRef::new(
        BRAND,
        "2125",
        "Medium Ginger",
        &[167, 138, 48],
    ),
    ThreadRef::new(
        BRAND,
        "2126",
        "Dark Ginger",
        &[167, 130, 50],
    ),
    ThreadRef::new(
        BRAND,
        "2127",
        "Light Copper",
        &[234, 169, 82],
    ),
    ThreadRef::new(
        BRAND,
        "2128",
        "Copper",
        &[190, 119, 34],
    ),
    ThreadRef::new(
        BRAND,
        "2129",
        "Pure Pearl",
        &[253, 250, 217],
    ),
    ThreadRef::new(
        BRAND,
        "2130",
        "Soft Butter",
        &[254, 250, 200],
    ),
    ThreadRef::new(
        BRAND,
        "2131",
        "Light Butter",
        &[252, 240, 159],
    ),
    ThreadRef::new(
        BRAND,
        "2132",
        "Medium Butter",
        &[255, 224, 101],
    ),
    ThreadRef::new(
        BRAND,
        "2133",
        "Bright Yellow",
        &[254, 202, 62],
    ),
    ThreadRef::new(
        BRAND,
        "2134",
        "Deep Yellow",
        &[253, 193, 30],
    ),
    ThreadRef::new(
        BRAND,
        "2135",
        "Dark Yellow",
        &[255, 177, 9],
    ),
    ThreadRef::new(
        BRAND,
        "2136",
        "Peach and Cream",
        &[247, 246, 227],
    ),
    ThreadRef::new(
        BRAND,
        "2137",
        "Antique Linen",
        &[255, 252, 222],
    ),
    ThreadRef::new(
        BRAND,
        "2138",
        "Flesh",
        &[252, 233, 205],
    ),
    ThreadRef::new(
        BRAND,
        "2139",
        "Sheer Sunlight",
        &[255, 225, 174],
    ),
    ThreadRef::new(
        BRAND,
        "2140",
        "Medium Peach",
        &[252, 201, 128],
    ),
    ThreadRef::new(
        BRAND,
        "2141",
        "Dark Peach",
        &[245, 161, 54],
    ),
    ThreadRef::new(
        BRAND,
        "2142",
        "Orange Sorbet",
        &[245, 156, 51],
    ),
    ThreadRef::new(
        BRAND,
        "3101",
        "Soft Blue",
        &[238, 248, 250],
    ),
    ThreadRef::new(
        BRAND,
        "3102",
        "Light Blue",
        &[215, 221, 223],
    ),
    ThreadRef::new(
        BRAND,
        "3103",
        "Medium Blue",
        &[165, 191, 194],
    ),
    ThreadRef::new(
        BRAND,
        "3104",
        "Seashell Blue",
        &[167, 203, 229],
    ),
    ThreadRef::new(
        BRAND,
        "3105",
        "Chambray",
        &[132, 165, 202],
    ),
    ThreadRef::new(
        BRAND,
        "3106",
        "Light Denim",
        &[151, 175, 201],
    ),
    ThreadRef::new(
        BRAND,
        "3107",
        "Medium Denim",
        &[78, 95, 138],
    ),
    ThreadRef::new(
        BRAND,
        "3108",
        "Paris Blue",
        &[112, 138, 199],
    ),
    ThreadRef::new(
        BRAND,
        "3109",
        "Indigo Blue",
        &[69, 100, 151],
    ),
    ThreadRef::new(
        BRAND,
        "3110",
        "Blue Jazz",
        &[104, 136, 177],
    ),
    ThreadRef::new(
        BRAND,
        "3111",
        "Dark Denim",
        &[45, 70, 124],
    ),
    ThreadRef::new(
        BRAND,
        "3112",
        "Soft Steel Blue",
        &[158, 176, 176],
    ),
    ThreadRef::new(
        BRAND,
        "3113",
        "Snow Blue",
        &[154, 189, 189],
    ),
    ThreadRef::new(
        BRAND,
        "3114",
        "Medium Steel Blue",
        &[118, 136, 124],
    ),
    ThreadRef::new(
        BRAND,
        "3115",
        "Stirling Blue",
        &[103, 120, 136],
    ),
    ThreadRef::new(
        BRAND,
        "3116",
        "Dark Steel Blue",
        &[89, 105, 121],
    ),
    ThreadRef::new(
        BRAND,
        "3117",
        "Metal Blue",
        &[73, 99, 94],
    ),
    ThreadRef::new(
        BRAND,
        "3118",
        "Light Royal Blue",
        &[111, 118, 182],
    ),
    ThreadRef::new(
        BRAND,
        "3119",
        "Medium Royal Blue",
        &[52, 59, 147],
    ),
    ThreadRef::new(
        BRAND,
        "3120",
        "Dark Royal Blue",
        &[52, 30, 126],
    ),
    ThreadRef::new(
        BRAND,
        "3121",
        "Light Periwinkle",
        &[136, 120, 191],
    ),
    ThreadRef::new(
        BRAND,
        "3122",
        "Dark Periwinkle",
        &[94, 93, 125],
    ),
    ThreadRef::new(
        BRAND,
        "3123",
        "Royal Blue",
        &[65, 82, 136],
    ),
    ThreadRef::new(
        BRAND,
        "3124",
        "Navy Blue",
        &[25, 12, 86],
    ),
    ThreadRef::new(
        BRAND,
        "3125",
        "Midnight Navy",
        &[18, 8, 65],
    ),
    ThreadRef::new(
        BRAND,
        "3126",
        "Soft Ocean Blue",
        &[103, 178, 199],
    ),
    ThreadRef::new(
        BRAND,
        "3127",
        "Light Ocean Blue",
        &[83, 181, 211],
    ),
    ThreadRef::new(
        BRAND,
        "3128",
        "Medium Ocean Blue",
        &[38, 149, 196],
    ),
    ThreadRef::new(
        BRAND,
        "3129",
        "Bright Ocean Blue",
        &[1, 110, 166],
    ),
    ThreadRef::new(
        BRAND,
        "3130",
        "Light Frost",
        &[148, 219, 217],
    ),
    ThreadRef::new(
        BRAND,
        "3131",
        "Medium Frost",
        &[101, 206, 221],
    ),
    ThreadRef::new(
        BRAND,
        "3132",
        "Aquamarine",
        &[6, 138, 181],
    ),
    ThreadRef::new(
        BRAND,
        "3133",
        "Light Lake Blue",
        &[115, 156, 161],
    ),
    ThreadRef::new(
        BRAND,
        "3134",
        "Lake Blue",
        &[82, 146, 164],
    ),
    ThreadRef::new(
        BRAND,
        "3135",
        "Imperial Blue",
        &[36, 96, 128],
    ),
    ThreadRef::new(
        BRAND,
        "3136",
        "Dark Imperial Blue",
        &[34, 95, 120],
    ),
    ThreadRef::new(
        BRAND,
        "3137",
        "Pale Blue",
        &[241, 246, 240],
    ),
    ThreadRef::new(
        BRAND,
        "3138",
        "Light Sky Blue",
        &[227, 240, 227],
    ),
    ThreadRef::new(
        BRAND,
        "3139",
        "Sky Blue",
        &[202, 239, 226],
    ),
    ThreadRef::new(
        BRAND,
        "3140",
        "Light Pacific Blue",
        &[182, 243, 242],
    ),
    ThreadRef::new(
        BRAND,
        "3141",
        "Pacific Blue",
        &[93, 207, 219],
    ),
    ThreadRef::new(
        BRAND,
        "3142",
        "Dark Pacific Blue",
        &[99, 184, 197],
    ),
    ThreadRef::new(
        BRAND,
        "3143",
        "Bright Pacific Blue",
        &[3, 133, 176],
    ),
    ThreadRef::new(
        BRAND,
        "3144",
        "Ice Blue",
        &[224, 231, 232],
    ),
    ThreadRef::new(
        BRAND,
        "3145",
        "Medium Ice Blue",
        &[194, 206, 212],
    ),
    ThreadRef::new(
        BRAND,
        "3146",
        "Dark Ice Blue",
        &[176, 197, 217],
    ),
    ThreadRef::new(
        BRAND,
        "3147",
        "Slate Blue",
        &[127, 151, 167],
    ),
    ThreadRef::new(
        BRAND,
        "3148",
        "Medium Slate Blue",
        &[85, 100, 133],
    ),
    ThreadRef::new(
        BRAND,
        "3149",
        "Dark Slate Blue",
        &[78, 102, 139],
    ),
    ThreadRef::new(
        BRAND,
        "3150",
        "Midnight Blue",
        &[29, 30, 25],
    ),
    ThreadRef::new(
        BRAND,
        "4101",
        "Light Pistachio",
        &[202, 198, 130],
    ),
    ThreadRef::new(
        BRAND,
        "4102",
        "Medium Pistachio",
        &[156, 156, 2],
    ),
    ThreadRef::new(
        BRAND,
        "4103",
        "Dark Pistachio",
        &[143, 137, 7],
    ),
    ThreadRef::new(
        BRAND,
        "4104",
        "Pale Celery",
        &[249, 255, 248],
    ),
    ThreadRef::new(
        BRAND,
        "4105",
        "Light Celery",
        &[238, 252, 226],
    ),
    ThreadRef::new(
        BRAND,
        "4106",
        "Pale Oak Green",
        &[212, 240, 190],
    ),
    ThreadRef::new(
        BRAND,
        "4107",
        "Light Oak Green",
        &[205, 249, 184],
    ),
    ThreadRef::new(
        BRAND,
        "4108",
        "Pale Emerald",
        &[165, 221, 120],
    ),
    ThreadRef::new(
        BRAND,
        "4109",
        "Bright Emerald",
        &[55, 193, 48],
    ),
    ThreadRef::new(
        BRAND,
        "4110",
        "Medium Emerald",
        &[112, 207, 60],
    ),
    ThreadRef::new(
        BRAND,
        "4111",
        "Pale Kelly Green",
        &[77, 114, 65],
    ),
    ThreadRef::new(
        BRAND,
        "4112",
        "Kelly Green",
        &[59, 171, 94],
    ),
    ThreadRef::new(
        BRAND,
        "4113",
        "Dark Kelly Green",
        &[55, 89, 28],
    ),
    ThreadRef::new(
        BRAND,
        "4114",
        "Moss Green",
        &[179, 187, 147],
    ),
    ThreadRef::new(
        BRAND,
        "4115",
        "Dark Palmetto Green",
        &[94, 109, 48],
    ),
    ThreadRef::new(
        BRAND,
        "4116",
        "Palmetto Green",
        &[137, 148, 67],
    ),
    ThreadRef::new(
        BRAND,
        "4117",
        "Medium Palmetto Green",
        &[106, 112, 48],
    ),
    ThreadRef::new(
        BRAND,
        "4118",
        "Alligator Green",
        &[97, 91, 45],
    ),
    ThreadRef::new(
        BRAND,
        "4119",
        "Lilypad Green",
        &[81, 90, 29],
    ),
    ThreadRef::new(
        BRAND,
        "4120",
        "Spring Green",
        &[204, 197, 83],
    ),
    ThreadRef::new(
        BRAND,
        "4121",
        "Brass",
        &[150, 137, 53],
    ),
    ThreadRef::new(
        BRAND,
        "4122",
        "Bronze",
        &[134, 131, 46],
    ),
    ThreadRef::new(
        BRAND,
        "4123",
        "Bright Olive Green",
        &[149, 181, 50],
    ),
    ThreadRef::new(
        BRAND,
        "4124",
        "Medium Olive Green",
        &[94, 129, 39],
    ),
    ThreadRef::new(
        BRAND,
        "4125",
        "Dark Olive Green",
        &[54, 101, 41],
    ),
    ThreadRef::new(
        BRAND,
        "4126",
        "Soft Mint Green",
        &[182, 218, 164],
    ),
    ThreadRef::new(
        BRAND,
        "4127",
        "Light Mint Green",
        &[181, 227, 189],
    ),
    ThreadRef::new(
        BRAND,
        "4128",
        "Medium Mint Green",
        &[150, 202, 152],
    ),
    ThreadRef::new(
        BRAND,
        "4129",
        "Bright Mint Green",
        &[14, 180, 92],
    ),
    ThreadRef::new(
        BRAND,
        "4130",
        "Dark Mint Green",
        &[27, 76, 51],
    ),
    ThreadRef::new(
        BRAND,
        "4131",
        "Soft Evergreen",
        &[36, 100, 48],
    ),
    ThreadRef::new(
        BRAND,
        "4132",
        "Medium Evergreen",
        &[96, 131, 77],
    ),
    ThreadRef::new(
        BRAND,
        "4133",
        "Evergreen",
        &[77, 130, 97],
    ),
    ThreadRef::new(
        BRAND,
        "4134",
        "Dark Evergreen",
        &[31, 60, 38],
    ),
    ThreadRef::new(
        BRAND,
        "4135",
        "Light Aqua",
        &[200, 255, 244],
    ),
    ThreadRef::new(
        BRAND,
        "4136",
        "Soft Aqua",
        &[167, 226, 217],
    ),
    ThreadRef::new(
        BRAND,
        "4137",
        "Medium Aqua",
        &[109, 212, 196],
    ),
    ThreadRef::new(
        BRAND,
        "4138",
        "Aqua",
        &[33, 173, 155],
    ),
    ThreadRef::new(
        BRAND,
        "4139",
        "Dark Aqua",
        &[24, 172, 139],
    ),
    ThreadRef::new(
        BRAND,
        "4140",
        "Bright Aqua",
        &[6, 137, 120],
    ),
    ThreadRef::new(
        BRAND,
        "4141",
        "Turquoise",
        &[6, 113, 120],
    ),
    ThreadRef::new(
        BRAND,
        "4142",
        "Misty Mint",
        &[230, 245, 243],
    ),
    ThreadRef::new(
        BRAND,
        "4143",
        "Baby Blue",
        &[199, 225, 222],
    ),
    ThreadRef::new(
        BRAND,
        "4144",
        "Exotic Green",
        &[170, 195, 189],
    ),
    ThreadRef::new(
        BRAND,
        "4145",
        "Pale Meadow Green",
        &[196, 216, 116],
    ),
    ThreadRef::new(
        BRAND,
        "4146",
        "Meadow Green",
        &[140, 167, 62],
    ),
    ThreadRef::new(
        BRAND,
        "4147",
        "Bright Moss",
        &[152, 175, 86],
    ),
    ThreadRef::new(
        BRAND,
        "4148",
        "Dark Moss",
        &[134, 134, 48],
    ),
    ThreadRef::new(
        BRAND,
        "4149",
        "Pale Green Oak",
        &[174, 190, 86],
    ),
    ThreadRef::new(
        BRAND,
        "4150",
        "Green Oak",
        &[195, 220, 130],
    ),
    ThreadRef::new(
        BRAND,
        "4151",
        "Dark Green Oak",
        &[163, 204, 72],
    ),
    ThreadRef::new(
        BRAND,
        "4152",
        "Bright Green Oak",
        &[134, 197, 88],
    ),
    ThreadRef::new(
        BRAND,
        "4153",
        "Medium Lime",
        &[82, 165, 71],
    ),
    ThreadRef::new(
        BRAND,
        "4154",
        "Lime Green",
        &[14, 150, 57],
    ),
    ThreadRef::new(
        BRAND,
        "4155",
        "Swamp Green",
        &[78, 115, 61],
    ),
    ThreadRef::new(
        BRAND,
        "4156",
        "Safari Green",
        &[47, 89, 42],
    ),
    ThreadRef::new(
        BRAND,
        "4157",
        "Dark Green",
        &[48, 81, 53],
    ),
    ThreadRef::new(
        BRAND,
        "4158",
        "Deep Green",
        &[53, 88, 62],
    ),
    ThreadRef::new(
        BRAND,
        "5101",
        "Candleglow",
        &[253, 246, 240],
    ),
    ThreadRef::new(
        BRAND,
        "5102",
        "Orchid",
        &[255, 230, 241],
    ),
    ThreadRef::new(
        BRAND,
        "5103",
        "Satin Wine",
        &[203, 170, 191],
    ),
    ThreadRef::new(
        BRAND,
        "5104",
        "Soft Tulip",
        &[210, 185, 219],
    ),
    ThreadRef::new(
        BRAND,
        "5105",
        "Light Tulip",
        &[223, 189, 236],
    ),
    ThreadRef::new(
        BRAND,
        "5106",
        "Medium Tulip",
        &[186, 140, 199],
    ),
    ThreadRef::new(
        BRAND,
        "5107",
        "Dark Tulip",
        &[163, 134, 177],
    ),
    ThreadRef::new(
        BRAND,
        "5108",
        "Bright Tulip",
        &[116, 87, 150],
    ),
    ThreadRef::new(
        BRAND,
        "5109",
        "Mulberry",
        &[132, 85, 143],
    ),
    ThreadRef::new(
        BRAND,
        "5110",
        "Bright Mulberry",
        &[104, 40, 118],
    ),
    ThreadRef::new(
        BRAND,
        "5111",
        "Amethyst",
        &[126, 81, 116],
    ),
    ThreadRef::new(
        BRAND,
        "5112",
        "Dark Purple",
        &[94, 40, 116],
    ),
    ThreadRef::new(
        BRAND,
        "5113",
        "Pale Grape",
        &[222, 217, 247],
    ),
    ThreadRef::new(
        BRAND,
        "5114",
        "Light Grape",
        &[144, 139, 171],
    ),
    ThreadRef::new(
        BRAND,
        "5115",
        "Grape",
        &[159, 140, 172],
    ),
    ThreadRef::new(
        BRAND,
        "5116",
        "Bright Grape",
        &[85, 134, 121],
    ),
    ThreadRef::new(
        BRAND,
        "5117",
        "Dark Amethyst",
        &[79, 38, 61],
    ),
    ThreadRef::new(
        BRAND,
        "5118",
        "Light Mauve",
        &[126, 98, 182],
    ),
    ThreadRef::new(
        BRAND,
        "5119",
        "Dark Mauve",
        &[48, 52, 91],
    ),
    ThreadRef::new(
        BRAND,
        "5120",
        "Bright Plum",
        &[102, 66, 110],
    ),
    ThreadRef::new(
        BRAND,
        "6101",
        "Grey",
        &[224, 210, 200],
    ),
    ThreadRef::new(
        BRAND,
        "6102",
        "Pearl Grey",
        &[232, 235, 238],
    ),
    ThreadRef::new(
        BRAND,
        "6103",
        "Cinder Grey",
        &[202, 199, 212],
    ),
    ThreadRef::new(
        BRAND,
        "6104",
        "Pebble Grey",
        &[195, 205, 193],
    ),
    ThreadRef::new(
        BRAND,
        "6105",
        "Stone Grey",
        &[170, 170, 158],
    ),
    ThreadRef::new(
        BRAND,
        "6106",
        "Smoke Grey",
        &[149, 141, 126],
    ),
    ThreadRef::new(
        BRAND,
        "6107",
        "Charcoal",
        &[100, 105, 94],
    ),
    ThreadRef::new(
        BRAND,
        "6108",
        "Pale Sterling Grey",
        &[181, 180, 184],
    ),
    ThreadRef::new(
        BRAND,
        "6109",
        "Sterling Grey",
        &[116, 114, 127],
    ),
    ThreadRef::new(
        BRAND,
        "6110",
        "Twilight Grey",
        &[161, 157, 150],
    ),
    ThreadRef::new(
        BRAND,
        "6111",
        "Dark Twilight Grey",
        &[76, 73, 91],
    ),
    ThreadRef::new(
        BRAND,
        "6112",
        "Midnight Metal",
        &[47, 45, 36],
    ),
    ThreadRef::new(
        BRAND,
        "6113",
        "Midnight Blue",
        &[22, 25, 42],
    ),
    ThreadRef::new(
        BRAND,
        "6114",
        "Cream Wheat",
        &[244, 237, 209],
    ),
    ThreadRef::new(
        BRAND,
        "6115",
        "Spun Gold",
        &[209, 202, 163],
    ),
    ThreadRef::new(
        BRAND,
        "6116",
        "Grey Khaki",
        &[157, 157, 121],
    ),
    ThreadRef::new(
        BRAND,
        "6117",
        "Earth",
        &[114, 114, 64],
    ),
    ThreadRef::new(
        BRAND,
        "6118",
        "Tan",
        &[240, 224, 170],
    ),
    ThreadRef::new(
        BRAND,
        "6119",
        "Medium Tan",
        &[209, 188, 131],
    ),
    ThreadRef::new(
        BRAND,
        "6120",
        "Dark Tan",
        &[161, 143, 66],
    ),
    ThreadRef::new(
        BRAND,
        "6121",
        "Dark Khaki",
        &[79, 82, 28],
    ),
    ThreadRef::new(
        BRAND,
        "6122",
        "Pure Diamond",
        &[248, 247, 240],
    ),
    ThreadRef::new(
        BRAND,
        "6123",
        "Barely Pink",
        &[249, 229, 210],
    ),
    ThreadRef::new(
        BRAND,
        "6124",
        "Sheer Ivory",
        &[243, 233, 199],
    ),
    ThreadRef::new(
        BRAND,
        "6125",
        "Soft Demure",
        &[234, 211, 191],
    ),
    ThreadRef::new(
        BRAND,
        "6126",
        "Demure",
        &[221, 168, 148],
    ),
    ThreadRef::new(
        BRAND,
        "6127",
        "Butterscotch",
        &[220, 186, 128],
    ),
    ThreadRef::new(
        BRAND,
        "6128",
        "Caramel",
        &[117, 86, 37],
    ),
    ThreadRef::new(
        BRAND,
        "6129",
        "Frosty White",
        &[253, 245, 229],
    ),
    ThreadRef::new(
        BRAND,
        "6130",
        "Iced Silver",
        &[253, 255, 250],
    ),
    ThreadRef::new(
        BRAND,
        "6131",
        "White Silk",
        &[255, 248, 243],
    ),
    ThreadRef::new(
        BRAND,
        "6132",
        "Siver Sand",
        &[227, 214, 191],
    ),
    ThreadRef::new(
        BRAND,
        "6133",
        "Skin Light",
        &[241, 203, 154],
    ),
    ThreadRef::new(
        BRAND,
        "6134",
        "Taupe",
        &[194, 180, 150],
    ),
    ThreadRef::new(
        BRAND,
        "6135",
        "Sage Green",
        &[136, 120, 79],
    ),
    ThreadRef::new(
        BRAND,
        "7101",
        "Sheer Beige",
        &[242, 223, 186],
    ),
    ThreadRef::new(
        BRAND,
        "7102",
        "Nude Gold",
        &[245, 231, 206],
    ),
    ThreadRef::new(
        BRAND,
        "7103",
        "Iced Mocha",
        &[213, 192, 173],
    ),
    ThreadRef::new(
        BRAND,
        "7104",
        "Golden Sand",
        &[220, 181, 196],
    ),
    ThreadRef::new(
        BRAND,
        "7105",
        "Brown Sugar",
        &[195, 158, 80],
    ),
    ThreadRef::new(
        BRAND,
        "7106",
        "Brown Toast",
        &[180, 150, 107],
    ),
    ThreadRef::new(
        BRAND,
        "7107",
        "Cinnamon",
        &[127, 100, 68],
    ),
    ThreadRef::new(
        BRAND,
        "7108",
        "Cedar",
        &[154, 113, 42],
    ),
    ThreadRef::new(
        BRAND,
        "7109",
        "Coffee",
        &[143, 98, 33],
    ),
    ThreadRef::new(
        BRAND,
        "7110",
        "Mocha",
        &[108, 75, 39],
    ),
    ThreadRef::new(
        BRAND,
        "7111",
        "Toffee",
        &[94, 58, 19],
    ),
    ThreadRef::new(
        BRAND,
        "7112",
        "Blackberry",
        &[80, 40, 10],
    ),
    ThreadRef::new(
        BRAND,
        "7113",
        "Peach Tulip",
        &[255, 219, 183],
    ),
    ThreadRef::new(
        BRAND,
        "7114",
        "Butterfly Orange",
        &[255, 152, 165],
    ),
    ThreadRef::new(
        BRAND,
        "7115",
        "Light Rust",
        &[224, 145, 97],
    ),
    ThreadRef::new(
        BRAND,
        "7116",
        "Golden Rust",
        &[202, 118, 43],
    ),
    ThreadRef::new(
        BRAND,
        "7117",
        "Dark Rust",
        &[162, 83, 30],
    ),
    ThreadRef::new(
        BRAND,
        "7118",
        "Bright Rust",
        &[154, 62, 6],
    ),
    ThreadRef::new(
        BRAND,
        "7119",
        "Pale Salmon",
        &[246, 222, 222],
    ),
    ThreadRef::new(
        BRAND,
        "7120",
        "Salmon",
        &[186, 147, 134],
    ),
    ThreadRef::new(
        BRAND,
        "7121",
        "Dark Salmon",
        &[154, 103, 100],
    ),
    ThreadRef::new(
        BRAND,
        "7122",
        "Light Copper Brown",
        &[113, 61, 19],
    ),
    ThreadRef::new(
        BRAND,
        "7123",
        "Dark Copper Brown",
        &[103, 40, 15],
    ),
    ThreadRef::new(
        BRAND,
        "7124",
        "Coffee Bean",
        &[48, 34, 28],
    ),
    ThreadRef::new(
        BRAND,
        "7125",
        "Espresso",
        &[24, 17, 14],
    ),
    ThreadRef::new(
        BRAND,
        "7126",
        "Silver Tinsel",
        &[248, 255, 249],
    ),
    ThreadRef::new(
        BRAND,
        "7127",
        "Silver Grey",
        &[235, 250, 233],
    ),
    ThreadRef::new(
        BRAND,
        "7128",
        "Granite",
        &[214, 228, 213],
    ),
    ThreadRef::new(
        BRAND,
        "7129",
        "Steel Glaze",
        &[162, 178, 161],
    ),
    ThreadRef::new(
        BRAND,
        "7130",
        "Dark Silver",
        &[134, 142, 134],
    ),
    ThreadRef::new(
        BRAND,
        "7131",
        "Parsley",
        &[93, 91, 65],
    ),
    ThreadRef::new(
        BRAND,
        "7132",
        "Grey Suede",
        &[72, 76, 73],
    ),
    ThreadRef::new(
        BRAND,
        "7133",
        "Natural Plum",
        &[149, 140, 149],
    ),
    ThreadRef::new(
        BRAND,
        "7134",
        "Natural Burgundy",
        &[90, 76, 82],
    ),
    ThreadRef::new(
        BRAND,
        "7135",
        "Sapphire",
        &[48, 41, 77],
    ),
    ThreadRef::new(
        BRAND,
        "7136",
        "Blue Velvet",
        &[33, 44, 71],
    ),
    ThreadRef::new(
        BRAND,
        "7137",
        "Soft Beige",
        &[221, 191, 155],
    ),
    ThreadRef::new(
        BRAND,
        "7138",
        "Medium Beige",
        &[199, 152, 91],
    ),
    ThreadRef::new(
        BRAND,
        "7139",
        "Golden Honey",
        &[237, 192, 111],
    ),
    ThreadRef::new(
        BRAND,
        "7140",
        "Medium Honey",
        &[208, 161, 78],
    ),
    ThreadRef::new(
        BRAND,
        "7141",
        "Light Blue Grey",
        &[128, 123, 129],
    ),
    ThreadRef::new(
        BRAND,
        "7142",
        "Medium Blue Grey",
        &[103, 118, 133],
    ),
    ThreadRef::new(
        BRAND,
        "7143",
        "Dark Blue Grey",
        &[8, 29, 56],
    ),
    ThreadRef::new(
        BRAND,
        "7144",
        "Silver Blue",
        &[99, 118, 144],
    ),
    ThreadRef::new(
        BRAND,
        "7145",
        "Smurf Blue",
        &[29, 97, 189],
    ),
    ThreadRef::new(
        BRAND,
        "7146",
        "Dark Smurf Blue",
        &[21, 47, 182],
    ),
    ThreadRef::new(
        BRAND,
        "7147",
        "Dark Blue Aqua",
        &[0, 49, 86],
    ),
    ThreadRef::new(
        BRAND,
        "7148",
        "Midnight Navy",
        &[17, 29, 66],
    ),
    ThreadRef::new(
        BRAND,
        "8101",
        "White",
        &[255, 255, 255],
    ),
    ThreadRef::new(
        BRAND,
        "8102",
        "Soft White",
        &[242, 235, 222],
    ),
    ThreadRef::new(
        BRAND,
        "8103",
        "Off White",
        &[246, 241, 230],
    ),
    ThreadRef::new(
        BRAND,
        "8104",
        "Antique White",
        &[251, 247, 241],
    ),
    ThreadRef::new(
        BRAND,
        "9000",
        "Black",
        &[0, 0, 0],
    ),
];