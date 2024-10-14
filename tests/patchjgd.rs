//! 国土地理院によるオリジナルの PatchJGD と比較するテスト。
#![cfg(feature = "patchjgd")]

use jgd::{Jgd2000, LatLon};

mod testing;

#[test]
fn sendai() {
    let ret = Jgd2000::new(LatLon(38.26, 140.87))
        .unwrap()
        .to_jgd2011()
        .degrees();
    let patchjgd = LatLon(38.259991997, 140.870036378);
    testing::assert_distance(ret, patchjgd);
}

#[test]
fn iwaki_1() {
    let ret = Jgd2000::new(LatLon(37.090536, 140.840350))
        .unwrap()
        .to_jgd2011()
        .degrees();
    let patchjgd = LatLon(37.090532997, 140.840375142);
    testing::assert_distance(ret, patchjgd);
}

/// パラメータグリッドがない地域
#[test]
fn iwaki_2() {
    let ret = Jgd2000::new(LatLon(37.093698, 140.829111))
        .unwrap()
        .to_jgd2011()
        .degrees();
    let patchjgd = LatLon(37.093698, 140.829111);
    testing::assert_distance(ret, patchjgd);
}
