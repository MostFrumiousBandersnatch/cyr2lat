#!/usr/bin/env node
import * as lib from './dist/index.js';

const REPEAT = 1000;

console.log('start latToCyr');
console.time('latToCyr');

for (let i = 0; i < REPEAT; i++) {
    lib.convertLatToCyr('qwerty');
}
console.timeEnd('latToCyr');


console.log('start cyrToLat');
console.time('cyrToLat');

for (let i = 0; i < REPEAT; i++) {
    lib.convertCyrToLat('йцукен');
}

console.timeEnd('cyrToLat');
