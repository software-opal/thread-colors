#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Marathon Rayon";
pub const THREADS: [ThreadRef; 317] = [
    ThreadRef::new(
        BRAND,
        "1001",
        "Natural White",
        &[242, 242, 226],
    ),
    ThreadRef::new(
        BRAND,
        "1002",
        "Eggshell",
        &[248, 241, 211],
    ),
    ThreadRef::new(
        BRAND,
        "1003",
        "Medium Yellow",
        &[242, 239, 134],
    ),
    ThreadRef::new(
        BRAND,
        "1004",
        "Medium Yellow",
        &[245, 238, 126],
    ),
    ThreadRef::new(
        BRAND,
        "1005",
        "Daffodil",
        &[249, 231, 21],
    ),
    ThreadRef::new(
        BRAND,
        "1006",
        "Pale Yellow",
        &[241, 235, 135],
    ),
    ThreadRef::new(
        BRAND,
        "1007",
        "Canary Yellow",
        &[249, 229, 39],
    ),
    ThreadRef::new(
        BRAND,
        "1008",
        "Moonbeam",
        &[249, 223, 22],
    ),
    ThreadRef::new(
        BRAND,
        "1009",
        "Canary",
        &[250, 215, 22],
    ),
    ThreadRef::new(
        BRAND,
        "1010",
        "Tusk",
        &[248, 231, 171],
    ),
    ThreadRef::new(
        BRAND,
        "1011",
        "Star Gold",
        &[253, 218, 87],
    ),
    ThreadRef::new(
        BRAND,
        "1012",
        "Yellow Poppy",
        &[240, 179, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1015",
        "Almond",
        &[199, 127, 7],
    ),
    ThreadRef::new(
        BRAND,
        "1016",
        "Pale Green/Yellow",
        &[182, 172, 89],
    ),
    ThreadRef::new(
        BRAND,
        "1017",
        "Popcorn",
        &[227, 214, 125],
    ),
    ThreadRef::new(
        BRAND,
        "1018",
        "Black Eyed Susie",
        &[198, 174, 15],
    ),
    ThreadRef::new(
        BRAND,
        "1019",
        "Light Pink",
        &[242, 214, 217],
    ),
    ThreadRef::new(
        BRAND,
        "1020",
        "Pink Bazaar",
        &[240, 199, 211],
    ),
    ThreadRef::new(
        BRAND,
        "1021",
        "Memphis Belle",
        &[244, 191, 210],
    ),
    ThreadRef::new(
        BRAND,
        "1022",
        "Carnatio",
        &[242, 176, 194],
    ),
    ThreadRef::new(
        BRAND,
        "1024",
        "Begonia",
        &[249, 78, 143],
    ),
    ThreadRef::new(
        BRAND,
        "1025",
        "Cherry Blossom",
        &[234, 16, 106],
    ),
    ThreadRef::new(
        BRAND,
        "1026",
        "Dark Pink",
        &[203, 3, 86],
    ),
    ThreadRef::new(
        BRAND,
        "1027",
        "Floral Pink",
        &[237, 114, 171],
    ),
    ThreadRef::new(
        BRAND,
        "1029",
        "Oyster",
        &[246, 229, 229],
    ),
    ThreadRef::new(
        BRAND,
        "1030",
        "Rosewater",
        &[251, 221, 214],
    ),
    ThreadRef::new(
        BRAND,
        "1031",
        "Desert Bloom",
        &[246, 189, 191],
    ),
    ThreadRef::new(
        BRAND,
        "1032",
        "Orchid Pink",
        &[251, 191, 195],
    ),
    ThreadRef::new(
        BRAND,
        "1033",
        "Emily Pink",
        &[253, 173, 177],
    ),
    ThreadRef::new(
        BRAND,
        "1034",
        "Azalea",
        &[252, 117, 143],
    ),
    ThreadRef::new(
        BRAND,
        "1035",
        "Medium Pink",
        &[220, 129, 140],
    ),
    ThreadRef::new(
        BRAND,
        "1037",
        "Crystal Pink",
        &[245, 210, 205],
    ),
    ThreadRef::new(
        BRAND,
        "1038",
        "Crystal Pink",
        &[248, 210, 205],
    ),
    ThreadRef::new(
        BRAND,
        "1039",
        "Bisque",
        &[241, 197, 176],
    ),
    ThreadRef::new(
        BRAND,
        "1040",
        "Blossom",
        &[240, 182, 160],
    ),
    ThreadRef::new(
        BRAND,
        "1041",
        "Cadmium Orange",
        &[252, 158, 113],
    ),
    ThreadRef::new(
        BRAND,
        "1042",
        "Orange",
        &[255, 153, 64],
    ),
    ThreadRef::new(
        BRAND,
        "1043",
        "Golden Poppy",
        &[232, 117, 19],
    ),
    ThreadRef::new(
        BRAND,
        "1044",
        "Pumpkin",
        &[246, 126, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1045",
        "Dark Tex Orange",
        &[250, 99, 2],
    ),
    ThreadRef::new(
        BRAND,
        "1046",
        "Sunkist",
        &[247, 74, 3],
    ),
    ThreadRef::new(
        BRAND,
        "1047",
        "Scarlet",
        &[239, 43, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1048",
        "Rosewood",
        &[233, 19, 47],
    ),
    ThreadRef::new(
        BRAND,
        "1049",
        "Tuxedo Red",
        &[215, 42, 41],
    ),
    ThreadRef::new(
        BRAND,
        "1050",
        "Foxy Red",
        &[207, 19, 38],
    ),
    ThreadRef::new(
        BRAND,
        "1051",
        "Shell Pink",
        &[251, 129, 128],
    ),
    ThreadRef::new(
        BRAND,
        "1052",
        "Rosewood",
        &[218, 28, 63],
    ),
    ThreadRef::new(
        BRAND,
        "1053",
        "Medium Red/Orange",
        &[225, 67, 81],
    ),
    ThreadRef::new(
        BRAND,
        "1054",
        "Atom Red",
        &[168, 12, 54],
    ),
    ThreadRef::new(
        BRAND,
        "1056",
        "Canal Blue",
        &[163, 193, 202],
    ),
    ThreadRef::new(
        BRAND,
        "1057",
        "Sky Blue",
        &[169, 206, 227],
    ),
    ThreadRef::new(
        BRAND,
        "1058",
        "Caribbean Blue",
        &[118, 170, 220],
    ),
    ThreadRef::new(
        BRAND,
        "1059",
        "Mid Windsor",
        &[0, 163, 222],
    ),
    ThreadRef::new(
        BRAND,
        "1060",
        "Cadet Blue",
        &[192, 209, 229],
    ),
    ThreadRef::new(
        BRAND,
        "1062",
        "Sky Blue",
        &[156, 196, 226],
    ),
    ThreadRef::new(
        BRAND,
        "1063",
        "Bright Blue",
        &[102, 137, 205],
    ),
    ThreadRef::new(
        BRAND,
        "1064",
        "Dolphin Blue",
        &[58, 117, 196],
    ),
    ThreadRef::new(
        BRAND,
        "1066",
        "Blue",
        &[14, 27, 141],
    ),
    ThreadRef::new(
        BRAND,
        "1068",
        "Blue",
        &[0, 48, 94],
    ),
    ThreadRef::new(
        BRAND,
        "1069",
        "Royal Blue",
        &[13, 25, 117],
    ),
    ThreadRef::new(
        BRAND,
        "1070",
        "Blue Ribbon",
        &[0, 49, 75],
    ),
    ThreadRef::new(
        BRAND,
        "1071",
        "Blue Spruce",
        &[0, 70, 83],
    ),
    ThreadRef::new(
        BRAND,
        "1072",
        "Deep Windsor",
        &[0, 64, 83],
    ),
    ThreadRef::new(
        BRAND,
        "1073",
        "Rose Pink",
        &[225, 206, 224],
    ),
    ThreadRef::new(
        BRAND,
        "1074",
        "Palest Mauve",
        &[215, 177, 215],
    ),
    ThreadRef::new(
        BRAND,
        "1075",
        "Light Pink",
        &[207, 162, 212],
    ),
    ThreadRef::new(
        BRAND,
        "1076",
        "Mid Lilac",
        &[217, 168, 217],
    ),
    ThreadRef::new(
        BRAND,
        "1077",
        "Purple",
        &[186, 124, 189],
    ),
    ThreadRef::new(
        BRAND,
        "1080",
        "Royal Purple",
        &[87, 12, 112],
    ),
    ThreadRef::new(
        BRAND,
        "1081",
        "Amethyst",
        &[155, 109, 200],
    ),
    ThreadRef::new(
        BRAND,
        "1082",
        "Amethyst",
        &[137, 80, 191],
    ),
    ThreadRef::new(
        BRAND,
        "1084",
        "Violet Blue",
        &[52, 40, 117],
    ),
    ThreadRef::new(
        BRAND,
        "1085",
        "Light Purple",
        &[165, 161, 214],
    ),
    ThreadRef::new(
        BRAND,
        "1086",
        "Violet Blue",
        &[75, 49, 173],
    ),
    ThreadRef::new(
        BRAND,
        "1088",
        "Medium Purple",
        &[179, 103, 133],
    ),
    ThreadRef::new(
        BRAND,
        "1089",
        "Port Wine",
        &[82, 37, 84],
    ),
    ThreadRef::new(
        BRAND,
        "1090",
        "Blackberry",
        &[72, 43, 88],
    ),
    ThreadRef::new(
        BRAND,
        "1091",
        "Soothing Sea",
        &[204, 227, 222],
    ),
    ThreadRef::new(
        BRAND,
        "1092",
        "Blue Glass",
        &[209, 228, 212],
    ),
    ThreadRef::new(
        BRAND,
        "1093",
        "Mint Julep",
        &[170, 222, 213],
    ),
    ThreadRef::new(
        BRAND,
        "1095",
        "Surf Blue",
        &[0, 167, 219],
    ),
    ThreadRef::new(
        BRAND,
        "1096",
        "Blue/Green",
        &[0, 145, 202],
    ),
    ThreadRef::new(
        BRAND,
        "1097",
        "Medium Blue",
        &[0, 115, 198],
    ),
    ThreadRef::new(
        BRAND,
        "1098",
        "Peacock",
        &[0, 78, 110],
    ),
    ThreadRef::new(
        BRAND,
        "1099",
        "Turquoise",
        &[114, 210, 223],
    ),
    ThreadRef::new(
        BRAND,
        "1100",
        "Periwinkle",
        &[42, 197, 218],
    ),
    ThreadRef::new(
        BRAND,
        "1101",
        "Aquamarine",
        &[0, 174, 200],
    ),
    ThreadRef::new(
        BRAND,
        "1102",
        "Baltic Blue",
        &[0, 151, 212],
    ),
    ThreadRef::new(
        BRAND,
        "1103",
        "California Blue",
        &[0, 160, 196],
    ),
    ThreadRef::new(
        BRAND,
        "1104",
        "Tropical Wave",
        &[122, 211, 193],
    ),
    ThreadRef::new(
        BRAND,
        "1105",
        "M. D. Green",
        &[0, 193, 182],
    ),
    ThreadRef::new(
        BRAND,
        "1106",
        "Fern Green",
        &[0, 120, 112],
    ),
    ThreadRef::new(
        BRAND,
        "1107",
        "M. D. Green",
        &[0, 115, 116],
    ),
    ThreadRef::new(
        BRAND,
        "1108",
        "Dark Blue",
        &[0, 101, 128],
    ),
    ThreadRef::new(
        BRAND,
        "1109",
        "Ambrosia",
        &[213, 232, 205],
    ),
    ThreadRef::new(
        BRAND,
        "1110",
        "Blue Glass",
        &[204, 230, 213],
    ),
    ThreadRef::new(
        BRAND,
        "1111",
        "Pale Aqua",
        &[150, 218, 176],
    ),
    ThreadRef::new(
        BRAND,
        "1112",
        "Seafoam",
        &[111, 206, 156],
    ),
    ThreadRef::new(
        BRAND,
        "1113",
        "Pale Green",
        &[123, 210, 182],
    ),
    ThreadRef::new(
        BRAND,
        "1114",
        "Bluestone",
        &[0, 201, 147],
    ),
    ThreadRef::new(
        BRAND,
        "1115",
        "Vibrant Green",
        &[0, 157, 75],
    ),
    ThreadRef::new(
        BRAND,
        "1116",
        "Dark Green",
        &[0, 134, 81],
    ),
    ThreadRef::new(
        BRAND,
        "1117",
        "Green Forest",
        &[0, 106, 91],
    ),
    ThreadRef::new(
        BRAND,
        "1118",
        "Peppermint",
        &[0, 152, 135],
    ),
    ThreadRef::new(
        BRAND,
        "1119",
        "Dark Olive Green",
        &[212, 233, 164],
    ),
    ThreadRef::new(
        BRAND,
        "1120",
        "Sharp Green",
        &[187, 232, 96],
    ),
    ThreadRef::new(
        BRAND,
        "1121",
        "Spruce",
        &[170, 221, 110],
    ),
    ThreadRef::new(
        BRAND,
        "1122",
        "Dark Green",
        &[186, 196, 6],
    ),
    ThreadRef::new(
        BRAND,
        "1123",
        "Green/Yellow",
        &[86, 172, 27],
    ),
    ThreadRef::new(
        BRAND,
        "1125",
        "Harvest Green",
        &[31, 181, 59],
    ),
    ThreadRef::new(
        BRAND,
        "1126",
        "Kelly",
        &[62, 144, 50],
    ),
    ThreadRef::new(
        BRAND,
        "1127",
        "Tapioca",
        &[227, 208, 185],
    ),
    ThreadRef::new(
        BRAND,
        "1131",
        "Opaline",
        &[239, 197, 157],
    ),
    ThreadRef::new(
        BRAND,
        "1132",
        "Sea Mist",
        &[221, 204, 165],
    ),
    ThreadRef::new(
        BRAND,
        "1138",
        "Bamboo",
        &[234, 170, 123],
    ),
    ThreadRef::new(
        BRAND,
        "1140",
        "Tawny Birch",
        &[176, 137, 113],
    ),
    ThreadRef::new(
        BRAND,
        "1141",
        "Light Bronze",
        &[193, 144, 97],
    ),
    ThreadRef::new(
        BRAND,
        "1142",
        "Fawn",
        &[179, 131, 96],
    ),
    ThreadRef::new(
        BRAND,
        "1144",
        "Light Cocoa",
        &[141, 89, 50],
    ),
    ThreadRef::new(
        BRAND,
        "1145",
        "Rose Beige",
        &[204, 175, 183],
    ),
    ThreadRef::new(
        BRAND,
        "1148",
        "Medium Purple",
        &[179, 103, 133],
    ),
    ThreadRef::new(
        BRAND,
        "1149",
        "Rose Dust",
        &[183, 114, 144],
    ),
    ThreadRef::new(
        BRAND,
        "1153",
        "New Berry",
        &[173, 0, 118],
    ),
    ThreadRef::new(
        BRAND,
        "1154",
        "Medium Purple",
        &[155, 0, 111],
    ),
    ThreadRef::new(
        BRAND,
        "1155",
        "Burgundy",
        &[166, 4, 69],
    ),
    ThreadRef::new(
        BRAND,
        "1156",
        "Red Jubilee",
        &[118, 39, 60],
    ),
    ThreadRef::new(
        BRAND,
        "1157",
        "Dark Rust",
        &[193, 57, 40],
    ),
    ThreadRef::new(
        BRAND,
        "1158",
        "Burgundy",
        &[124, 33, 41],
    ),
    ThreadRef::new(
        BRAND,
        "1159",
        "Dark Orange/Red",
        &[146, 52, 56],
    ),
    ThreadRef::new(
        BRAND,
        "1160",
        "Coffee Bean",
        &[93, 46, 41],
    ),
    ThreadRef::new(
        BRAND,
        "1161",
        "Dark Brown",
        &[100, 56, 38],
    ),
    ThreadRef::new(
        BRAND,
        "1164",
        "Light Purple",
        &[196, 198, 206],
    ),
    ThreadRef::new(
        BRAND,
        "1165",
        "Skylight",
        &[203, 194, 200],
    ),
    ThreadRef::new(
        BRAND,
        "1166",
        "Orchid Ice",
        &[226, 204, 212],
    ),
    ThreadRef::new(
        BRAND,
        "1167",
        "Celestial Blue",
        &[175, 188, 192],
    ),
    ThreadRef::new(
        BRAND,
        "1168",
        "Sea Foam",
        &[166, 190, 171],
    ),
    ThreadRef::new(
        BRAND,
        "1170",
        "Moss",
        &[150, 171, 154],
    ),
    ThreadRef::new(
        BRAND,
        "1172",
        "Sage",
        &[84, 119, 49],
    ),
    ThreadRef::new(
        BRAND,
        "1175",
        "Light Grey/Green",
        &[177, 187, 178],
    ),
    ThreadRef::new(
        BRAND,
        "1176",
        "Slate Blue",
        &[110, 134, 169],
    ),
    ThreadRef::new(
        BRAND,
        "1178",
        "Dark Grey",
        &[74, 76, 73],
    ),
    ThreadRef::new(
        BRAND,
        "1180",
        "Black",
        &[56, 56, 56],
    ),
    ThreadRef::new(
        BRAND,
        "1182",
        "Ivory",
        &[250, 233, 212],
    ),
    ThreadRef::new(
        BRAND,
        "1183",
        "Wheat",
        &[250, 224, 139],
    ),
    ThreadRef::new(
        BRAND,
        "1184",
        "Crystal Gray",
        &[227, 210, 195],
    ),
    ThreadRef::new(
        BRAND,
        "1187",
        "Black Eyed Susie",
        &[199, 161, 14],
    ),
    ThreadRef::new(
        BRAND,
        "1188",
        "Buttercup",
        &[254, 200, 31],
    ),
    ThreadRef::new(
        BRAND,
        "1189",
        "Scholastic",
        &[253, 190, 73],
    ),
    ThreadRef::new(
        BRAND,
        "1190",
        "Honeydew",
        &[251, 155, 14],
    ),
    ThreadRef::new(
        BRAND,
        "1192",
        "Dusty Rose",
        &[255, 162, 178],
    ),
    ThreadRef::new(
        BRAND,
        "1193",
        "Pale Pink",
        &[251, 177, 212],
    ),
    ThreadRef::new(
        BRAND,
        "1194",
        "Honeysuckle",
        &[226, 62, 40],
    ),
    ThreadRef::new(
        BRAND,
        "1195",
        "Pastel Lilac",
        &[187, 176, 211],
    ),
    ThreadRef::new(
        BRAND,
        "1196",
        "Brushed Burgundy",
        &[109, 32, 65],
    ),
    ThreadRef::new(
        BRAND,
        "1197",
        "Cristy Blue",
        &[165, 187, 224],
    ),
    ThreadRef::new(
        BRAND,
        "1200",
        "Soldier Blue",
        &[89, 97, 169],
    ),
    ThreadRef::new(
        BRAND,
        "1201",
        "Jamie Blue",
        &[48, 70, 180],
    ),
    ThreadRef::new(
        BRAND,
        "1202",
        "Royal",
        &[30, 29, 120],
    ),
    ThreadRef::new(
        BRAND,
        "1203",
        "Blue Ink",
        &[44, 38, 92],
    ),
    ThreadRef::new(
        BRAND,
        "1204",
        "Blue",
        &[0, 51, 91],
    ),
    ThreadRef::new(
        BRAND,
        "1205",
        "Bleached Aqua",
        &[187, 224, 225],
    ),
    ThreadRef::new(
        BRAND,
        "1207",
        "Peacock Green",
        &[0, 152, 125],
    ),
    ThreadRef::new(
        BRAND,
        "1208",
        "Greenstone",
        &[0, 131, 115],
    ),
    ThreadRef::new(
        BRAND,
        "1209",
        "Imperial Aqua",
        &[0, 109, 102],
    ),
    ThreadRef::new(
        BRAND,
        "1210",
        "Blue Blush",
        &[213, 216, 212],
    ),
    ThreadRef::new(
        BRAND,
        "1212",
        "Orchid Ice",
        &[226, 212, 214],
    ),
    ThreadRef::new(
        BRAND,
        "1216",
        "Midnight",
        &[70, 78, 82],
    ),
    ThreadRef::new(
        BRAND,
        "1219",
        "Gold",
        &[235, 177, 16],
    ),
    ThreadRef::new(
        BRAND,
        "1221",
        "Tamarack",
        &[157, 157, 8],
    ),
    ThreadRef::new(
        BRAND,
        "1224",
        "Copper",
        &[175, 117, 6],
    ),
    ThreadRef::new(
        BRAND,
        "1227",
        "Petal Pink",
        &[255, 160, 190],
    ),
    ThreadRef::new(
        BRAND,
        "1230",
        "Fuschia",
        &[236, 0, 146],
    ),
    ThreadRef::new(
        BRAND,
        "1232",
        "Crimson",
        &[214, 2, 112],
    ),
    ThreadRef::new(
        BRAND,
        "1233",
        "Buff Orange",
        &[254, 183, 120],
    ),
    ThreadRef::new(
        BRAND,
        "1234",
        "Flamingo Pink",
        &[251, 157, 163],
    ),
    ThreadRef::new(
        BRAND,
        "1239",
        "Medium Pink",
        &[217, 133, 189],
    ),
    ThreadRef::new(
        BRAND,
        "1241",
        "Dark Maroon",
        &[81, 47, 70],
    ),
    ThreadRef::new(
        BRAND,
        "1242",
        "Plum",
        &[72, 40, 54],
    ),
    ThreadRef::new(
        BRAND,
        "1243",
        "Wildfire",
        &[162, 37, 56],
    ),
    ThreadRef::new(
        BRAND,
        "1244",
        "Maroon",
        &[126, 31, 63],
    ),
    ThreadRef::new(
        BRAND,
        "1246",
        "Red Licorice",
        &[112, 25, 61],
    ),
    ThreadRef::new(
        BRAND,
        "1247",
        "Jay Blue",
        &[93, 120, 203],
    ),
    ThreadRef::new(
        BRAND,
        "1248",
        "Violet",
        &[95, 105, 196],
    ),
    ThreadRef::new(
        BRAND,
        "1250",
        "Wonder Blue",
        &[94, 152, 172],
    ),
    ThreadRef::new(
        BRAND,
        "1253",
        "Ocean Blue",
        &[0, 82, 186],
    ),
    ThreadRef::new(
        BRAND,
        "1254",
        "Blue",
        &[47, 52, 142],
    ),
    ThreadRef::new(
        BRAND,
        "1256",
        "Salem Blue",
        &[0, 62, 106],
    ),
    ThreadRef::new(
        BRAND,
        "1260",
        "Violet Tulip",
        &[158, 146, 199],
    ),
    ThreadRef::new(
        BRAND,
        "1261",
        "Tulip",
        &[198, 171, 219],
    ),
    ThreadRef::new(
        BRAND,
        "1263",
        "Violet",
        &[116, 82, 189],
    ),
    ThreadRef::new(
        BRAND,
        "1266",
        "Mint Julep",
        &[149, 222, 219],
    ),
    ThreadRef::new(
        BRAND,
        "1267",
        "Turquoise",
        &[127, 215, 220],
    ),
    ThreadRef::new(
        BRAND,
        "1270",
        "Medium Blue/Green",
        &[82, 192, 226],
    ),
    ThreadRef::new(
        BRAND,
        "1271",
        "Pretty Blue",
        &[0, 122, 167],
    ),
    ThreadRef::new(
        BRAND,
        "1272",
        "Mint",
        &[180, 226, 192],
    ),
    ThreadRef::new(
        BRAND,
        "1274",
        "J. Turquoise",
        &[0, 134, 137],
    ),
    ThreadRef::new(
        BRAND,
        "1275",
        "Aquamarine",
        &[0, 184, 200],
    ),
    ThreadRef::new(
        BRAND,
        "1277",
        "Cyan",
        &[0, 157, 160],
    ),
    ThreadRef::new(
        BRAND,
        "1278",
        "Marine Aqua",
        &[0, 161, 185],
    ),
    ThreadRef::new(
        BRAND,
        "1279",
        "Brite Jade",
        &[0, 141, 129],
    ),
    ThreadRef::new(
        BRAND,
        "1284",
        "Meadow",
        &[87, 143, 21],
    ),
    ThreadRef::new(
        BRAND,
        "1285",
        "Bright Green",
        &[0, 183, 96],
    ),
    ThreadRef::new(
        BRAND,
        "1289",
        "Green Forest",
        &[0, 97, 87],
    ),
    ThreadRef::new(
        BRAND,
        "1290",
        "Dark Green",
        &[0, 80, 66],
    ),
    ThreadRef::new(
        BRAND,
        "1293",
        "Dark Blue",
        &[0, 53, 58],
    ),
    ThreadRef::new(
        BRAND,
        "1294",
        "Green",
        &[66, 72, 23],
    ),
    ThreadRef::new(
        BRAND,
        "1295",
        "Khaki",
        &[74, 70, 19],
    ),
    ThreadRef::new(
        BRAND,
        "1296",
        "Water Lilly",
        &[85, 105, 86],
    ),
    ThreadRef::new(
        BRAND,
        "1297",
        "Hedge",
        &[64, 74, 38],
    ),
    ThreadRef::new(
        BRAND,
        "1298",
        "Mitchell Green",
        &[35, 70, 54],
    ),
    ThreadRef::new(
        BRAND,
        "1299",
        "Birch",
        &[227, 217, 192],
    ),
    ThreadRef::new(
        BRAND,
        "1300",
        "Seashell",
        &[215, 204, 175],
    ),
    ThreadRef::new(
        BRAND,
        "1301",
        "Oxford Tan",
        &[180, 181, 143],
    ),
    ThreadRef::new(
        BRAND,
        "1302",
        "Gold",
        &[231, 179, 131],
    ),
    ThreadRef::new(
        BRAND,
        "1307",
        "Dark Brown",
        &[126, 81, 61],
    ),
    ThreadRef::new(
        BRAND,
        "1308",
        "Dark Brown",
        &[82, 42, 37],
    ),
    ThreadRef::new(
        BRAND,
        "1310",
        "Brown Mule",
        &[66, 46, 45],
    ),
    ThreadRef::new(
        BRAND,
        "1311",
        "Illusion Blue",
        &[213, 212, 213],
    ),
    ThreadRef::new(
        BRAND,
        "1312",
        "Palest Ivory",
        &[233, 226, 215],
    ),
    ThreadRef::new(
        BRAND,
        "1316",
        "Skylight",
        &[187, 183, 175],
    ),
    ThreadRef::new(
        BRAND,
        "1319",
        "Banner Gray",
        &[146, 150, 148],
    ),
    ThreadRef::new(
        BRAND,
        "1320",
        "Platinum",
        &[137, 143, 140],
    ),
    ThreadRef::new(
        BRAND,
        "1321",
        "Metal",
        &[103, 109, 111],
    ),
    ThreadRef::new(
        BRAND,
        "1324",
        "Light Midnight",
        &[54, 64, 92],
    ),
    ThreadRef::new(
        BRAND,
        "1325",
        "Lake Blue",
        &[37, 85, 126],
    ),
    ThreadRef::new(
        BRAND,
        "1326",
        "Grape",
        &[243, 209, 190],
    ),
    ThreadRef::new(
        BRAND,
        "1327",
        "Oceanic Green",
        &[0, 133, 144],
    ),
    ThreadRef::new(
        BRAND,
        "1328",
        "Cerise",
        &[225, 7, 71],
    ),
    ThreadRef::new(
        BRAND,
        "1329",
        "Ruby Glint",
        &[226, 41, 130],
    ),
    ThreadRef::new(
        BRAND,
        "1331",
        "Wine",
        &[128, 42, 80],
    ),
    ThreadRef::new(
        BRAND,
        "1332",
        "Light Orange",
        &[232, 136, 144],
    ),
    ThreadRef::new(
        BRAND,
        "1333",
        "Pumpkin",
        &[247, 127, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1334",
        "Spanish Red",
        &[205, 46, 49],
    ),
    ThreadRef::new(
        BRAND,
        "1335",
        "Cerise",
        &[230, 6, 58],
    ),
    ThreadRef::new(
        BRAND,
        "1336",
        "New Berry",
        &[162, 5, 127],
    ),
    ThreadRef::new(
        BRAND,
        "1339",
        "Pale Gray Green",
        &[200, 199, 165],
    ),
    ThreadRef::new(
        BRAND,
        "1341",
        "Dark Aqua Green",
        &[0, 127, 152],
    ),
    ThreadRef::new(
        BRAND,
        "1342",
        "Vibrant Green",
        &[0, 158, 96],
    ),
    ThreadRef::new(
        BRAND,
        "1343",
        "Green Bay",
        &[0, 106, 83],
    ),
    ThreadRef::new(
        BRAND,
        "1344",
        "Dark Green",
        &[36, 79, 50],
    ),
    ThreadRef::new(
        BRAND,
        "1345",
        "Harbor Green",
        &[43, 76, 64],
    ),
    ThreadRef::new(
        BRAND,
        "1346",
        "Dark Green",
        &[0, 86, 65],
    ),
    ThreadRef::new(
        BRAND,
        "1347",
        "Evergreen",
        &[0, 68, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1013",
        "Grain",
        &[235, 179, 140],
    ),
    ThreadRef::new(
        BRAND,
        "1014",
        "Pink Sand",
        &[224, 176, 145],
    ),
    ThreadRef::new(
        BRAND,
        "1023",
        "Prism Pink",
        &[250, 171, 186],
    ),
    ThreadRef::new(
        BRAND,
        "1229",
        "Cashmere Rose",
        &[209, 138, 150],
    ),
    ThreadRef::new(
        BRAND,
        "1028",
        "Aurora Pink",
        &[237, 140, 166],
    ),
    ThreadRef::new(
        BRAND,
        "1191",
        "Cadmium Orange",
        &[250, 148, 105],
    ),
    ThreadRef::new(
        BRAND,
        "1036",
        "Heather Rose",
        &[173, 107, 117],
    ),
    ThreadRef::new(
        BRAND,
        "1151",
        "Heather Rose",
        &[173, 107, 117],
    ),
    ThreadRef::new(
        BRAND,
        "1152",
        "Dusty Lavender",
        &[166, 117, 148],
    ),
    ThreadRef::new(
        BRAND,
        "1055",
        "Illusion Blue",
        &[209, 212, 214],
    ),
    ThreadRef::new(
        BRAND,
        "1198",
        "Placid Blue",
        &[150, 173, 209],
    ),
    ThreadRef::new(
        BRAND,
        "1061",
        "Placid Blue",
        &[150, 173, 209],
    ),
    ThreadRef::new(
        BRAND,
        "1065",
        "Wedgewood",
        &[102, 117, 171],
    ),
    ThreadRef::new(
        BRAND,
        "1255",
        "Dusted Peri",
        &[115, 107, 153],
    ),
    ThreadRef::new(
        BRAND,
        "1249",
        "Lavender Violet",
        &[125, 125, 163],
    ),
    ThreadRef::new(
        BRAND,
        "1067",
        "Lavender Violet",
        &[125, 125, 163],
    ),
    ThreadRef::new(
        BRAND,
        "1150",
        "Pastel Lavender",
        &[219, 166, 194],
    ),
    ThreadRef::new(
        BRAND,
        "1078",
        "African Violet",
        &[181, 138, 179],
    ),
    ThreadRef::new(
        BRAND,
        "1079",
        "Purple Haze",
        &[138, 117, 150],
    ),
    ThreadRef::new(
        BRAND,
        "1087",
        "Daybreak",
        &[135, 125, 153],
    ),
    ThreadRef::new(
        BRAND,
        "1262",
        "Aster Purple",
        &[130, 112, 158],
    ),
    ThreadRef::new(
        BRAND,
        "1083",
        "Purple Haze",
        &[138, 117, 150],
    ),
    ThreadRef::new(
        BRAND,
        "1146",
        "Mauve Shadows",
        &[186, 153, 161],
    ),
    ThreadRef::new(
        BRAND,
        "1147",
        "Regal Orchid",
        &[171, 138, 166],
    ),
    ThreadRef::new(
        BRAND,
        "1287",
        "Aqua",
        &[105, 163, 173],
    ),
    ThreadRef::new(
        BRAND,
        "1094",
        "Aquamarine",
        &[166, 201, 212],
    ),
    ThreadRef::new(
        BRAND,
        "1276",
        "Marine Blue",
        &[122, 173, 179],
    ),
    ThreadRef::new(
        BRAND,
        "1199",
        "Allure",
        &[115, 138, 179],
    ),
    ThreadRef::new(
        BRAND,
        "1124",
        "Brilliant Green",
        &[143, 191, 135],
    ),
    ThreadRef::new(
        BRAND,
        "1206",
        "Pigeon",
        &[176, 179, 168],
    ),
    ThreadRef::new(
        BRAND,
        "1340",
        "Flint Gray",
        &[161, 158, 145],
    ),
    ThreadRef::new(
        BRAND,
        "1330",
        "Seagrass",
        &[143, 150, 130],
    ),
    ThreadRef::new(
        BRAND,
        "1173",
        "Vertiver Green",
        &[125, 120, 102],
    ),
    ThreadRef::new(
        BRAND,
        "1265",
        "Pale Blue",
        &[191, 209, 201],
    ),
    ThreadRef::new(
        BRAND,
        "1169",
        "Surf Spray",
        &[176, 196, 189],
    ),
    ThreadRef::new(
        BRAND,
        "1171",
        "Foget-Me-Not",
        &[143, 176, 191],
    ),
    ThreadRef::new(
        BRAND,
        "1282",
        "Jadeite",
        &[148, 163, 150],
    ),
    ThreadRef::new(
        BRAND,
        "1288",
        "Stone Blue",
        &[135, 156, 161],
    ),
    ThreadRef::new(
        BRAND,
        "1291",
        "Evergreen",
        &[0, 68, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1292",
        "Evergreen",
        &[0, 68, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1348",
        "Snow White",
        &[247, 237, 222],
    ),
    ThreadRef::new(
        BRAND,
        "1181",
        "Lily White",
        &[237, 232, 222],
    ),
    ThreadRef::new(
        BRAND,
        "1186",
        "Light Taupe",
        &[179, 158, 140],
    ),
    ThreadRef::new(
        BRAND,
        "1338",
        "Oxford Tan",
        &[186, 171, 148],
    ),
    ThreadRef::new(
        BRAND,
        "1337",
        "Simply Taupe",
        &[176, 163, 145],
    ),
    ThreadRef::new(
        BRAND,
        "1133",
        "Ashes Of Roses",
        &[184, 173, 166],
    ),
    ThreadRef::new(
        BRAND,
        "1134",
        "String",
        &[179, 166, 153],
    ),
    ThreadRef::new(
        BRAND,
        "1135",
        "Opal Gray",
        &[166, 158, 153],
    ),
    ThreadRef::new(
        BRAND,
        "1303",
        "Opal Gray",
        &[166, 158, 153],
    ),
    ThreadRef::new(
        BRAND,
        "1304",
        "Sphinx",
        &[179, 158, 145],
    ),
    ThreadRef::new(
        BRAND,
        "1143",
        "Zinc",
        &[153, 138, 135],
    ),
    ThreadRef::new(
        BRAND,
        "1162",
        "Cinder",
        &[143, 130, 125],
    ),
    ThreadRef::new(
        BRAND,
        "1128",
        "Crystal Gray",
        &[222, 207, 194],
    ),
    ThreadRef::new(
        BRAND,
        "1129",
        "Peach Whip",
        &[227, 196, 181],
    ),
    ThreadRef::new(
        BRAND,
        "1185",
        "Rose Smoke",
        &[212, 184, 166],
    ),
    ThreadRef::new(
        BRAND,
        "1130",
        "Mushroom",
        &[194, 173, 158],
    ),
    ThreadRef::new(
        BRAND,
        "1136",
        "Burnished Lilac",
        &[201, 171, 171],
    ),
    ThreadRef::new(
        BRAND,
        "1137",
        "Dusty Pink",
        &[232, 176, 156],
    ),
    ThreadRef::new(
        BRAND,
        "1139",
        "Fawn",
        &[176, 148, 138],
    ),
    ThreadRef::new(
        BRAND,
        "1305",
        "Cameo Brown",
        &[194, 140, 122],
    ),
    ThreadRef::new(
        BRAND,
        "1306",
        "Ash Rose",
        &[186, 133, 122],
    ),
    ThreadRef::new(
        BRAND,
        "1163",
        "Sprout Green",
        &[212, 219, 209],
    ),
    ThreadRef::new(
        BRAND,
        "1314",
        "Pearl Blue",
        &[173, 184, 186],
    ),
    ThreadRef::new(
        BRAND,
        "1318",
        "Highrise",
        &[179, 181, 176],
    ),
    ThreadRef::new(
        BRAND,
        "1177",
        "Tempest",
        &[125, 133, 153],
    ),
    ThreadRef::new(
        BRAND,
        "1323",
        "Gray Ridge",
        &[135, 120, 130],
    ),
    ThreadRef::new(
        BRAND,
        "1179",
        "Lily White",
        &[237, 232, 222],
    ),
    ThreadRef::new(
        BRAND,
        "1211",
        "Lilac Gray",
        &[196, 186, 186],
    ),
    ThreadRef::new(
        BRAND,
        "1214",
        "Dapple Gray",
        &[166, 163, 171],
    ),
    ThreadRef::new(
        BRAND,
        "1215",
        "Gull Gray",
        &[171, 163, 161],
    ),
    ThreadRef::new(
        BRAND,
        "1317",
        "Highrise",
        &[179, 181, 176],
    ),
    ThreadRef::new(
        BRAND,
        "1174",
        "Lavender Aura",
        &[161, 158, 168],
    ),
    ThreadRef::new(
        BRAND,
        "1213",
        "Gull Gray",
        &[171, 163, 161],
    ),
    ThreadRef::new(
        BRAND,
        "1322",
        "Silver Rose",
        &[150, 140, 148],
    ),
];