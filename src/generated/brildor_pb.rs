#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "Brildor PB";
const THREADS: [ThreadRef; 260] = [
    ThreadRef {
        brand: BRAND,
        code: "1021",
        name: "Light Orange/Red",
        color: &[183, 47, 23],
    },
    ThreadRef {
        brand: BRAND,
        code: "1033",
        name: "Navy",
        color: &[17, 36, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "1037",
        name: "Blueberry Blue",
        color: &[34, 38, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "1039",
        name: "Blackberry",
        color: &[75, 42, 97],
    },
    ThreadRef {
        brand: BRAND,
        code: "1060",
        name: "Almond Oil",
        color: &[247, 223, 193],
    },
    ThreadRef {
        brand: BRAND,
        code: "1071",
        name: "Jonquil Very Light",
        color: &[255, 255, 221],
    },
    ThreadRef {
        brand: BRAND,
        code: "1072",
        name: "Field Green",
        color: &[46, 62, 33],
    },
    ThreadRef {
        brand: BRAND,
        code: "1090",
        name: "White",
        color: &[240, 243, 242],
    },
    ThreadRef {
        brand: BRAND,
        code: "1100",
        name: "White",
        color: &[250, 251, 252],
    },
    ThreadRef {
        brand: BRAND,
        code: "1101",
        name: "Pale Yellow-Green",
        color: &[238, 246, 232],
    },
    ThreadRef {
        brand: BRAND,
        code: "1102",
        name: "Reddish Brown",
        color: &[246, 76, 16],
    },
    ThreadRef {
        brand: BRAND,
        code: "1103",
        name: "Vanilla Ice",
        color: &[241, 241, 211],
    },
    ThreadRef {
        brand: BRAND,
        code: "1104",
        name: "Tan Light",
        color: &[232, 218, 181],
    },
    ThreadRef {
        brand: BRAND,
        code: "1106",
        name: "Light Brown",
        color: &[191, 170, 156],
    },
    ThreadRef {
        brand: BRAND,
        code: "1107",
        name: "Cornsilk",
        color: &[238, 207, 125],
    },
    ThreadRef {
        brand: BRAND,
        code: "1109",
        name: "Pale Orange/Brown",
        color: &[203, 149, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "1110",
        name: "Mocha Brown Very Dark",
        color: &[85, 55, 25],
    },
    ThreadRef {
        brand: BRAND,
        code: "1112",
        name: "Ice Blue",
        color: &[208, 238, 250],
    },
    ThreadRef {
        brand: BRAND,
        code: "1113",
        name: "Ice Blue",
        color: &[208, 235, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "1114",
        name: "Pale Silver",
        color: &[212, 220, 240],
    },
    ThreadRef {
        brand: BRAND,
        code: "1115",
        name: "Pastal Blue",
        color: &[199, 210, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "1116",
        name: "Ice Blue",
        color: &[208, 229, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "1117",
        name: "Pastal Blue",
        color: &[189, 213, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "1119",
        name: "Medium Blue",
        color: &[139, 179, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "1120",
        name: "Medium Blue",
        color: &[115, 153, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "1121",
        name: "Medium Blue",
        color: &[62, 98, 184],
    },
    ThreadRef {
        brand: BRAND,
        code: "1122",
        name: "Navy Blue",
        color: &[14, 29, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "1123",
        name: "Silver Grey",
        color: &[238, 255, 232],
    },
    ThreadRef {
        brand: BRAND,
        code: "1124",
        name: "Jonquil Very Light",
        color: &[255, 255, 219],
    },
    ThreadRef {
        brand: BRAND,
        code: "1125",
        name: "Sprite",
        color: &[206, 255, 226],
    },
    ThreadRef {
        brand: BRAND,
        code: "1126",
        name: "Pale Aqua",
        color: &[143, 234, 186],
    },
    ThreadRef {
        brand: BRAND,
        code: "1128",
        name: "Bright Blue",
        color: &[83, 168, 180],
    },
    ThreadRef {
        brand: BRAND,
        code: "1129",
        name: "Dark Olive Green",
        color: &[218, 245, 160],
    },
    ThreadRef {
        brand: BRAND,
        code: "1130",
        name: "Light Green",
        color: &[151, 238, 152],
    },
    ThreadRef {
        brand: BRAND,
        code: "1131",
        name: "Dark Olive Green",
        color: &[208, 243, 161],
    },
    ThreadRef {
        brand: BRAND,
        code: "1132",
        name: "Flite Green",
        color: &[185, 215, 77],
    },
    ThreadRef {
        brand: BRAND,
        code: "1133",
        name: "Pale Green Oak",
        color: &[179, 192, 81],
    },
    ThreadRef {
        brand: BRAND,
        code: "1135",
        name: "Bright Green",
        color: &[126, 221, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "1137",
        name: "Ivy",
        color: &[10, 59, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "1138",
        name: "Pearl Grey",
        color: &[232, 236, 239],
    },
    ThreadRef {
        brand: BRAND,
        code: "1139",
        name: "Gravel",
        color: &[169, 154, 148],
    },
    ThreadRef {
        brand: BRAND,
        code: "1140",
        name: "Steel Grey",
        color: &[172, 181, 162],
    },
    ThreadRef {
        brand: BRAND,
        code: "1142",
        name: "Soft Tulip",
        color: &[213, 181, 222],
    },
    ThreadRef {
        brand: BRAND,
        code: "1144",
        name: "Dark Purple",
        color: &[130, 37, 93],
    },
    ThreadRef {
        brand: BRAND,
        code: "1145",
        name: "Rosewater",
        color: &[253, 224, 220],
    },
    ThreadRef {
        brand: BRAND,
        code: "1151",
        name: "Pale Pink",
        color: &[250, 164, 182],
    },
    ThreadRef {
        brand: BRAND,
        code: "1152",
        name: "Pale Pink",
        color: &[255, 182, 213],
    },
    ThreadRef {
        brand: BRAND,
        code: "1153",
        name: "Light Pink",
        color: &[222, 158, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "1155",
        name: "Dark Pink",
        color: &[251, 156, 202],
    },
    ThreadRef {
        brand: BRAND,
        code: "1156",
        name: "Black Avocado Green",
        color: &[68, 65, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "1157",
        name: "Azalea",
        color: &[225, 80, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "1158",
        name: "Crimson Red",
        color: &[210, 35, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "1159",
        name: "Champagne Pink",
        color: &[249, 173, 156],
    },
    ThreadRef {
        brand: BRAND,
        code: "1161",
        name: "Linen",
        color: &[255, 225, 159],
    },
    ThreadRef {
        brand: BRAND,
        code: "1162",
        name: "Celery",
        color: &[255, 253, 150],
    },
    ThreadRef {
        brand: BRAND,
        code: "1163",
        name: "Glow",
        color: &[255, 252, 114],
    },
    ThreadRef {
        brand: BRAND,
        code: "1164",
        name: "Golden Earth",
        color: &[218, 137, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "1165",
        name: "Scholastic",
        color: &[255, 187, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "1166",
        name: "Golden Brown medium",
        color: &[195, 134, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "1167",
        name: "Amber",
        color: &[244, 181, 76],
    },
    ThreadRef {
        brand: BRAND,
        code: "1168",
        name: "Fluorescent Orange",
        color: &[255, 84, 40],
    },
    ThreadRef {
        brand: BRAND,
        code: "1169",
        name: "Foxy Red",
        color: &[219, 12, 15],
    },
    ThreadRef {
        brand: BRAND,
        code: "1170",
        name: "Wild Fire",
        color: &[195, 18, 26],
    },
    ThreadRef {
        brand: BRAND,
        code: "1173",
        name: "Lt. Army Green",
        color: &[95, 102, 41],
    },
    ThreadRef {
        brand: BRAND,
        code: "1176",
        name: "Green/Yellow",
        color: &[69, 94, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "1177",
        name: "Canary",
        color: &[255, 221, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "1179",
        name: "Pale Green",
        color: &[105, 221, 132],
    },
    ThreadRef {
        brand: BRAND,
        code: "1181",
        name: "Blithe",
        color: &[62, 139, 194],
    },
    ThreadRef {
        brand: BRAND,
        code: "1182",
        name: "Dark Navy",
        color: &[38, 27, 76],
    },
    ThreadRef {
        brand: BRAND,
        code: "1184",
        name: "Dark Navy",
        color: &[36, 32, 81],
    },
    ThreadRef {
        brand: BRAND,
        code: "1185",
        name: "Dark Blue/Green",
        color: &[58, 99, 94],
    },
    ThreadRef {
        brand: BRAND,
        code: "1186",
        name: "Light Pink",
        color: &[243, 227, 244],
    },
    ThreadRef {
        brand: BRAND,
        code: "1187",
        name: "Sugar Plum",
        color: &[160, 101, 172],
    },
    ThreadRef {
        brand: BRAND,
        code: "1188",
        name: "Dark Red S",
        color: &[140, 33, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "1189",
        name: "Plum",
        color: &[68, 39, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "1199",
        name: "Heather Pink",
        color: &[208, 131, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "1201",
        name: "Cilantro",
        color: &[102, 138, 129],
    },
    ThreadRef {
        brand: BRAND,
        code: "1207",
        name: "Olive Green",
        color: &[21, 43, 26],
    },
    ThreadRef {
        brand: BRAND,
        code: "1210",
        name: "Deep Purple",
        color: &[65, 47, 140],
    },
    ThreadRef {
        brand: BRAND,
        code: "1211",
        name: "Violet Blue",
        color: &[60, 42, 109],
    },
    ThreadRef {
        brand: BRAND,
        code: "1212",
        name: "Easter Green",
        color: &[125, 176, 121],
    },
    ThreadRef {
        brand: BRAND,
        code: "1218",
        name: "Willow",
        color: &[113, 129, 110],
    },
    ThreadRef {
        brand: BRAND,
        code: "1240",
        name: "Brown",
        color: &[134, 86, 40],
    },
    ThreadRef {
        brand: BRAND,
        code: "1241",
        name: "Charcoal",
        color: &[21, 19, 38],
    },
    ThreadRef {
        brand: BRAND,
        code: "1250",
        name: "Cobalt Bleu",
        color: &[202, 228, 228],
    },
    ThreadRef {
        brand: BRAND,
        code: "1254",
        name: "Silver Cloud",
        color: &[187, 184, 166],
    },
    ThreadRef {
        brand: BRAND,
        code: "1255",
        name: "Tawny Birch",
        color: &[195, 135, 106],
    },
    ThreadRef {
        brand: BRAND,
        code: "1263",
        name: "Cranberry",
        color: &[125, 6, 11],
    },
    ThreadRef {
        brand: BRAND,
        code: "1264",
        name: "Medium Lime",
        color: &[78, 171, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "1270",
        name: "Meilee Green",
        color: &[250, 255, 140],
    },
    ThreadRef {
        brand: BRAND,
        code: "1290",
        name: "Old Gold",
        color: &[200, 145, 54],
    },
    ThreadRef {
        brand: BRAND,
        code: "1300",
        name: "Pale Yellow",
        color: &[255, 254, 184],
    },
    ThreadRef {
        brand: BRAND,
        code: "1305",
        name: "Scarlet",
        color: &[243, 47, 23],
    },
    ThreadRef {
        brand: BRAND,
        code: "1308",
        name: "Coral Flame",
        color: &[197, 90, 97],
    },
    ThreadRef {
        brand: BRAND,
        code: "1326",
        name: "Green",
        color: &[19, 65, 31],
    },
    ThreadRef {
        brand: BRAND,
        code: "1331",
        name: "Marine Blue",
        color: &[87, 91, 206],
    },
    ThreadRef {
        brand: BRAND,
        code: "1334",
        name: "Cherokee Red",
        color: &[127, 27, 17],
    },
    ThreadRef {
        brand: BRAND,
        code: "1352",
        name: "Orange",
        color: &[255, 129, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "1353",
        name: "Dark Purple",
        color: &[31, 27, 57],
    },
    ThreadRef {
        brand: BRAND,
        code: "1355",
        name: "Midnight Navy",
        color: &[32, 25, 45],
    },
    ThreadRef {
        brand: BRAND,
        code: "1362",
        name: "Pooh",
        color: &[253, 188, 139],
    },
    ThreadRef {
        brand: BRAND,
        code: "1374",
        name: "Olive Green",
        color: &[12, 41, 30],
    },
    ThreadRef {
        brand: BRAND,
        code: "1421",
        name: "Imperial Blue",
        color: &[36, 98, 126],
    },
    ThreadRef {
        brand: BRAND,
        code: "1422",
        name: "Dark Green",
        color: &[7, 96, 32],
    },
    ThreadRef {
        brand: BRAND,
        code: "1430",
        name: "Pale Turquoise",
        color: &[131, 203, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "1442",
        name: "Dark Teal",
        color: &[20, 49, 78],
    },
    ThreadRef {
        brand: BRAND,
        code: "1463",
        name: "Juniper Medium Dark",
        color: &[92, 115, 85],
    },
    ThreadRef {
        brand: BRAND,
        code: "1515",
        name: "Flag Blue",
        color: &[24, 45, 61],
    },
    ThreadRef {
        brand: BRAND,
        code: "1532",
        name: "Coral Reef",
        color: &[255, 151, 130],
    },
    ThreadRef {
        brand: BRAND,
        code: "1536",
        name: "Dark Purple/Navy",
        color: &[50, 23, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "1541",
        name: "Dark Purple Blue",
        color: &[43, 23, 111],
    },
    ThreadRef {
        brand: BRAND,
        code: "1552",
        name: "Caper",
        color: &[133, 132, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "1602",
        name: "Dark Purple",
        color: &[25, 21, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "1610",
        name: "Jade",
        color: &[55, 135, 117],
    },
    ThreadRef {
        brand: BRAND,
        code: "1612",
        name: "Dark Navy",
        color: &[67, 66, 78],
    },
    ThreadRef {
        brand: BRAND,
        code: "1613",
        name: "Lt Jade",
        color: &[10, 168, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "1625",
        name: "Rust",
        color: &[214, 108, 22],
    },
    ThreadRef {
        brand: BRAND,
        code: "1640",
        name: "Yellow Cream",
        color: &[239, 220, 103],
    },
    ThreadRef {
        brand: BRAND,
        code: "1644",
        name: "Atlantic",
        color: &[27, 52, 70],
    },
    ThreadRef {
        brand: BRAND,
        code: "1672",
        name: "Light Brown",
        color: &[204, 182, 130],
    },
    ThreadRef {
        brand: BRAND,
        code: "1674",
        name: "Black",
        color: &[11, 14, 17],
    },
    ThreadRef {
        brand: BRAND,
        code: "1703",
        name: "Carmine",
        color: &[201, 11, 14],
    },
    ThreadRef {
        brand: BRAND,
        code: "1710",
        name: "Emerald Green",
        color: &[24, 154, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "1721",
        name: "Yellow",
        color: &[224, 217, 21],
    },
    ThreadRef {
        brand: BRAND,
        code: "1732",
        name: "Lime",
        color: &[179, 185, 41],
    },
    ThreadRef {
        brand: BRAND,
        code: "1741",
        name: "Terra Cotta",
        color: &[162, 55, 70],
    },
    ThreadRef {
        brand: BRAND,
        code: "1743",
        name: "Dark Navy",
        color: &[42, 54, 82],
    },
    ThreadRef {
        brand: BRAND,
        code: "1761",
        name: "Pale Salmon",
        color: &[227, 169, 172],
    },
    ThreadRef {
        brand: BRAND,
        code: "1776",
        name: "Dark Fawn",
        color: &[94, 80, 65],
    },
    ThreadRef {
        brand: BRAND,
        code: "1797",
        name: "Blue Twirl",
        color: &[25, 66, 136],
    },
    ThreadRef {
        brand: BRAND,
        code: "1800",
        name: "Sunkist",
        color: &[239, 69, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "1804",
        name: "Red",
        color: &[235, 31, 18],
    },
    ThreadRef {
        brand: BRAND,
        code: "1840",
        name: "Confetti",
        color: &[240, 115, 124],
    },
    ThreadRef {
        brand: BRAND,
        code: "1842",
        name: "Dark Grey/Blue",
        color: &[75, 96, 124],
    },
    ThreadRef {
        brand: BRAND,
        code: "1846",
        name: "Dark Brown",
        color: &[55, 21, 20],
    },
    ThreadRef {
        brand: BRAND,
        code: "1855",
        name: "Dark Oak",
        color: &[62, 24, 23],
    },
    ThreadRef {
        brand: BRAND,
        code: "1873",
        name: "Pale Green",
        color: &[147, 160, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "1874",
        name: "Medium Green/Yellow",
        color: &[145, 134, 120],
    },
    ThreadRef {
        brand: BRAND,
        code: "1900",
        name: "Red Jubilee",
        color: &[185, 8, 21],
    },
    ThreadRef {
        brand: BRAND,
        code: "1902",
        name: "Dark Red",
        color: &[161, 7, 19],
    },
    ThreadRef {
        brand: BRAND,
        code: "1906",
        name: "Dark Red",
        color: &[146, 8, 14],
    },
    ThreadRef {
        brand: BRAND,
        code: "1913",
        name: "Cranberry",
        color: &[137, 16, 13],
    },
    ThreadRef {
        brand: BRAND,
        code: "1921",
        name: "Rust",
        color: &[170, 44, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "1932",
        name: "Bark",
        color: &[134, 50, 19],
    },
    ThreadRef {
        brand: BRAND,
        code: "1934",
        name: "Dark Olive",
        color: &[64, 87, 10],
    },
    ThreadRef {
        brand: BRAND,
        code: "1940",
        name: "Brick Dark",
        color: &[183, 79, 16],
    },
    ThreadRef {
        brand: BRAND,
        code: "1941",
        name: "Old Gold",
        color: &[192, 142, 53],
    },
    ThreadRef {
        brand: BRAND,
        code: "1945",
        name: "Dark Brown",
        color: &[68, 43, 27],
    },
    ThreadRef {
        brand: BRAND,
        code: "1953",
        name: "Ash",
        color: &[59, 78, 108],
    },
    ThreadRef {
        brand: BRAND,
        code: "1971",
        name: "Winter Sage",
        color: &[187, 189, 187],
    },
    ThreadRef {
        brand: BRAND,
        code: "1988",
        name: "Veggie Green",
        color: &[9, 146, 41],
    },
    ThreadRef {
        brand: BRAND,
        code: "1999",
        name: "Black",
        color: &[10, 14, 16],
    },
    ThreadRef {
        brand: BRAND,
        code: "2010",
        name: "Orange Sorbet",
        color: &[240, 157, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "2045",
        name: "Purple",
        color: &[208, 145, 200],
    },
    ThreadRef {
        brand: BRAND,
        code: "2091",
        name: "Lark",
        color: &[185, 164, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "2106",
        name: "Hawaiian Sunrise",
        color: &[255, 228, 82],
    },
    ThreadRef {
        brand: BRAND,
        code: "2108",
        name: "Star Gold",
        color: &[255, 218, 103],
    },
    ThreadRef {
        brand: BRAND,
        code: "2111",
        name: "Medium Orange",
        color: &[244, 160, 90],
    },
    ThreadRef {
        brand: BRAND,
        code: "2120",
        name: "Daffodil",
        color: &[254, 255, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "2122",
        name: "Manila",
        color: &[255, 237, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "2131",
        name: "Cinder",
        color: &[144, 141, 143],
    },
    ThreadRef {
        brand: BRAND,
        code: "2132",
        name: "Bronze",
        color: &[130, 127, 1],
    },
    ThreadRef {
        brand: BRAND,
        code: "2133",
        name: "Meadow Green",
        color: &[105, 124, 2],
    },
    ThreadRef {
        brand: BRAND,
        code: "2152",
        name: "Petal Pink",
        color: &[220, 103, 125],
    },
    ThreadRef {
        brand: BRAND,
        code: "2202",
        name: "Carnation Ultra Light",
        color: &[254, 234, 236],
    },
    ThreadRef {
        brand: BRAND,
        code: "2203",
        name: "Pale Pink",
        color: &[250, 153, 180],
    },
    ThreadRef {
        brand: BRAND,
        code: "2204",
        name: "Light Cerise",
        color: &[240, 106, 132],
    },
    ThreadRef {
        brand: BRAND,
        code: "2205",
        name: "Medium Red/Orange",
        color: &[229, 66, 84],
    },
    ThreadRef {
        brand: BRAND,
        code: "2207",
        name: "Salmon",
        color: &[242, 149, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "2209",
        name: "Medium Red/Orange",
        color: &[230, 80, 84],
    },
    ThreadRef {
        brand: BRAND,
        code: "2210",
        name: "Light Orange/Red",
        color: &[168, 40, 27],
    },
    ThreadRef {
        brand: BRAND,
        code: "2211",
        name: "Red Jubilee",
        color: &[185, 10, 13],
    },
    ThreadRef {
        brand: BRAND,
        code: "2212",
        name: "Deep Scarlet",
        color: &[156, 8, 11],
    },
    ThreadRef {
        brand: BRAND,
        code: "2214",
        name: "Brown",
        color: &[93, 14, 18],
    },
    ThreadRef {
        brand: BRAND,
        code: "2216",
        name: "Black",
        color: &[39, 11, 13],
    },
    ThreadRef {
        brand: BRAND,
        code: "2228",
        name: "Peony Very Light",
        color: &[250, 228, 221],
    },
    ThreadRef {
        brand: BRAND,
        code: "2230",
        name: "Light Blue/Green",
        color: &[138, 211, 236],
    },
    ThreadRef {
        brand: BRAND,
        code: "2232",
        name: "Memphis Belle",
        color: &[252, 200, 214],
    },
    ThreadRef {
        brand: BRAND,
        code: "2241",
        name: "Sugar Plum",
        color: &[158, 110, 167],
    },
    ThreadRef {
        brand: BRAND,
        code: "2300",
        name: "Medium Red",
        color: &[184, 19, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "2301",
        name: "Orchid Pink",
        color: &[249, 202, 244],
    },
    ThreadRef {
        brand: BRAND,
        code: "2304",
        name: "Boysenberry",
        color: &[152, 48, 103],
    },
    ThreadRef {
        brand: BRAND,
        code: "2305",
        name: "White",
        color: &[254, 227, 249],
    },
    ThreadRef {
        brand: BRAND,
        code: "2308",
        name: "Cachet",
        color: &[118, 58, 147],
    },
    ThreadRef {
        brand: BRAND,
        code: "2310",
        name: "Medium Pink",
        color: &[215, 112, 187],
    },
    ThreadRef {
        brand: BRAND,
        code: "2335",
        name: "Dark Mint Green",
        color: &[27, 76, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "2403",
        name: "Ocean Blue",
        color: &[95, 127, 151],
    },
    ThreadRef {
        brand: BRAND,
        code: "2404",
        name: "Blueball",
        color: &[62, 114, 165],
    },
    ThreadRef {
        brand: BRAND,
        code: "2405",
        name: "Blue Dusk",
        color: &[36, 80, 136],
    },
    ThreadRef {
        brand: BRAND,
        code: "2410",
        name: "Dark Blue",
        color: &[29, 52, 147],
    },
    ThreadRef {
        brand: BRAND,
        code: "2411",
        name: "Provence",
        color: &[23, 42, 119],
    },
    ThreadRef {
        brand: BRAND,
        code: "2414",
        name: "Midnight Blue",
        color: &[28, 19, 42],
    },
    ThreadRef {
        brand: BRAND,
        code: "2428",
        name: "Paper White",
        color: &[252, 253, 253],
    },
    ThreadRef {
        brand: BRAND,
        code: "2430",
        name: "Little Boy Blue",
        color: &[124, 165, 213],
    },
    ThreadRef {
        brand: BRAND,
        code: "2431",
        name: "Corn Flower",
        color: &[83, 104, 182],
    },
    ThreadRef {
        brand: BRAND,
        code: "2434",
        name: "Medium Blue",
        color: &[81, 107, 159],
    },
    ThreadRef {
        brand: BRAND,
        code: "2438",
        name: "Medium Blue",
        color: &[161, 179, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "2439",
        name: "Team Blue",
        color: &[33, 28, 139],
    },
    ThreadRef {
        brand: BRAND,
        code: "2440",
        name: "Navy",
        color: &[22, 26, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "2441",
        name: "Medium blue",
        color: &[56, 160, 218],
    },
    ThreadRef {
        brand: BRAND,
        code: "2442",
        name: "Sapphire",
        color: &[53, 135, 208],
    },
    ThreadRef {
        brand: BRAND,
        code: "2443",
        name: "Magic Blue",
        color: &[34, 95, 178],
    },
    ThreadRef {
        brand: BRAND,
        code: "2501",
        name: "Palest Mint",
        color: &[223, 255, 227],
    },
    ThreadRef {
        brand: BRAND,
        code: "2502",
        name: "Pale Aqua",
        color: &[125, 205, 154],
    },
    ThreadRef {
        brand: BRAND,
        code: "2503",
        name: "Jade",
        color: &[17, 129, 92],
    },
    ThreadRef {
        brand: BRAND,
        code: "2504",
        name: "Pale Green",
        color: &[208, 255, 195],
    },
    ThreadRef {
        brand: BRAND,
        code: "2506",
        name: "Dark Maroon",
        color: &[118, 29, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "2507",
        name: "Poison Green",
        color: &[78, 196, 91],
    },
    ThreadRef {
        brand: BRAND,
        code: "2509",
        name: "Dark Green",
        color: &[7, 90, 30],
    },
    ThreadRef {
        brand: BRAND,
        code: "2510",
        name: "Medium Mint Green",
        color: &[155, 209, 147],
    },
    ThreadRef {
        brand: BRAND,
        code: "2511",
        name: "Pistachio Green dark",
        color: &[44, 92, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "2513",
        name: "Meilee Green",
        color: &[244, 255, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "2514",
        name: "Olive Green Light",
        color: &[189, 255, 9],
    },
    ThreadRef {
        brand: BRAND,
        code: "2515",
        name: "Peapod",
        color: &[150, 190, 24],
    },
    ThreadRef {
        brand: BRAND,
        code: "2518",
        name: "Blue Spruce",
        color: &[37, 65, 21],
    },
    ThreadRef {
        brand: BRAND,
        code: "2527",
        name: "Lime Green",
        color: &[161, 223, 19],
    },
    ThreadRef {
        brand: BRAND,
        code: "2531",
        name: "Green",
        color: &[8, 125, 35],
    },
    ThreadRef {
        brand: BRAND,
        code: "2538",
        name: "Palmetto",
        color: &[73, 107, 25],
    },
    ThreadRef {
        brand: BRAND,
        code: "2604",
        name: "Sand Dune",
        color: &[123, 91, 57],
    },
    ThreadRef {
        brand: BRAND,
        code: "2607",
        name: "Gold",
        color: &[225, 196, 100],
    },
    ThreadRef {
        brand: BRAND,
        code: "2608",
        name: "Artisan's Gold",
        color: &[221, 163, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "2610",
        name: "Coffee Dark",
        color: &[120, 50, 19],
    },
    ThreadRef {
        brand: BRAND,
        code: "2612",
        name: "Medium Honey",
        color: &[202, 161, 82],
    },
    ThreadRef {
        brand: BRAND,
        code: "2613",
        name: "Pollen Gold",
        color: &[218, 155, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "2615",
        name: "Peach Fuzz",
        color: &[253, 198, 156],
    },
    ThreadRef {
        brand: BRAND,
        code: "2616",
        name: "Dark Rust",
        color: &[184, 51, 41],
    },
    ThreadRef {
        brand: BRAND,
        code: "2618",
        name: "Burgundy",
        color: &[138, 36, 25],
    },
    ThreadRef {
        brand: BRAND,
        code: "2620",
        name: "Brown",
        color: &[127, 73, 45],
    },
    ThreadRef {
        brand: BRAND,
        code: "2621",
        name: "Black",
        color: &[42, 26, 16],
    },
    ThreadRef {
        brand: BRAND,
        code: "2623",
        name: "Espresso",
        color: &[20, 9, 8],
    },
    ThreadRef {
        brand: BRAND,
        code: "2645",
        name: "Pink Sand",
        color: &[226, 173, 137],
    },
    ThreadRef {
        brand: BRAND,
        code: "2648",
        name: "PaleYellow",
        color: &[255, 230, 158],
    },
    ThreadRef {
        brand: BRAND,
        code: "2650",
        name: "Rice Paper",
        color: &[217, 186, 97],
    },
    ThreadRef {
        brand: BRAND,
        code: "2651",
        name: "Medium Honey",
        color: &[207, 160, 77],
    },
    ThreadRef {
        brand: BRAND,
        code: "2674",
        name: "Petrol Blue",
        color: &[77, 82, 97],
    },
    ThreadRef {
        brand: BRAND,
        code: "2701",
        name: "Cream",
        color: &[217, 213, 185],
    },
    ThreadRef {
        brand: BRAND,
        code: "2702",
        name: "Aqua Gray",
        color: &[165, 171, 165],
    },
    ThreadRef {
        brand: BRAND,
        code: "2704",
        name: "Bleu Mist Dark",
        color: &[83, 94, 99],
    },
    ThreadRef {
        brand: BRAND,
        code: "2706",
        name: "Black",
        color: &[50, 58, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "2707",
        name: "Black",
        color: &[29, 33, 42],
    },
    ThreadRef {
        brand: BRAND,
        code: "2711",
        name: "Dark Purple",
        color: &[82, 28, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "2715",
        name: "Dk. Chestnut",
        color: &[72, 14, 46],
    },
    ThreadRef {
        brand: BRAND,
        code: "2810",
        name: "Medium Purple",
        color: &[91, 35, 78],
    },
    ThreadRef {
        brand: BRAND,
        code: "2822",
        name: "Apple Green",
        color: &[167, 215, 100],
    },
    ThreadRef {
        brand: BRAND,
        code: "2833",
        name: "Meadow Green",
        color: &[99, 144, 27],
    },
    ThreadRef {
        brand: BRAND,
        code: "2901",
        name: "Gold",
        color: &[195, 165, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "2910",
        name: "Gold",
        color: &[222, 250, 24],
    },
    ThreadRef {
        brand: BRAND,
        code: "2911",
        name: "Erin Green",
        color: &[63, 230, 7],
    },
    ThreadRef {
        brand: BRAND,
        code: "2912",
        name: "Deep Coral",
        color: &[241, 0, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "2920",
        name: "Pink",
        color: &[242, 54, 94],
    },
    ThreadRef {
        brand: BRAND,
        code: "2921",
        name: "Dusty Rose",
        color: &[254, 96, 129],
    },
    ThreadRef {
        brand: BRAND,
        code: "2922",
        name: "Fluo Pink",
        color: &[255, 64, 86],
    },
    ThreadRef {
        brand: BRAND,
        code: "2930",
        name: "Singh Mist",
        color: &[255, 165, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "2931",
        name: "Medium Orange",
        color: &[255, 131, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "2932",
        name: "Persimmon",
        color: &[255, 112, 84],
    },
    ThreadRef {
        brand: BRAND,
        code: "2937",
        name: "Crimson Red Light",
        color: &[255, 35, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "2940",
        name: "Fluorescent Green",
        color: &[68, 246, 30],
    },
    ThreadRef {
        brand: BRAND,
        code: "2946",
        name: "Sunkist",
        color: &[255, 73, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "2967",
        name: "Gold/Yellow",
        color: &[233, 255, 73],
    },
    ThreadRef {
        brand: BRAND,
        code: "2983",
        name: "Gold/Yellow",
        color: &[255, 240, 0],
    },
];