#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Sulky PolyDeco";
pub const THREADS: [ThreadRef; 138] = [
    ThreadRef::new(
        BRAND,
        "1001",
        "Bright White",
        &[249, 249, 255],
    ),
    ThreadRef::new(
        BRAND,
        "1002",
        "Soft White",
        &[249, 249, 244],
    ),
    ThreadRef::new(
        BRAND,
        "1005",
        "Black",
        &[0, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1011",
        "Steel Gray",
        &[183, 169, 172],
    ),
    ThreadRef::new(
        BRAND,
        "1017",
        "Pastel Peach",
        &[239, 223, 189],
    ),
    ThreadRef::new(
        BRAND,
        "1019",
        "Peach",
        &[236, 160, 130],
    ),
    ThreadRef::new(
        BRAND,
        "1020",
        "Dark Peach",
        &[240, 130, 120],
    ),
    ThreadRef::new(
        BRAND,
        "1021",
        "Maple",
        &[235, 102, 2],
    ),
    ThreadRef::new(
        BRAND,
        "1023",
        "Yellow",
        &[255, 230, 105],
    ),
    ThreadRef::new(
        BRAND,
        "1024",
        "Goldenrod",
        &[255, 184, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1025",
        "Mine Gold",
        &[215, 128, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1028",
        "Baby Blue",
        &[190, 195, 225],
    ),
    ThreadRef::new(
        BRAND,
        "1029",
        "Med. Blue",
        &[160, 195, 235],
    ),
    ThreadRef::new(
        BRAND,
        "1032",
        "Med. Purple",
        &[230, 140, 235],
    ),
    ThreadRef::new(
        BRAND,
        "1033",
        "Dk. Orchid",
        &[216, 100, 150],
    ),
    ThreadRef::new(
        BRAND,
        "1034",
        "Burgundy",
        &[198, 50, 60],
    ),
    ThreadRef::new(
        BRAND,
        "1035",
        "Dk. Burgundy",
        &[121, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1037",
        "Lt. Red",
        &[249, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1039",
        "True Red",
        &[235, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1040",
        "Med. Dk. Khaki",
        &[135, 115, 117],
    ),
    ThreadRef::new(
        BRAND,
        "1041",
        "Med. Dk. Gray",
        &[140, 127, 131],
    ),
    ThreadRef::new(
        BRAND,
        "1042",
        "Bright Navy Blue",
        &[50, 30, 80],
    ),
    ThreadRef::new(
        BRAND,
        "1043",
        "Dk. Navy",
        &[25, 5, 37],
    ),
    ThreadRef::new(
        BRAND,
        "1044",
        "Midnight Blue",
        &[29, 6, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1045",
        "Lt. Teal",
        &[138, 204, 198],
    ),
    ThreadRef::new(
        BRAND,
        "1046",
        "Teal",
        &[64, 180, 166],
    ),
    ThreadRef::new(
        BRAND,
        "1047",
        "Mint Green",
        &[166, 194, 132],
    ),
    ThreadRef::new(
        BRAND,
        "1051",
        "Xmas Green",
        &[30, 100, 25],
    ),
    ThreadRef::new(
        BRAND,
        "1055",
        "Tawny Tan",
        &[235, 188, 128],
    ),
    ThreadRef::new(
        BRAND,
        "1056",
        "Med. Tawny Tan",
        &[175, 91, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1057",
        "Dk. Tawny Tan",
        &[100, 39, 2],
    ),
    ThreadRef::new(
        BRAND,
        "1058",
        "Tawny Brown",
        &[102, 53, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1061",
        "Pale Yellow",
        &[255, 247, 185],
    ),
    ThreadRef::new(
        BRAND,
        "1064",
        "Pale Peach",
        &[230, 180, 170],
    ),
    ThreadRef::new(
        BRAND,
        "1065",
        "Orange Yellow",
        &[255, 145, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1067",
        "Lemon Yellow",
        &[255, 255, 133],
    ),
    ThreadRef::new(
        BRAND,
        "1070",
        "Gold",
        &[246, 206, 105],
    ),
    ThreadRef::new(
        BRAND,
        "1071",
        "Off White",
        &[249, 249, 234],
    ),
    ThreadRef::new(
        BRAND,
        "1074",
        "Pale Powder Blue",
        &[214, 213, 232],
    ),
    ThreadRef::new(
        BRAND,
        "1076",
        "Royal Blue",
        &[90, 90, 139],
    ),
    ThreadRef::new(
        BRAND,
        "1077",
        "Jade Tint",
        &[190, 205, 200],
    ),
    ThreadRef::new(
        BRAND,
        "1078",
        "Tangerine",
        &[255, 102, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1079",
        "Emerald Green",
        &[23, 85, 35],
    ),
    ThreadRef::new(
        BRAND,
        "1080",
        "Orchid",
        &[220, 130, 160],
    ),
    ThreadRef::new(
        BRAND,
        "1082",
        "Ecru",
        &[247, 227, 187],
    ),
    ThreadRef::new(
        BRAND,
        "1090",
        "Deep Peacock",
        &[22, 98, 95],
    ),
    ThreadRef::new(
        BRAND,
        "1094",
        "Med. Turquoise",
        &[38, 191, 202],
    ),
    ThreadRef::new(
        BRAND,
        "1096",
        "Dk. Turquoise",
        &[15, 105, 120],
    ),
    ThreadRef::new(
        BRAND,
        "1101",
        "True Green",
        &[9, 133, 49],
    ),
    ThreadRef::new(
        BRAND,
        "1103",
        "Dk. Khaki",
        &[2, 20, 15],
    ),
    ThreadRef::new(
        BRAND,
        "1108",
        "Lt. Mauve",
        &[250, 164, 164],
    ),
    ThreadRef::new(
        BRAND,
        "1109",
        "Hot Pink",
        &[220, 100, 150],
    ),
    ThreadRef::new(
        BRAND,
        "1112",
        "Royal Purple",
        &[70, 1, 110],
    ),
    ThreadRef::new(
        BRAND,
        "1113",
        "Pastel Mauve",
        &[240, 200, 180],
    ),
    ThreadRef::new(
        BRAND,
        "1119",
        "Dk. Mauve",
        &[180, 110, 117],
    ),
    ThreadRef::new(
        BRAND,
        "1121",
        "Pink",
        &[250, 185, 203],
    ),
    ThreadRef::new(
        BRAND,
        "1122",
        "Purple",
        &[130, 40, 142],
    ),
    ThreadRef::new(
        BRAND,
        "1124",
        "Sun Yellow",
        &[255, 236, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1126",
        "Tan",
        &[220, 140, 23],
    ),
    ThreadRef::new(
        BRAND,
        "1127",
        "Med. Ecru",
        &[250, 236, 198],
    ),
    ThreadRef::new(
        BRAND,
        "1128",
        "Dk. Ecru",
        &[195, 148, 113],
    ),
    ThreadRef::new(
        BRAND,
        "1129",
        "Brown",
        &[106, 31, 6],
    ),
    ThreadRef::new(
        BRAND,
        "1130",
        "Dark Brown",
        &[85, 22, 2],
    ),
    ThreadRef::new(
        BRAND,
        "1135",
        "Pastel Yellow",
        &[255, 240, 114],
    ),
    ThreadRef::new(
        BRAND,
        "1137",
        "Yellow Orange",
        &[255, 190, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1143",
        "True Blue",
        &[74, 88, 112],
    ),
    ThreadRef::new(
        BRAND,
        "1145",
        "Powder Blue",
        &[180, 225, 235],
    ),
    ThreadRef::new(
        BRAND,
        "1147",
        "Xmas Red",
        &[235, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1149",
        "Deep Ecru",
        &[232, 200, 156],
    ),
    ThreadRef::new(
        BRAND,
        "1154",
        "Coral",
        &[250, 153, 153],
    ),
    ThreadRef::new(
        BRAND,
        "1156",
        "Lt. Army Green",
        &[99, 99, 39],
    ),
    ThreadRef::new(
        BRAND,
        "1158",
        "Dk. Maple",
        &[186, 69, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1159",
        "Temple Gold",
        &[211, 157, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1162",
        "Deep Teal",
        &[16, 57, 74],
    ),
    ThreadRef::new(
        BRAND,
        "1166",
        "Med. Steel Gray",
        &[142, 126, 126],
    ),
    ThreadRef::new(
        BRAND,
        "1169",
        "Bayberry Red",
        &[156, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1170",
        "Lt. Brown",
        &[151, 95, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1171",
        "Weathered Blue",
        &[8, 24, 14],
    ),
    ThreadRef::new(
        BRAND,
        "1172",
        "Med. Weathered Blue",
        &[110, 120, 140],
    ),
    ThreadRef::new(
        BRAND,
        "1174",
        "Dk. Pine Green",
        &[13, 41, 4],
    ),
    ThreadRef::new(
        BRAND,
        "1176",
        "Med. Dk. Avocado",
        &[81, 83, 8],
    ),
    ThreadRef::new(
        BRAND,
        "1177",
        "Avocado",
        &[137, 152, 18],
    ),
    ThreadRef::new(
        BRAND,
        "1179",
        "Dk. Taupe",
        &[143, 98, 61],
    ),
    ThreadRef::new(
        BRAND,
        "1180",
        "Med. Taupe",
        &[165, 137, 115],
    ),
    ThreadRef::new(
        BRAND,
        "1181",
        "Rust",
        &[204, 7, 30],
    ),
    ThreadRef::new(
        BRAND,
        "1183",
        "Black Cherry",
        &[50, 6, 20],
    ),
    ThreadRef::new(
        BRAND,
        "1184",
        "Orange Red",
        &[255, 102, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1185",
        "Golden Yellow",
        &[252, 190, 5],
    ),
    ThreadRef::new(
        BRAND,
        "1187",
        "Mimosa Yellow",
        &[255, 229, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1189",
        "Dk. Chestnut",
        &[75, 18, 45],
    ),
    ThreadRef::new(
        BRAND,
        "1190",
        "Med. Burgundy",
        &[160, 70, 86],
    ),
    ThreadRef::new(
        BRAND,
        "1191",
        "Dk. Rose",
        &[189, 30, 96],
    ),
    ThreadRef::new(
        BRAND,
        "1192",
        "Fuchsia",
        &[210, 30, 130],
    ),
    ThreadRef::new(
        BRAND,
        "1193",
        "Lavender",
        &[230, 175, 210],
    ),
    ThreadRef::new(
        BRAND,
        "1194",
        "Lt. Purple",
        &[210, 116, 215],
    ),
    ThreadRef::new(
        BRAND,
        "1195",
        "Dk. Purple",
        &[55, 1, 80],
    ),
    ThreadRef::new(
        BRAND,
        "1196",
        "Blue",
        &[150, 195, 225],
    ),
    ThreadRef::new(
        BRAND,
        "1197",
        "Med. Navy",
        &[34, 15, 52],
    ),
    ThreadRef::new(
        BRAND,
        "1198",
        "Dusty Navy",
        &[60, 80, 117],
    ),
    ThreadRef::new(
        BRAND,
        "1200",
        "Med. Dk. Navy",
        &[20, 11, 45],
    ),
    ThreadRef::new(
        BRAND,
        "1203",
        "Lt. Weathered Blue",
        &[174, 184, 195],
    ),
    ThreadRef::new(
        BRAND,
        "1206",
        "Dark Jade",
        &[30, 110, 111],
    ),
    ThreadRef::new(
        BRAND,
        "1208",
        "Mallard Green",
        &[12, 61, 3],
    ),
    ThreadRef::new(
        BRAND,
        "1209",
        "Lt. Avocado",
        &[189, 209, 99],
    ),
    ThreadRef::new(
        BRAND,
        "1211",
        "Lt. Khaki",
        &[149, 164, 144],
    ),
    ThreadRef::new(
        BRAND,
        "1212",
        "Khaki",
        &[99, 99, 45],
    ),
    ThreadRef::new(
        BRAND,
        "1214",
        "Med. Chestnut",
        &[100, 40, 40],
    ),
    ThreadRef::new(
        BRAND,
        "1218",
        "Silver Gray",
        &[223, 223, 203],
    ),
    ThreadRef::new(
        BRAND,
        "1219",
        "Gray",
        &[152, 136, 140],
    ),
    ThreadRef::new(
        BRAND,
        "1220",
        "Charcoal Gray",
        &[118, 89, 96],
    ),
    ThreadRef::new(
        BRAND,
        "1222",
        "Lt. Baby Blue",
        &[209, 219, 255],
    ),
    ThreadRef::new(
        BRAND,
        "1224",
        "Bright Pink",
        &[240, 160, 185],
    ),
    ThreadRef::new(
        BRAND,
        "1225",
        "Pastel Pink",
        &[250, 203, 203],
    ),
    ThreadRef::new(
        BRAND,
        "1230",
        "Dk. Teal",
        &[11, 65, 51],
    ),
    ThreadRef::new(
        BRAND,
        "1231",
        "Med. Rose",
        &[229, 50, 106],
    ),
    ThreadRef::new(
        BRAND,
        "1235",
        "Deep Purple",
        &[120, 50, 152],
    ),
    ThreadRef::new(
        BRAND,
        "1236",
        "Lt. Silver",
        &[234, 228, 228],
    ),
    ThreadRef::new(
        BRAND,
        "1252",
        "Bright Peacock",
        &[9, 161, 168],
    ),
    ThreadRef::new(
        BRAND,
        "1255",
        "Deep Orchid",
        &[190, 25, 130],
    ),
    ThreadRef::new(
        BRAND,
        "1258",
        "Coral Reed",
        &[240, 196, 160],
    ),
    ThreadRef::new(
        BRAND,
        "1259",
        "Salmon Peach",
        &[226, 130, 100],
    ),
    ThreadRef::new(
        BRAND,
        "1503",
        "Green Peacock",
        &[52, 150, 105],
    ),
    ThreadRef::new(
        BRAND,
        "1508",
        "Putty",
        &[193, 203, 185],
    ),
    ThreadRef::new(
        BRAND,
        "1510",
        "Lime Green",
        &[122, 179, 29],
    ),
    ThreadRef::new(
        BRAND,
        "1511",
        "Deep Rose",
        &[238, 80, 120],
    ),
    ThreadRef::new(
        BRAND,
        "1513",
        "Wild Peacock",
        &[0, 122, 103],
    ),
    ThreadRef::new(
        BRAND,
        "1517",
        "Coachman Green",
        &[1, 79, 58],
    ),
    ThreadRef::new(
        BRAND,
        "1533",
        "Lt. Rose",
        &[205, 5, 77],
    ),
    ThreadRef::new(
        BRAND,
        "1535",
        "Team Blue",
        &[35, 35, 139],
    ),
    ThreadRef::new(
        BRAND,
        "1536",
        "Midnight Teal",
        &[8, 23, 5],
    ),
    ThreadRef::new(
        BRAND,
        "1901",
        "Neon Yellow",
        &[216, 254, 40],
    ),
    ThreadRef::new(
        BRAND,
        "1904",
        "Neon Green",
        &[144, 255, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1905",
        "Neon Gold",
        &[255, 241, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1909",
        "Neon Pink",
        &[255, 111, 150],
    ),
    ThreadRef::new(
        BRAND,
        "1910",
        "Neon Fuchsia",
        &[255, 0, 130],
    ),
    ThreadRef::new(
        BRAND,
        "1913",
        "Neon Red",
        &[255, 0, 68],
    ),
    ThreadRef::new(
        BRAND,
        "1914",
        "Neon Orange",
        &[255, 158, 6],
    ),
    ThreadRef::new(
        BRAND,
        "1953",
        "Neon Tangerine",
        &[255, 72, 0],
    ),
];