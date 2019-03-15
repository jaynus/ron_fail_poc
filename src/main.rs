use bitflags::bitflags;
use option_set::option_set;

option_set! {
    pub struct Test: UpperCamel + u8 {
        const One = 1;
        const Two = 1 << 1;
        const Three  = 1 << 2;
    }
}



fn main() {
    let flag = Test::One | Test::Two;

    let json_ser = serde_json::ser::to_string(&flag).unwrap();
    let ron_ser = ron::ser::to_string(&flag).unwrap();

    let json_der: Test = serde_json::de::from_str(json_ser.as_str()).unwrap();
    let ron_ser: Test = ron::de::from_str(ron_ser.as_str()).unwrap();
}
