type Dictionary = Map<number, number>;

const MAPPING: [string, string][] = [
  ['`', 'ё'],
  ['@', '"'],
  ['#', '№'],
  ['$', ';'],
  ['%', '%'],
  ['^', ':'],
  ['&', '?'],

  ['q', 'й'],
  ['w', 'ц'],
  ['e', 'у'],
  ['r', 'к'],
  ['t', 'е'],
  ['y', 'н'],
  ['u', 'г'],
  ['i', 'ш'],
  ['o', 'щ'],
  ['p', 'з'],
  ['[', 'х'],
  [']', 'ъ'],

  ['Q', 'Й'],
  ['W', 'Ц'],
  ['E', 'У'],
  ['R', 'К'],
  ['T', 'Е'],
  ['Y', 'Н'],
  ['U', 'Г'],
  ['I', 'Ш'],
  ['O', 'Щ'],
  ['P', 'З'],
  ['{', 'Х'],
  ['}', 'Ъ'],

  ['a', 'ф'],
  ['s', 'ы'],
  ['d', 'в'],
  ['f', 'а'],
  ['g', 'п'],
  ['h', 'р'],
  ['j', 'о'],
  ['k', 'л'],
  ['l', 'д'],
  [';', 'ж'],
  ["'", 'э'],
  ['\\', '|'],

  ['A', 'Ф'],
  ['S', 'Ы'],
  ['D', 'В'],
  ['F', 'А'],
  ['G', 'П'],
  ['H', 'Р'],
  ['J', 'О'],
  ['K', 'Л'],
  ['L', 'Д'],
  [':', 'Ж'],
  ['"', 'Э'],
  ['|', '/'],

  ['z', 'я'],
  ['x', 'ч'],
  ['c', 'с'],
  ['v', 'м'],
  ['b', 'и'],
  ['n', 'т'],
  ['m', 'ь'],
  [',', 'б'],
  ['.', 'ю'],
  ['/', '.'],

  ['Z', 'Я'],
  ['X', 'Ч'],
  ['C', 'С'],
  ['V', 'М'],
  ['B', 'И'],
  ['N', 'Т'],
  ['M', 'Ь'],
  ['<', 'Б'],
  ['>', 'Ю'],
  ['?', ','],
];

const toCharCodes = ([s1, s2]: [string, string]): [number, number] => [
  s1.charCodeAt(0), s2.charCodeAt(0)
];

const LAT_TO_CYR_MAP: Dictionary = new Map(MAPPING.map(toCharCodes));
const CYR_TO_LAT_MAP: Dictionary = new Map(
  MAPPING.map(([lat, cyr]) => [cyr, lat]).map(toCharCodes)
);

console.log(MAPPING.map(toCharCodes));

const convert = (dMap: Dictionary) => (foo: string): string => {
  const len = foo.length;
  let res = '';

  for (let i = 0; i < len; i += 1) {
    const code = foo.charCodeAt(i);
    res += String.fromCharCode(dMap.get(code) || code);
  }

  return res;
};
export const convertLatToCyr = convert(LAT_TO_CYR_MAP);
export const convertCyrToLat = convert(CYR_TO_LAT_MAP);
