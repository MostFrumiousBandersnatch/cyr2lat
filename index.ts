type Dictionary = Map<string, string>;

const MAPPING: [string, string][] = [
  ["`", "ё"],

  ["q", "й"],
  ["w", "ц"],
  ["e", "у"],
  ["r", "к"],
  ["t", "е"],
  ["y", "н"],
  ["u", "г"],
  ["i", "ш"],
  ["o", "щ"],
  ["p", "з"],
  ["[", "х"],
  ["]", "ъ"],
  
  ["a", "ф"],
  ["s", "ы"],
  ["d", "в"],
  ["f", "а"],
  ["g", "п"],
  ["h", "р"],
  ["j", "о"],
  ["k", "л"],
  ["l", "д"],
  [";", "ж"],
  ["'", "э"],
  ["\\", "|"],

  ["z", "я"],
  ["x", "ч"],
  ["c", "с"],
  ["v", "м"],
  ["b", "и"],
  ["n", "т"],
  ["m", "ь"],
  [",", "б"],
  [".", "ю"],
  ["/", "."],
];

const LAT_TO_CYR_MAP: Dictionary = new Map(MAPPING);
const CYR_TO_LAT_MAP: Dictionary = new Map(
  MAPPING.map(([lat, cyr]) => [cyr, lat])
);

const convert = (dMap: Dictionary) => (foo: string): string =>
  foo.replace(/\S{1}/g, (ch) => dMap.get(ch) || ch);

export const convertLatToCyr = convert(LAT_TO_CYR_MAP);
export const convertCyrToLat = convert(CYR_TO_LAT_MAP);
