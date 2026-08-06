#![cfg(feature = "dotos-text")]

use dotos::{DotosDecode, DotosEncode, DotosSource};
use meta_signal_message::schema::lib::*;
use signal_message::schema::lib::{
    z2VL2C, z2VPa3, z2VPn2, z2VQY5, z2VRJp, z2VRPH, z2VUUz, z2VUqb, z2VYZK, z2VZv9, z2VaVk,
};

const CANONICAL: &str = include_str!("../examples/canonical.dotos");

fn path(value: &str) -> z2VQY5 {
    z2VQY5::new(value.to_owned())
}

fn configuration() -> z2VL2C {
    z2VL2C {
        field_0: z2VUUz::new(path("/run/persona/X/message.sock")),
        field_1: z2VPa3::new(z2VYZK::new(0o660)),
        field_2: z2VRJp::new(path("/run/persona/X/message-supervision.sock")),
        field_3: z2VaVk::new(z2VYZK::new(0o600)),
        field_4: z2VZv9::new(path("/run/persona/X/router.sock")),
        field_5: z2VRPH::new(vec![]),
        field_6: z2VUqb::z2Vd9P(z2VPn2::new(1000)),
    }
}

fn witness<Value>(value: Value)
where
    Value: DotosDecode + DotosEncode + PartialEq + std::fmt::Debug,
{
    let text = value.to_dotos();
    assert!(CANONICAL.contains(&text), "missing canonical line: {text}");
    assert_eq!(
        DotosSource::new(&text).parse::<Value>().expect("decode"),
        value
    );
}

#[test]
fn readable_owner_roles_round_trip() {
    witness(z2Vc2e::z2VWNS(configuration()));
    witness(z2VYLc::z2VT5g(z2VZEw::new(z2VLUj::new(z2VYdt::new(7)))));
    witness(z2VYLc::z2VcWw(z2VTC7::new(z2VW54::new(z2VWBb::z2Vay1))));
    witness(z2VYLc::z2Vc4F(z2VR6z {
        field_0: z2VUdf::new(z2VY5P::z2Vdbu),
        field_1: z2VKyZ::new(z2VM7X::z2VX9E),
    }));
}
