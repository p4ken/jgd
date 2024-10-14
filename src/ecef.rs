use std::ops::{Add, Sub};

use crate::LatLon;

/// Earth-centered, Earth-fixed coordinate.
#[derive(Debug, Clone, Copy)]
pub struct ECEF {
    x: f64,
    y: f64,
    z: f64,
}
impl ECEF {
    pub(crate) const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
impl Add for ECEF {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for ECEF {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// GRS80 ellipsoid.
pub const GRS80: Ellipsoid = Ellipsoid {
    equatorial_radius: 6378137.0,
    polar_radius: 6356752.31424518,
};

/// Bessel ellipsoid.
pub const BESSEL: Ellipsoid = Ellipsoid {
    equatorial_radius: 6377397.155,
    polar_radius: 6356078.963,
};

/// Earth ellipsoid.
#[derive(Debug, Clone)]
pub struct Ellipsoid {
    // 赤道半径 (メートル)
    equatorial_radius: f64,

    // 極半径 (メートル)
    polar_radius: f64,
}
impl Ellipsoid {
    /// Converts a geodetic coordinate to [ECEF].
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

    /// Converts a [ECEF] coordinate to geodetic.
    pub fn to_geodetic(&self, ecef: ECEF) -> LatLon {
        let p = ecef.x.hypot(ecef.y);
        let theta = ((ecef.z * self.equatorial_radius) / (p * self.polar_radius)).atan();
        let lat = (ecef.z + self.polar_eccentricity() * self.polar_radius * (theta.sin().powi(3)))
            .atan2(
                p - self.equatorial_eccentricity() * self.equatorial_radius * (theta.cos().powi(3)),
            );
        let lon = ecef.y.atan2(ecef.x);
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
