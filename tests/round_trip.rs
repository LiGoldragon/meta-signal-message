use meta_signal_message::schema::lib::*;
use signal_frame::{ExchangeIdentifier, ExchangeLane, LaneSequence, Reply, SessionEpoch, SubReply};
use signal_message::schema::lib::{
    z2VL2C, z2VPa3, z2VPn2, z2VQY5, z2VRJp, z2VRPH, z2VUUz, z2VUqb, z2VYZK, z2VZv9, z2VaVk,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(2),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

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

fn outputs() -> [z2VYLc; 3] {
    [
        z2VYLc::z2VT5g(z2VZEw::new(z2VLUj::new(z2VYdt::new(7)))),
        z2VYLc::z2VcWw(z2VTC7::new(z2VW54::new(z2VWBb::z2Vay1))),
        z2VYLc::z2Vc4F(z2VR6z {
            field_0: z2VUdf::new(z2VY5P::z2Vdbu),
            field_1: z2VKyZ::new(z2VM7X::z2VX9E),
        }),
    ]
}

#[test]
fn owner_request_imports_and_round_trips_the_producer_type() {
    let input = z2Vc2e::z2VWNS(configuration());
    assert_eq!(input.route(), InputRoute::Configure);
    let bytes = input
        .clone()
        .into_frame(exchange())
        .encode_length_prefixed()
        .expect("encode request");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode request");
    let FrameBody::Request { request, .. } = decoded.into_body() else {
        panic!("expected request")
    };
    assert_eq!(request.payloads().head(), &input);
}

#[test]
fn every_owner_reply_round_trips() {
    for output in outputs() {
        let expected = output.clone();
        let bytes = output
            .into_reply_frame(exchange())
            .encode_length_prefixed()
            .expect("encode reply");
        let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");
        let FrameBody::Reply { reply, .. } = decoded.into_body() else {
            panic!("expected reply")
        };
        let Reply::Accepted { per_operation, .. } = reply else {
            panic!("expected accepted reply")
        };
        let SubReply::Ok(actual) = per_operation.into_head() else {
            panic!("expected reply payload")
        };
        assert_eq!(actual, expected);
    }
}

#[test]
fn route_order_is_explicit() {
    assert_eq!(InputRoute::Configure as u8, 0);
    assert_eq!(OutputRoute::ConfigurationApplied as u8, 0);
    assert_eq!(OutputRoute::OperationUnimplemented as u8, 2);
}
