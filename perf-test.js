#!/usr/bin/env node

import * as js_lib from './dist/index.js';
import wasm_lib from './wasm/cyr_2_lat_qwerty/pkg/node/cyr_2_lat_qwerty.js';

const latStrings = [
  'qwerty',
  'asdfgh',
  'zxcvbn',
  'uiop[]',
  "hjkl;'",
  'best regards',
];
const cyrStrings = [
  'йцукен',
  'фывапр',
  'ячсмит',
  'гшщзхъ',
  'ролджэ',
  'иуые купфквы',
];

const pickRandorm = l => l[Math.floor(Math.random() * l.length)];

const REPEAT = 1000;

const TEST_LAT_DATA = [...Array(REPEAT)].map(() => pickRandorm(latStrings));
const TEST_CYR_DATA = [...Array(REPEAT)].map(() => pickRandorm(cyrStrings));

//console.log(TEST_LAT_DATA);
//console.log(TEST_CYR_DATA);

for (let [lib, title] of [
  [js_lib, 'js'],
  [wasm_lib, 'wasm'],
]) {
  let result;

  console.log(`\nstart ${title}->latToCyr (${REPEAT} iter)`);
  console.time('latToCyr');
  result = TEST_LAT_DATA.map(lib.convertLatToCyr);
  console.timeEnd('latToCyr');
  //console.log(result);

  console.log(`\nstart ${title}->cyrToLat (${REPEAT} iter)`);
  console.time('cyrToLat');
  result = TEST_CYR_DATA.map(lib.convertCyrToLat);
  console.timeEnd('cyrToLat');
  //console.log(result);
}

//console.log(wasm_lib.__wasm.memory.buffer);
