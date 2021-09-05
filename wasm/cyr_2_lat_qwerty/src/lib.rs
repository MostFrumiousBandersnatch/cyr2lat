#![feature(test)]
#[macro_use]

extern crate lazy_static;
extern crate test;

use wasm_bindgen::prelude::*;

mod cyr_to_lat_qwerty {
    use std::collections::HashMap;
    use std::iter::FromIterator;

    struct QWERTYKey {
        lat: char,
        cyr: char,
    }

    type Dict = HashMap<char, char>;

    static BOARD: [QWERTYKey; 66] = [
        QWERTYKey {
            lat: 'q', cyr: 'й'
        },
        QWERTYKey {
            lat: 'w', cyr: 'ц'
        },
        QWERTYKey {
            lat: 'e', cyr: 'у'
        },
        QWERTYKey {
            lat: 'r', cyr: 'к'
        },
        QWERTYKey {
            lat: 't', cyr: 'е'
        },
        QWERTYKey {
            lat: 'y', cyr: 'н'
        },
        QWERTYKey {
            lat: 'u', cyr: 'г'
        },
        QWERTYKey {
            lat: 'i', cyr: 'ш'
        },
        QWERTYKey {
            lat: 'o', cyr: 'щ'
        },
        QWERTYKey {
            lat: 'p', cyr: 'з'
        },
        QWERTYKey {
            lat: '[', cyr: 'х'
        },
        QWERTYKey {
            lat: ']', cyr: 'ъ'
        },
        QWERTYKey {
            lat: 'a', cyr: 'ф'
        },
        QWERTYKey {
            lat: 's', cyr: 'ы'
        },
        QWERTYKey {
            lat: 'd', cyr: 'в'
        },
        QWERTYKey {
            lat: 'f', cyr: 'а'
        },
        QWERTYKey {
            lat: 'g', cyr: 'п'
        },
        QWERTYKey {
            lat: 'h', cyr: 'р'
        },
        QWERTYKey {
            lat: 'j', cyr: 'о'
        },
        QWERTYKey {
            lat: 'k', cyr: 'л'
        },
        QWERTYKey {
            lat: 'l', cyr: 'д'
        },
        QWERTYKey {
            lat: ';', cyr: 'ж'
        },
        QWERTYKey {
            lat: '\'',
            cyr: 'э',
        },
        QWERTYKey {
            lat: 'z', cyr: 'я'
        },
        QWERTYKey {
            lat: 'x', cyr: 'ч'
        },
        QWERTYKey {
            lat: 'c', cyr: 'с'
        },
        QWERTYKey {
            lat: 'v', cyr: 'м'
        },
        QWERTYKey {
            lat: 'b', cyr: 'и'
        },
        QWERTYKey {
            lat: 'n', cyr: 'т'
        },
        QWERTYKey {
            lat: 'm', cyr: 'ь'
        },
        QWERTYKey {
            lat: ',', cyr: 'б'
        },
        QWERTYKey {
            lat: '.', cyr: 'ю'
        },
        QWERTYKey {
            lat: '`', cyr: 'ё'
        },
        QWERTYKey {
            lat: 'Q', cyr: 'Й'
        },
        QWERTYKey {
            lat: 'W', cyr: 'Ц'
        },
        QWERTYKey {
            lat: 'E', cyr: 'У'
        },
        QWERTYKey {
            lat: 'R', cyr: 'К'
        },
        QWERTYKey {
            lat: 'T', cyr: 'Е'
        },
        QWERTYKey {
            lat: 'Y', cyr: 'Н'
        },
        QWERTYKey {
            lat: 'U', cyr: 'Г'
        },
        QWERTYKey {
            lat: 'I', cyr: 'Ш'
        },
        QWERTYKey {
            lat: 'O', cyr: 'Щ'
        },
        QWERTYKey {
            lat: 'P', cyr: 'З'
        },
        QWERTYKey {
            lat: '{', cyr: 'Х'
        },
        QWERTYKey {
            lat: '}', cyr: 'Ъ'
        },
        QWERTYKey {
            lat: 'A', cyr: 'Ф'
        },
        QWERTYKey {
            lat: 'S', cyr: 'Ы'
        },
        QWERTYKey {
            lat: 'D', cyr: 'В'
        },
        QWERTYKey {
            lat: 'F', cyr: 'А'
        },
        QWERTYKey {
            lat: 'G', cyr: 'П'
        },
        QWERTYKey {
            lat: 'H', cyr: 'Р'
        },
        QWERTYKey {
            lat: 'J', cyr: 'О'
        },
        QWERTYKey {
            lat: 'K', cyr: 'Л'
        },
        QWERTYKey {
            lat: 'L', cyr: 'Д'
        },
        QWERTYKey {
            lat: ':', cyr: 'Ж'
        },
        QWERTYKey {
            lat: '"', cyr: 'Э'
        },
        QWERTYKey {
            lat: 'Z', cyr: 'Я'
        },
        QWERTYKey {
            lat: 'X', cyr: 'Ч'
        },
        QWERTYKey {
            lat: 'C', cyr: 'С'
        },
        QWERTYKey {
            lat: 'V', cyr: 'М'
        },
        QWERTYKey {
            lat: 'B', cyr: 'И'
        },
        QWERTYKey {
            lat: 'N', cyr: 'Т'
        },
        QWERTYKey {
            lat: 'M', cyr: 'Ь'
        },
        QWERTYKey {
            lat: '<', cyr: 'Б'
        },
        QWERTYKey {
            lat: '>', cyr: 'Ю'
        },
        QWERTYKey {
            lat: '~', cyr: 'Ё'
        },
    ];

    lazy_static! {
        pub static ref LAT_2_CYR_MAP: Dict = IntoIterator::into_iter(&BOARD)
            .map(|key: &QWERTYKey| (key.lat, key.cyr))
            .collect();
        pub static ref CYR_2_LAT_MAP: Dict = IntoIterator::into_iter(&BOARD)
            .map(|key: &QWERTYKey| (key.cyr, key.lat))
            .collect();
    }

    pub fn convert(dict: &Dict, source: &str) -> String {
        String::from_iter(source.chars().map(|ch: char| match dict.get(&ch) {
            Some(x) => *x,
            None => ch,
        }))
    }
}

#[wasm_bindgen(js_name = convertLatToCyr)]
pub fn convert_lat_to_cyr(source: &str) -> String {
    cyr_to_lat_qwerty::convert(&cyr_to_lat_qwerty::LAT_2_CYR_MAP, source)
}

#[wasm_bindgen(js_name = convertCyrToLat)]
pub fn convert_cyr_to_lat(source: &str) -> String {
    cyr_to_lat_qwerty::convert(&cyr_to_lat_qwerty::CYR_2_LAT_MAP, source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    
    #[test]
    fn lat_to_cyr_basic_conversion() {
        assert_eq!(convert_lat_to_cyr("qwerty"), "йцукен");
        assert_eq!(convert_lat_to_cyr("asdfgh"), "фывапр");
    }

    #[test]
    fn lat_to_cyr_distinguish_cases() {
        assert_eq!(convert_lat_to_cyr("qWeRtY"), "йЦуКеН");
        assert_eq!(convert_lat_to_cyr("AsDfGh"), "ФыВаПр");
    }

    #[test]
    fn cyr_to_lat_basic_conversion() {
        assert_eq!(convert_cyr_to_lat("йцукен"), "qwerty");
        assert_eq!(convert_cyr_to_lat("фывапр"), "asdfgh");
    }

    #[test]
    fn cyr_to_lat_distinguish_cases() {
        assert_eq!(convert_cyr_to_lat("йЦуКеН"), "qWeRtY");
        assert_eq!(convert_cyr_to_lat("ФыВаПр"), "AsDfGh");
    }

    #[bench]
    fn bench_lat_to_cyr(b: &mut Bencher) {
        b.iter(|| convert_cyr_to_lat("qWeRtY"))
    }

    #[bench]
    fn bench_cyr_to_lat(b: &mut Bencher) {
        b.iter(|| convert_cyr_to_lat("ФыВаПр"))
    }
}
