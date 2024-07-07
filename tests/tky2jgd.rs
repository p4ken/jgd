//! 国土地理院によるオリジナルの TKY2JGD と比較するテスト。
#![cfg(feature = "tky2jgd")]

use approx::assert_abs_diff_eq;
use jgd::{Dms, LatLon, Tokyo};

/// 許容誤差: ±1mm
const MM_IN_DEGREES: f64 = 0.000000009;

fn assert_tky2jgd(tokyo: LatLon<Dms>, expected: LatLon<Dms>) {
    let LatLon(lat0, lon0) = expected.to_degrees();
    let jgd2000 = Tokyo::new(tokyo.to_degrees()).to_jgd2000().degrees();
    assert_abs_diff_eq!(lat0, jgd2000.lat(), epsilon = MM_IN_DEGREES);
    assert_abs_diff_eq!(lon0, jgd2000.lon(), epsilon = MM_IN_DEGREES);
}

#[test]
fn 村松() {
    let tokyo = LatLon(Dms(36, 27, 39.20500), Dms(140, 35, 06.11100));
    let expected = LatLon(Dms(36, 27, 50.58487), Dms(140, 34, 54.10080));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 高野() {
    let tokyo = LatLon(Dms(36, 25, 45.63400), Dms(140, 32, 47.46200));
    let expected = LatLon(Dms(36, 25, 57.02524), Dms(140, 32, 35.46640));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 東石川() {
    let tokyo = LatLon(Dms(36, 24, 51.26200), Dms(140, 32, 15.86100));
    let expected = LatLon(Dms(36, 25, 02.65997), Dms(140, 32, 03.86700));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 長砂() {
    let tokyo = LatLon(Dms(36, 24, 45.41400), Dms(140, 34, 58.52400));
    let expected = LatLon(Dms(36, 24, 56.81069), Dms(140, 34, 46.51725));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 防風() {
    let tokyo = LatLon(Dms(36, 24, 26.50200), Dms(140, 36, 17.04000));
    let expected = LatLon(Dms(36, 24, 37.90364), Dms(140, 36, 05.02858));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 雷() {
    let tokyo = LatLon(Dms(36, 24, 09.22100), Dms(140, 31, 26.34100));
    let expected = LatLon(Dms(36, 24, 20.61785), Dms(140, 31, 14.36101));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 前浜() {
    let tokyo = LatLon(Dms(36, 22, 57.11200), Dms(140, 36, 16.01100));
    let expected = LatLon(Dms(36, 23, 08.52178), Dms(140, 36, 03.99552));
    assert_tky2jgd(tokyo, expected);
}

#[test]
fn 海上() {
    let tokyo = LatLon(Dms(36, 18, 35.99000), Dms(143, 00, 00.00000));
    let expected = LatLon(Dms(36, 18, 47.72512), Dms(142, 59, 47.29009));
    assert_tky2jgd(tokyo, expected);
}
