use jgd::{Jgd2000, LatLon, Tokyo97};

mod testing;

#[test]
fn towgs84() {
    let ret = Tokyo97::new(LatLon(35., 135.))
        .unwrap()
        .to_jgd2000()
        .degrees();
    let proj = LatLon(35.00319718, 134.99720425);
    testing::assert_distance(ret, proj);
}

#[test]
fn towgs84_inverse() {
    let ret = Jgd2000::new(LatLon(35., 135.))
        .unwrap()
        .to_tokyo97()
        .degrees();
    let proj = LatLon(34.99680236, 135.00279591);
    testing::assert_distance(ret, proj);
}
