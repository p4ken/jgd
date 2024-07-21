use crate::{coord::ECEF, LatLon};

/// GRS80楕円体
pub const GRS80: Ellipsoid = Ellipsoid {
    equatorial_radius: 6378137.0,
    polar_radius: 6356752.31424518,
};

/// Bessel楕円体
pub const BESSEL: Ellipsoid = Ellipsoid {
    equatorial_radius: 6377397.155,
    polar_radius: 6356078.963,
};

/// 回転楕円体。
#[derive(Debug, Clone)]
pub struct Ellipsoid {
    // 赤道半径 (メートル)
    equatorial_radius: f64,

    // 極半径 (メートル)
    polar_radius: f64,
}
impl Ellipsoid {
    /// 三次元直交座標に変換する。
    pub fn to_ecef(&self, degree: LatLon) -> ECEF {
        let LatLon(lat, lon) = degree.map(f64::to_radians);
        let geoid = self.equatorial_radius
            / (1.0 - self.equatorial_eccentricity() * lat.sin().powi(2)).sqrt();
        ECEF::new(
            geoid * lat.cos() * lon.cos(),
            geoid * lat.cos() * lon.sin(),
            geoid * (1.0 - self.equatorial_eccentricity()) * lat.sin(),
        )
    }

    /// 測地座標に変換する。
    pub fn to_geodetic(&self, ecef: ECEF) -> LatLon {
        let p = ecef.x().hypot(ecef.y());
        let theta = ((ecef.z() * self.equatorial_radius) / (p * self.polar_radius)).atan();
        let lat = (ecef.z()
            + self.polar_eccentricity() * self.polar_radius * (theta.sin().powi(3)))
        .atan2(p - self.equatorial_eccentricity() * self.equatorial_radius * (theta.cos().powi(3)));
        let lon = ecef.y().atan2(ecef.x());
        LatLon(lat, lon).map(f64::to_degrees)
    }

    /// 赤道離心率 = (赤道半径^2 - 極半径^2) / 赤道半径^2
    fn equatorial_eccentricity(&self) -> f64 {
        let e2 = self.equatorial_radius.powi(2);
        let p2 = self.polar_radius.powi(2);
        (e2 - p2) / e2
    }

    /// 極離心率 = (赤道半径^2 - 極半径^2) / 極半径^2
    fn polar_eccentricity(&self) -> f64 {
        let e2 = self.equatorial_radius.powi(2);
        let p2 = self.polar_radius.powi(2);
        (e2 - p2) / p2
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;

    use super::{BESSEL, GRS80};

    #[test]
    fn grs80() {
        assert_ulps_eq!(GRS80.equatorial_eccentricity(), 0.006694379990141124);
        assert_ulps_eq!(GRS80.polar_eccentricity(), 0.006739496742276239);
    }

    #[test]
    fn bessel() {
        assert_ulps_eq!(BESSEL.equatorial_eccentricity(), 0.006674372174974933);
        assert_ulps_eq!(BESSEL.polar_eccentricity(), 0.006719218741581313);
    }
}
