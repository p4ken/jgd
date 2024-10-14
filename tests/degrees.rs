use jgd::{Jgd2000, LatLon, Tokyo, Tokyo97};

mod testing;

#[test]
fn degrees_error() {
    let err = Tokyo::new(LatLon(35., 181.)).unwrap_err();
    assert_eq!(format!("{}", err), "degrees out of range");
}

#[test]
fn degrees_error_reversed() {
    let err = Tokyo::new(LatLon(135., 35.)).unwrap_err();
    assert_eq!(
        format!("{}", err),
        "degrees out of range; may be lat and lon reversed?"
    );
}

#[test]
fn degrees_error_tokyo97() {
    let err = Tokyo97::new(LatLon(35., 181.)).unwrap_err();
    assert_eq!(format!("{}", err), "degrees out of range");
}

#[test]
fn degrees_error_jgd2000() {
    let err = Jgd2000::new(LatLon(35., 181.)).unwrap_err();
    assert_eq!(format!("{}", err), "degrees out of range");
}
