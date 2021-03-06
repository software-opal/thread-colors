#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Brildor PB";
pub const THREADS: [ThreadRef; 260] = [
    ThreadRef::new(
        BRAND,
        "1090",
        "White",
        &[240, 243, 242],
    ),
    ThreadRef::new(
        BRAND,
        "1100",
        "White",
        &[250, 251, 252],
    ),
    ThreadRef::new(
        BRAND,
        "1971",
        "Winter Sage",
        &[187, 189, 187],
    ),
    ThreadRef::new(
        BRAND,
        "2702",
        "Aqua Gray",
        &[165, 171, 165],
    ),
    ThreadRef::new(
        BRAND,
        "1140",
        "Steel Grey",
        &[172, 181, 162],
    ),
    ThreadRef::new(
        BRAND,
        "2704",
        "Bleu Mist Dark",
        &[83, 94, 99],
    ),
    ThreadRef::new(
        BRAND,
        "2706",
        "Black",
        &[50, 58, 59],
    ),
    ThreadRef::new(
        BRAND,
        "2707",
        "Black",
        &[29, 33, 42],
    ),
    ThreadRef::new(
        BRAND,
        "1674",
        "Black",
        &[11, 14, 17],
    ),
    ThreadRef::new(
        BRAND,
        "1999",
        "Black",
        &[10, 14, 16],
    ),
    ThreadRef::new(
        BRAND,
        "2701",
        "Cream",
        &[217, 213, 185],
    ),
    ThreadRef::new(
        BRAND,
        "1139",
        "Gravel",
        &[169, 154, 148],
    ),
    ThreadRef::new(
        BRAND,
        "1218",
        "Willow",
        &[113, 129, 110],
    ),
    ThreadRef::new(
        BRAND,
        "2131",
        "Cinder",
        &[144, 141, 143],
    ),
    ThreadRef::new(
        BRAND,
        "2674",
        "Petrol Blue",
        &[77, 82, 97],
    ),
    ThreadRef::new(
        BRAND,
        "1612",
        "Dark Navy",
        &[67, 66, 78],
    ),
    ThreadRef::new(
        BRAND,
        "1177",
        "Canary",
        &[255, 221, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2122",
        "Manila",
        &[255, 237, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2120",
        "Daffodil",
        &[254, 255, 58],
    ),
    ThreadRef::new(
        BRAND,
        "1270",
        "Meilee Green",
        &[250, 255, 140],
    ),
    ThreadRef::new(
        BRAND,
        "1101",
        "Pale Yellow-Green",
        &[238, 246, 232],
    ),
    ThreadRef::new(
        BRAND,
        "1071",
        "Jonquil Very Light",
        &[255, 255, 221],
    ),
    ThreadRef::new(
        BRAND,
        "1300",
        "Pale Yellow",
        &[255, 254, 184],
    ),
    ThreadRef::new(
        BRAND,
        "1162",
        "Celery",
        &[255, 253, 150],
    ),
    ThreadRef::new(
        BRAND,
        "1163",
        "Glow",
        &[255, 252, 114],
    ),
    ThreadRef::new(
        BRAND,
        "2106",
        "Hawaiian Sunrise",
        &[255, 228, 82],
    ),
    ThreadRef::new(
        BRAND,
        "2108",
        "Star Gold",
        &[255, 218, 103],
    ),
    ThreadRef::new(
        BRAND,
        "1165",
        "Scholastic",
        &[255, 187, 69],
    ),
    ThreadRef::new(
        BRAND,
        "2010",
        "Orange Sorbet",
        &[240, 157, 52],
    ),
    ThreadRef::new(
        BRAND,
        "1102",
        "Reddish Brown",
        &[246, 76, 16],
    ),
    ThreadRef::new(
        BRAND,
        "1800",
        "Sunkist",
        &[239, 69, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1804",
        "Red",
        &[235, 31, 18],
    ),
    ThreadRef::new(
        BRAND,
        "1305",
        "Scarlet",
        &[243, 47, 23],
    ),
    ThreadRef::new(
        BRAND,
        "1640",
        "Yellow Cream",
        &[239, 220, 103],
    ),
    ThreadRef::new(
        BRAND,
        "1167",
        "Amber",
        &[244, 181, 76],
    ),
    ThreadRef::new(
        BRAND,
        "1334",
        "Cherokee Red",
        &[127, 27, 17],
    ),
    ThreadRef::new(
        BRAND,
        "1021",
        "Light Orange/Red",
        &[183, 47, 23],
    ),
    ThreadRef::new(
        BRAND,
        "1168",
        "Fluorescent Orange",
        &[255, 84, 40],
    ),
    ThreadRef::new(
        BRAND,
        "2111",
        "Medium Orange",
        &[244, 160, 90],
    ),
    ThreadRef::new(
        BRAND,
        "1060",
        "Almond Oil",
        &[247, 223, 193],
    ),
    ThreadRef::new(
        BRAND,
        "1362",
        "Pooh",
        &[253, 188, 139],
    ),
    ThreadRef::new(
        BRAND,
        "1352",
        "Orange",
        &[255, 129, 69],
    ),
    ThreadRef::new(
        BRAND,
        "2645",
        "Pink Sand",
        &[226, 173, 137],
    ),
    ThreadRef::new(
        BRAND,
        "1255",
        "Tawny Birch",
        &[195, 135, 106],
    ),
    ThreadRef::new(
        BRAND,
        "2618",
        "Burgundy",
        &[138, 36, 25],
    ),
    ThreadRef::new(
        BRAND,
        "2616",
        "Dark Rust",
        &[184, 51, 41],
    ),
    ThreadRef::new(
        BRAND,
        "2210",
        "Light Orange/Red",
        &[168, 40, 27],
    ),
    ThreadRef::new(
        BRAND,
        "2216",
        "Black",
        &[39, 11, 13],
    ),
    ThreadRef::new(
        BRAND,
        "2214",
        "Brown",
        &[93, 14, 18],
    ),
    ThreadRef::new(
        BRAND,
        "1263",
        "Cranberry",
        &[125, 6, 11],
    ),
    ThreadRef::new(
        BRAND,
        "2212",
        "Deep Scarlet",
        &[156, 8, 11],
    ),
    ThreadRef::new(
        BRAND,
        "2211",
        "Red Jubilee",
        &[185, 10, 13],
    ),
    ThreadRef::new(
        BRAND,
        "1703",
        "Carmine",
        &[201, 11, 14],
    ),
    ThreadRef::new(
        BRAND,
        "1169",
        "Foxy Red",
        &[219, 12, 15],
    ),
    ThreadRef::new(
        BRAND,
        "1913",
        "Cranberry",
        &[137, 16, 13],
    ),
    ThreadRef::new(
        BRAND,
        "1902",
        "Dark Red",
        &[161, 7, 19],
    ),
    ThreadRef::new(
        BRAND,
        "1170",
        "Wild Fire",
        &[195, 18, 26],
    ),
    ThreadRef::new(
        BRAND,
        "2209",
        "Medium Red/Orange",
        &[230, 80, 84],
    ),
    ThreadRef::new(
        BRAND,
        "1840",
        "Confetti",
        &[240, 115, 124],
    ),
    ThreadRef::new(
        BRAND,
        "1906",
        "Dark Red",
        &[146, 8, 14],
    ),
    ThreadRef::new(
        BRAND,
        "1900",
        "Red Jubilee",
        &[185, 8, 21],
    ),
    ThreadRef::new(
        BRAND,
        "2205",
        "Medium Red/Orange",
        &[229, 66, 84],
    ),
    ThreadRef::new(
        BRAND,
        "2204",
        "Light Cerise",
        &[240, 106, 132],
    ),
    ThreadRef::new(
        BRAND,
        "2203",
        "Pale Pink",
        &[250, 153, 180],
    ),
    ThreadRef::new(
        BRAND,
        "1151",
        "Pale Pink",
        &[250, 164, 182],
    ),
    ThreadRef::new(
        BRAND,
        "2232",
        "Memphis Belle",
        &[252, 200, 214],
    ),
    ThreadRef::new(
        BRAND,
        "2202",
        "Carnation Ultra Light",
        &[254, 234, 236],
    ),
    ThreadRef::new(
        BRAND,
        "1921",
        "Rust",
        &[170, 44, 51],
    ),
    ThreadRef::new(
        BRAND,
        "1741",
        "Terra Cotta",
        &[162, 55, 70],
    ),
    ThreadRef::new(
        BRAND,
        "2152",
        "Petal Pink",
        &[220, 103, 125],
    ),
    ThreadRef::new(
        BRAND,
        "1199",
        "Heather Pink",
        &[208, 131, 144],
    ),
    ThreadRef::new(
        BRAND,
        "1761",
        "Pale Salmon",
        &[227, 169, 172],
    ),
    ThreadRef::new(
        BRAND,
        "1145",
        "Rosewater",
        &[253, 224, 220],
    ),
    ThreadRef::new(
        BRAND,
        "1308",
        "Coral Flame",
        &[197, 90, 97],
    ),
    ThreadRef::new(
        BRAND,
        "2228",
        "Peony Very Light",
        &[250, 228, 221],
    ),
    ThreadRef::new(
        BRAND,
        "1159",
        "Champagne Pink",
        &[249, 173, 156],
    ),
    ThreadRef::new(
        BRAND,
        "1532",
        "Coral Reef",
        &[255, 151, 130],
    ),
    ThreadRef::new(
        BRAND,
        "2207",
        "Salmon",
        &[242, 149, 112],
    ),
    ThreadRef::new(
        BRAND,
        "2615",
        "Peach Fuzz",
        &[253, 198, 156],
    ),
    ThreadRef::new(
        BRAND,
        "1188",
        "Dark Red S",
        &[140, 33, 49],
    ),
    ThreadRef::new(
        BRAND,
        "2300",
        "Medium Red",
        &[184, 19, 58],
    ),
    ThreadRef::new(
        BRAND,
        "1158",
        "Crimson Red",
        &[210, 35, 83],
    ),
    ThreadRef::new(
        BRAND,
        "1157",
        "Azalea",
        &[225, 80, 123],
    ),
    ThreadRef::new(
        BRAND,
        "1155",
        "Dark Pink",
        &[251, 156, 202],
    ),
    ThreadRef::new(
        BRAND,
        "1152",
        "Pale Pink",
        &[255, 182, 213],
    ),
    ThreadRef::new(
        BRAND,
        "2301",
        "Orchid Pink",
        &[249, 202, 244],
    ),
    ThreadRef::new(
        BRAND,
        "2305",
        "White",
        &[254, 227, 249],
    ),
    ThreadRef::new(
        BRAND,
        "1153",
        "Light Pink",
        &[222, 158, 198],
    ),
    ThreadRef::new(
        BRAND,
        "2310",
        "Medium Pink",
        &[215, 112, 187],
    ),
    ThreadRef::new(
        BRAND,
        "1144",
        "Dark Purple",
        &[130, 37, 93],
    ),
    ThreadRef::new(
        BRAND,
        "2304",
        "Boysenberry",
        &[152, 48, 103],
    ),
    ThreadRef::new(
        BRAND,
        "2715",
        "Dk. Chestnut",
        &[72, 14, 46],
    ),
    ThreadRef::new(
        BRAND,
        "1536",
        "Dark Purple/Navy",
        &[50, 23, 49],
    ),
    ThreadRef::new(
        BRAND,
        "2414",
        "Midnight Blue",
        &[28, 19, 42],
    ),
    ThreadRef::new(
        BRAND,
        "2506",
        "Dark Maroon",
        &[118, 29, 47],
    ),
    ThreadRef::new(
        BRAND,
        "2711",
        "Dark Purple",
        &[82, 28, 52],
    ),
    ThreadRef::new(
        BRAND,
        "1189",
        "Plum",
        &[68, 39, 56],
    ),
    ThreadRef::new(
        BRAND,
        "2241",
        "Sugar Plum",
        &[158, 110, 167],
    ),
    ThreadRef::new(
        BRAND,
        "1186",
        "Light Pink",
        &[243, 227, 244],
    ),
    ThreadRef::new(
        BRAND,
        "1142",
        "Soft Tulip",
        &[213, 181, 222],
    ),
    ThreadRef::new(
        BRAND,
        "1187",
        "Sugar Plum",
        &[160, 101, 172],
    ),
    ThreadRef::new(
        BRAND,
        "2308",
        "Cachet",
        &[118, 58, 147],
    ),
    ThreadRef::new(
        BRAND,
        "1039",
        "Blackberry",
        &[75, 42, 97],
    ),
    ThreadRef::new(
        BRAND,
        "1353",
        "Dark Purple",
        &[31, 27, 57],
    ),
    ThreadRef::new(
        BRAND,
        "2045",
        "Purple",
        &[208, 145, 200],
    ),
    ThreadRef::new(
        BRAND,
        "2810",
        "Medium Purple",
        &[91, 35, 78],
    ),
    ThreadRef::new(
        BRAND,
        "1602",
        "Dark Purple",
        &[25, 21, 71],
    ),
    ThreadRef::new(
        BRAND,
        "1541",
        "Dark Purple Blue",
        &[43, 23, 111],
    ),
    ThreadRef::new(
        BRAND,
        "1210",
        "Deep Purple",
        &[65, 47, 140],
    ),
    ThreadRef::new(
        BRAND,
        "1211",
        "Violet Blue",
        &[60, 42, 109],
    ),
    ThreadRef::new(
        BRAND,
        "1182",
        "Dark Navy",
        &[38, 27, 76],
    ),
    ThreadRef::new(
        BRAND,
        "1355",
        "Midnight Navy",
        &[32, 25, 45],
    ),
    ThreadRef::new(
        BRAND,
        "1331",
        "Marine Blue",
        &[87, 91, 206],
    ),
    ThreadRef::new(
        BRAND,
        "2439",
        "Team Blue",
        &[33, 28, 139],
    ),
    ThreadRef::new(
        BRAND,
        "1113",
        "Ice Blue",
        &[208, 235, 255],
    ),
    ThreadRef::new(
        BRAND,
        "1117",
        "Pastal Blue",
        &[189, 213, 255],
    ),
    ThreadRef::new(
        BRAND,
        "1119",
        "Medium Blue",
        &[139, 179, 255],
    ),
    ThreadRef::new(
        BRAND,
        "1120",
        "Medium Blue",
        &[115, 153, 255],
    ),
    ThreadRef::new(
        BRAND,
        "2431",
        "Corn Flower",
        &[83, 104, 182],
    ),
    ThreadRef::new(
        BRAND,
        "1121",
        "Medium Blue",
        &[62, 98, 184],
    ),
    ThreadRef::new(
        BRAND,
        "2410",
        "Dark Blue",
        &[29, 52, 147],
    ),
    ThreadRef::new(
        BRAND,
        "2411",
        "Provence",
        &[23, 42, 119],
    ),
    ThreadRef::new(
        BRAND,
        "1122",
        "Navy Blue",
        &[14, 29, 87],
    ),
    ThreadRef::new(
        BRAND,
        "1184",
        "Dark Navy",
        &[36, 32, 81],
    ),
    ThreadRef::new(
        BRAND,
        "2440",
        "Navy",
        &[22, 26, 62],
    ),
    ThreadRef::new(
        BRAND,
        "1241",
        "Charcoal",
        &[21, 19, 38],
    ),
    ThreadRef::new(
        BRAND,
        "1138",
        "Pearl Grey",
        &[232, 236, 239],
    ),
    ThreadRef::new(
        BRAND,
        "1114",
        "Pale Silver",
        &[212, 220, 240],
    ),
    ThreadRef::new(
        BRAND,
        "1115",
        "Pastal Blue",
        &[199, 210, 255],
    ),
    ThreadRef::new(
        BRAND,
        "2438",
        "Medium Blue",
        &[161, 179, 255],
    ),
    ThreadRef::new(
        BRAND,
        "2434",
        "Medium Blue",
        &[81, 107, 159],
    ),
    ThreadRef::new(
        BRAND,
        "1037",
        "Blueberry Blue",
        &[34, 38, 75],
    ),
    ThreadRef::new(
        BRAND,
        "1953",
        "Ash",
        &[59, 78, 108],
    ),
    ThreadRef::new(
        BRAND,
        "1743",
        "Dark Navy",
        &[42, 54, 82],
    ),
    ThreadRef::new(
        BRAND,
        "2441",
        "Medium blue",
        &[56, 160, 218],
    ),
    ThreadRef::new(
        BRAND,
        "2442",
        "Sapphire",
        &[53, 135, 208],
    ),
    ThreadRef::new(
        BRAND,
        "2443",
        "Magic Blue",
        &[34, 95, 178],
    ),
    ThreadRef::new(
        BRAND,
        "1797",
        "Blue Twirl",
        &[25, 66, 136],
    ),
    ThreadRef::new(
        BRAND,
        "1842",
        "Dark Grey/Blue",
        &[75, 96, 124],
    ),
    ThreadRef::new(
        BRAND,
        "2403",
        "Ocean Blue",
        &[95, 127, 151],
    ),
    ThreadRef::new(
        BRAND,
        "1116",
        "Ice Blue",
        &[208, 229, 255],
    ),
    ThreadRef::new(
        BRAND,
        "2430",
        "Little Boy Blue",
        &[124, 165, 213],
    ),
    ThreadRef::new(
        BRAND,
        "2404",
        "Blueball",
        &[62, 114, 165],
    ),
    ThreadRef::new(
        BRAND,
        "2405",
        "Blue Dusk",
        &[36, 80, 136],
    ),
    ThreadRef::new(
        BRAND,
        "2428",
        "Paper White",
        &[252, 253, 253],
    ),
    ThreadRef::new(
        BRAND,
        "1112",
        "Ice Blue",
        &[208, 238, 250],
    ),
    ThreadRef::new(
        BRAND,
        "2230",
        "Light Blue/Green",
        &[138, 211, 236],
    ),
    ThreadRef::new(
        BRAND,
        "1181",
        "Blithe",
        &[62, 139, 194],
    ),
    ThreadRef::new(
        BRAND,
        "1250",
        "Cobalt Bleu",
        &[202, 228, 228],
    ),
    ThreadRef::new(
        BRAND,
        "1430",
        "Pale Turquoise",
        &[131, 203, 205],
    ),
    ThreadRef::new(
        BRAND,
        "1128",
        "Bright Blue",
        &[83, 168, 180],
    ),
    ThreadRef::new(
        BRAND,
        "1421",
        "Imperial Blue",
        &[36, 98, 126],
    ),
    ThreadRef::new(
        BRAND,
        "1442",
        "Dark Teal",
        &[20, 49, 78],
    ),
    ThreadRef::new(
        BRAND,
        "1033",
        "Navy",
        &[17, 36, 64],
    ),
    ThreadRef::new(
        BRAND,
        "1644",
        "Atlantic",
        &[27, 52, 70],
    ),
    ThreadRef::new(
        BRAND,
        "1515",
        "Flag Blue",
        &[24, 45, 61],
    ),
    ThreadRef::new(
        BRAND,
        "1185",
        "Dark Blue/Green",
        &[58, 99, 94],
    ),
    ThreadRef::new(
        BRAND,
        "1201",
        "Cilantro",
        &[102, 138, 129],
    ),
    ThreadRef::new(
        BRAND,
        "1125",
        "Sprite",
        &[206, 255, 226],
    ),
    ThreadRef::new(
        BRAND,
        "1126",
        "Pale Aqua",
        &[143, 234, 186],
    ),
    ThreadRef::new(
        BRAND,
        "1610",
        "Jade",
        &[55, 135, 117],
    ),
    ThreadRef::new(
        BRAND,
        "2501",
        "Palest Mint",
        &[223, 255, 227],
    ),
    ThreadRef::new(
        BRAND,
        "2502",
        "Pale Aqua",
        &[125, 205, 154],
    ),
    ThreadRef::new(
        BRAND,
        "2503",
        "Jade",
        &[17, 129, 92],
    ),
    ThreadRef::new(
        BRAND,
        "1137",
        "Ivy",
        &[10, 59, 37],
    ),
    ThreadRef::new(
        BRAND,
        "1374",
        "Olive Green",
        &[12, 41, 30],
    ),
    ThreadRef::new(
        BRAND,
        "1207",
        "Olive Green",
        &[21, 43, 26],
    ),
    ThreadRef::new(
        BRAND,
        "2509",
        "Dark Green",
        &[7, 90, 30],
    ),
    ThreadRef::new(
        BRAND,
        "2531",
        "Green",
        &[8, 125, 35],
    ),
    ThreadRef::new(
        BRAND,
        "1988",
        "Veggie Green",
        &[9, 146, 41],
    ),
    ThreadRef::new(
        BRAND,
        "1613",
        "Lt Jade",
        &[10, 168, 47],
    ),
    ThreadRef::new(
        BRAND,
        "2507",
        "Poison Green",
        &[78, 196, 91],
    ),
    ThreadRef::new(
        BRAND,
        "1422",
        "Dark Green",
        &[7, 96, 32],
    ),
    ThreadRef::new(
        BRAND,
        "1326",
        "Green",
        &[19, 65, 31],
    ),
    ThreadRef::new(
        BRAND,
        "1179",
        "Pale Green",
        &[105, 221, 132],
    ),
    ThreadRef::new(
        BRAND,
        "1710",
        "Emerald Green",
        &[24, 154, 83],
    ),
    ThreadRef::new(
        BRAND,
        "2335",
        "Dark Mint Green",
        &[27, 76, 47],
    ),
    ThreadRef::new(
        BRAND,
        "2511",
        "Pistachio Green dark",
        &[44, 92, 58],
    ),
    ThreadRef::new(
        BRAND,
        "1212",
        "Easter Green",
        &[125, 176, 121],
    ),
    ThreadRef::new(
        BRAND,
        "2510",
        "Medium Mint Green",
        &[155, 209, 147],
    ),
    ThreadRef::new(
        BRAND,
        "1264",
        "Medium Lime",
        &[78, 171, 64],
    ),
    ThreadRef::new(
        BRAND,
        "1135",
        "Bright Green",
        &[126, 221, 75],
    ),
    ThreadRef::new(
        BRAND,
        "1131",
        "Dark Olive Green",
        &[208, 243, 161],
    ),
    ThreadRef::new(
        BRAND,
        "1130",
        "Light Green",
        &[151, 238, 152],
    ),
    ThreadRef::new(
        BRAND,
        "2504",
        "Pale Green",
        &[208, 255, 195],
    ),
    ThreadRef::new(
        BRAND,
        "1123",
        "Silver Grey",
        &[238, 255, 232],
    ),
    ThreadRef::new(
        BRAND,
        "1176",
        "Green/Yellow",
        &[69, 94, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1129",
        "Dark Olive Green",
        &[218, 245, 160],
    ),
    ThreadRef::new(
        BRAND,
        "2822",
        "Apple Green",
        &[167, 215, 100],
    ),
    ThreadRef::new(
        BRAND,
        "2833",
        "Meadow Green",
        &[99, 144, 27],
    ),
    ThreadRef::new(
        BRAND,
        "2538",
        "Palmetto",
        &[73, 107, 25],
    ),
    ThreadRef::new(
        BRAND,
        "2518",
        "Blue Spruce",
        &[37, 65, 21],
    ),
    ThreadRef::new(
        BRAND,
        "2515",
        "Peapod",
        &[150, 190, 24],
    ),
    ThreadRef::new(
        BRAND,
        "1934",
        "Dark Olive",
        &[64, 87, 10],
    ),
    ThreadRef::new(
        BRAND,
        "2514",
        "Olive Green Light",
        &[189, 255, 9],
    ),
    ThreadRef::new(
        BRAND,
        "2527",
        "Lime Green",
        &[161, 223, 19],
    ),
    ThreadRef::new(
        BRAND,
        "1124",
        "Jonquil Very Light",
        &[255, 255, 219],
    ),
    ThreadRef::new(
        BRAND,
        "2513",
        "Meilee Green",
        &[244, 255, 138],
    ),
    ThreadRef::new(
        BRAND,
        "1132",
        "Flite Green",
        &[185, 215, 77],
    ),
    ThreadRef::new(
        BRAND,
        "1133",
        "Pale Green Oak",
        &[179, 192, 81],
    ),
    ThreadRef::new(
        BRAND,
        "1173",
        "Lt. Army Green",
        &[95, 102, 41],
    ),
    ThreadRef::new(
        BRAND,
        "1156",
        "Black Avocado Green",
        &[68, 65, 37],
    ),
    ThreadRef::new(
        BRAND,
        "1072",
        "Field Green",
        &[46, 62, 33],
    ),
    ThreadRef::new(
        BRAND,
        "2133",
        "Meadow Green",
        &[105, 124, 2],
    ),
    ThreadRef::new(
        BRAND,
        "2132",
        "Bronze",
        &[130, 127, 1],
    ),
    ThreadRef::new(
        BRAND,
        "1732",
        "Lime",
        &[179, 185, 41],
    ),
    ThreadRef::new(
        BRAND,
        "1721",
        "Yellow",
        &[224, 217, 21],
    ),
    ThreadRef::new(
        BRAND,
        "2901",
        "Gold",
        &[195, 165, 37],
    ),
    ThreadRef::new(
        BRAND,
        "1463",
        "Juniper Medium Dark",
        &[92, 115, 85],
    ),
    ThreadRef::new(
        BRAND,
        "1776",
        "Dark Fawn",
        &[94, 80, 65],
    ),
    ThreadRef::new(
        BRAND,
        "1873",
        "Pale Green",
        &[147, 160, 133],
    ),
    ThreadRef::new(
        BRAND,
        "1254",
        "Silver Cloud",
        &[187, 184, 166],
    ),
    ThreadRef::new(
        BRAND,
        "1874",
        "Medium Green/Yellow",
        &[145, 134, 120],
    ),
    ThreadRef::new(
        BRAND,
        "1106",
        "Light Brown",
        &[191, 170, 156],
    ),
    ThreadRef::new(
        BRAND,
        "1104",
        "Tan Light",
        &[232, 218, 181],
    ),
    ThreadRef::new(
        BRAND,
        "1103",
        "Vanilla Ice",
        &[241, 241, 211],
    ),
    ThreadRef::new(
        BRAND,
        "1672",
        "Light Brown",
        &[204, 182, 130],
    ),
    ThreadRef::new(
        BRAND,
        "2091",
        "Lark",
        &[185, 164, 112],
    ),
    ThreadRef::new(
        BRAND,
        "1552",
        "Caper",
        &[133, 132, 63],
    ),
    ThreadRef::new(
        BRAND,
        "2612",
        "Medium Honey",
        &[202, 161, 82],
    ),
    ThreadRef::new(
        BRAND,
        "1164",
        "Golden Earth",
        &[218, 137, 51],
    ),
    ThreadRef::new(
        BRAND,
        "1290",
        "Old Gold",
        &[200, 145, 54],
    ),
    ThreadRef::new(
        BRAND,
        "1166",
        "Golden Brown medium",
        &[195, 134, 51],
    ),
    ThreadRef::new(
        BRAND,
        "1625",
        "Rust",
        &[214, 108, 22],
    ),
    ThreadRef::new(
        BRAND,
        "1940",
        "Brick Dark",
        &[183, 79, 16],
    ),
    ThreadRef::new(
        BRAND,
        "1932",
        "Bark",
        &[134, 50, 19],
    ),
    ThreadRef::new(
        BRAND,
        "2610",
        "Coffee Dark",
        &[120, 50, 19],
    ),
    ThreadRef::new(
        BRAND,
        "2620",
        "Brown",
        &[127, 73, 45],
    ),
    ThreadRef::new(
        BRAND,
        "1855",
        "Dark Oak",
        &[62, 24, 23],
    ),
    ThreadRef::new(
        BRAND,
        "1846",
        "Dark Brown",
        &[55, 21, 20],
    ),
    ThreadRef::new(
        BRAND,
        "2623",
        "Espresso",
        &[20, 9, 8],
    ),
    ThreadRef::new(
        BRAND,
        "2621",
        "Black",
        &[42, 26, 16],
    ),
    ThreadRef::new(
        BRAND,
        "2604",
        "Sand Dune",
        &[123, 91, 57],
    ),
    ThreadRef::new(
        BRAND,
        "1945",
        "Dark Brown",
        &[68, 43, 27],
    ),
    ThreadRef::new(
        BRAND,
        "2650",
        "Rice Paper",
        &[217, 186, 97],
    ),
    ThreadRef::new(
        BRAND,
        "1240",
        "Brown",
        &[134, 86, 40],
    ),
    ThreadRef::new(
        BRAND,
        "1941",
        "Old Gold",
        &[192, 142, 53],
    ),
    ThreadRef::new(
        BRAND,
        "2613",
        "Pollen Gold",
        &[218, 155, 51],
    ),
    ThreadRef::new(
        BRAND,
        "2608",
        "Artisan's Gold",
        &[221, 163, 63],
    ),
    ThreadRef::new(
        BRAND,
        "1107",
        "Cornsilk",
        &[238, 207, 125],
    ),
    ThreadRef::new(
        BRAND,
        "1161",
        "Linen",
        &[255, 225, 159],
    ),
    ThreadRef::new(
        BRAND,
        "2648",
        "PaleYellow",
        &[255, 230, 158],
    ),
    ThreadRef::new(
        BRAND,
        "2607",
        "Gold",
        &[225, 196, 100],
    ),
    ThreadRef::new(
        BRAND,
        "2651",
        "Medium Honey",
        &[207, 160, 77],
    ),
    ThreadRef::new(
        BRAND,
        "1109",
        "Pale Orange/Brown",
        &[203, 149, 71],
    ),
    ThreadRef::new(
        BRAND,
        "1110",
        "Mocha Brown Very Dark",
        &[85, 55, 25],
    ),
    ThreadRef::new(
        BRAND,
        "2930",
        "Singh Mist",
        &[255, 165, 69],
    ),
    ThreadRef::new(
        BRAND,
        "2931",
        "Medium Orange",
        &[255, 131, 56],
    ),
    ThreadRef::new(
        BRAND,
        "2946",
        "Sunkist",
        &[255, 73, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2937",
        "Crimson Red Light",
        &[255, 35, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2932",
        "Persimmon",
        &[255, 112, 84],
    ),
    ThreadRef::new(
        BRAND,
        "2922",
        "Fluo Pink",
        &[255, 64, 86],
    ),
    ThreadRef::new(
        BRAND,
        "2983",
        "Gold/Yellow",
        &[255, 240, 0],
    ),
    ThreadRef::new(
        BRAND,
        "2920",
        "Pink",
        &[242, 54, 94],
    ),
    ThreadRef::new(
        BRAND,
        "2921",
        "Dusty Rose",
        &[254, 96, 129],
    ),
    ThreadRef::new(
        BRAND,
        "2912",
        "Deep Coral",
        &[241, 0, 62],
    ),
    ThreadRef::new(
        BRAND,
        "2967",
        "Gold/Yellow",
        &[233, 255, 73],
    ),
    ThreadRef::new(
        BRAND,
        "2910",
        "Gold",
        &[222, 250, 24],
    ),
    ThreadRef::new(
        BRAND,
        "2940",
        "Fluorescent Green",
        &[68, 246, 30],
    ),
    ThreadRef::new(
        BRAND,
        "2911",
        "Erin Green",
        &[63, 230, 7],
    ),
];