use anyhow::Result;
use jgd::{Jgd2000, LatLon, Tokyo97};

mod testing;

#[test]
fn towgs84() -> Result<()> {
    testing::assert_distance(
        Tokyo97::new(LatLon(35., 135.))?.to_jgd2000().degrees(),
        LatLon(35.00319718, 134.99720425),
    );
    Ok(())
}

#[test]
fn towgs84_inverse() {
    let ret = Jgd2000::new(LatLon(35., 135.)).to_tokyo97().degrees();
    let proj = LatLon(34.99680236, 135.00279591);
    testing::assert_distance(ret, proj);
}
