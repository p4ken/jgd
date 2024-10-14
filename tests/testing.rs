use geo::{GeodesicDistance, Point};
use jgd::LatLon;

const MILLI_METERS: f64 = 0.001;
const ACCURACY: f64 = 1. * MILLI_METERS;

#[allow(dead_code)]
pub fn assert_distance(left: LatLon, right: LatLon) {
    let [p0, p1] = [left, right].map(|LatLon(lat, lon)| Point::new(lon, lat));
    let meters = p0.geodesic_distance(&p1);
    assert!(
        meters < ACCURACY,
        "
    distance: {} meters
        left: {:?}
       right: {:?}
",
        meters,
        right,
        left
    );
}
