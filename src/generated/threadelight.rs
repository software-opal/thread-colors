#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "ThreaDelighT";
const THREADS: [ThreadRef; 260] = [
    ThreadRef {
        brand: BRAND,
        code: "P507",
        name: "Pale Peach",
        color: &[255, 236, 224],
    },
    ThreadRef {
        brand: BRAND,
        code: "P509",
        name: "Peach",
        color: &[248, 196, 178],
    },
    ThreadRef {
        brand: BRAND,
        code: "P510",
        name: "Salmon",
        color: &[244, 169, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "P512",
        name: "Coral MD",
        color: &[220, 91, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "P513",
        name: "Coral DK",
        color: &[194, 49, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "P515",
        name: "Rose Pink",
        color: &[255, 142, 159],
    },
    ThreadRef {
        brand: BRAND,
        code: "P516",
        name: "Light Carnation",
        color: &[241, 114, 134],
    },
    ThreadRef {
        brand: BRAND,
        code: "P518",
        name: "Dark Carnation",
        color: &[236, 64, 104],
    },
    ThreadRef {
        brand: BRAND,
        code: "P519",
        name: "Pale Dusty Rose",
        color: &[245, 207, 207],
    },
    ThreadRef {
        brand: BRAND,
        code: "P520",
        name: "Candy Pink",
        color: &[241, 174, 186],
    },
    ThreadRef {
        brand: BRAND,
        code: "P521",
        name: "Dusty Rose LT",
        color: &[230, 140, 155],
    },
    ThreadRef {
        brand: BRAND,
        code: "P522",
        name: "Dusty Rose",
        color: &[226, 132, 141],
    },
    ThreadRef {
        brand: BRAND,
        code: "P523",
        name: "Strawberry LT",
        color: &[219, 109, 125],
    },
    ThreadRef {
        brand: BRAND,
        code: "P524",
        name: "Raspberry",
        color: &[205, 87, 101],
    },
    ThreadRef {
        brand: BRAND,
        code: "P525",
        name: "Raspberry DK",
        color: &[168, 68, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "P532",
        name: "Christmas Red BR",
        color: &[209, 38, 39],
    },
    ThreadRef {
        brand: BRAND,
        code: "P533",
        name: "Christmas Red",
        color: &[177, 39, 42],
    },
    ThreadRef {
        brand: BRAND,
        code: "P536",
        name: "Red Fruit",
        color: &[151, 62, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "P537",
        name: "Garnet MD",
        color: &[134, 57, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "P538",
        name: "Deep Wine Red",
        color: &[113, 57, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "P551",
        name: "Mauve LT",
        color: &[234, 182, 200],
    },
    ThreadRef {
        brand: BRAND,
        code: "P552",
        name: "Mauve MD",
        color: &[211, 136, 155],
    },
    ThreadRef {
        brand: BRAND,
        code: "P553",
        name: "Raspberry Mauve",
        color: &[185, 101, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "P555",
        name: "Mauve DK",
        color: &[124, 61, 68],
    },
    ThreadRef {
        brand: BRAND,
        code: "P559",
        name: "Rosebud Pink",
        color: &[252, 196, 216],
    },
    ThreadRef {
        brand: BRAND,
        code: "P560",
        name: "Cranberry LT",
        color: &[248, 162, 190],
    },
    ThreadRef {
        brand: BRAND,
        code: "P561",
        name: "Cranberry",
        color: &[243, 149, 176],
    },
    ThreadRef {
        brand: BRAND,
        code: "P562",
        name: "Cranberry MD",
        color: &[215, 83, 126],
    },
    ThreadRef {
        brand: BRAND,
        code: "P564",
        name: "Cranberry DK",
        color: &[180, 52, 97],
    },
    ThreadRef {
        brand: BRAND,
        code: "P565",
        name: "Pink Plum LT",
        color: &[237, 181, 219],
    },
    ThreadRef {
        brand: BRAND,
        code: "P566",
        name: "Pink Plum MD",
        color: &[223, 146, 189],
    },
    ThreadRef {
        brand: BRAND,
        code: "P567",
        name: "Light Plum",
        color: &[191, 88, 147],
    },
    ThreadRef {
        brand: BRAND,
        code: "P568",
        name: "Magenta",
        color: &[171, 55, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "P570",
        name: "Plum DK",
        color: &[129, 33, 62],
    },
    ThreadRef {
        brand: BRAND,
        code: "P571",
        name: "Pale Shell Pink",
        color: &[235, 217, 207],
    },
    ThreadRef {
        brand: BRAND,
        code: "P572",
        name: "Dusty Pink LT",
        color: &[223, 179, 166],
    },
    ThreadRef {
        brand: BRAND,
        code: "P573",
        name: "Dusty Pink MD",
        color: &[187, 129, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "P575",
        name: "Earth Pink",
        color: &[159, 87, 78],
    },
    ThreadRef {
        brand: BRAND,
        code: "P576",
        name: "Mars Red",
        color: &[136, 66, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "P577",
        name: "Antique Mauve",
        color: &[218, 183, 182],
    },
    ThreadRef {
        brand: BRAND,
        code: "P578",
        name: "Litchee Mauve",
        color: &[209, 167, 167],
    },
    ThreadRef {
        brand: BRAND,
        code: "P579",
        name: "Heather Lilac",
        color: &[188, 138, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "P581",
        name: "Antique Mauve DK",
        color: &[142, 93, 92],
    },
    ThreadRef {
        brand: BRAND,
        code: "P583",
        name: "Dark Garnet",
        color: &[107, 59, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "P591",
        name: "Blackcurrant",
        color: &[71, 24, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "P592",
        name: "Lavender LT",
        color: &[222, 196, 222],
    },
    ThreadRef {
        brand: BRAND,
        code: "P593",
        name: "Lavender MD",
        color: &[205, 171, 217],
    },
    ThreadRef {
        brand: BRAND,
        code: "P594",
        name: "Violet LT",
        color: &[181, 133, 199],
    },
    ThreadRef {
        brand: BRAND,
        code: "P595",
        name: "Violet MD",
        color: &[143, 100, 152],
    },
    ThreadRef {
        brand: BRAND,
        code: "P597",
        name: "Violet DK",
        color: &[95, 46, 95],
    },
    ThreadRef {
        brand: BRAND,
        code: "P601",
        name: "Blue Violet LT",
        color: &[204, 213, 231],
    },
    ThreadRef {
        brand: BRAND,
        code: "P602",
        name: "Blue Violet",
        color: &[161, 167, 213],
    },
    ThreadRef {
        brand: BRAND,
        code: "P603",
        name: "Wisteria Violet",
        color: &[152, 158, 212],
    },
    ThreadRef {
        brand: BRAND,
        code: "P604",
        name: "Blue Violet DK",
        color: &[125, 123, 185],
    },
    ThreadRef {
        brand: BRAND,
        code: "P605",
        name: "Deep Violet",
        color: &[103, 95, 163],
    },
    ThreadRef {
        brand: BRAND,
        code: "P606",
        name: "Lavender Blue LT",
        color: &[168, 196, 229],
    },
    ThreadRef {
        brand: BRAND,
        code: "P607",
        name: "Lavender Blue MD",
        color: &[130, 154, 206],
    },
    ThreadRef {
        brand: BRAND,
        code: "P608",
        name: "Lavender Blue DK",
        color: &[105, 118, 183],
    },
    ThreadRef {
        brand: BRAND,
        code: "P612",
        name: "Cornflower Blue",
        color: &[84, 98, 152],
    },
    ThreadRef {
        brand: BRAND,
        code: "P613",
        name: "Cornflower Blue DK",
        color: &[66, 72, 116],
    },
    ThreadRef {
        brand: BRAND,
        code: "P614",
        name: "Sky Blue",
        color: &[174, 207, 228],
    },
    ThreadRef {
        brand: BRAND,
        code: "P616",
        name: "Delft Blue MD",
        color: &[121, 158, 202],
    },
    ThreadRef {
        brand: BRAND,
        code: "P617",
        name: "Delft Blue DK",
        color: &[76, 113, 179],
    },
    ThreadRef {
        brand: BRAND,
        code: "P618",
        name: "Royal Blue",
        color: &[56, 86, 140],
    },
    ThreadRef {
        brand: BRAND,
        code: "P619",
        name: "Royal Blue DK",
        color: &[42, 62, 114],
    },
    ThreadRef {
        brand: BRAND,
        code: "P620",
        name: "Marine Blue",
        color: &[30, 49, 96],
    },
    ThreadRef {
        brand: BRAND,
        code: "P621",
        name: "Antique Blue LT",
        color: &[194, 214, 228],
    },
    ThreadRef {
        brand: BRAND,
        code: "P623",
        name: "Seagull Blue",
        color: &[137, 162, 185],
    },
    ThreadRef {
        brand: BRAND,
        code: "P624",
        name: "Blue Gray",
        color: &[107, 130, 151],
    },
    ThreadRef {
        brand: BRAND,
        code: "P626",
        name: "Antique Blue DK",
        color: &[47, 77, 93],
    },
    ThreadRef {
        brand: BRAND,
        code: "P627",
        name: "Cloud Blue",
        color: &[233, 241, 240],
    },
    ThreadRef {
        brand: BRAND,
        code: "P628",
        name: "Baby Blue LT",
        color: &[206, 228, 232],
    },
    ThreadRef {
        brand: BRAND,
        code: "P629",
        name: "Pale Baby Blue",
        color: &[186, 216, 224],
    },
    ThreadRef {
        brand: BRAND,
        code: "P630",
        name: "Azur Blue",
        color: &[162, 199, 214],
    },
    ThreadRef {
        brand: BRAND,
        code: "P631",
        name: "Baby Blue",
        color: &[126, 173, 200],
    },
    ThreadRef {
        brand: BRAND,
        code: "P633",
        name: "Baby Blue DK",
        color: &[88, 128, 175],
    },
    ThreadRef {
        brand: BRAND,
        code: "P634",
        name: "Night Blue",
        color: &[56, 97, 137],
    },
    ThreadRef {
        brand: BRAND,
        code: "P636",
        name: "Navy Blue",
        color: &[48, 70, 100],
    },
    ThreadRef {
        brand: BRAND,
        code: "P637",
        name: "Blueberry Blue",
        color: &[35, 39, 73],
    },
    ThreadRef {
        brand: BRAND,
        code: "P638",
        name: "Navy Blue DK",
        color: &[39, 45, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "P639",
        name: "Morning Sky Blue",
        color: &[195, 225, 232],
    },
    ThreadRef {
        brand: BRAND,
        code: "P640",
        name: "Forget-Me-Not Blue",
        color: &[173, 213, 229],
    },
    ThreadRef {
        brand: BRAND,
        code: "P641",
        name: "Light Blue",
        color: &[131, 176, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "P642",
        name: "Medium Blue",
        color: &[94, 145, 188],
    },
    ThreadRef {
        brand: BRAND,
        code: "P643",
        name: "Blue DK",
        color: &[64, 112, 161],
    },
    ThreadRef {
        brand: BRAND,
        code: "P644",
        name: "Ocean Blue",
        color: &[38, 87, 135],
    },
    ThreadRef {
        brand: BRAND,
        code: "P645",
        name: "Light Sky Blue",
        color: &[176, 217, 230],
    },
    ThreadRef {
        brand: BRAND,
        code: "P651",
        name: "Electric Blue LT",
        color: &[0, 166, 222],
    },
    ThreadRef {
        brand: BRAND,
        code: "P652",
        name: "Electric Blue MD",
        color: &[0, 139, 204],
    },
    ThreadRef {
        brand: BRAND,
        code: "P653",
        name: "Caribbean Blue",
        color: &[3, 112, 175],
    },
    ThreadRef {
        brand: BRAND,
        code: "P654",
        name: "Sea Mist Blue",
        color: &[199, 232, 232],
    },
    ThreadRef {
        brand: BRAND,
        code: "P655",
        name: "Peacock Blue LT",
        color: &[127, 193, 205],
    },
    ThreadRef {
        brand: BRAND,
        code: "P656",
        name: "Peacock Blue",
        color: &[56, 145, 170],
    },
    ThreadRef {
        brand: BRAND,
        code: "P658",
        name: "Peacock Blue DK",
        color: &[2, 104, 138],
    },
    ThreadRef {
        brand: BRAND,
        code: "P665",
        name: "Bright Turquoise LT",
        color: &[41, 194, 222],
    },
    ThreadRef {
        brand: BRAND,
        code: "P666",
        name: "Bright Turquoise",
        color: &[19, 178, 207],
    },
    ThreadRef {
        brand: BRAND,
        code: "P667",
        name: "Bright Turquoise DK",
        color: &[0, 152, 184],
    },
    ThreadRef {
        brand: BRAND,
        code: "P668",
        name: "Teal Green LT",
        color: &[125, 186, 176],
    },
    ThreadRef {
        brand: BRAND,
        code: "P669",
        name: "Teal green MD",
        color: &[60, 143, 133],
    },
    ThreadRef {
        brand: BRAND,
        code: "P670",
        name: "Teal Green DK",
        color: &[29, 111, 102],
    },
    ThreadRef {
        brand: BRAND,
        code: "P671",
        name: "Light Sea Green",
        color: &[186, 232, 217],
    },
    ThreadRef {
        brand: BRAND,
        code: "P675",
        name: "Emerald Green",
        color: &[16, 135, 105],
    },
    ThreadRef {
        brand: BRAND,
        code: "P677",
        name: "Bright Green",
        color: &[72, 174, 147],
    },
    ThreadRef {
        brand: BRAND,
        code: "P678",
        name: "Aquamarine LT",
        color: &[117, 197, 173],
    },
    ThreadRef {
        brand: BRAND,
        code: "P681",
        name: "Aquamarine Green DK",
        color: &[14, 116, 91],
    },
    ThreadRef {
        brand: BRAND,
        code: "P690",
        name: "Blue Green LT",
        color: &[171, 208, 190],
    },
    ThreadRef {
        brand: BRAND,
        code: "P692",
        name: "Blue Green MD",
        color: &[130, 177, 154],
    },
    ThreadRef {
        brand: BRAND,
        code: "P693",
        name: "Blue Green",
        color: &[104, 152, 126],
    },
    ThreadRef {
        brand: BRAND,
        code: "P694",
        name: "Blue Green-DK",
        color: &[74, 120, 98],
    },
    ThreadRef {
        brand: BRAND,
        code: "P695",
        name: "Ivy Green",
        color: &[46, 76, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "P696",
        name: "Malachite Green LT",
        color: &[175, 217, 186],
    },
    ThreadRef {
        brand: BRAND,
        code: "P697",
        name: "Jade LT",
        color: &[135, 195, 158],
    },
    ThreadRef {
        brand: BRAND,
        code: "P698",
        name: "Jade MD",
        color: &[89, 155, 109],
    },
    ThreadRef {
        brand: BRAND,
        code: "P699",
        name: "Jade DK",
        color: &[55, 117, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "P700",
        name: "Pale Green",
        color: &[187, 233, 193],
    },
    ThreadRef {
        brand: BRAND,
        code: "P701",
        name: "Field Green",
        color: &[144, 209, 158],
    },
    ThreadRef {
        brand: BRAND,
        code: "P703",
        name: "Peppermint Green",
        color: &[95, 183, 128],
    },
    ThreadRef {
        brand: BRAND,
        code: "P704",
        name: "Golf Green",
        color: &[61, 153, 98],
    },
    ThreadRef {
        brand: BRAND,
        code: "P706",
        name: "Emerald Green DK",
        color: &[1, 117, 67],
    },
    ThreadRef {
        brand: BRAND,
        code: "P709",
        name: "Pistachio Green LT",
        color: &[196, 226, 180],
    },
    ThreadRef {
        brand: BRAND,
        code: "P710",
        name: "Pistachio Green",
        color: &[152, 193, 139],
    },
    ThreadRef {
        brand: BRAND,
        code: "P712",
        name: "Pistachio Green DK",
        color: &[88, 138, 93],
    },
    ThreadRef {
        brand: BRAND,
        code: "P713",
        name: "Pistachio Green DK",
        color: &[44, 92, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "P714",
        name: "Deep Forest Green",
        color: &[20, 77, 41],
    },
    ThreadRef {
        brand: BRAND,
        code: "P715",
        name: "Forest Green LT",
        color: &[135, 180, 110],
    },
    ThreadRef {
        brand: BRAND,
        code: "P716",
        name: "Forest Green MD",
        color: &[119, 167, 93],
    },
    ThreadRef {
        brand: BRAND,
        code: "P717",
        name: "Forest Green",
        color: &[94, 140, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "P718",
        name: "Forest Green DK",
        color: &[58, 116, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "P719",
        name: "Yellow Green LT",
        color: &[208, 228, 175],
    },
    ThreadRef {
        brand: BRAND,
        code: "P720",
        name: "Lettuce Heart Green",
        color: &[166, 195, 121],
    },
    ThreadRef {
        brand: BRAND,
        code: "P721",
        name: "Insect Green",
        color: &[127, 162, 92],
    },
    ThreadRef {
        brand: BRAND,
        code: "P722",
        name: "Hunter Green",
        color: &[100, 135, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "P723",
        name: "Hunter Green MD",
        color: &[72, 109, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "P724",
        name: "Hunter Green DK",
        color: &[60, 98, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "P735",
        name: "Avocado Green LT",
        color: &[193, 206, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "P736",
        name: "Tarragon Green",
        color: &[149, 174, 88],
    },
    ThreadRef {
        brand: BRAND,
        code: "P737",
        name: "Olive Green",
        color: &[128, 160, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "P739",
        name: "Avocado Green MD",
        color: &[96, 126, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "P740",
        name: "Avocado Green DK",
        color: &[84, 101, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "P742",
        name: "Undergrowth Green",
        color: &[73, 92, 59],
    },
    ThreadRef {
        brand: BRAND,
        code: "P744",
        name: "Lime Green",
        color: &[160, 199, 105],
    },
    ThreadRef {
        brand: BRAND,
        code: "P745",
        name: "Springtime Green",
        color: &[129, 184, 94],
    },
    ThreadRef {
        brand: BRAND,
        code: "P746",
        name: "Fresh Green",
        color: &[92, 164, 84],
    },
    ThreadRef {
        brand: BRAND,
        code: "P747",
        name: "Christmas Green",
        color: &[61, 140, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "P748",
        name: "Christmas Green MD",
        color: &[44, 122, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "P749",
        name: "Christmas Green DK",
        color: &[16, 107, 61],
    },
    ThreadRef {
        brand: BRAND,
        code: "P750",
        name: "Parrot Green LT",
        color: &[152, 190, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "P751",
        name: "Parrot Green",
        color: &[107, 169, 32],
    },
    ThreadRef {
        brand: BRAND,
        code: "P752",
        name: "Parrot Green MD",
        color: &[79, 140, 13],
    },
    ThreadRef {
        brand: BRAND,
        code: "P753",
        name: "Parrot Green DK",
        color: &[59, 114, 34],
    },
    ThreadRef {
        brand: BRAND,
        code: "P755",
        name: "Moss Green",
        color: &[132, 150, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "P756",
        name: "Moss Green DK",
        color: &[110, 130, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "P758",
        name: "Olive Green MD",
        color: &[168, 167, 72],
    },
    ThreadRef {
        brand: BRAND,
        code: "P759",
        name: "Olive Green",
        color: &[136, 137, 47],
    },
    ThreadRef {
        brand: BRAND,
        code: "P761",
        name: "Olive Green DK",
        color: &[110, 114, 44],
    },
    ThreadRef {
        brand: BRAND,
        code: "P768",
        name: "Light Brass",
        color: &[205, 190, 103],
    },
    ThreadRef {
        brand: BRAND,
        code: "P769",
        name: "Golden Olive LT",
        color: &[187, 169, 80],
    },
    ThreadRef {
        brand: BRAND,
        code: "P770",
        name: "Green Bronze LT",
        color: &[170, 149, 58],
    },
    ThreadRef {
        brand: BRAND,
        code: "P771",
        name: "Golden Olive MD",
        color: &[140, 125, 49],
    },
    ThreadRef {
        brand: BRAND,
        code: "P772",
        name: "Golden Olive DK",
        color: &[124, 112, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "P778",
        name: "Yellow Beige LT",
        color: &[237, 235, 189],
    },
    ThreadRef {
        brand: BRAND,
        code: "P779",
        name: "Yellow Beige",
        color: &[217, 212, 153],
    },
    ThreadRef {
        brand: BRAND,
        code: "P785",
        name: "Topaz MD",
        color: &[204, 161, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "P786",
        name: "Topaz DK",
        color: &[182, 126, 54],
    },
    ThreadRef {
        brand: BRAND,
        code: "P787",
        name: "Topaz VDK",
        color: &[149, 100, 40],
    },
    ThreadRef {
        brand: BRAND,
        code: "P788",
        name: "Chestnut Tree Brown",
        color: &[140, 74, 35],
    },
    ThreadRef {
        brand: BRAND,
        code: "P789",
        name: "Vanilla",
        color: &[254, 249, 212],
    },
    ThreadRef {
        brand: BRAND,
        code: "P790",
        name: "Sand Gold",
        color: &[224, 217, 161],
    },
    ThreadRef {
        brand: BRAND,
        code: "P791",
        name: "Old Gold LT",
        color: &[226, 202, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "P792",
        name: "Old Gold MD",
        color: &[204, 172, 88],
    },
    ThreadRef {
        brand: BRAND,
        code: "P793",
        name: "Old Gold DK",
        color: &[173, 142, 65],
    },
    ThreadRef {
        brand: BRAND,
        code: "P799",
        name: "Autumn Gold LT",
        color: &[241, 207, 135],
    },
    ThreadRef {
        brand: BRAND,
        code: "P800",
        name: "Autumn Gold MD",
        color: &[242, 179, 92],
    },
    ThreadRef {
        brand: BRAND,
        code: "P801",
        name: "Autumn Gold DK",
        color: &[227, 142, 57],
    },
    ThreadRef {
        brand: BRAND,
        code: "P802",
        name: "Light Lemon",
        color: &[250, 243, 134],
    },
    ThreadRef {
        brand: BRAND,
        code: "P803",
        name: "Lemon",
        color: &[245, 237, 89],
    },
    ThreadRef {
        brand: BRAND,
        code: "P804",
        name: "Bright Yellow",
        color: &[238, 202, 2],
    },
    ThreadRef {
        brand: BRAND,
        code: "P805",
        name: "Pale Yellow",
        color: &[252, 239, 170],
    },
    ThreadRef {
        brand: BRAND,
        code: "P806",
        name: "Primrose Yellow",
        color: &[255, 236, 129],
    },
    ThreadRef {
        brand: BRAND,
        code: "P807",
        name: "Topaz LT",
        color: &[252, 214, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "P808",
        name: "Topaz",
        color: &[243, 193, 57],
    },
    ThreadRef {
        brand: BRAND,
        code: "P810",
        name: "Banana Yellow",
        color: &[240, 235, 158],
    },
    ThreadRef {
        brand: BRAND,
        code: "P811",
        name: "Pale Yellow",
        color: &[242, 229, 129],
    },
    ThreadRef {
        brand: BRAND,
        code: "P812",
        name: "Yellow MD",
        color: &[246, 216, 87],
    },
    ThreadRef {
        brand: BRAND,
        code: "P813",
        name: "Tangerine LT",
        color: &[250, 194, 60],
    },
    ThreadRef {
        brand: BRAND,
        code: "P814",
        name: "Tangerine MD",
        color: &[250, 158, 13],
    },
    ThreadRef {
        brand: BRAND,
        code: "P815",
        name: "Orange",
        color: &[242, 125, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "P816",
        name: "Canary BRT",
        color: &[245, 214, 2],
    },
    ThreadRef {
        brand: BRAND,
        code: "P817",
        name: "Canary DK",
        color: &[246, 175, 6],
    },
    ThreadRef {
        brand: BRAND,
        code: "P819",
        name: "Pumpkin Light",
        color: &[250, 128, 61],
    },
    ThreadRef {
        brand: BRAND,
        code: "P820",
        name: "Sunset Orange",
        color: &[245, 100, 29],
    },
    ThreadRef {
        brand: BRAND,
        code: "P821",
        name: "Fire Orange",
        color: &[229, 84, 33],
    },
    ThreadRef {
        brand: BRAND,
        code: "P822",
        name: "Saffron Orange",
        color: &[202, 82, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "P823",
        name: "Bright Orange",
        color: &[244, 70, 52],
    },
    ThreadRef {
        brand: BRAND,
        code: "P824",
        name: "Bright Orange Red",
        color: &[218, 38, 34],
    },
    ThreadRef {
        brand: BRAND,
        code: "P826",
        name: "Peach",
        color: &[247, 170, 144],
    },
    ThreadRef {
        brand: BRAND,
        code: "P827",
        name: "Apricot MD",
        color: &[247, 126, 91],
    },
    ThreadRef {
        brand: BRAND,
        code: "P829",
        name: "Orange Spice LT",
        color: &[246, 167, 102],
    },
    ThreadRef {
        brand: BRAND,
        code: "P830",
        name: "Orange Spice MD",
        color: &[239, 127, 64],
    },
    ThreadRef {
        brand: BRAND,
        code: "P832",
        name: "Terracotta",
        color: &[217, 129, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "P833",
        name: "Copper",
        color: &[196, 109, 56],
    },
    ThreadRef {
        brand: BRAND,
        code: "P834",
        name: "Copper MD",
        color: &[175, 89, 45],
    },
    ThreadRef {
        brand: BRAND,
        code: "P836",
        name: "Red Copper DK",
        color: &[145, 77, 45],
    },
    ThreadRef {
        brand: BRAND,
        code: "P837",
        name: "Pale Beech Wood",
        color: &[239, 198, 141],
    },
    ThreadRef {
        brand: BRAND,
        code: "P838",
        name: "Mahogany LT",
        color: &[231, 169, 106],
    },
    ThreadRef {
        brand: BRAND,
        code: "P839",
        name: "Mahogany",
        color: &[207, 131, 72],
    },
    ThreadRef {
        brand: BRAND,
        code: "P840",
        name: "Mahogany MD",
        color: &[170, 103, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "P841",
        name: "Brown",
        color: &[141, 82, 32],
    },
    ThreadRef {
        brand: BRAND,
        code: "P842",
        name: "Mahogany DK",
        color: &[120, 78, 36],
    },
    ThreadRef {
        brand: BRAND,
        code: "P843",
        name: "Rosewood LT",
        color: &[188, 138, 108],
    },
    ThreadRef {
        brand: BRAND,
        code: "P845",
        name: "Rosewood DK",
        color: &[118, 72, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "P847",
        name: "Caramel Brown",
        color: &[222, 167, 85],
    },
    ThreadRef {
        brand: BRAND,
        code: "P848",
        name: "Golden Brown MD",
        color: &[196, 135, 51],
    },
    ThreadRef {
        brand: BRAND,
        code: "P850",
        name: "Chestnut Brown",
        color: &[139, 91, 44],
    },
    ThreadRef {
        brand: BRAND,
        code: "P851",
        name: "Pale Eggshell Cream",
        color: &[244, 235, 218],
    },
    ThreadRef {
        brand: BRAND,
        code: "P852",
        name: "Tawny LT",
        color: &[238, 219, 188],
    },
    ThreadRef {
        brand: BRAND,
        code: "P853",
        name: "Tawny",
        color: &[238, 202, 164],
    },
    ThreadRef {
        brand: BRAND,
        code: "P856",
        name: "Nutmeg Brown",
        color: &[205, 166, 135],
    },
    ThreadRef {
        brand: BRAND,
        code: "P860",
        name: "Cocoa",
        color: &[138, 98, 69],
    },
    ThreadRef {
        brand: BRAND,
        code: "P870",
        name: "Cream",
        color: &[243, 238, 219],
    },
    ThreadRef {
        brand: BRAND,
        code: "P871",
        name: "Tan Light",
        color: &[229, 218, 178],
    },
    ThreadRef {
        brand: BRAND,
        code: "P872",
        name: "Tan Medium",
        color: &[217, 195, 143],
    },
    ThreadRef {
        brand: BRAND,
        code: "P873",
        name: "Camel",
        color: &[217, 184, 121],
    },
    ThreadRef {
        brand: BRAND,
        code: "P874",
        name: "Tan",
        color: &[205, 165, 104],
    },
    ThreadRef {
        brand: BRAND,
        code: "P875",
        name: "Tobacco Brown",
        color: &[181, 135, 71],
    },
    ThreadRef {
        brand: BRAND,
        code: "P876",
        name: "Brown LT",
        color: &[146, 106, 50],
    },
    ThreadRef {
        brand: BRAND,
        code: "P877",
        name: "Chocolate Brown",
        color: &[117, 88, 45],
    },
    ThreadRef {
        brand: BRAND,
        code: "P878",
        name: "Coffee Brown",
        color: &[98, 74, 42],
    },
    ThreadRef {
        brand: BRAND,
        code: "P880",
        name: "Coffee Brown DK",
        color: &[80, 68, 43],
    },
    ThreadRef {
        brand: BRAND,
        code: "P881",
        name: "Ebony",
        color: &[58, 57, 38],
    },
    ThreadRef {
        brand: BRAND,
        code: "P886",
        name: "Beige Brown LT",
        color: &[208, 198, 171],
    },
    ThreadRef {
        brand: BRAND,
        code: "P887",
        name: "Deer Brown",
        color: &[171, 156, 125],
    },
    ThreadRef {
        brand: BRAND,
        code: "P888",
        name: "Beige Brown-MD",
        color: &[134, 119, 88],
    },
    ThreadRef {
        brand: BRAND,
        code: "P889",
        name: "Beige Brown-DK",
        color: &[111, 94, 67],
    },
    ThreadRef {
        brand: BRAND,
        code: "P890",
        name: "Dark Wood",
        color: &[82, 68, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "P892",
        name: "Brown Gray LT",
        color: &[182, 184, 148],
    },
    ThreadRef {
        brand: BRAND,
        code: "P894",
        name: "Brown Gray DK",
        color: &[113, 115, 91],
    },
    ThreadRef {
        brand: BRAND,
        code: "P896",
        name: "Beige Gray LT",
        color: &[227, 225, 201],
    },
    ThreadRef {
        brand: BRAND,
        code: "P897",
        name: "Beige Gray MD",
        color: &[203, 203, 172],
    },
    ThreadRef {
        brand: BRAND,
        code: "P899",
        name: "Beige Gray DK",
        color: &[158, 156, 112],
    },
    ThreadRef {
        brand: BRAND,
        code: "P900",
        name: "Cappuccino Brown",
        color: &[136, 126, 90],
    },
    ThreadRef {
        brand: BRAND,
        code: "P902",
        name: "Mocha Brown LT",
        color: &[221, 218, 197],
    },
    ThreadRef {
        brand: BRAND,
        code: "P904",
        name: "Mocha Brown MD",
        color: &[165, 156, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "P905",
        name: "Metal Brown",
        color: &[107, 99, 63],
    },
    ThreadRef {
        brand: BRAND,
        code: "P906",
        name: "Mocha Brown DK",
        color: &[82, 75, 48],
    },
    ThreadRef {
        brand: BRAND,
        code: "P911",
        name: "Pale Pearl Gray",
        color: &[213, 221, 209],
    },
    ThreadRef {
        brand: BRAND,
        code: "P912",
        name: "Pepper Gray",
        color: &[186, 191, 173],
    },
    ThreadRef {
        brand: BRAND,
        code: "P913",
        name: "Rock Gray",
        color: &[161, 168, 143],
    },
    ThreadRef {
        brand: BRAND,
        code: "P914",
        name: "Platinum Gray",
        color: &[135, 142, 121],
    },
    ThreadRef {
        brand: BRAND,
        code: "P915",
        name: "Steel Gray DK",
        color: &[109, 118, 96],
    },
    ThreadRef {
        brand: BRAND,
        code: "P916",
        name: "Pepper Black",
        color: &[87, 91, 75],
    },
    ThreadRef {
        brand: BRAND,
        code: "P917",
        name: "Pearl Gray",
        color: &[217, 223, 225],
    },
    ThreadRef {
        brand: BRAND,
        code: "P918",
        name: "Chrome Gray",
        color: &[184, 193, 197],
    },
    ThreadRef {
        brand: BRAND,
        code: "P919",
        name: "Steel Gray LT",
        color: &[168, 173, 176],
    },
    ThreadRef {
        brand: BRAND,
        code: "P920",
        name: "Steel Gray MD",
        color: &[140, 143, 145],
    },
    ThreadRef {
        brand: BRAND,
        code: "P921",
        name: "Steel Gray DK",
        color: &[109, 118, 121],
    },
    ThreadRef {
        brand: BRAND,
        code: "P922",
        name: "Pewter Gray",
        color: &[91, 97, 94],
    },
    ThreadRef {
        brand: BRAND,
        code: "P923",
        name: "Pewter Gray DK",
        color: &[75, 83, 73],
    },
    ThreadRef {
        brand: BRAND,
        code: "P924",
        name: "Black",
        color: &[0, 0, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "P946",
        name: "White",
        color: &[255, 255, 255],
    },
];