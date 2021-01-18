#!/usr/bin/env node
import * as lib from './dist/index.js';

const latStrings = ['qwerty', 'asdfgh', 'zxcvbn', 'uiop[]', 'hjkl;\''];
const cyrStrings = ['йцукен', 'фывапр', 'ячсмит', 'гшщзхъ', 'ролджэ'];

const pickRandorm = l => l[Math.floor(Math.random() * l.length)];


const REPEAT = 1000;

console.log('start latToCyr');
console.time('latToCyr');

for (let i = 0; i < REPEAT; i++) {
    lib.convertLatToCyr(pickRandorm(latStrings));
}
console.timeEnd('latToCyr');


console.log('start cyrToLat');
console.time('cyrToLat');

for (let i = 0; i < REPEAT; i++) {
    lib.convertCyrToLat(pickRandorm(cyrStrings));
}

console.timeEnd('cyrToLat');
