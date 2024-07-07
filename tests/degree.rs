use approx::assert_abs_diff_eq;
use jgd::{Dms, LatLon};

#[test]
fn from_dms() {
    let LatLon(lat, lon) = LatLon(Dms(1, 6, 36.), Dms(2, 30, 0.)).to_degrees();
    assert_eq!(lat, 1.11);
    assert_eq!(lon, 2.50);
}

#[test]
fn to_dms() {
    let (lat, lon) = LatLon(1.11, 2.50).to_dms();
    assert_eq!(lat.d(), 1);
    assert_eq!(lat.m(), 6);
    assert_abs_diff_eq!(lat.s(), 36., epsilon = 0.00000001);

    assert_eq!(lon.d(), 2);
    assert_eq!(lon.m(), 30);
    assert_abs_diff_eq!(lon.s(), 0., epsilon = 0.00000001);
}
