#![allow(
    clippy::incompatible_msrv, // https://github.com/rust-lang/rust-clippy/issues/12257
)]

use std::f32;
#[cfg(not(miri))]
use std::f64;

macro_rules! tests {
    (
        $(
            $name:ident ( $float:expr ) = $expected:literal;
        )*
    ) => {
        $(
            #[test]
            fn $name() {
                assert_eq!(precise::to_string($float), $expected);
            }
        )*
    };
}

tests! {
    _1_f32(1.0f32) = "1.0";
    _90_f32(90.0f32) = "90.0";

    _0_1_f32(0.1f32) = "0.100000001490116119384765625";
    _0_123_f32(0.123f32) = "0.123000003397464752197265625";
    _1_23_f32(1.23f32) = "1.230000019073486328125";

    min_positive_f32(f32::MIN_POSITIVE) = "0.000000000000000000000000000000000000011754943508222875079687365372222456778186655567720875215087517062784172594547271728515625";
    min_f32(f32::MIN) = "-340282346638528859811704183484516925440.0";
    max_f32(f32::MAX) = "340282346638528859811704183484516925440.0";

    pos_nan_f32(f32::NAN.copysign(1.0)) = "NaN";
    neg_nan_f32(f32::NAN.copysign(-1.0)) = "NaN";
    pos_inf_f32(f32::INFINITY) = "inf";
    neg_inf_f32(f32::NEG_INFINITY) = "-inf";
}

#[cfg(not(miri))]
tests! {
    _1_f64(1.0f64) = "1.0";
    _90_f64(90.0f64) = "90.0";

    _0_1_f64(0.1f64) = "0.1000000000000000055511151231257827021181583404541015625";
    _0_123_f64(0.123f64) = "0.1229999999999999982236431605997495353221893310546875";
    _1_23_f64(1.23f64) = "1.229999999999999982236431605997495353221893310546875";

    min_positive_f64(f64::MIN_POSITIVE) = "0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002225073858507201383090232717332404064219215980462331830553327416887204434813918195854283159012511020564067339731035811005152434161553460108856012385377718821130777993532002330479610147442583636071921565046942503734208375250806650616658158948720491179968591639648500635908770118304874799780887753749949451580451605050915399856582470818645113537935804992115981085766051992433352114352390148795699609591288891602992641511063466313393663477586513029371762047325631781485664350872122828637642044846811407613911477062801689853244110024161447421618567166150540154285084716752901903161322778896729707373123334086988983175067838846926092773977972858659654941091369095406136467568702398678315290680984617210924625396728515625";
    min_f64(f64::MIN) = "-179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368.0";
    max_f64(f64::MAX) = "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368.0";

    pos_nan_f64(f64::NAN.copysign(1.0)) = "NaN";
    neg_nan_f64(f64::NAN.copysign(-1.0)) = "NaN";
    pos_inf_f64(f64::INFINITY) = "inf";
    neg_inf_f64(f64::NEG_INFINITY) = "-inf";
}
