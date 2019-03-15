use bitflags::bitflags;
use option_set::option_set;


bitflags! {
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct TestGood: u8 {
        const One = 1;
        const Two = 1 << 1;
        const Three  = 1 << 2;
    }
}

option_set! {
    pub struct TestBad: UpperCamel + u8 {
        const One = 1;
        const Two = 1 << 1;
        const Three  = 1 << 2;
    }
}


fn main() {
    let flag_good = TestGood::One | TestGood::Two;

    let json_ser_good = serde_json::ser::to_string(&flag_good).unwrap();
    let ron_ser_good = ron::ser::to_string(&flag_good).unwrap();

    let json_der_good: TestGood = serde_json::de::from_str(json_ser_good.as_str()).unwrap();
    let ron_ser_good: TestGood = ron::de::from_str(ron_ser_good.as_str()).unwrap();


    let flag_bad = TestBad::One | TestBad::Two;

    let json_ser_bad = serde_json::ser::to_string(&flag_bad).unwrap();
    let ron_ser_bad = ron::ser::to_string(&flag_bad).unwrap();

    let json_der_bad: TestBad = serde_json::de::from_str(json_ser_bad.as_str()).unwrap();
    let ron_ser_bad: TestBad = ron::de::from_str(ron_ser_bad.as_str()).unwrap();
}
