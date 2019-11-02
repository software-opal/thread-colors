#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Gunold";
pub const THREADS: [ThreadRef; 130] = [
    ThreadRef::new(
        BRAND,
        "61005",
        "Black",
        &[0, 0, 0],
    ),
    ThreadRef::new(
        BRAND,
        "61129",
        "Brown",
        &[102, 51, 0],
    ),
    ThreadRef::new(
        BRAND,
        "61130",
        "Expresso",
        &[102, 55, 0],
    ),
    ThreadRef::new(
        BRAND,
        "61078",
        "Dk Tex Orange",
        &[255, 102, 0],
    ),
    ThreadRef::new(
        BRAND,
        "61184",
        "Sunkist",
        &[255, 105, 0],
    ),
    ThreadRef::new(
        BRAND,
        "61065",
        "Pumpkin",
        &[255, 204, 0],
    ),
    ThreadRef::new(
        BRAND,
        "61057",
        "Sienna",
        &[125, 80, 41],
    ),
    ThreadRef::new(
        BRAND,
        "61183",
        "Smokey",
        &[102, 51, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61169",
        "Auburn",
        &[204, 51, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61181",
        "Dark Rust",
        &[204, 72, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61103",
        "Dk Green",
        &[0, 73, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61171",
        "Green Sail",
        &[0, 75, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61174",
        "Green Petal",
        &[0, 77, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61536",
        "Green Sail2",
        &[0, 79, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61058",
        "Chocolate",
        &[153, 102, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61158",
        "Date",
        &[153, 105, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61021",
        "Rust",
        &[225, 120, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61176",
        "Palmetto",
        &[153, 153, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61025",
        "Almond",
        &[204, 153, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61056",
        "Almond2",
        &[204, 155, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61126",
        "Toast",
        &[204, 157, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61024",
        "Yellow",
        &[255, 153, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61137",
        "Yellow2",
        &[255, 155, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61185",
        "Yellow3",
        &[255, 157, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61067",
        "Daffodil",
        &[204, 204, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61159",
        "Sun Gold",
        &[240, 204, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61023",
        "Star Gold",
        &[225, 225, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61135",
        "Glow",
        &[225, 227, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61187",
        "Star Gold2",
        &[225, 229, 51],
    ),
    ThreadRef::new(
        BRAND,
        "61128",
        "Wicker",
        &[170, 120, 53],
    ),
    ThreadRef::new(
        BRAND,
        "61170",
        "Lt Cocoa",
        &[140, 95, 65],
    ),
    ThreadRef::new(
        BRAND,
        "61043",
        "Midnight Navy",
        &[0, 0, 75],
    ),
    ThreadRef::new(
        BRAND,
        "61044",
        "Midnight Navy2",
        &[0, 5, 75],
    ),
    ThreadRef::new(
        BRAND,
        "61039",
        "Red Berry",
        &[255, 0, 75],
    ),
    ThreadRef::new(
        BRAND,
        "61147",
        "Red Berry2",
        &[255, 5, 75],
    ),
    ThreadRef::new(
        BRAND,
        "61035",
        "Red Jubilee",
        &[102, 25, 75],
    ),
    ThreadRef::new(
        BRAND,
        "61189",
        "Dark Maroon",
        &[102, 27, 75],
    ),
    ThreadRef::new(
        BRAND,
        "61220",
        "Twilight",
        &[75, 75, 75],
    ),
    ThreadRef::new(
        BRAND,
        "61119",
        "Bashful Pink",
        &[255, 80, 90],
    ),
    ThreadRef::new(
        BRAND,
        "61124",
        "Goldenrod",
        &[255, 240, 90],
    ),
    ThreadRef::new(
        BRAND,
        "61037",
        "Persimmon",
        &[255, 0, 102],
    ),
    ThreadRef::new(
        BRAND,
        "61190",
        "Wine",
        &[153, 80, 102],
    ),
    ThreadRef::new(
        BRAND,
        "61214",
        "Wine2",
        &[153, 83, 102],
    ),
    ThreadRef::new(
        BRAND,
        "61079",
        "Greenstone",
        &[51, 120, 102],
    ),
    ThreadRef::new(
        BRAND,
        "61208",
        "Green Forest",
        &[51, 123, 102],
    ),
    ThreadRef::new(
        BRAND,
        "61230",
        "Fern Green",
        &[51, 125, 102],
    ),
    ThreadRef::new(
        BRAND,
        "61513",
        "Oceanic Green",
        &[51, 127, 102],
    ),
    ThreadRef::new(
        BRAND,
        "61517",
        "Fern Green2",
        &[51, 129, 102],
    ),
    ThreadRef::new(
        BRAND,
        "61156",
        "Sand Dune",
        &[153, 121, 102],
    ),
    ThreadRef::new(
        BRAND,
        "61055",
        "Ashley Gold",
        &[204, 153, 102],
    ),
    ThreadRef::new(
        BRAND,
        "61510",
        "Erin Green",
        &[153, 204, 102],
    ),
    ThreadRef::new(
        BRAND,
        "61047",
        "Green Oak",
        &[140, 230, 104],
    ),
    ThreadRef::new(
        BRAND,
        "61177",
        "Peapod",
        &[140, 233, 104],
    ),
    ThreadRef::new(
        BRAND,
        "61192",
        "Cherry Punch",
        &[215, 0, 120],
    ),
    ThreadRef::new(
        BRAND,
        "61533",
        "Crimson",
        &[215, 5, 120],
    ),
    ThreadRef::new(
        BRAND,
        "61200",
        "Blue Ribbon",
        &[0, 40, 120],
    ),
    ThreadRef::new(
        BRAND,
        "61162",
        "Dark Teal",
        &[61, 89, 120],
    ),
    ThreadRef::new(
        BRAND,
        "61231",
        "Ruby Glint",
        &[230, 51, 125],
    ),
    ThreadRef::new(
        BRAND,
        "61511",
        "Ruby Glint2",
        &[230, 55, 125],
    ),
    ThreadRef::new(
        BRAND,
        "61051",
        "Dk Emerald",
        &[0, 153, 130],
    ),
    ThreadRef::new(
        BRAND,
        "61019",
        "Flamingo",
        &[240, 153, 130],
    ),
    ThreadRef::new(
        BRAND,
        "61020",
        "Coral",
        &[240, 155, 130],
    ),
    ThreadRef::new(
        BRAND,
        "61259",
        "Melon",
        &[240, 157, 130],
    ),
    ThreadRef::new(
        BRAND,
        "61112",
        "Purple Accent",
        &[125, 0, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61195",
        "Purple Accent2",
        &[125, 5, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61122",
        "Purple Shadow",
        &[153, 0, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61255",
        "Mulberry",
        &[153, 5, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61197",
        "Purple Twist",
        &[102, 2, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61235",
        "Purple Maze",
        &[102, 5, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61032",
        "Iris",
        &[120, 51, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61194",
        "Iris2",
        &[120, 55, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61034",
        "Azalea",
        &[255, 51, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61096",
        "Baltic Blue",
        &[51, 102, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61206",
        "Teal",
        &[51, 105, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61101",
        "Lt Kelly",
        &[51, 204, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61061",
        "Maize",
        &[255, 250, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61209",
        "Pistachio",
        &[225, 255, 153],
    ),
    ThreadRef::new(
        BRAND,
        "61041",
        "Silvery Gray",
        &[153, 153, 154],
    ),
    ThreadRef::new(
        BRAND,
        "61166",
        "Metal",
        &[153, 155, 154],
    ),
    ThreadRef::new(
        BRAND,
        "61224",
        "Rose",
        &[255, 154, 154],
    ),
    ThreadRef::new(
        BRAND,
        "61258",
        "Peach",
        &[255, 180, 160],
    ),
    ThreadRef::new(
        BRAND,
        "61070",
        "Topaz",
        &[220, 200, 160],
    ),
    ThreadRef::new(
        BRAND,
        "61040",
        "Storm Gray",
        &[153, 153, 165],
    ),
    ThreadRef::new(
        BRAND,
        "61219",
        "Storm Gray2",
        &[153, 155, 165],
    ),
    ThreadRef::new(
        BRAND,
        "61198",
        "Imperial Blue",
        &[50, 100, 170],
    ),
    ThreadRef::new(
        BRAND,
        "61149",
        "Bamboo",
        &[204, 180, 170],
    ),
    ThreadRef::new(
        BRAND,
        "61180",
        "Rattan",
        &[204, 185, 170],
    ),
    ThreadRef::new(
        BRAND,
        "61033",
        "Plum",
        &[204, 0, 180],
    ),
    ThreadRef::new(
        BRAND,
        "61080",
        "Plum2",
        &[204, 5, 180],
    ),
    ThreadRef::new(
        BRAND,
        "61179",
        "Ducky Mauve",
        &[204, 153, 186],
    ),
    ThreadRef::new(
        BRAND,
        "61154",
        "Begonia",
        &[255, 80, 190],
    ),
    ThreadRef::new(
        BRAND,
        "61191",
        "Passion",
        &[255, 85, 190],
    ),
    ThreadRef::new(
        BRAND,
        "61108",
        "Pink Mist",
        &[255, 185, 190],
    ),
    ThreadRef::new(
        BRAND,
        "61017",
        "Opaline",
        &[230, 204, 190],
    ),
    ThreadRef::new(
        BRAND,
        "61127",
        "Ecru",
        &[230, 207, 190],
    ),
    ThreadRef::new(
        BRAND,
        "61042",
        "Empire Blue",
        &[51, 51, 200],
    ),
    ThreadRef::new(
        BRAND,
        "61076",
        "Fire Blue",
        &[51, 55, 200],
    ),
    ThreadRef::new(
        BRAND,
        "61109",
        "Floral Pink",
        &[255, 130, 204],
    ),
    ThreadRef::new(
        BRAND,
        "61211",
        "Willow",
        &[51, 191, 204],
    ),
    ThreadRef::new(
        BRAND,
        "61212",
        "Willow2",
        &[51, 195, 204],
    ),
    ThreadRef::new(
        BRAND,
        "61508",
        "Willow3",
        &[51, 198, 204],
    ),
    ThreadRef::new(
        BRAND,
        "61236",
        "Pearl Grey",
        &[204, 204, 204],
    ),
    ThreadRef::new(
        BRAND,
        "61225",
        "Pink",
        &[255, 204, 204],
    ),
    ThreadRef::new(
        BRAND,
        "61011",
        "Skylight",
        &[204, 210, 204],
    ),
    ThreadRef::new(
        BRAND,
        "61218",
        "Gray",
        &[204, 215, 204],
    ),
    ThreadRef::new(
        BRAND,
        "61121",
        "Pale Orchid",
        &[255, 170, 220],
    ),
    ThreadRef::new(
        BRAND,
        "61064",
        "Opal Mist",
        &[255, 204, 220],
    ),
    ThreadRef::new(
        BRAND,
        "61082",
        "Ivory",
        &[255, 240, 220],
    ),
    ThreadRef::new(
        BRAND,
        "61090",
        "Mystic Teal",
        &[0, 204, 230],
    ),
    ThreadRef::new(
        BRAND,
        "61252",
        "Aquamarine",
        &[0, 208, 230],
    ),
    ThreadRef::new(
        BRAND,
        "61113",
        "Bisque",
        &[255, 230, 230],
    ),
    ThreadRef::new(
        BRAND,
        "61193",
        "Tulip",
        &[240, 180, 235],
    ),
    ThreadRef::new(
        BRAND,
        "61002",
        "Natural White",
        &[255, 255, 235],
    ),
    ThreadRef::new(
        BRAND,
        "61071",
        "Eggshell",
        &[255, 255, 240],
    ),
    ThreadRef::new(
        BRAND,
        "61046",
        "Turquoise",
        &[0, 215, 245],
    ),
    ThreadRef::new(
        BRAND,
        "61094",
        "Periwinkle",
        &[0, 215, 245],
    ),
    ThreadRef::new(
        BRAND,
        "61503",
        "Md Green",
        &[0, 220, 245],
    ),
    ThreadRef::new(
        BRAND,
        "61172",
        "Misty",
        &[90, 140, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61143",
        "Jay Blue",
        &[165, 165, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61535",
        "Blue",
        &[165, 170, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61196",
        "Tropic Blue",
        &[150, 175, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61028",
        "Ultra Blue",
        &[210, 220, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61029",
        "Ultra Blue2",
        &[210, 223, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61074",
        "Sky Blue",
        &[210, 226, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61145",
        "Baby Blue",
        &[210, 229, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61222",
        "Sky Blue2",
        &[210, 233, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61203",
        "Ice Blue",
        &[240, 250, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61001",
        "Snow White",
        &[250, 250, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61045",
        "Mint Julep",
        &[0, 250, 255],
    ),
    ThreadRef::new(
        BRAND,
        "61077",
        "Lt Blue",
        &[0, 255, 255],
    ),
];