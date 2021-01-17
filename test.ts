import * as assert from 'assert';
import { describe } from 'mocha';

import { convertLatToCyr, convertCyrToLat } from './index';

describe('latin to cyrillic conversion', function () {
  it('should work', function () {
    assert.equal(convertLatToCyr('qwerty'), 'йцукен');
    assert.equal(convertLatToCyr('asdfgh'), 'фывапр');
  });
});

describe('cyrillic to latin conversion', function () {
  it('should work', function () {
    assert.equal(convertCyrToLat('йцукен'), 'qwerty');
  });
});
