use bitflags::*;
use option_set::option_set;

#[macro_use]
mod bitflags_serial;

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

bitflags_serial! {
    pub struct TestBadTwo: u8 {
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

    // option_set
    let flag_bad = TestBad::One | TestBad::Two;

    let json_ser_bad = serde_json::ser::to_string(&flag_bad).unwrap();
    let ron_ser_bad = ron::ser::to_string(&flag_bad).unwrap();

    let json_der_bad = serde_json::de::from_str::<TestBad>(json_ser_bad.as_str()).unwrap();
    let ron_der_bad =  ron::de::from_str::<TestBad>(ron_ser_bad.as_str());
    match ron_der_bad {
        Ok(_) => { println!("* RON option_set succeeded!"); },
        Err(_) => { println!("! RON option_set FAILED!"); },
    }

    // bitflags_serial
    let flag_bad_two = TestBadTwo::One | TestBadTwo::Two;

    let json_ser_bad_two = serde_json::ser::to_string(&flag_bad_two).unwrap();
    let ron_ser_bad_two = ron::ser::to_string(&flag_bad_two).unwrap();

    let json_der_bad_two = serde_json::de::from_str::<TestBadTwo>(json_ser_bad_two.as_str());
    match json_der_bad_two {
        Ok(_) => { println!("* JSON bitflags_serial succeeded!"); },
        Err(_) => { println!("! JSON bitflags_serial FAILED!"); },
    }
    let ron_der_bad_two = ron::de::from_str::<TestBadTwo>(ron_ser_bad_two.as_str());
    match ron_der_bad_two {
        Ok(_) => { println!("* RON bitflags_serial succeeded!"); },
        Err(_) => { println!("! RON bitflags_serial FAILED!"); },
    }
}
