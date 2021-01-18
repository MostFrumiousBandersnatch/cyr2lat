import * as assert from 'assert';
import { describe } from 'mocha';

import { convertLatToCyr, convertCyrToLat } from './index';

describe('latin to cyrillic conversion', function () {
  it('should just work', function () {
    assert.equal(convertLatToCyr('qwerty'), 'йцукен');
    assert.equal(convertLatToCyr('asdfgh'), 'фывапр');
  });

  it('should distinguish cases', function () {
    assert.equal(convertLatToCyr('qWeRtY'), 'йЦуКеН');
    assert.equal(convertLatToCyr('AsDfGh'), 'ФыВаПр');
  });
});

describe('cyrillic to latin conversion', function () {
  it('should work', function () {
    assert.equal(convertCyrToLat('йцукен'), 'qwerty');
    assert.equal(convertCyrToLat('фывапр'), 'asdfgh');
  });

  it('should distinguish cases', function () {
    assert.equal(convertCyrToLat('йЦуКеН'), 'qWeRtY');
    assert.equal(convertCyrToLat('ФыВаПр'), 'AsDfGh');
  });
});
