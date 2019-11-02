#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "Gunold COTTY";
const THREADS: [ThreadRef; 118] = [
    ThreadRef {
        brand: BRAND,
        code: "100",
        name: "Snow White",
        color: &[255, 255, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "101",
        name: "N/A",
        color: &[221, 8, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "102",
        name: "Deep Pink",
        color: &[191, 27, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "103",
        name: "Raspberry",
        color: &[139, 33, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "104",
        name: "Orangeade",
        color: &[232, 78, 15],
    },
    ThreadRef {
        brand: BRAND,
        code: "105",
        name: "Honeysuckle",
        color: &[230, 51, 35],
    },
    ThreadRef {
        brand: BRAND,
        code: "106",
        name: "Granet Very Dark",
        color: &[135, 28, 77],
    },
    ThreadRef {
        brand: BRAND,
        code: "107",
        name: "Sangria",
        color: &[198, 26, 39],
    },
    ThreadRef {
        brand: BRAND,
        code: "112",
        name: "Dark Teal",
        color: &[0, 81, 84],
    },
    ThreadRef {
        brand: BRAND,
        code: "113",
        name: "Frosty Spruce",
        color: &[0, 97, 74],
    },
    ThreadRef {
        brand: BRAND,
        code: "114",
        name: "Fresh green",
        color: &[77, 158, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "115",
        name: "Dark Green",
        color: &[0, 111, 76],
    },
    ThreadRef {
        brand: BRAND,
        code: "116",
        name: "Jungle Green",
        color: &[0, 115, 65],
    },
    ThreadRef {
        brand: BRAND,
        code: "117",
        name: "Irish Spring",
        color: &[0, 124, 102],
    },
    ThreadRef {
        brand: BRAND,
        code: "118",
        name: "Coachman Green",
        color: &[0, 78, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "120",
        name: "Blue Satin",
        color: &[0, 103, 119],
    },
    ThreadRef {
        brand: BRAND,
        code: "122",
        name: "Blue",
        color: &[20, 56, 127],
    },
    ThreadRef {
        brand: BRAND,
        code: "124",
        name: "Eggplant",
        color: &[26, 29, 86],
    },
    ThreadRef {
        brand: BRAND,
        code: "125",
        name: "Teal",
        color: &[0, 118, 156],
    },
    ThreadRef {
        brand: BRAND,
        code: "126",
        name: "Medium blue",
        color: &[70, 156, 208],
    },
    ThreadRef {
        brand: BRAND,
        code: "127",
        name: "Fire Blue",
        color: &[22, 65, 148],
    },
    ThreadRef {
        brand: BRAND,
        code: "128",
        name: "Sapphire",
        color: &[40, 129, 190],
    },
    ThreadRef {
        brand: BRAND,
        code: "129",
        name: "Sky Blue",
        color: &[136, 188, 231],
    },
    ThreadRef {
        brand: BRAND,
        code: "130",
        name: "Silver Lining",
        color: &[201, 224, 238],
    },
    ThreadRef {
        brand: BRAND,
        code: "133",
        name: "Mango",
        color: &[232, 124, 40],
    },
    ThreadRef {
        brand: BRAND,
        code: "134",
        name: "Bright Pineapple",
        color: &[245, 203, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "135",
        name: "Sunshine",
        color: &[248, 223, 126],
    },
    ThreadRef {
        brand: BRAND,
        code: "136",
        name: "Golden Rod",
        color: &[245, 157, 36],
    },
    ThreadRef {
        brand: BRAND,
        code: "137",
        name: "Pale Sunflower",
        color: &[238, 230, 164],
    },
    ThreadRef {
        brand: BRAND,
        code: "140",
        name: "Black",
        color: &[47, 38, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "142",
        name: "Gobelin Blue",
        color: &[83, 109, 115],
    },
    ThreadRef {
        brand: BRAND,
        code: "143",
        name: "Stormy Sea",
        color: &[114, 130, 128],
    },
    ThreadRef {
        brand: BRAND,
        code: "147",
        name: "Pistachio",
        color: &[156, 160, 57],
    },
    ThreadRef {
        brand: BRAND,
        code: "148",
        name: "Sea Glass",
        color: &[53, 187, 180],
    },
    ThreadRef {
        brand: BRAND,
        code: "151",
        name: "Southampton",
        color: &[0, 150, 143],
    },
    ThreadRef {
        brand: BRAND,
        code: "152",
        name: "Opal Blue",
        color: &[197, 221, 208],
    },
    ThreadRef {
        brand: BRAND,
        code: "153",
        name: "Beauty Rose Medium",
        color: &[224, 51, 115],
    },
    ThreadRef {
        brand: BRAND,
        code: "154",
        name: "Emberglow",
        color: &[237, 105, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "156",
        name: "Creamy White",
        color: &[224, 213, 187],
    },
    ThreadRef {
        brand: BRAND,
        code: "157",
        name: "Lead",
        color: &[122, 137, 137],
    },
    ThreadRef {
        brand: BRAND,
        code: "159",
        name: "Stone Grey",
        color: &[177, 173, 173],
    },
    ThreadRef {
        brand: BRAND,
        code: "161",
        name: "Ash Blonde",
        color: &[199, 178, 150],
    },
    ThreadRef {
        brand: BRAND,
        code: "162",
        name: "Laurel Green",
        color: &[170, 189, 146],
    },
    ThreadRef {
        brand: BRAND,
        code: "169",
        name: "Carnation",
        color: &[219, 81, 109],
    },
    ThreadRef {
        brand: BRAND,
        code: "181",
        name: "Dark Purple",
        color: &[92, 38, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "199",
        name: "Purple Maze",
        color: &[60, 30, 108],
    },
    ThreadRef {
        brand: BRAND,
        code: "200",
        name: "Dark Pink",
        color: &[192, 24, 88],
    },
    ThreadRef {
        brand: BRAND,
        code: "208",
        name: "Orchid",
        color: &[223, 128, 158],
    },
    ThreadRef {
        brand: BRAND,
        code: "209",
        name: "Light Melon",
        color: &[229, 183, 186],
    },
    ThreadRef {
        brand: BRAND,
        code: "211",
        name: "Orchid Pink",
        color: &[237, 175, 181],
    },
    ThreadRef {
        brand: BRAND,
        code: "214",
        name: "Peach Bloosom",
        color: &[232, 130, 122],
    },
    ThreadRef {
        brand: BRAND,
        code: "221",
        name: "Pale Salmon",
        color: &[228, 172, 173],
    },
    ThreadRef {
        brand: BRAND,
        code: "231",
        name: "Dark Green",
        color: &[218, 215, 76],
    },
    ThreadRef {
        brand: BRAND,
        code: "235",
        name: "Dark Green",
        color: &[33, 64, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "236",
        name: "Bronze Green",
        color: &[50, 62, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "237",
        name: "Caviar",
        color: &[34, 38, 34],
    },
    ThreadRef {
        brand: BRAND,
        code: "240",
        name: "Royal Crest",
        color: &[40, 0, 26],
    },
    ThreadRef {
        brand: BRAND,
        code: "243",
        name: "Suns Purple",
        color: &[89, 53, 140],
    },
    ThreadRef {
        brand: BRAND,
        code: "244",
        name: "Delft",
        color: &[68, 94, 136],
    },
    ThreadRef {
        brand: BRAND,
        code: "246",
        name: "Starlight Blue",
        color: &[50, 90, 162],
    },
    ThreadRef {
        brand: BRAND,
        code: "262",
        name: "Pink",
        color: &[213, 106, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "278",
        name: "Light Pink",
        color: &[220, 191, 214],
    },
    ThreadRef {
        brand: BRAND,
        code: "279",
        name: "Dark Royal Blue",
        color: &[70, 36, 127],
    },
    ThreadRef {
        brand: BRAND,
        code: "280",
        name: "Grape",
        color: &[162, 142, 173],
    },
    ThreadRef {
        brand: BRAND,
        code: "281",
        name: "Mink Brown",
        color: &[138, 70, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "285",
        name: "Black Red",
        color: &[50, 23, 28],
    },
    ThreadRef {
        brand: BRAND,
        code: "292",
        name: "Cherry Red",
        color: &[167, 33, 41],
    },
    ThreadRef {
        brand: BRAND,
        code: "293",
        name: "Dusky Burgandy",
        color: &[72, 36, 46],
    },
    ThreadRef {
        brand: BRAND,
        code: "294",
        name: "Dark Chocolate",
        color: &[43, 2, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "295",
        name: "Espresso",
        color: &[23, 13, 7],
    },
    ThreadRef {
        brand: BRAND,
        code: "298",
        name: "Light Cocoa",
        color: &[125, 71, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "299",
        name: "Dark Maroon",
        color: &[97, 30, 41],
    },
    ThreadRef {
        brand: BRAND,
        code: "300",
        name: "Jet Black",
        color: &[0, 0, 6],
    },
    ThreadRef {
        brand: BRAND,
        code: "301",
        name: "Mimosa",
        color: &[221, 160, 44],
    },
    ThreadRef {
        brand: BRAND,
        code: "302",
        name: "Dark Brown",
        color: &[48, 31, 39],
    },
    ThreadRef {
        brand: BRAND,
        code: "303",
        name: "Peach",
        color: &[232, 187, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "304",
        name: "Cloud Dancer",
        color: &[245, 241, 233],
    },
    ThreadRef {
        brand: BRAND,
        code: "305",
        name: "Enchanted Sea",
        color: &[45, 89, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "306",
        name: "Olive Drab",
        color: &[93, 94, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "307",
        name: "Pastel Green",
        color: &[175, 209, 169],
    },
    ThreadRef {
        brand: BRAND,
        code: "308",
        name: "Harbor Green",
        color: &[79, 109, 94],
    },
    ThreadRef {
        brand: BRAND,
        code: "309",
        name: "Carousel Green",
        color: &[157, 180, 161],
    },
    ThreadRef {
        brand: BRAND,
        code: "310",
        name: "Teal",
        color: &[39, 111, 157],
    },
    ThreadRef {
        brand: BRAND,
        code: "311",
        name: "Medium Plum",
        color: &[215, 129, 163],
    },
    ThreadRef {
        brand: BRAND,
        code: "312",
        name: "Deep Rose",
        color: &[152, 55, 67],
    },
    ThreadRef {
        brand: BRAND,
        code: "313",
        name: "Baroque Rose",
        color: &[189, 98, 109],
    },
    ThreadRef {
        brand: BRAND,
        code: "314",
        name: "Rose Wine",
        color: &[172, 96, 110],
    },
    ThreadRef {
        brand: BRAND,
        code: "315",
        name: "Champagne Mist",
        color: &[227, 216, 185],
    },
    ThreadRef {
        brand: BRAND,
        code: "316",
        name: "Hyacinth",
        color: &[105, 172, 223],
    },
    ThreadRef {
        brand: BRAND,
        code: "317",
        name: "Buttercup",
        color: &[246, 202, 33],
    },
    ThreadRef {
        brand: BRAND,
        code: "318",
        name: "Umbra Grey",
        color: &[65, 65, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "319",
        name: "Tan",
        color: &[192, 156, 113],
    },
    ThreadRef {
        brand: BRAND,
        code: "322",
        name: "Purple",
        color: &[164, 73, 132],
    },
    ThreadRef {
        brand: BRAND,
        code: "323",
        name: "Light Green/Yellow",
        color: &[171, 165, 151],
    },
    ThreadRef {
        brand: BRAND,
        code: "324",
        name: "Crabapple",
        color: &[204, 113, 98],
    },
    ThreadRef {
        brand: BRAND,
        code: "325",
        name: "Dark Pink",
        color: &[166, 33, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "326",
        name: "Titanium",
        color: &[233, 229, 236],
    },
    ThreadRef {
        brand: BRAND,
        code: "327",
        name: "Natural White",
        color: &[245, 237, 234],
    },
    ThreadRef {
        brand: BRAND,
        code: "328",
        name: "Baby Blue Light",
        color: &[202, 224, 229],
    },
    ThreadRef {
        brand: BRAND,
        code: "330",
        name: "Midnight Navy",
        color: &[0, 11, 40],
    },
    ThreadRef {
        brand: BRAND,
        code: "332",
        name: "Light Grey",
        color: &[156, 166, 171],
    },
    ThreadRef {
        brand: BRAND,
        code: "333",
        name: "Pure Diamond",
        color: &[246, 245, 238],
    },
    ThreadRef {
        brand: BRAND,
        code: "334",
        name: "Carnatio",
        color: &[235, 167, 196],
    },
    ThreadRef {
        brand: BRAND,
        code: "335",
        name: "Cocoa",
        color: &[58, 28, 35],
    },
    ThreadRef {
        brand: BRAND,
        code: "336",
        name: "Fuchsia",
        color: &[158, 33, 95],
    },
    ThreadRef {
        brand: BRAND,
        code: "337",
        name: "Rich Burgundy",
        color: &[82, 33, 35],
    },
    ThreadRef {
        brand: BRAND,
        code: "338",
        name: "Teal",
        color: &[46, 105, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "339",
        name: "Desma Blue",
        color: &[108, 117, 135],
    },
    ThreadRef {
        brand: BRAND,
        code: "340",
        name: "Window Grey",
        color: &[153, 158, 172],
    },
    ThreadRef {
        brand: BRAND,
        code: "342",
        name: "Light Grape",
        color: &[146, 142, 171],
    },
    ThreadRef {
        brand: BRAND,
        code: "344",
        name: "Dark Brown",
        color: &[84, 70, 65],
    },
    ThreadRef {
        brand: BRAND,
        code: "346",
        name: "Hazel",
        color: &[148, 106, 94],
    },
    ThreadRef {
        brand: BRAND,
        code: "348",
        name: "Columbia Blue",
        color: &[76, 150, 203],
    },
    ThreadRef {
        brand: BRAND,
        code: "350",
        name: "Fire Blue",
        color: &[0, 86, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "351",
        name: "Evergreen",
        color: &[0, 68, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "352",
        name: "Atom Red",
        color: &[160, 33, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "353",
        name: "Royal",
        color: &[44, 73, 139],
    },
    ThreadRef {
        brand: BRAND,
        code: "354",
        name: "Charcoal",
        color: &[14, 26, 37],
    },
];