#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "Madeira Frosted Matt";
const THREADS: [ThreadRef; 189] = [
    ThreadRef {
        brand: BRAND,
        code: "7027",
        name: "Dark Green",
        color: &[38, 109, 43],
    },
    ThreadRef {
        brand: BRAND,
        code: "7041",
        name: "Dark Grey",
        color: &[146, 144, 155],
    },
    ThreadRef {
        brand: BRAND,
        code: "7058",
        name: "MOSS Green Very Light",
        color: &[217, 212, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "7109",
        name: "Dark Purple",
        color: &[126, 47, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "7125",
        name: "Hazlenut",
        color: &[213, 155, 115],
    },
    ThreadRef {
        brand: BRAND,
        code: "7142",
        name: "Regatta",
        color: &[81, 121, 180],
    },
    ThreadRef {
        brand: BRAND,
        code: "7144",
        name: "Light Purple",
        color: &[123, 132, 171],
    },
    ThreadRef {
        brand: BRAND,
        code: "7147",
        name: "Scalloped Blue",
        color: &[161, 172, 202],
    },
    ThreadRef {
        brand: BRAND,
        code: "7165",
        name: "Atom Red",
        color: &[185, 56, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "7170",
        name: "Medium Purple",
        color: &[99, 57, 81],
    },
    ThreadRef {
        brand: BRAND,
        code: "7255",
        name: "Sangria",
        color: &[154, 66, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "7286",
        name: "Dahlia",
        color: &[91, 48, 130],
    },
    ThreadRef {
        brand: BRAND,
        code: "7310",
        name: "Violet Mist",
        color: &[211, 180, 186],
    },
    ThreadRef {
        brand: BRAND,
        code: "7313",
        name: "Moderate Blue",
        color: &[161, 196, 215],
    },
    ThreadRef {
        brand: BRAND,
        code: "7364",
        name: "Burnt Orange",
        color: &[206, 111, 55],
    },
    ThreadRef {
        brand: BRAND,
        code: "7595",
        name: "Wild Romance",
        color: &[231, 98, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "7598",
        name: "Deviled Orange",
        color: &[238, 126, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "7599",
        name: "Platinum",
        color: &[185, 212, 141],
    },
    ThreadRef {
        brand: BRAND,
        code: "7611",
        name: "Pastel Blue",
        color: &[202, 213, 217],
    },
    ThreadRef {
        brand: BRAND,
        code: "7612",
        name: "Purple Cloud",
        color: &[158, 162, 191],
    },
    ThreadRef {
        brand: BRAND,
        code: "7613",
        name: "Palest Ivory",
        color: &[200, 209, 211],
    },
    ThreadRef {
        brand: BRAND,
        code: "7616",
        name: "Flamingo",
        color: &[250, 125, 120],
    },
    ThreadRef {
        brand: BRAND,
        code: "7621",
        name: "Moonbeam",
        color: &[253, 119, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "7623",
        name: "Moonbeam",
        color: &[251, 234, 91],
    },
    ThreadRef {
        brand: BRAND,
        code: "7624",
        name: "Moonbeam",
        color: &[254, 191, 84],
    },
    ThreadRef {
        brand: BRAND,
        code: "7625",
        name: "Manila",
        color: &[241, 184, 134],
    },
    ThreadRef {
        brand: BRAND,
        code: "7627",
        name: "Pollen Gold",
        color: &[207, 197, 219],
    },
    ThreadRef {
        brand: BRAND,
        code: "7628",
        name: "Ice Blue",
        color: &[184, 212, 236],
    },
    ThreadRef {
        brand: BRAND,
        code: "7630",
        name: "Whisper Pink",
        color: &[183, 165, 200],
    },
    ThreadRef {
        brand: BRAND,
        code: "7635",
        name: "Rich Plum",
        color: &[148, 121, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "7639",
        name: "Dark Orange/ Brown",
        color: &[182, 67, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "7640",
        name: "Metal",
        color: &[98, 105, 110],
    },
    ThreadRef {
        brand: BRAND,
        code: "7641",
        name: "Granite Grey",
        color: &[41, 52, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "7643",
        name: "Aqua Pearl",
        color: &[13, 29, 55],
    },
    ThreadRef {
        brand: BRAND,
        code: "7645",
        name: "Green Haze",
        color: &[182, 221, 216],
    },
    ThreadRef {
        brand: BRAND,
        code: "7647",
        name: "Bleached Aqua",
        color: &[177, 209, 159],
    },
    ThreadRef {
        brand: BRAND,
        code: "7648",
        name: "Forestgreen Light",
        color: &[171, 207, 145],
    },
    ThreadRef {
        brand: BRAND,
        code: "7649",
        name: "Lime",
        color: &[169, 192, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "7650",
        name: "Kelly",
        color: &[29, 137, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "7652",
        name: "Jade Green",
        color: &[0, 140, 137],
    },
    ThreadRef {
        brand: BRAND,
        code: "7653",
        name: "Pink Mist",
        color: &[235, 221, 221],
    },
    ThreadRef {
        brand: BRAND,
        code: "7657",
        name: "Medium Pink",
        color: &[167, 113, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "7658",
        name: "K. A. Bronze",
        color: &[166, 83, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "7661",
        name: "Vanilla Ice",
        color: &[242, 240, 214],
    },
    ThreadRef {
        brand: BRAND,
        code: "7662",
        name: "Bullion",
        color: &[169, 148, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "7665",
        name: "Sterling",
        color: &[128, 115, 101],
    },
    ThreadRef {
        brand: BRAND,
        code: "7666",
        name: "PALE Yellow",
        color: &[250, 248, 186],
    },
    ThreadRef {
        brand: BRAND,
        code: "7668",
        name: "Pastel Lilac",
        color: &[147, 170, 149],
    },
    ThreadRef {
        brand: BRAND,
        code: "7670",
        name: "Sunkist",
        color: &[221, 193, 124],
    },
    ThreadRef {
        brand: BRAND,
        code: "7674",
        name: "Pale Gray Green",
        color: &[168, 196, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "7680",
        name: "Dk. Plum",
        color: &[98, 72, 104],
    },
    ThreadRef {
        brand: BRAND,
        code: "7682",
        name: "Sea Mist",
        color: &[211, 194, 178],
    },
    ThreadRef {
        brand: BRAND,
        code: "7689",
        name: "Lead",
        color: &[129, 137, 136],
    },
    ThreadRef {
        brand: BRAND,
        code: "7690",
        name: "Dark Blue",
        color: &[4, 52, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "7692",
        name: "Crystal Lake",
        color: &[193, 234, 231],
    },
    ThreadRef {
        brand: BRAND,
        code: "7694",
        name: "Pale Blue",
        color: &[121, 182, 176],
    },
    ThreadRef {
        brand: BRAND,
        code: "7695",
        name: "Aqua Velva",
        color: &[32, 163, 188],
    },
    ThreadRef {
        brand: BRAND,
        code: "7704",
        name: "Coachman Green",
        color: &[3, 79, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "7706",
        name: "Whisper Pink",
        color: &[183, 168, 76],
    },
    ThreadRef {
        brand: BRAND,
        code: "7710",
        name: "DK Fushia",
        color: &[177, 66, 134],
    },
    ThreadRef {
        brand: BRAND,
        code: "7711",
        name: "Pastel Violet",
        color: &[167, 156, 201],
    },
    ThreadRef {
        brand: BRAND,
        code: "7713",
        name: "Bisque",
        color: &[234, 218, 221],
    },
    ThreadRef {
        brand: BRAND,
        code: "7718",
        name: "Highrise",
        color: &[145, 155, 161],
    },
    ThreadRef {
        brand: BRAND,
        code: "7721",
        name: "Canary",
        color: &[240, 165, 189],
    },
    ThreadRef {
        brand: BRAND,
        code: "7723",
        name: "Sunshine",
        color: &[242, 221, 166],
    },
    ThreadRef {
        brand: BRAND,
        code: "7725",
        name: "Pink",
        color: &[226, 184, 108],
    },
    ThreadRef {
        brand: BRAND,
        code: "7729",
        name: "Light Pink",
        color: &[156, 121, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "7730",
        name: "Foxy Red",
        color: &[182, 114, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "7731",
        name: "Cerise",
        color: &[216, 192, 206],
    },
    ThreadRef {
        brand: BRAND,
        code: "7733",
        name: "Columbia Blue",
        color: &[82, 143, 199],
    },
    ThreadRef {
        brand: BRAND,
        code: "7735",
        name: "Sunshine",
        color: &[240, 224, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "7739",
        name: "Black",
        color: &[45, 69, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "7741",
        name: "Dark Charcoal",
        color: &[85, 93, 92],
    },
    ThreadRef {
        brand: BRAND,
        code: "7742",
        name: "Dark Navy",
        color: &[39, 62, 102],
    },
    ThreadRef {
        brand: BRAND,
        code: "7745",
        name: "Clayrust",
        color: &[107, 83, 65],
    },
    ThreadRef {
        brand: BRAND,
        code: "7746",
        name: "Medium Green/Blue",
        color: &[92, 167, 159],
    },
    ThreadRef {
        brand: BRAND,
        code: "7747",
        name: "Cabernet",
        color: &[152, 30, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "7748",
        name: "Platinum",
        color: &[179, 213, 136],
    },
    ThreadRef {
        brand: BRAND,
        code: "7749",
        name: "Warm Wine",
        color: &[99, 206, 105],
    },
    ThreadRef {
        brand: BRAND,
        code: "7752",
        name: "Flesh Pink",
        color: &[240, 184, 168],
    },
    ThreadRef {
        brand: BRAND,
        code: "7753",
        name: "Blue Blush",
        color: &[235, 173, 146],
    },
    ThreadRef {
        brand: BRAND,
        code: "7754",
        name: "Billowing Sail",
        color: &[225, 59, 46],
    },
    ThreadRef {
        brand: BRAND,
        code: "7755",
        name: "Moonbeam",
        color: &[253, 173, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "7760",
        name: "Water Lilly",
        color: &[81, 138, 177],
    },
    ThreadRef {
        brand: BRAND,
        code: "7761",
        name: "Wild Pink",
        color: &[215, 222, 251],
    },
    ThreadRef {
        brand: BRAND,
        code: "7767",
        name: "Dark Ink",
        color: &[48, 37, 128],
    },
    ThreadRef {
        brand: BRAND,
        code: "7770",
        name: "Sterling",
        color: &[127, 182, 100],
    },
    ThreadRef {
        brand: BRAND,
        code: "7771",
        name: "Flesh Pink",
        color: &[233, 177, 98],
    },
    ThreadRef {
        brand: BRAND,
        code: "7773",
        name: "Sunkist",
        color: &[221, 153, 65],
    },
    ThreadRef {
        brand: BRAND,
        code: "7775",
        name: "Teal",
        color: &[15, 117, 156],
    },
    ThreadRef {
        brand: BRAND,
        code: "7777",
        name: "Paprika",
        color: &[226, 145, 141],
    },
    ThreadRef {
        brand: BRAND,
        code: "7778",
        name: "Canary",
        color: &[240, 114, 45],
    },
    ThreadRef {
        brand: BRAND,
        code: "7779",
        name: "Natural White",
        color: &[234, 86, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "7781",
        name: "Atom Red",
        color: &[161, 33, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "7786",
        name: "Dark Pink",
        color: &[192, 63, 101],
    },
    ThreadRef {
        brand: BRAND,
        code: "7787",
        name: "Sunkist",
        color: &[221, 69, 130],
    },
    ThreadRef {
        brand: BRAND,
        code: "7790",
        name: "Red Bittersweet",
        color: &[140, 122, 39],
    },
    ThreadRef {
        brand: BRAND,
        code: "7791",
        name: "Wildfire",
        color: &[189, 125, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "7792",
        name: "LT Olive",
        color: &[183, 161, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "7793",
        name: "Seafrost",
        color: &[136, 124, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "7795",
        name: "Holly Green",
        color: &[75, 80, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "7797",
        name: "Blue Horizon",
        color: &[25, 144, 185],
    },
    ThreadRef {
        brand: BRAND,
        code: "7800",
        name: "Moonless Night",
        color: &[31, 40, 43],
    },
    ThreadRef {
        brand: BRAND,
        code: "7801",
        name: "Bright White",
        color: &[240, 245, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "7803",
        name: "Iced Silver",
        color: &[253, 255, 250],
    },
    ThreadRef {
        brand: BRAND,
        code: "7804",
        name: "Medium Palmetto",
        color: &[122, 131, 78],
    },
    ThreadRef {
        brand: BRAND,
        code: "7809",
        name: "Light Pink",
        color: &[197, 196, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "7810",
        name: "Palm Leaf",
        color: &[181, 183, 170],
    },
    ThreadRef {
        brand: BRAND,
        code: "7811",
        name: "Vapor",
        color: &[195, 190, 184],
    },
    ThreadRef {
        brand: BRAND,
        code: "7815",
        name: "Star Gold",
        color: &[226, 203, 211],
    },
    ThreadRef {
        brand: BRAND,
        code: "7816",
        name: "Peach",
        color: &[231, 196, 203],
    },
    ThreadRef {
        brand: BRAND,
        code: "7818",
        name: "Petal Pink",
        color: &[225, 203, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "7820",
        name: "Sunkist",
        color: &[224, 125, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "7821",
        name: "Ivory",
        color: &[207, 44, 37],
    },
    ThreadRef {
        brand: BRAND,
        code: "7822",
        name: "Wild Pink",
        color: &[214, 207, 192],
    },
    ThreadRef {
        brand: BRAND,
        code: "7823",
        name: "Lemon",
        color: &[223, 255, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "7825",
        name: "Lemon",
        color: &[243, 230, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "7827",
        name: "Green Dust",
        color: &[123, 183, 194],
    },
    ThreadRef {
        brand: BRAND,
        code: "7828",
        name: "Seafrost",
        color: &[135, 175, 215],
    },
    ThreadRef {
        brand: BRAND,
        code: "7832",
        name: "Iris",
        color: &[131, 81, 171],
    },
    ThreadRef {
        brand: BRAND,
        code: "7833",
        name: "Pistachio",
        color: &[152, 165, 85],
    },
    ThreadRef {
        brand: BRAND,
        code: "7834",
        name: "Black Eyed Susie",
        color: &[210, 165, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "7837",
        name: "Medium Red/Orange",
        color: &[231, 70, 86],
    },
    ThreadRef {
        brand: BRAND,
        code: "7838",
        name: "Lime Cream",
        color: &[194, 42, 53],
    },
    ThreadRef {
        brand: BRAND,
        code: "7840",
        name: "Dark Grey",
        color: &[139, 142, 151],
    },
    ThreadRef {
        brand: BRAND,
        code: "7841",
        name: "Stormy Cloud Blue",
        color: &[65, 78, 86],
    },
    ThreadRef {
        brand: BRAND,
        code: "7842",
        name: "Flag Blue",
        color: &[22, 59, 147],
    },
    ThreadRef {
        brand: BRAND,
        code: "7843",
        name: "Dark Indigo",
        color: &[0, 36, 90],
    },
    ThreadRef {
        brand: BRAND,
        code: "7844",
        name: "Mood Indigo",
        color: &[32, 42, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "7845",
        name: "Sharp Green",
        color: &[150, 204, 170],
    },
    ThreadRef {
        brand: BRAND,
        code: "7848",
        name: "Banner Gray",
        color: &[137, 181, 105],
    },
    ThreadRef {
        brand: BRAND,
        code: "7851",
        name: "Frosty Spruce",
        color: &[2, 95, 70],
    },
    ThreadRef {
        brand: BRAND,
        code: "7852",
        name: "Dark Aqua Green",
        color: &[44, 130, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "7855",
        name: "Orange",
        color: &[246, 206, 180],
    },
    ThreadRef {
        brand: BRAND,
        code: "7859",
        name: "Egyptian Brown",
        color: &[75, 54, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "7860",
        name: "Star Gold",
        color: &[226, 220, 194],
    },
    ThreadRef {
        brand: BRAND,
        code: "7866",
        name: "PaleYellow",
        color: &[244, 236, 157],
    },
    ThreadRef {
        brand: BRAND,
        code: "7868",
        name: "Peacock Green",
        color: &[10, 150, 121],
    },
    ThreadRef {
        brand: BRAND,
        code: "7869",
        name: "Canary",
        color: &[240, 164, 89],
    },
    ThreadRef {
        brand: BRAND,
        code: "7870",
        name: "Moonbeam",
        color: &[252, 208, 145],
    },
    ThreadRef {
        brand: BRAND,
        code: "7871",
        name: "Fawn",
        color: &[160, 180, 208],
    },
    ThreadRef {
        brand: BRAND,
        code: "7874",
        name: "Green Haze",
        color: &[182, 198, 231],
    },
    ThreadRef {
        brand: BRAND,
        code: "7882",
        name: "Sunkist",
        color: &[224, 206, 190],
    },
    ThreadRef {
        brand: BRAND,
        code: "7884",
        name: "Buff Orange",
        color: &[213, 188, 167],
    },
    ThreadRef {
        brand: BRAND,
        code: "7885",
        name: "Ruby Glint",
        color: &[191, 149, 108],
    },
    ThreadRef {
        brand: BRAND,
        code: "7886",
        name: "Star Gold",
        color: &[226, 223, 224],
    },
    ThreadRef {
        brand: BRAND,
        code: "7892",
        name: "Bleached Aqua",
        color: &[176, 215, 211],
    },
    ThreadRef {
        brand: BRAND,
        code: "7893",
        name: "Aquamarine",
        color: &[98, 184, 195],
    },
    ThreadRef {
        brand: BRAND,
        code: "7899",
        name: "Dark Pink",
        color: &[190, 90, 61],
    },
    ThreadRef {
        brand: BRAND,
        code: "7900",
        name: "Seashell",
        color: &[204, 210, 184],
    },
    ThreadRef {
        brand: BRAND,
        code: "7902",
        name: "Holly Leaf",
        color: &[16, 93, 83],
    },
    ThreadRef {
        brand: BRAND,
        code: "7903",
        name: "Deep Green",
        color: &[33, 86, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "7908",
        name: "Moonbeam",
        color: &[252, 101, 103],
    },
    ThreadRef {
        brand: BRAND,
        code: "7909",
        name: "Neon Pink",
        color: &[255, 86, 142],
    },
    ThreadRef {
        brand: BRAND,
        code: "7911",
        name: "Pink Mist",
        color: &[236, 222, 226],
    },
    ThreadRef {
        brand: BRAND,
        code: "7917",
        name: "Rapture Rose",
        color: &[201, 114, 130],
    },
    ThreadRef {
        brand: BRAND,
        code: "7919",
        name: "Sunkist",
        color: &[221, 105, 132],
    },
    ThreadRef {
        brand: BRAND,
        code: "7920",
        name: "Bleached Aqua",
        color: &[176, 177, 132],
    },
    ThreadRef {
        brand: BRAND,
        code: "7921",
        name: "Crystal Pink",
        color: &[230, 189, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "7922",
        name: "Nassau Blue",
        color: &[80, 63, 141],
    },
    ThreadRef {
        brand: BRAND,
        code: "7924",
        name: "Lemon",
        color: &[243, 227, 42],
    },
    ThreadRef {
        brand: BRAND,
        code: "7931",
        name: "Best Brown",
        color: &[55, 44, 36],
    },
    ThreadRef {
        brand: BRAND,
        code: "7934",
        name: "Sky Blue",
        color: &[22, 97, 167],
    },
    ThreadRef {
        brand: BRAND,
        code: "7936",
        name: "Black Eyed Susie",
        color: &[161, 146, 113],
    },
    ThreadRef {
        brand: BRAND,
        code: "7937",
        name: "Moonbeam",
        color: &[253, 155, 9],
    },
    ThreadRef {
        brand: BRAND,
        code: "7938",
        name: "Peach",
        color: &[231, 219, 197],
    },
    ThreadRef {
        brand: BRAND,
        code: "7939",
        name: "Deer Brown",
        color: &[169, 156, 125],
    },
    ThreadRef {
        brand: BRAND,
        code: "7941",
        name: "Peach",
        color: &[231, 200, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "7946",
        name: "Moonbeam",
        color: &[253, 146, 14],
    },
    ThreadRef {
        brand: BRAND,
        code: "7948",
        name: "Shell Pink",
        color: &[232, 152, 171],
    },
    ThreadRef {
        brand: BRAND,
        code: "7949",
        name: "Pink Mist",
        color: &[235, 234, 232],
    },
    ThreadRef {
        brand: BRAND,
        code: "7950",
        name: "Flite Green",
        color: &[160, 241, 13],
    },
    ThreadRef {
        brand: BRAND,
        code: "7951",
        name: "Arctic Ice",
        color: &[250, 197, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "7952",
        name: "Canary",
        color: &[240, 153, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "7953",
        name: "Vapor",
        color: &[196, 211, 224],
    },
    ThreadRef {
        brand: BRAND,
        code: "7957",
        name: "Black Chrome",
        color: &[70, 63, 55],
    },
    ThreadRef {
        brand: BRAND,
        code: "7962",
        name: "Dark Blue",
        color: &[11, 59, 93],
    },
    ThreadRef {
        brand: BRAND,
        code: "7965",
        name: "Arctic Ice",
        color: &[250, 148, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "7969",
        name: "Light Pink",
        color: &[156, 165, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "7970",
        name: "Dark Green",
        color: &[49, 106, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "7971",
        name: "Flamingo",
        color: &[249, 212, 67],
    },
    ThreadRef {
        brand: BRAND,
        code: "7976",
        name: "Cobalt Bleu Very Dark",
        color: &[26, 52, 86],
    },
    ThreadRef {
        brand: BRAND,
        code: "7977",
        name: "Blue Horizon",
        color: &[25, 144, 186],
    },
    ThreadRef {
        brand: BRAND,
        code: "7980",
        name: "Manila",
        color: &[243, 211, 38],
    },
    ThreadRef {
        brand: BRAND,
        code: "7982",
        name: "Brown",
        color: &[113, 36, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "7984",
        name: "Shell Pink",
        color: &[232, 110, 152],
    },
    ThreadRef {
        brand: BRAND,
        code: "7986",
        name: "Vapor",
        color: &[197, 35, 33],
    },
    ThreadRef {
        brand: BRAND,
        code: "7988",
        name: "Nile",
        color: &[11, 195, 96],
    },
    ThreadRef {
        brand: BRAND,
        code: "7995",
        name: "Daffodil",
        color: &[244, 234, 85],
    },
];