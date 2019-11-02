#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Admelody Poly";
pub const THREADS: [ThreadRef; 320] = [
    ThreadRef::new(
        BRAND,
        "6001",
        "Powder Pink",
        &[255, 187, 193],
    ),
    ThreadRef::new(
        BRAND,
        "6002",
        "Salmon Rose",
        &[252, 151, 158],
    ),
    ThreadRef::new(
        BRAND,
        "6003",
        "Geranium",
        &[182, 66, 75],
    ),
    ThreadRef::new(
        BRAND,
        "6004",
        "Hibiscus",
        &[207, 58, 64],
    ),
    ThreadRef::new(
        BRAND,
        "6005",
        "Fiery Red",
        &[223, 45, 52],
    ),
    ThreadRef::new(
        BRAND,
        "6006",
        "Calypso Coral",
        &[226, 77, 103],
    ),
    ThreadRef::new(
        BRAND,
        "6007",
        "Teaberry",
        &[206, 47, 82],
    ),
    ThreadRef::new(
        BRAND,
        "6008",
        "Barberry",
        &[155, 33, 60],
    ),
    ThreadRef::new(
        BRAND,
        "6009",
        "Rose Red",
        &[167, 44, 80],
    ),
    ThreadRef::new(
        BRAND,
        "6010",
        "Raspberry",
        &[171, 49, 73],
    ),
    ThreadRef::new(
        BRAND,
        "6011",
        "Crimson",
        &[126, 23, 42],
    ),
    ThreadRef::new(
        BRAND,
        "6012",
        "Candy Pink",
        &[250, 156, 195],
    ),
    ThreadRef::new(
        BRAND,
        "6013",
        "Carmine Rose",
        &[207, 91, 144],
    ),
    ThreadRef::new(
        BRAND,
        "6014",
        "Camellia Rose",
        &[231, 106, 148],
    ),
    ThreadRef::new(
        BRAND,
        "6015",
        "Beetroot Purple",
        &[169, 38, 94],
    ),
    ThreadRef::new(
        BRAND,
        "6016",
        "Fuchsia Purple",
        &[166, 44, 102],
    ),
    ThreadRef::new(
        BRAND,
        "6017",
        "Baton Rouge",
        &[113, 33, 75],
    ),
    ThreadRef::new(
        BRAND,
        "6018",
        "Raspberry Rose",
        &[197, 61, 153],
    ),
    ThreadRef::new(
        BRAND,
        "6019",
        "Fuchsia Red",
        &[147, 47, 119],
    ),
    ThreadRef::new(
        BRAND,
        "6020",
        "Hollyhock",
        &[95, 38, 92],
    ),
    ThreadRef::new(
        BRAND,
        "6021",
        "Blushing Bride",
        &[254, 206, 221],
    ),
    ThreadRef::new(
        BRAND,
        "6022",
        "Quartz Pink",
        &[255, 188, 204],
    ),
    ThreadRef::new(
        BRAND,
        "6023",
        "Pink Lemonade",
        &[255, 150, 174],
    ),
    ThreadRef::new(
        BRAND,
        "6024",
        "Rapture Rose",
        &[194, 109, 128],
    ),
    ThreadRef::new(
        BRAND,
        "6025",
        "Carmine",
        &[140, 70, 91],
    ),
    ThreadRef::new(
        BRAND,
        "6026",
        "Beet Red",
        &[92, 35, 56],
    ),
    ThreadRef::new(
        BRAND,
        "6027",
        "Anemone",
        &[85, 33, 56],
    ),
    ThreadRef::new(
        BRAND,
        "6028",
        "Orchid Haze",
        &[148, 116, 140],
    ),
    ThreadRef::new(
        BRAND,
        "6029",
        "Phlox",
        &[74, 39, 70],
    ),
    ThreadRef::new(
        BRAND,
        "6030",
        "Purple Passion",
        &[82, 49, 76],
    ),
    ThreadRef::new(
        BRAND,
        "6031",
        "Deep Purple",
        &[60, 40, 55],
    ),
    ThreadRef::new(
        BRAND,
        "6032",
        "Prune Purple",
        &[52, 37, 45],
    ),
    ThreadRef::new(
        BRAND,
        "6033",
        "Pink Dogwood",
        &[254, 216, 214],
    ),
    ThreadRef::new(
        BRAND,
        "6034",
        "Creole Pink",
        &[233, 195, 186],
    ),
    ThreadRef::new(
        BRAND,
        "6035",
        "Pale Mauve",
        &[187, 143, 145],
    ),
    ThreadRef::new(
        BRAND,
        "6036",
        "Chutney",
        &[123, 76, 72],
    ),
    ThreadRef::new(
        BRAND,
        "6037",
        "Mellow Mauve",
        &[112, 87, 90],
    ),
    ThreadRef::new(
        BRAND,
        "6038",
        "Mahogany",
        &[91, 60, 59],
    ),
    ThreadRef::new(
        BRAND,
        "6039",
        "Henna",
        &[82, 39, 32],
    ),
    ThreadRef::new(
        BRAND,
        "6040",
        "Deep Mahogany",
        &[59, 44, 43],
    ),
    ThreadRef::new(
        BRAND,
        "6041",
        "Flame Orange",
        &[222, 106, 0],
    ),
    ThreadRef::new(
        BRAND,
        "6042",
        "Orange Peel",
        &[230, 96, 10],
    ),
    ThreadRef::new(
        BRAND,
        "6043",
        "Vermillion Orange",
        &[221, 75, 13],
    ),
    ThreadRef::new(
        BRAND,
        "6044",
        "Spicy Orange",
        &[212, 57, 11],
    ),
    ThreadRef::new(
        BRAND,
        "6045",
        "Mandarin Red",
        &[173, 42, 21],
    ),
    ThreadRef::new(
        BRAND,
        "6046",
        "High Risk Red",
        &[171, 30, 29],
    ),
    ThreadRef::new(
        BRAND,
        "6047",
        "Flame Scarlet",
        &[170, 35, 36],
    ),
    ThreadRef::new(
        BRAND,
        "6048",
        "Aurora Red",
        &[151, 19, 27],
    ),
    ThreadRef::new(
        BRAND,
        "6049",
        "Formula One",
        &[133, 31, 34],
    ),
    ThreadRef::new(
        BRAND,
        "6050",
        "Apricot Blush",
        &[255, 170, 161],
    ),
    ThreadRef::new(
        BRAND,
        "6051",
        "Peach Amber",
        &[247, 136, 118],
    ),
    ThreadRef::new(
        BRAND,
        "6052",
        "Georgia Peach",
        &[223, 107, 102],
    ),
    ThreadRef::new(
        BRAND,
        "6053",
        "Emberglow",
        &[194, 78, 61],
    ),
    ThreadRef::new(
        BRAND,
        "6054",
        "Paprika",
        &[148, 50, 45],
    ),
    ThreadRef::new(
        BRAND,
        "6055",
        "Scarlet",
        &[115, 34, 33],
    ),
    ThreadRef::new(
        BRAND,
        "6056",
        "Tango Red",
        &[102, 29, 33],
    ),
    ThreadRef::new(
        BRAND,
        "6057",
        "Red Rust",
        &[119, 33, 44],
    ),
    ThreadRef::new(
        BRAND,
        "6058",
        "Deep Claret",
        &[96, 35, 45],
    ),
    ThreadRef::new(
        BRAND,
        "6059",
        "Red Bud",
        &[104, 33, 53],
    ),
    ThreadRef::new(
        BRAND,
        "6060",
        "Earth Red",
        &[111, 42, 58],
    ),
    ThreadRef::new(
        BRAND,
        "6061",
        "Pale Peach",
        &[252, 203, 169],
    ),
    ThreadRef::new(
        BRAND,
        "6062",
        "Spanish Villa",
        &[250, 204, 176],
    ),
    ThreadRef::new(
        BRAND,
        "6063",
        "Peach Nectar",
        &[249, 170, 146],
    ),
    ThreadRef::new(
        BRAND,
        "6064",
        "Shrimp",
        &[235, 149, 125],
    ),
    ThreadRef::new(
        BRAND,
        "6065",
        "Fresh Salmon",
        &[242, 116, 78],
    ),
    ThreadRef::new(
        BRAND,
        "6066",
        "Sunset",
        &[213, 82, 37],
    ),
    ThreadRef::new(
        BRAND,
        "6067",
        "Red Clay",
        &[148, 49, 32],
    ),
    ThreadRef::new(
        BRAND,
        "6068",
        "Burnt Ochre",
        &[119, 47, 27],
    ),
    ThreadRef::new(
        BRAND,
        "6069",
        "Cinnabar",
        &[114, 52, 48],
    ),
    ThreadRef::new(
        BRAND,
        "6070",
        "Pale Tan",
        &[185, 151, 126],
    ),
    ThreadRef::new(
        BRAND,
        "6071",
        "Strawberry Pink",
        &[174, 135, 102],
    ),
    ThreadRef::new(
        BRAND,
        "6072",
        "Ginger Bread",
        &[102, 54, 37],
    ),
    ThreadRef::new(
        BRAND,
        "6073",
        "Arabian Spice",
        &[85, 43, 30],
    ),
    ThreadRef::new(
        BRAND,
        "6074",
        "Tortoise Shell",
        &[98, 65, 51],
    ),
    ThreadRef::new(
        BRAND,
        "6075",
        "Friar Brown",
        &[69, 47, 37],
    ),
    ThreadRef::new(
        BRAND,
        "6076",
        "Peach Nougat",
        &[255, 180, 129],
    ),
    ThreadRef::new(
        BRAND,
        "6077",
        "Butterum",
        &[174, 100, 52],
    ),
    ThreadRef::new(
        BRAND,
        "6078",
        "Caramel",
        &[175, 75, 23],
    ),
    ThreadRef::new(
        BRAND,
        "6079",
        "Rust",
        &[144, 74, 39],
    ),
    ThreadRef::new(
        BRAND,
        "6080",
        "Amberglow",
        &[138, 64, 38],
    ),
    ThreadRef::new(
        BRAND,
        "6081",
        "Mimosa",
        &[223, 164, 48],
    ),
    ThreadRef::new(
        BRAND,
        "6082",
        "Arrowwood",
        &[193, 133, 39],
    ),
    ThreadRef::new(
        BRAND,
        "6083",
        "Golden Yellow",
        &[186, 119, 13],
    ),
    ThreadRef::new(
        BRAND,
        "6084",
        "Artisa's Gold",
        &[214, 133, 36],
    ),
    ThreadRef::new(
        BRAND,
        "6085",
        "Citrus",
        &[245, 141, 0],
    ),
    ThreadRef::new(
        BRAND,
        "6086",
        "Old Gold",
        &[249, 147, 0],
    ),
    ThreadRef::new(
        BRAND,
        "6087",
        "Marigold",
        &[240, 153, 68],
    ),
    ThreadRef::new(
        BRAND,
        "6088",
        "Radiant Yellow",
        &[211, 119, 17],
    ),
    ThreadRef::new(
        BRAND,
        "6089",
        "Flax",
        &[231, 165, 90],
    ),
    ThreadRef::new(
        BRAND,
        "6090",
        "Inca Gold",
        &[169, 102, 29],
    ),
    ThreadRef::new(
        BRAND,
        "6091",
        "Buckthorn Brown",
        &[124, 80, 33],
    ),
    ThreadRef::new(
        BRAND,
        "6092",
        "Glazed Ginger",
        &[122, 72, 34],
    ),
    ThreadRef::new(
        BRAND,
        "6093",
        "Chipmunk",
        &[110, 78, 51],
    ),
    ThreadRef::new(
        BRAND,
        "6094",
        "Monks Robe",
        &[91, 61, 35],
    ),
    ThreadRef::new(
        BRAND,
        "6095",
        "Golden Tan",
        &[208, 168, 94],
    ),
    ThreadRef::new(
        BRAND,
        "6096",
        "Parsnip",
        &[190, 164, 107],
    ),
    ThreadRef::new(
        BRAND,
        "6097",
        "Willow",
        &[145, 129, 84],
    ),
    ThreadRef::new(
        BRAND,
        "6098",
        "Fall Leaf",
        &[159, 132, 76],
    ),
    ThreadRef::new(
        BRAND,
        "6099",
        "Honey",
        &[159, 112, 44],
    ),
    ThreadRef::new(
        BRAND,
        "6100",
        "Green Sulphur",
        &[152, 112, 38],
    ),
    ThreadRef::new(
        BRAND,
        "6101",
        "Pearly Cream",
        &[250, 247, 220],
    ),
    ThreadRef::new(
        BRAND,
        "6102",
        "Papyrus",
        &[234, 234, 199],
    ),
    ThreadRef::new(
        BRAND,
        "6103",
        "Rutabaga",
        &[248, 236, 189],
    ),
    ThreadRef::new(
        BRAND,
        "6104",
        "Wood Ash",
        &[202, 190, 156],
    ),
    ThreadRef::new(
        BRAND,
        "6105",
        "Glass Green",
        &[217, 217, 176],
    ),
    ThreadRef::new(
        BRAND,
        "6106",
        "Leomonade",
        &[249, 235, 134],
    ),
    ThreadRef::new(
        BRAND,
        "6107",
        "Custard",
        &[252, 216, 112],
    ),
    ThreadRef::new(
        BRAND,
        "6108",
        "Cornsilk",
        &[249, 222, 96],
    ),
    ThreadRef::new(
        BRAND,
        "6109",
        "Aurora",
        &[254, 221, 78],
    ),
    ThreadRef::new(
        BRAND,
        "6110",
        "Empire Yellow",
        &[255, 225, 0],
    ),
    ThreadRef::new(
        BRAND,
        "6111",
        "Blazing Yellow",
        &[253, 237, 60],
    ),
    ThreadRef::new(
        BRAND,
        "6112",
        "Vibrant Yellow",
        &[255, 206, 0],
    ),
    ThreadRef::new(
        BRAND,
        "6113",
        "Goldenrod",
        &[253, 196, 56],
    ),
    ThreadRef::new(
        BRAND,
        "6114",
        "Dandelion",
        &[255, 195, 0],
    ),
    ThreadRef::new(
        BRAND,
        "6115",
        "Freesia",
        &[209, 149, 13],
    ),
    ThreadRef::new(
        BRAND,
        "6116",
        "Lemon Chrome",
        &[235, 163, 13],
    ),
    ThreadRef::new(
        BRAND,
        "6117",
        "Snapdragon",
        &[255, 213, 114],
    ),
    ThreadRef::new(
        BRAND,
        "6118",
        "Sulphur",
        &[229, 179, 29],
    ),
    ThreadRef::new(
        BRAND,
        "6119",
        "Oil Yellow",
        &[157, 121, 27],
    ),
    ThreadRef::new(
        BRAND,
        "6120",
        "Golden Palm",
        &[156, 130, 0],
    ),
    ThreadRef::new(
        BRAND,
        "6121",
        "Orchid Bouquet",
        &[206, 186, 226],
    ),
    ThreadRef::new(
        BRAND,
        "6122",
        "Lavender Herb",
        &[141, 117, 157],
    ),
    ThreadRef::new(
        BRAND,
        "6123",
        "Crocus",
        &[158, 113, 161],
    ),
    ThreadRef::new(
        BRAND,
        "6124",
        "Iris Orchid",
        &[147, 101, 161],
    ),
    ThreadRef::new(
        BRAND,
        "6125",
        "Hyacinth",
        &[140, 106, 173],
    ),
    ThreadRef::new(
        BRAND,
        "6126",
        "Dahlia",
        &[103, 49, 129],
    ),
    ThreadRef::new(
        BRAND,
        "6127",
        "Redish Purple",
        &[105, 38, 110],
    ),
    ThreadRef::new(
        BRAND,
        "6128",
        "Amaranth Purple",
        &[72, 24, 87],
    ),
    ThreadRef::new(
        BRAND,
        "6129",
        "Clover",
        &[94, 50, 92],
    ),
    ThreadRef::new(
        BRAND,
        "6130",
        "Grape Juice",
        &[75, 50, 85],
    ),
    ThreadRef::new(
        BRAND,
        "6131",
        "Pastel Lilac",
        &[154, 153, 199],
    ),
    ThreadRef::new(
        BRAND,
        "6132",
        "Violet Tulip",
        &[121, 129, 181],
    ),
    ThreadRef::new(
        BRAND,
        "6133",
        "Passion Flower",
        &[91, 75, 147],
    ),
    ThreadRef::new(
        BRAND,
        "6134",
        "Helliotrope",
        &[63, 41, 117],
    ),
    ThreadRef::new(
        BRAND,
        "6135",
        "Black Purple",
        &[46, 38, 106],
    ),
    ThreadRef::new(
        BRAND,
        "6136",
        "Gentian Violet",
        &[48, 42, 85],
    ),
    ThreadRef::new(
        BRAND,
        "6137",
        "Darkest Purple",
        &[48, 42, 65],
    ),
    ThreadRef::new(
        BRAND,
        "6138",
        "Lupine",
        &[158, 140, 182],
    ),
    ThreadRef::new(
        BRAND,
        "6139",
        "Plum Gray",
        &[113, 111, 134],
    ),
    ThreadRef::new(
        BRAND,
        "6140",
        "Mysterioso",
        &[33, 33, 41],
    ),
    ThreadRef::new(
        BRAND,
        "6141",
        "Baby Blue",
        &[172, 214, 240],
    ),
    ThreadRef::new(
        BRAND,
        "6142",
        "Powder Blue",
        &[151, 192, 228],
    ),
    ThreadRef::new(
        BRAND,
        "6143",
        "Angel Falls",
        &[148, 197, 234],
    ),
    ThreadRef::new(
        BRAND,
        "6144",
        "Dusk Blue",
        &[115, 172, 208],
    ),
    ThreadRef::new(
        BRAND,
        "6145",
        "Placid Blue",
        &[124, 175, 222],
    ),
    ThreadRef::new(
        BRAND,
        "6146",
        "Azure Blue",
        &[64, 126, 187],
    ),
    ThreadRef::new(
        BRAND,
        "6147",
        "Turkish Sea",
        &[38, 83, 146],
    ),
    ThreadRef::new(
        BRAND,
        "6148",
        "True Blue",
        &[18, 56, 106],
    ),
    ThreadRef::new(
        BRAND,
        "6149",
        "Grapemist",
        &[101, 137, 202],
    ),
    ThreadRef::new(
        BRAND,
        "6150",
        "Baja blue",
        &[84, 107, 174],
    ),
    ThreadRef::new(
        BRAND,
        "6151",
        "Blue Iris",
        &[64, 73, 149],
    ),
    ThreadRef::new(
        BRAND,
        "6152",
        "Clematis blue",
        &[33, 42, 114],
    ),
    ThreadRef::new(
        BRAND,
        "6153",
        "Dazzling Blue",
        &[32, 59, 120],
    ),
    ThreadRef::new(
        BRAND,
        "6154",
        "Bright Navy",
        &[28, 46, 100],
    ),
    ThreadRef::new(
        BRAND,
        "6155",
        "Blueprint",
        &[27, 46, 84],
    ),
    ThreadRef::new(
        BRAND,
        "6156",
        "Mazarine Blue",
        &[35, 44, 80],
    ),
    ThreadRef::new(
        BRAND,
        "6157",
        "True navy",
        &[27, 47, 78],
    ),
    ThreadRef::new(
        BRAND,
        "6158",
        "Insignia Blue",
        &[29, 47, 64],
    ),
    ThreadRef::new(
        BRAND,
        "6159",
        "Mood Indigo",
        &[29, 40, 50],
    ),
    ThreadRef::new(
        BRAND,
        "6160",
        "Eclipse Blue",
        &[29, 35, 46],
    ),
    ThreadRef::new(
        BRAND,
        "6161",
        "Light Turquoise",
        &[96, 190, 222],
    ),
    ThreadRef::new(
        BRAND,
        "6162",
        "Dresdon Blue",
        &[0, 134, 186],
    ),
    ThreadRef::new(
        BRAND,
        "6163",
        "Blue Aster",
        &[0, 95, 150],
    ),
    ThreadRef::new(
        BRAND,
        "6164",
        "Victoria Blue",
        &[0, 84, 143],
    ),
    ThreadRef::new(
        BRAND,
        "6165",
        "Campanula",
        &[31, 97, 157],
    ),
    ThreadRef::new(
        BRAND,
        "6166",
        "Strong Blue",
        &[7, 66, 125],
    ),
    ThreadRef::new(
        BRAND,
        "6167",
        "Moonlight Blue",
        &[20, 49, 80],
    ),
    ThreadRef::new(
        BRAND,
        "6168",
        "Winter Sky",
        &[134, 166, 181],
    ),
    ThreadRef::new(
        BRAND,
        "6169",
        "Blue Bell",
        &[125, 169, 199],
    ),
    ThreadRef::new(
        BRAND,
        "6170",
        "Coronet Blue",
        &[89, 131, 167],
    ),
    ThreadRef::new(
        BRAND,
        "6171",
        "Niagra",
        &[65, 129, 164],
    ),
    ThreadRef::new(
        BRAND,
        "6172",
        "Cendre Blue",
        &[55, 105, 139],
    ),
    ThreadRef::new(
        BRAND,
        "6173",
        "Storm Blue",
        &[47, 116, 139],
    ),
    ThreadRef::new(
        BRAND,
        "6174",
        "Citadel",
        &[68, 100, 115],
    ),
    ThreadRef::new(
        BRAND,
        "6175",
        "Ink blue",
        &[24, 75, 94],
    ),
    ThreadRef::new(
        BRAND,
        "6176",
        "Federal Blue",
        &[37, 77, 114],
    ),
    ThreadRef::new(
        BRAND,
        "6177",
        "Stellar Blue",
        &[24, 70, 99],
    ),
    ThreadRef::new(
        BRAND,
        "6178",
        "Dark Denim",
        &[35, 63, 85],
    ),
    ThreadRef::new(
        BRAND,
        "6179",
        "Indian Teal",
        &[26, 54, 65],
    ),
    ThreadRef::new(
        BRAND,
        "6180",
        "Black Iris",
        &[27, 43, 56],
    ),
    ThreadRef::new(
        BRAND,
        "6181",
        "Birch",
        &[209, 200, 177],
    ),
    ThreadRef::new(
        BRAND,
        "6182",
        "Feather Gray",
        &[180, 167, 145],
    ),
    ThreadRef::new(
        BRAND,
        "6183",
        "Apricot Gelao",
        &[248, 216, 160],
    ),
    ThreadRef::new(
        BRAND,
        "6184",
        "Almond Buff",
        &[207, 181, 142],
    ),
    ThreadRef::new(
        BRAND,
        "6185",
        "Lark",
        &[174, 144, 99],
    ),
    ThreadRef::new(
        BRAND,
        "6186",
        "Tan",
        &[163, 132, 95],
    ),
    ThreadRef::new(
        BRAND,
        "6187",
        "Doe",
        &[142, 122, 95],
    ),
    ThreadRef::new(
        BRAND,
        "6188",
        "Dried Tobacco",
        &[117, 88, 42],
    ),
    ThreadRef::new(
        BRAND,
        "6189",
        "Apricot Illusion",
        &[249, 207, 159],
    ),
    ThreadRef::new(
        BRAND,
        "6190",
        "Sheepskin",
        &[213, 179, 143],
    ),
    ThreadRef::new(
        BRAND,
        "6191",
        "Biscuit",
        &[162, 117, 72],
    ),
    ThreadRef::new(
        BRAND,
        "6192",
        "Simply Taupe",
        &[134, 126, 112],
    ),
    ThreadRef::new(
        BRAND,
        "6193",
        "Natural",
        &[138, 117, 102],
    ),
    ThreadRef::new(
        BRAND,
        "6194",
        "Partridge",
        &[103, 84, 72],
    ),
    ThreadRef::new(
        BRAND,
        "6195",
        "Dark Sand",
        &[102, 92, 75],
    ),
    ThreadRef::new(
        BRAND,
        "6196",
        "Chocolate Chip",
        &[84, 76, 65],
    ),
    ThreadRef::new(
        BRAND,
        "6197",
        "Canteen",
        &[86, 76, 63],
    ),
    ThreadRef::new(
        BRAND,
        "6198",
        "Mink",
        &[122, 106, 85],
    ),
    ThreadRef::new(
        BRAND,
        "6199",
        "Kangaroo",
        &[92, 75, 48],
    ),
    ThreadRef::new(
        BRAND,
        "6200",
        "Sepia",
        &[87, 73, 54],
    ),
    ThreadRef::new(
        BRAND,
        "6201",
        "Thistle",
        &[197, 205, 230],
    ),
    ThreadRef::new(
        BRAND,
        "6202",
        "Gull",
        &[106, 106, 110],
    ),
    ThreadRef::new(
        BRAND,
        "6203",
        "Excalibur",
        &[101, 106, 115],
    ),
    ThreadRef::new(
        BRAND,
        "6204",
        "Graystone",
        &[71, 71, 81],
    ),
    ThreadRef::new(
        BRAND,
        "6205",
        "Shark",
        &[97, 102, 108],
    ),
    ThreadRef::new(
        BRAND,
        "6206",
        "Griffin",
        &[134, 141, 143],
    ),
    ThreadRef::new(
        BRAND,
        "6207",
        "Pearl Blue",
        &[158, 171, 177],
    ),
    ThreadRef::new(
        BRAND,
        "6208",
        "Quarry",
        &[148, 160, 166],
    ),
    ThreadRef::new(
        BRAND,
        "6209",
        "Celestial Blue",
        &[157, 174, 189],
    ),
    ThreadRef::new(
        BRAND,
        "6210",
        "Blue Shadow",
        &[116, 143, 163],
    ),
    ThreadRef::new(
        BRAND,
        "6211",
        "Captains Blue",
        &[71, 101, 124],
    ),
    ThreadRef::new(
        BRAND,
        "6212",
        "Ensign Blue",
        &[74, 92, 113],
    ),
    ThreadRef::new(
        BRAND,
        "6213",
        "Estate Blue",
        &[59, 75, 96],
    ),
    ThreadRef::new(
        BRAND,
        "6214",
        "Monument",
        &[102, 116, 129],
    ),
    ThreadRef::new(
        BRAND,
        "6215",
        "Stargazar",
        &[62, 76, 82],
    ),
    ThreadRef::new(
        BRAND,
        "6216",
        "Moonless Night",
        &[35, 43, 46],
    ),
    ThreadRef::new(
        BRAND,
        "6217",
        "Opal Gray",
        &[107, 113, 110],
    ),
    ThreadRef::new(
        BRAND,
        "6218",
        "Charcoal Gray",
        &[104, 107, 104],
    ),
    ThreadRef::new(
        BRAND,
        "6219",
        "Gargoyle",
        &[93, 99, 98],
    ),
    ThreadRef::new(
        BRAND,
        "6220",
        "Plum Kitten",
        &[88, 92, 92],
    ),
    ThreadRef::new(
        BRAND,
        "6221",
        "Wind Chime",
        &[198, 195, 194],
    ),
    ThreadRef::new(
        BRAND,
        "6222",
        "Silver Gray",
        &[184, 176, 170],
    ),
    ThreadRef::new(
        BRAND,
        "6223",
        "Stucco",
        &[152, 129, 121],
    ),
    ThreadRef::new(
        BRAND,
        "6224",
        "Fudgesickle",
        &[81, 63, 55],
    ),
    ThreadRef::new(
        BRAND,
        "6225",
        "Chocolate Brown",
        &[52, 46, 42],
    ),
    ThreadRef::new(
        BRAND,
        "6226",
        "Shale",
        &[47, 47, 47],
    ),
    ThreadRef::new(
        BRAND,
        "6227",
        "White Sand",
        &[229, 231, 225],
    ),
    ThreadRef::new(
        BRAND,
        "6228",
        "Silver Birch",
        &[198, 208, 208],
    ),
    ThreadRef::new(
        BRAND,
        "6229",
        "Gray violet",
        &[175, 182, 178],
    ),
    ThreadRef::new(
        BRAND,
        "6230",
        "High Rise",
        &[179, 189, 189],
    ),
    ThreadRef::new(
        BRAND,
        "6231",
        "Limestone",
        &[140, 153, 151],
    ),
    ThreadRef::new(
        BRAND,
        "6232",
        "Cloudburst",
        &[106, 107, 103],
    ),
    ThreadRef::new(
        BRAND,
        "6233",
        "Cement",
        &[166, 163, 145],
    ),
    ThreadRef::new(
        BRAND,
        "6234",
        "Spray Green",
        &[144, 141, 125],
    ),
    ThreadRef::new(
        BRAND,
        "6235",
        "Seneca Rock",
        &[138, 134, 117],
    ),
    ThreadRef::new(
        BRAND,
        "6236",
        "Timber Wolf",
        &[94, 87, 69],
    ),
    ThreadRef::new(
        BRAND,
        "6237",
        "Olive Drab",
        &[62, 58, 36],
    ),
    ThreadRef::new(
        BRAND,
        "6238",
        "Brindle",
        &[85, 85, 76],
    ),
    ThreadRef::new(
        BRAND,
        "6239",
        "Beluga",
        &[57, 61, 53],
    ),
    ThreadRef::new(
        BRAND,
        "6240",
        "Dark gull Gray",
        &[57, 59, 58],
    ),
    ThreadRef::new(
        BRAND,
        "6241",
        "Egret",
        &[250, 252, 240],
    ),
    ThreadRef::new(
        BRAND,
        "6242",
        "Snow White",
        &[249, 253, 251],
    ),
    ThreadRef::new(
        BRAND,
        "6243",
        "Bright White",
        &[240, 245, 255],
    ),
    ThreadRef::new(
        BRAND,
        "6244",
        "Blanc de Blanc",
        &[226, 235, 255],
    ),
    ThreadRef::new(
        BRAND,
        "6245",
        "Cloud Dancer",
        &[222, 230, 255],
    ),
    ThreadRef::new(
        BRAND,
        "6246",
        "Caviar",
        &[29, 34, 31],
    ),
    ThreadRef::new(
        BRAND,
        "6247",
        "Jasmine Green",
        &[186, 244, 123],
    ),
    ThreadRef::new(
        BRAND,
        "6248",
        "Grass Green",
        &[103, 145, 75],
    ),
    ThreadRef::new(
        BRAND,
        "6249",
        "Bud Green",
        &[116, 166, 77],
    ),
    ThreadRef::new(
        BRAND,
        "6250",
        "Online Lime",
        &[67, 106, 51],
    ),
    ThreadRef::new(
        BRAND,
        "6251",
        "Sulphur Spring",
        &[213, 217, 56],
    ),
    ThreadRef::new(
        BRAND,
        "6252",
        "Parrot Green",
        &[123, 157, 30],
    ),
    ThreadRef::new(
        BRAND,
        "6253",
        "Peridot",
        &[79, 105, 29],
    ),
    ThreadRef::new(
        BRAND,
        "6254",
        "Island Green",
        &[75, 156, 33],
    ),
    ThreadRef::new(
        BRAND,
        "6255",
        "Classic Green",
        &[52, 152, 52],
    ),
    ThreadRef::new(
        BRAND,
        "6256",
        "Fern Green",
        &[25, 139, 57],
    ),
    ThreadRef::new(
        BRAND,
        "6257",
        "Vibrant Green",
        &[58, 119, 46],
    ),
    ThreadRef::new(
        BRAND,
        "6258",
        "Bright Green",
        &[0, 126, 56],
    ),
    ThreadRef::new(
        BRAND,
        "6259",
        "Jelly Bean",
        &[0, 121, 66],
    ),
    ThreadRef::new(
        BRAND,
        "6260",
        "Verdant Green",
        &[1, 81, 41],
    ),
    ThreadRef::new(
        BRAND,
        "6261",
        "Eggshell Blue",
        &[162, 207, 202],
    ),
    ThreadRef::new(
        BRAND,
        "6262",
        "Blue Radiance",
        &[125, 207, 208],
    ),
    ThreadRef::new(
        BRAND,
        "6263",
        "Aquamarine",
        &[140, 212, 242],
    ),
    ThreadRef::new(
        BRAND,
        "6264",
        "Aquarius",
        &[66, 179, 210],
    ),
    ThreadRef::new(
        BRAND,
        "6265",
        "Blue Atoll",
        &[61, 172, 186],
    ),
    ThreadRef::new(
        BRAND,
        "6266",
        "Horizon Blue",
        &[35, 150, 175],
    ),
    ThreadRef::new(
        BRAND,
        "6267",
        "Blue Danube",
        &[0, 139, 169],
    ),
    ThreadRef::new(
        BRAND,
        "6268",
        "Blue Jewel",
        &[0, 110, 133],
    ),
    ThreadRef::new(
        BRAND,
        "6269",
        "Lyons Blue",
        &[0, 94, 115],
    ),
    ThreadRef::new(
        BRAND,
        "6270",
        "Pool Blue",
        &[120, 208, 189],
    ),
    ThreadRef::new(
        BRAND,
        "6271",
        "Blue Turquoise",
        &[61, 167, 159],
    ),
    ThreadRef::new(
        BRAND,
        "6272",
        "Ceramic",
        &[40, 166, 153],
    ),
    ThreadRef::new(
        BRAND,
        "6273",
        "Tile Blue",
        &[0, 143, 142],
    ),
    ThreadRef::new(
        BRAND,
        "6274",
        "Bluebird",
        &[26, 155, 159],
    ),
    ThreadRef::new(
        BRAND,
        "6275",
        "Enamel Blue",
        &[0, 104, 109],
    ),
    ThreadRef::new(
        BRAND,
        "6276",
        "Lake Blue",
        &[0, 121, 125],
    ),
    ThreadRef::new(
        BRAND,
        "6277",
        "Viridian Green",
        &[0, 131, 120],
    ),
    ThreadRef::new(
        BRAND,
        "6278",
        "Ocean Depths",
        &[5, 80, 90],
    ),
    ThreadRef::new(
        BRAND,
        "6279",
        "Deep Teal",
        &[12, 64, 66],
    ),
    ThreadRef::new(
        BRAND,
        "6280",
        "June Bug",
        &[29, 59, 55],
    ),
    ThreadRef::new(
        BRAND,
        "6281",
        "Sprucestone",
        &[156, 188, 154],
    ),
    ThreadRef::new(
        BRAND,
        "6282",
        "Hemlock",
        &[148, 197, 159],
    ),
    ThreadRef::new(
        BRAND,
        "6283",
        "Spring Bouquet",
        &[122, 205, 140],
    ),
    ThreadRef::new(
        BRAND,
        "6284",
        "Cabbage",
        &[120, 206, 163],
    ),
    ThreadRef::new(
        BRAND,
        "6285",
        "Marine Green",
        &[81, 174, 146],
    ),
    ThreadRef::new(
        BRAND,
        "6286",
        "Simply Green",
        &[4, 145, 106],
    ),
    ThreadRef::new(
        BRAND,
        "6287",
        "Deep Mint",
        &[30, 161, 98],
    ),
    ThreadRef::new(
        BRAND,
        "6288",
        "Bright Jade",
        &[0, 112, 70],
    ),
    ThreadRef::new(
        BRAND,
        "6289",
        "Pepper Green",
        &[0, 88, 57],
    ),
    ThreadRef::new(
        BRAND,
        "6290",
        "Darkest Jade",
        &[13, 82, 50],
    ),
    ThreadRef::new(
        BRAND,
        "6291",
        "Green-Blue Slate",
        &[56, 112, 111],
    ),
    ThreadRef::new(
        BRAND,
        "6292",
        "Greenlake",
        &[36, 116, 96],
    ),
    ThreadRef::new(
        BRAND,
        "6293",
        "Cadmium Green",
        &[0, 128, 105],
    ),
    ThreadRef::new(
        BRAND,
        "6294",
        "Ultramarine Green",
        &[0, 91, 70],
    ),
    ThreadRef::new(
        BRAND,
        "6295",
        "Parasailing",
        &[0, 84, 69],
    ),
    ThreadRef::new(
        BRAND,
        "6296",
        "Fairway",
        &[42, 75, 48],
    ),
    ThreadRef::new(
        BRAND,
        "6297",
        "Fir",
        &[47, 78, 64],
    ),
    ThreadRef::new(
        BRAND,
        "6298",
        "Foliage Green",
        &[35, 64, 41],
    ),
    ThreadRef::new(
        BRAND,
        "6299",
        "Alpine Green",
        &[21, 59, 46],
    ),
    ThreadRef::new(
        BRAND,
        "6300",
        "Shaded Spruce",
        &[18, 66, 63],
    ),
    ThreadRef::new(
        BRAND,
        "6301",
        "Lettuce Green",
        &[172, 190, 125],
    ),
    ThreadRef::new(
        BRAND,
        "6302",
        "Herbal Garden",
        &[145, 163, 101],
    ),
    ThreadRef::new(
        BRAND,
        "6303",
        "Pastel Yellow",
        &[255, 251, 191],
    ),
    ThreadRef::new(
        BRAND,
        "6304",
        "Shadow Green",
        &[204, 195, 123],
    ),
    ThreadRef::new(
        BRAND,
        "6305",
        "Golden Green",
        &[180, 162, 72],
    ),
    ThreadRef::new(
        BRAND,
        "6306",
        "Golden Olive",
        &[142, 121, 41],
    ),
    ThreadRef::new(
        BRAND,
        "6307",
        "Ecru Olive",
        &[103, 87, 28],
    ),
    ThreadRef::new(
        BRAND,
        "6308",
        "Moss",
        &[131, 124, 73],
    ),
    ThreadRef::new(
        BRAND,
        "6309",
        "Oasis",
        &[133, 125, 54],
    ),
    ThreadRef::new(
        BRAND,
        "6310",
        "Woodbine",
        &[111, 99, 33],
    ),
    ThreadRef::new(
        BRAND,
        "6311",
        "Calla Green",
        &[80, 83, 32],
    ),
    ThreadRef::new(
        BRAND,
        "6312",
        "Olive",
        &[79, 76, 30],
    ),
    ThreadRef::new(
        BRAND,
        "6313",
        "Cypress",
        &[63, 63, 40],
    ),
    ThreadRef::new(
        BRAND,
        "6314",
        "Avocado",
        &[68, 66, 31],
    ),
    ThreadRef::new(
        BRAND,
        "6315",
        "Olive Branch",
        &[99, 101, 76],
    ),
    ThreadRef::new(
        BRAND,
        "6316",
        "Green Bay",
        &[98, 114, 98],
    ),
    ThreadRef::new(
        BRAND,
        "6317",
        "Elm Green",
        &[62, 83, 61],
    ),
    ThreadRef::new(
        BRAND,
        "6318",
        "Bronze Green",
        &[46, 60, 37],
    ),
    ThreadRef::new(
        BRAND,
        "6319",
        "Pineneedle",
        &[52, 64, 52],
    ),
    ThreadRef::new(
        BRAND,
        "6320",
        "Kombu Green",
        &[50, 54, 39],
    ),
];