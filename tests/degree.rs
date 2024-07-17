use approx::assert_abs_diff_eq;
use jgd::{Dms, LatLon};

mod testing;

#[test]
fn from_dms() {
    let ret = LatLon(Dms(35, 39, 29.1572), Dms(139, 44, 28.8869)).to_degrees();
    let expected = LatLon(35.65809922, 139.74135747);
    testing::assert_distance(ret, expected);
}

#[test]
fn to_dms() {
    let LatLon(lat, lon) = LatLon(35.65809922, 139.74135747).to_dms();

    assert_eq!(lat.d(), 35);
    assert_eq!(lat.m(), 39);
    assert_abs_diff_eq!(lat.s(), 29.1572, epsilon = 0.00001);

    assert_eq!(lon.d(), 139);
    assert_eq!(lon.m(), 44);
    assert_abs_diff_eq!(lon.s(), 28.8869, epsilon = 0.00001);
}
