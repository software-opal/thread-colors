

export function convertBrandThreads({ brand, threads }) {
  return threads.map(({ code, color, name }) => ({
    code,
    color,
    name,
    brand,
    id: `${brand}#${code}#${name}`,
  }));
}

export function convertRollupThreads(brandMap) {
  return Object.entries(brandMap).map(
    ([brand, threads]) => convertBrandThreads({ brand, threads }),
  ).flat(1);
}
