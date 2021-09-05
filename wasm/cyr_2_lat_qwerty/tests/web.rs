#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate cyr_2_lat_qwerty;

mod tests {
    use wasm_bindgen_test::*;
    use cyr_2_lat_qwerty::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn lat_to_cyr_basic_conversion() {
        assert_eq!(convert_lat_to_cyr("qwerty"), "йцукен");
        assert_eq!(convert_lat_to_cyr("asdfgh"), "фывапр");
    }

    #[wasm_bindgen_test]
    fn lat_to_cyr_distinguish_cases() {
        assert_eq!(convert_lat_to_cyr("qWeRtY"), "йЦуКеН");
        assert_eq!(convert_lat_to_cyr("AsDfGh"), "ФыВаПр");
    }

    #[wasm_bindgen_test]
    fn cyr_to_lat_basic_conversion() {
        assert_eq!(convert_cyr_to_lat("йцукен"), "qwerty");
        assert_eq!(convert_cyr_to_lat("фывапр"), "asdfgh");
    }

    #[wasm_bindgen_test]
    fn cyr_to_lat_distinguish_cases() {
        assert_eq!(convert_cyr_to_lat("йЦуКеН"), "qWeRtY");
        assert_eq!(convert_cyr_to_lat("ФыВаПр"), "AsDfGh");
    }
}
