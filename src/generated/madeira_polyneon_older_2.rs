#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::ThreadRef;

pub const BRAND: &'static str = "Madeira Polyneon - older 2";
pub const THREADS: [ThreadRef; 172] = [
    ThreadRef::new(
        BRAND,
        "1612",
        "Rockport Blue",
        &[157, 175, 189],
    ),
    ThreadRef::new(
        BRAND,
        "1620",
        "Rose Cerise",
        &[255, 147, 162],
    ),
    ThreadRef::new(
        BRAND,
        "1625",
        "Gold/Orange",
        &[200, 128, 43],
    ),
    ThreadRef::new(
        BRAND,
        "1630",
        "Sweet Lavender",
        &[155, 153, 192],
    ),
    ThreadRef::new(
        BRAND,
        "1631",
        "Paisley Purple",
        &[146, 108, 167],
    ),
    ThreadRef::new(
        BRAND,
        "1632",
        "Dark Purple",
        &[71, 58, 86],
    ),
    ThreadRef::new(
        BRAND,
        "1633",
        "Plum Wine",
        &[95, 41, 103],
    ),
    ThreadRef::new(
        BRAND,
        "1635",
        "Dark Brown",
        &[106, 47, 65],
    ),
    ThreadRef::new(
        BRAND,
        "1640",
        "Western Grey",
        &[97, 101, 102],
    ),
    ThreadRef::new(
        BRAND,
        "1641",
        "Black",
        &[55, 58, 63],
    ),
    ThreadRef::new(
        BRAND,
        "1645",
        "Holiday",
        &[130, 202, 190],
    ),
    ThreadRef::new(
        BRAND,
        "1652",
        "Dark Aqua Green",
        &[57, 122, 124],
    ),
    ThreadRef::new(
        BRAND,
        "1657",
        "Rancho Brown",
        &[137, 95, 71],
    ),
    ThreadRef::new(
        BRAND,
        "1660",
        "Ashes Of Roses",
        &[184, 173, 169],
    ),
    ThreadRef::new(
        BRAND,
        "1662",
        "Moon Rock",
        &[146, 137, 120],
    ),
    ThreadRef::new(
        BRAND,
        "1675",
        "Caribbean Blue",
        &[116, 174, 214],
    ),
    ThreadRef::new(
        BRAND,
        "1680",
        "Dark Purple",
        &[98, 64, 97],
    ),
    ThreadRef::new(
        BRAND,
        "1681",
        "Dusky Rose",
        &[192, 57, 87],
    ),
    ThreadRef::new(
        BRAND,
        "1683",
        "Mimosa",
        &[252, 198, 89],
    ),
    ThreadRef::new(
        BRAND,
        "1703",
        "Dark Green",
        &[10, 90, 65],
    ),
    ThreadRef::new(
        BRAND,
        "1707",
        "Dusky Pink",
        &[199, 74, 92],
    ),
    ThreadRef::new(
        BRAND,
        "1710",
        "Dark Purple",
        &[157, 54, 123],
    ),
    ThreadRef::new(
        BRAND,
        "1711",
        "Sterling",
        &[163, 142, 175],
    ),
    ThreadRef::new(
        BRAND,
        "1721",
        "Medium Pink",
        &[234, 102, 149],
    ),
    ThreadRef::new(
        BRAND,
        "1722",
        "Purple",
        &[67, 57, 128],
    ),
    ThreadRef::new(
        BRAND,
        "1723",
        "Cream",
        &[246, 212, 174],
    ),
    ThreadRef::new(
        BRAND,
        "1744",
        "Dark Fawn",
        &[101, 84, 74],
    ),
    ThreadRef::new(
        BRAND,
        "1756",
        "Pale Green",
        &[93, 95, 48],
    ),
    ThreadRef::new(
        BRAND,
        "1757",
        "Dark Green/Yellow",
        &[103, 85, 49],
    ),
    ThreadRef::new(
        BRAND,
        "1760",
        "Petrol Blue",
        &[72, 118, 134],
    ),
    ThreadRef::new(
        BRAND,
        "1762",
        "Dark Blue",
        &[28, 86, 106],
    ),
    ThreadRef::new(
        BRAND,
        "1765",
        "Hazel",
        &[237, 114, 34],
    ),
    ThreadRef::new(
        BRAND,
        "1766",
        "Medium Purple",
        &[57, 60, 127],
    ),
    ThreadRef::new(
        BRAND,
        "1767",
        "Dark Blue",
        &[37, 59, 119],
    ),
    ThreadRef::new(
        BRAND,
        "1770",
        "Medium Grey/Green",
        &[72, 117, 62],
    ),
    ThreadRef::new(
        BRAND,
        "1771",
        "Amber",
        &[230, 166, 68],
    ),
    ThreadRef::new(
        BRAND,
        "1773",
        "Dark Orange/Brown",
        &[179, 103, 43],
    ),
    ThreadRef::new(
        BRAND,
        "1776",
        "Dark Blue",
        &[29, 79, 104],
    ),
    ThreadRef::new(
        BRAND,
        "1779",
        "Spiced Coral",
        &[215, 92, 87],
    ),
    ThreadRef::new(
        BRAND,
        "1782",
        "Wine",
        &[139, 54, 75],
    ),
    ThreadRef::new(
        BRAND,
        "1784",
        "Brown",
        &[114, 42, 54],
    ),
    ThreadRef::new(
        BRAND,
        "1788",
        "Dark Purple",
        &[128, 57, 113],
    ),
    ThreadRef::new(
        BRAND,
        "1789",
        "Dark Maroon",
        &[100, 60, 86],
    ),
    ThreadRef::new(
        BRAND,
        "1790",
        "Olive Green",
        &[127, 111, 49],
    ),
    ThreadRef::new(
        BRAND,
        "1791",
        "Wicker",
        &[171, 119, 61],
    ),
    ThreadRef::new(
        BRAND,
        "1792",
        "Orange/Brown",
        &[183, 138, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1800",
        "Black",
        &[1, 1, 1],
    ),
    ThreadRef::new(
        BRAND,
        "1801",
        "White",
        &[254, 254, 254],
    ),
    ThreadRef::new(
        BRAND,
        "1802",
        "White",
        &[234, 240, 254],
    ),
    ThreadRef::new(
        BRAND,
        "1803",
        "Eggshell",
        &[238, 236, 223],
    ),
    ThreadRef::new(
        BRAND,
        "1812",
        "Pigeon",
        &[176, 176, 168],
    ),
    ThreadRef::new(
        BRAND,
        "1815",
        "Memphis Belle",
        &[254, 195, 213],
    ),
    ThreadRef::new(
        BRAND,
        "1816",
        "Petal Pink",
        &[255, 178, 196],
    ),
    ThreadRef::new(
        BRAND,
        "1817",
        "Salmon",
        &[255, 174, 157],
    ),
    ThreadRef::new(
        BRAND,
        "1819",
        "Peony",
        &[253, 165, 163],
    ),
    ThreadRef::new(
        BRAND,
        "1823",
        "Gold/Yellow",
        &[249, 255, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1824",
        "Medium Yellow",
        &[255, 221, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1825",
        "Fluo Orange",
        &[254, 187, 8],
    ),
    ThreadRef::new(
        BRAND,
        "1826",
        "Buff Orange",
        &[255, 198, 125],
    ),
    ThreadRef::new(
        BRAND,
        "1827",
        "Bachelor Button",
        &[89, 190, 220],
    ),
    ThreadRef::new(
        BRAND,
        "1828",
        "Western Sky",
        &[70, 143, 176],
    ),
    ThreadRef::new(
        BRAND,
        "1829",
        "Medium Blue",
        &[0, 110, 171],
    ),
    ThreadRef::new(
        BRAND,
        "1830",
        "Blue Bonnet",
        &[110, 141, 195],
    ),
    ThreadRef::new(
        BRAND,
        "1831",
        "Hyacinth",
        &[162, 109, 165],
    ),
    ThreadRef::new(
        BRAND,
        "1832",
        "Kings Purple",
        &[107, 82, 148],
    ),
    ThreadRef::new(
        BRAND,
        "1833",
        "Dark Purple",
        &[124, 48, 120],
    ),
    ThreadRef::new(
        BRAND,
        "1835",
        "Brown",
        &[135, 49, 74],
    ),
    ThreadRef::new(
        BRAND,
        "1836",
        "Walnut",
        &[75, 53, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1837",
        "Paprika",
        &[255, 67, 38],
    ),
    ThreadRef::new(
        BRAND,
        "1838",
        "Medium Red",
        &[185, 34, 51],
    ),
    ThreadRef::new(
        BRAND,
        "1839",
        "Red Berry",
        &[177, 29, 51],
    ),
    ThreadRef::new(
        BRAND,
        "1840",
        "Pale Grey/Blue",
        &[118, 127, 136],
    ),
    ThreadRef::new(
        BRAND,
        "1841",
        "Dark Grey",
        &[80, 87, 95],
    ),
    ThreadRef::new(
        BRAND,
        "1842",
        "Solar Blue",
        &[0, 83, 151],
    ),
    ThreadRef::new(
        BRAND,
        "1843",
        "Royal Blue",
        &[19, 67, 115],
    ),
    ThreadRef::new(
        BRAND,
        "1844",
        "Navy",
        &[46, 48, 60],
    ),
    ThreadRef::new(
        BRAND,
        "1845",
        "Seafoam",
        &[118, 195, 161],
    ),
    ThreadRef::new(
        BRAND,
        "1846",
        "Medium Green/Blue",
        &[0, 153, 150],
    ),
    ThreadRef::new(
        BRAND,
        "1847",
        "Emerald Green",
        &[70, 172, 148],
    ),
    ThreadRef::new(
        BRAND,
        "1848",
        "Grass Green",
        &[122, 162, 89],
    ),
    ThreadRef::new(
        BRAND,
        "1850",
        "Bright Green",
        &[106, 204, 81],
    ),
    ThreadRef::new(
        BRAND,
        "1851",
        "Dark Green",
        &[0, 97, 56],
    ),
    ThreadRef::new(
        BRAND,
        "1852",
        "Medium Green/Blue",
        &[47, 129, 151],
    ),
    ThreadRef::new(
        BRAND,
        "1854",
        "Light Brown",
        &[165, 141, 137],
    ),
    ThreadRef::new(
        BRAND,
        "1855",
        "Umber",
        &[186, 152, 115],
    ),
    ThreadRef::new(
        BRAND,
        "1858",
        "Sepia",
        &[112, 62, 53],
    ),
    ThreadRef::new(
        BRAND,
        "1859",
        "Dark Grey",
        &[78, 64, 61],
    ),
    ThreadRef::new(
        BRAND,
        "1861",
        "Visor Gold",
        &[243, 207, 129],
    ),
    ThreadRef::new(
        BRAND,
        "1866",
        "PaleYellow",
        &[245, 228, 158],
    ),
    ThreadRef::new(
        BRAND,
        "1867",
        "Dark Olive Green",
        &[233, 255, 144],
    ),
    ThreadRef::new(
        BRAND,
        "1870",
        "Medium Orange",
        &[237, 172, 118],
    ),
    ThreadRef::new(
        BRAND,
        "1874",
        "Medium Blue/Grey",
        &[147, 183, 219],
    ),
    ThreadRef::new(
        BRAND,
        "1878",
        "Spanish Red",
        &[205, 48, 43],
    ),
    ThreadRef::new(
        BRAND,
        "1879",
        "Jungle Green",
        &[0, 123, 103],
    ),
    ThreadRef::new(
        BRAND,
        "1880",
        "Dark Purple",
        &[123, 63, 135],
    ),
    ThreadRef::new(
        BRAND,
        "1882",
        "Impatiens Pink",
        &[255, 201, 177],
    ),
    ThreadRef::new(
        BRAND,
        "1883",
        "Gold/Yellow",
        &[255, 247, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1884",
        "Mahogany Rose",
        &[199, 170, 138],
    ),
    ThreadRef::new(
        BRAND,
        "1886",
        "Pale Grey",
        &[198, 192, 192],
    ),
    ThreadRef::new(
        BRAND,
        "1888",
        "Medium Green/Blue",
        &[0, 168, 171],
    ),
    ThreadRef::new(
        BRAND,
        "1890",
        "Light Green/Blue",
        &[0, 125, 129],
    ),
    ThreadRef::new(
        BRAND,
        "1892",
        "Pale Turquoise",
        &[125, 199, 202],
    ),
    ThreadRef::new(
        BRAND,
        "1893",
        "Medium Turquoise",
        &[55, 179, 205],
    ),
    ThreadRef::new(
        BRAND,
        "1895",
        "Dark Blue/Green",
        &[0, 154, 176],
    ),
    ThreadRef::new(
        BRAND,
        "1896",
        "Blue/Green",
        &[0, 117, 143],
    ),
    ThreadRef::new(
        BRAND,
        "1900",
        "Hemlock",
        &[157, 188, 156],
    ),
    ThreadRef::new(
        BRAND,
        "1901",
        "Green/Yellow",
        &[99, 171, 43],
    ),
    ThreadRef::new(
        BRAND,
        "1903",
        "Dark Jade Green",
        &[56, 90, 76],
    ),
    ThreadRef::new(
        BRAND,
        "1907",
        "Bashful Pink",
        &[255, 64, 111],
    ),
    ThreadRef::new(
        BRAND,
        "1908",
        "Bashful Pink",
        &[255, 67, 107],
    ),
    ThreadRef::new(
        BRAND,
        "1909",
        "Fluorescent Dahlia",
        &[255, 107, 149],
    ),
    ThreadRef::new(
        BRAND,
        "1910",
        "Dark Pink",
        &[213, 40, 96],
    ),
    ThreadRef::new(
        BRAND,
        "1911",
        "Tulip",
        &[204, 174, 212],
    ),
    ThreadRef::new(
        BRAND,
        "1915",
        "Orchid Pink",
        &[252, 185, 194],
    ),
    ThreadRef::new(
        BRAND,
        "1917",
        "Chateau Rose",
        &[209, 117, 140],
    ),
    ThreadRef::new(
        BRAND,
        "1918",
        "Pale Grey",
        &[143, 144, 148],
    ),
    ThreadRef::new(
        BRAND,
        "1919",
        "Rose Wine",
        &[167, 88, 107],
    ),
    ThreadRef::new(
        BRAND,
        "1921",
        "Pale Pink",
        &[246, 152, 188],
    ),
    ThreadRef::new(
        BRAND,
        "1922",
        "Dark Purple",
        &[80, 52, 126],
    ),
    ThreadRef::new(
        BRAND,
        "1924",
        "Medium Yellow",
        &[251, 217, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1925",
        "Yellow",
        &[255, 217, 56],
    ),
    ThreadRef::new(
        BRAND,
        "1928",
        "Pale Brown",
        &[140, 114, 97],
    ),
    ThreadRef::new(
        BRAND,
        "1931",
        "Black",
        &[68, 59, 60],
    ),
    ThreadRef::new(
        BRAND,
        "1932",
        "Pale Turquoise",
        &[138, 200, 221],
    ),
    ThreadRef::new(
        BRAND,
        "1934",
        "Dark Blue",
        &[0, 93, 154],
    ),
    ThreadRef::new(
        BRAND,
        "1935",
        "Fluorescent Yellow",
        &[231, 255, 43],
    ),
    ThreadRef::new(
        BRAND,
        "1937",
        "Bright Orange",
        &[255, 166, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1938",
        "Oxford Tan",
        &[186, 169, 151],
    ),
    ThreadRef::new(
        BRAND,
        "1943",
        "Medium Purple",
        &[95, 116, 179],
    ),
    ThreadRef::new(
        BRAND,
        "1944",
        "Dark Navy",
        &[47, 60, 76],
    ),
    ThreadRef::new(
        BRAND,
        "1945",
        "Medium Brown",
        &[114, 77, 68],
    ),
    ThreadRef::new(
        BRAND,
        "1946",
        "Fluo Orange",
        &[255, 101, 31],
    ),
    ThreadRef::new(
        BRAND,
        "1947",
        "Fluo Pink",
        &[254, 67, 76],
    ),
    ThreadRef::new(
        BRAND,
        "1948",
        "Rose",
        &[254, 146, 172],
    ),
    ThreadRef::new(
        BRAND,
        "1950",
        "Fluorescent Green",
        &[124, 213, 59],
    ),
    ThreadRef::new(
        BRAND,
        "1951",
        "Gold/Orange",
        &[247, 154, 35],
    ),
    ThreadRef::new(
        BRAND,
        "1952",
        "Persimmon",
        &[253, 127, 104],
    ),
    ThreadRef::new(
        BRAND,
        "1953",
        "Pale Blue",
        &[164, 200, 226],
    ),
    ThreadRef::new(
        BRAND,
        "1954",
        "Fluo Pink",
        &[255, 72, 96],
    ),
    ThreadRef::new(
        BRAND,
        "1955",
        "Honeydew",
        &[254, 150, 27],
    ),
    ThreadRef::new(
        BRAND,
        "1956",
        "Medium Olive Green",
        &[142, 130, 88],
    ),
    ThreadRef::new(
        BRAND,
        "1957",
        "Dark Brown",
        &[77, 65, 49],
    ),
    ThreadRef::new(
        BRAND,
        "1959",
        "Shimmering Gold",
        &[182, 149, 20],
    ),
    ThreadRef::new(
        BRAND,
        "1965",
        "Fluo Orange",
        &[249, 102, 35],
    ),
    ThreadRef::new(
        BRAND,
        "1966",
        "Colonial Blue",
        &[42, 53, 81],
    ),
    ThreadRef::new(
        BRAND,
        "1967",
        "Dragon Fly",
        &[35, 56, 85],
    ),
    ThreadRef::new(
        BRAND,
        "1970",
        "Lime",
        &[54, 90, 62],
    ),
    ThreadRef::new(
        BRAND,
        "1971",
        "Golden Lights",
        &[251, 184, 43],
    ),
    ThreadRef::new(
        BRAND,
        "1972",
        "Fluo Orange",
        &[255, 194, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1973",
        "Unaka Sand",
        &[152, 95, 50],
    ),
    ThreadRef::new(
        BRAND,
        "1974",
        "Dark Rust Red",
        &[134, 63, 61],
    ),
    ThreadRef::new(
        BRAND,
        "1977",
        "Blue/Green",
        &[0, 131, 180],
    ),
    ThreadRef::new(
        BRAND,
        "1978",
        "Golden Poppy",
        &[254, 139, 74],
    ),
    ThreadRef::new(
        BRAND,
        "1980",
        "Buttercup",
        &[253, 198, 37],
    ),
    ThreadRef::new(
        BRAND,
        "1981",
        "Pale Maroon",
        &[126, 42, 55],
    ),
    ThreadRef::new(
        BRAND,
        "1983",
        "Dark Maroon",
        &[83, 56, 71],
    ),
    ThreadRef::new(
        BRAND,
        "1984",
        "Dark Pink",
        &[182, 29, 94],
    ),
    ThreadRef::new(
        BRAND,
        "1985",
        "Green",
        &[1, 95, 81],
    ),
    ThreadRef::new(
        BRAND,
        "1986",
        "Dark Pink",
        &[178, 38, 75],
    ),
    ThreadRef::new(
        BRAND,
        "1987",
        "Honeysuckle",
        &[229, 63, 37],
    ),
    ThreadRef::new(
        BRAND,
        "1988",
        "Green",
        &[0, 143, 68],
    ),
    ThreadRef::new(
        BRAND,
        "1989",
        "Medium Green",
        &[20, 158, 106],
    ),
    ThreadRef::new(
        BRAND,
        "1990",
        "Dark Pink",
        &[220, 95, 153],
    ),
    ThreadRef::new(
        BRAND,
        "1991",
        "Dark Aqua Green",
        &[0, 120, 128],
    ),
    ThreadRef::new(
        BRAND,
        "1992",
        "Dark Blue",
        &[0, 97, 122],
    ),
    ThreadRef::new(
        BRAND,
        "1993",
        "Bullet Red",
        &[213, 52, 93],
    ),
    ThreadRef::new(
        BRAND,
        "1994",
        "Fluo Pink",
        &[240, 81, 119],
    ),
    ThreadRef::new(
        BRAND,
        "1995",
        "Gold/Yellow",
        &[255, 255, 0],
    ),
    ThreadRef::new(
        BRAND,
        "1996",
        "Dark Green",
        &[40, 66, 65],
    ),
    ThreadRef::new(
        BRAND,
        "1997",
        "Black",
        &[61, 50, 64],
    ),
    ThreadRef::new(
        BRAND,
        "1998",
        "Pink/Brown",
        &[132, 100, 105],
    ),
    ThreadRef::new(
        BRAND,
        "1999",
        "Coffee Bean",
        &[95, 46, 49],
    ),
];