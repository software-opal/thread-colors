const parse = require('parse-color');
const { closest } = require('color-diff');

export function parseColor(color) {
  if (typeof color === 'string' || color instanceof String) {
    const { rgb: [R, G, B] } = parse(color);
    return { R, G, B };
  } else if (Object.prototype.hasOwnProperty.call(color, 'R') && Object.prototype.hasOwnProperty.call(color, 'G') && Object.prototype.hasOwnProperty.call(color, 'B')) {
    const { R, G, B } = color;
    return { R, G, B };
  } else if (Object.prototype.hasOwnProperty.call(color, 'length') && color.length === 3) {
    const [R, G, B] = color;
    return { R, G, B };
  } console.log('Cannot parse color: ', color);
  throw 'Error parsing color';
}

export const ColorPalette = Object.fromEntries([
  ['Red', '#ff0000'],
  ['Orange', '#ff7f00'],
  ['Yellow', '#ffff00'],
  ['Chartreuse', '#7fff00'],
  ['Green', '#00ff00'],
  ['Spring Green', '#00ff7f'],
  ['Cyan', '#00ffff'],
  ['Blueish', '#007fff'],
  ['Blue', '#0000ff'],
  ['Purple', '#7f00ff'],
  ['Purpleish', '#ff00ff'],
  ['Purplerish', '#ff007f'],
  ['Black', '#000000'],
  ['Brown', '#88540B'],
  ['White', '#ffffff'],
].map(([name, color]) => [
  name,
  {
    color,
    parsed: {
      ...parseColor(color),
      name,
      css: color,
    },
  },
]));

export const ColorPaletteParsedColors = Object.values(ColorPalette).map(({ parsed }) => parsed);


export function convertBrandThreads({ brand, threads }) {
  return threads.map(({ code, color, name }) => {
    const parsedColor = parseColor(color);
    return {
      code,
      color,
      parsedColor,
      closestColor: closest(parsedColor, ColorPaletteParsedColors),
      name,
      brand,
      id: `${brand}#${code}#${name}`,
    };
  });
}

export function convertRollupThreads(brandMap) {
  return Object.entries(brandMap).map(
    ([brand, threads]) => convertBrandThreads({ brand, threads }),
  ).flat(1);
}
