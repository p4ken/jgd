use approx::assert_abs_diff_eq;
use jgd::LatLon;

#[test]
fn from_s() {
    let LatLon(lat, lon) = LatLon::from_secs(3_600, 7_200);
    assert_eq!(lat, 1.0);
    assert_eq!(lon, 2.0);
}

#[test]
fn from_ms() {
    let LatLon(lat, lon) = LatLon::from_milli_secs(3_600_000, 7_200_000);
    assert_eq!(lat, 1.0);
    assert_eq!(lon, 2.0);
}

#[test]
fn from_us() {
    let LatLon(lat, lon) = LatLon::from_micro_secs(3_600_000_000., 7_200_000_000.);
    assert_eq!(lat, 1.0);
    assert_eq!(lon, 2.0);
}

#[test]
fn from_dms() {
    let LatLon(lat, lon) = LatLon::from_dms((1, 6, 36), (2, 30, 0));
    assert_eq!(lat, 1.11);
    assert_eq!(lon, 2.50);
}

#[test]
fn to_dms() {
    let (lat, lon) = LatLon::from_dms((1, 6, 59.99), (2, 30, 0.)).to_dms();
    assert_eq!(lat.d, 1);
    assert_eq!(lat.m, 6);
    assert_abs_diff_eq!(lat.s, 59.99, epsilon = 0.00000001);

    assert_eq!(lon.d, 2);
    assert_eq!(lon.m, 30);
    assert_abs_diff_eq!(lon.s, 0., epsilon = 0.00000001);
}
