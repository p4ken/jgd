//! 国土地理院によるオリジナルの TKY2JGD と比較するテスト。
#![cfg(feature = "tky2jgd")]

use approx::assert_abs_diff_eq;
use jgd::{LatLon, Tokyo};

/// 許容誤差: ±1mm
const MM_IN_DEGREES: f64 = 0.000000009;

fn assert_tky2jgd(tokyo: LatLon, expected: LatLon) {
    let LatLon(lat0, lon0) = expected;
    let LatLon(lat1, lon1) = Tokyo::new(tokyo).to_jgd2000().degrees();
    assert_abs_diff_eq!(lat0, lat1, epsilon = MM_IN_DEGREES);
    assert_abs_diff_eq!(lon0, lon1, epsilon = MM_IN_DEGREES);
}

#[test]
fn 村松() {
    let tokyo = LatLon::from_dms((36, 27, 39.20500), (140, 35, 06.11100));
    let expected = LatLon::from_dms((36, 27, 50.58487), (140, 34, 54.10080));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 高野() {
    let tokyo = LatLon::from_dms((36, 25, 45.63400), (140, 32, 47.46200));
    let expected = LatLon::from_dms((36, 25, 57.02524), (140, 32, 35.46640));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 東石川() {
    let tokyo = LatLon::from_dms((36, 24, 51.26200), (140, 32, 15.86100));
    let expected = LatLon::from_dms((36, 25, 02.65997), (140, 32, 03.86700));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 長砂() {
    let tokyo = LatLon::from_dms((36, 24, 45.41400), (140, 34, 58.52400));
    let expected = LatLon::from_dms((36, 24, 56.81069), (140, 34, 46.51725));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 防風() {
    let tokyo = LatLon::from_dms((36, 24, 26.50200), (140, 36, 17.04000));
    let expected = LatLon::from_dms((36, 24, 37.90364), (140, 36, 05.02858));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 雷() {
    let tokyo = LatLon::from_dms((36, 24, 09.22100), (140, 31, 26.34100));
    let expected = LatLon::from_dms((36, 24, 20.61785), (140, 31, 14.36101));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 前浜() {
    let tokyo = LatLon::from_dms((36, 22, 57.11200), (140, 36, 16.01100));
    let expected = LatLon::from_dms((36, 23, 08.52178), (140, 36, 03.99552));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 海上() {
    let tokyo = LatLon::from_dms((36, 18, 35.99000), (143, 00, 00.00000));
    let expected = LatLon::from_dms((36, 18, 47.72512), (142, 59, 47.29009));
    assert_tky2jgd(tokyo, expected);
}
