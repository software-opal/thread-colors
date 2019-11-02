#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

const BRAND: &'static str = "Viking Palette";
const THREADS: [ThreadRef; 33] = [
    ThreadRef {
        brand: BRAND,
        code: "1",
        name: "Black",
        color: &[0, 0, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "10",
        name: "Orange",
        color: &[255, 130, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "11",
        name: "Pink",
        color: &[255, 162, 181],
    },
    ThreadRef {
        brand: BRAND,
        code: "12",
        name: "Brown",
        color: &[198, 65, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "13",
        name: "White",
        color: &[255, 255, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "14",
        name: "Dark Blue",
        color: &[0, 0, 132],
    },
    ThreadRef {
        brand: BRAND,
        code: "15",
        name: "Dark Green",
        color: &[0, 130, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "16",
        name: "Dark Red",
        color: &[165, 0, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "17",
        name: "Light Red",
        color: &[255, 121, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "18",
        name: "Dark Purple",
        color: &[132, 0, 132],
    },
    ThreadRef {
        brand: BRAND,
        code: "19",
        name: "Light Purple",
        color: &[255, 130, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "2",
        name: "Blue",
        color: &[0, 0, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "2",
        name: "Blue (Cutwork #4)",
        color: &[0, 0, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "20",
        name: "Dark Yellow",
        color: &[198, 195, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "21",
        name: "Light Yellow",
        color: &[255, 255, 165],
    },
    ThreadRef {
        brand: BRAND,
        code: "22",
        name: "Dark Gray",
        color: &[66, 65, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "23",
        name: "Light Gray",
        color: &[198, 195, 198],
    },
    ThreadRef {
        brand: BRAND,
        code: "24",
        name: "Dark Orange",
        color: &[231, 65, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "24",
        name: "Dark Orange (Cutwork #1)",
        color: &[231, 65, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "25",
        name: "Light Orange",
        color: &[255, 174, 66],
    },
    ThreadRef {
        brand: BRAND,
        code: "26",
        name: "Dark Pink",
        color: &[255, 89, 123],
    },
    ThreadRef {
        brand: BRAND,
        code: "27",
        name: "Light Pink",
        color: &[255, 211, 214],
    },
    ThreadRef {
        brand: BRAND,
        code: "28",
        name: "Dark Brown",
        color: &[132, 32, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "29",
        name: "Light Brown",
        color: &[231, 97, 33],
    },
    ThreadRef {
        brand: BRAND,
        code: "3",
        name: "Light Green",
        color: &[0, 255, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "3",
        name: "Light Green (Cutwork #3)",
        color: &[0, 255, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "4",
        name: "Red",
        color: &[255, 0, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "5",
        name: "Purple",
        color: &[255, 0, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "6",
        name: "Yellow",
        color: &[255, 255, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "6",
        name: "Yellow (Cutwork #2)",
        color: &[255, 255, 0],
    },
    ThreadRef {
        brand: BRAND,
        code: "7",
        name: "Gray",
        color: &[132, 130, 132],
    },
    ThreadRef {
        brand: BRAND,
        code: "8",
        name: "Light Blue",
        color: &[0, 130, 255],
    },
    ThreadRef {
        brand: BRAND,
        code: "9",
        name: "Green",
        color: &[0, 255, 132],
    },
];