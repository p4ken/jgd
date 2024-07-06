//! 国土地理院によるオリジナルの PatchJGD と比較するテスト。
#![cfg(feature = "patchjgd")]

use approx::assert_abs_diff_eq;
use jgd::{Jgd2000, LatLon};

/// 許容誤差: ±1mm
const MM_IN_DEGREES: f64 = 0.000000009;

#[test]
fn sendai() {
    let (lat, lon) = Jgd2000::new(LatLon(38.26, 140.87)).to_jgd2011().into();
    assert_abs_diff_eq!(lat, 38.259991997, epsilon = MM_IN_DEGREES);
    assert_abs_diff_eq!(lon, 140.870036378, epsilon = MM_IN_DEGREES);
}

#[test]
fn iwaki_1() {
    let (lat, lon) = Jgd2000::new(LatLon(37.090536, 140.840350))
        .to_jgd2011()
        .into();
    assert_abs_diff_eq!(lat, 37.090532997, epsilon = MM_IN_DEGREES);
    assert_abs_diff_eq!(lon, 140.840375142, epsilon = MM_IN_DEGREES);
}

/// パラメータグリッドがない地域
#[test]
fn iwaki_2() {
    let (lat, lon) = Jgd2000::new(LatLon(37.093698, 140.829111))
        .to_jgd2011()
        .into();
    assert_abs_diff_eq!(lat, 37.093698, epsilon = MM_IN_DEGREES);
    assert_abs_diff_eq!(lon, 140.829111, epsilon = MM_IN_DEGREES);
}
