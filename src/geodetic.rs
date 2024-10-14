use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

pub const DEGREES: f64 = 1.;
pub const MINUTES: f64 = DEGREES * 60.;
pub const SECS: f64 = MINUTES * 60.;
pub const MILLI_SECS: f64 = SECS * 1_000.;
pub const MICRO_SECS: f64 = MILLI_SECS * 1_000.;

/// A pair of latitude and longitude.
///
/// # Examples
///
/// ```
/// use jgd::LatLon;
///
/// let degrees = LatLon(35.0, 135.0);
/// ```
///
/// Convert from [`Dms`] to degrees:
///
/// ```
/// use jgd::{Dms, LatLon};
///
/// let dms = LatLon(Dms(35, 0, 0.0), Dms(135, 0, 0.0));
/// let degrees = dms.to_degrees();
/// # assert_eq!(degrees, LatLon(35., 135.));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct LatLon<T = f64>(
    /// Latitude.
    pub T,
    /// Longitude.
    pub T,
);
impl<T> LatLon<T> {
    /// Returns latitude.
    pub fn lat(&self) -> &T {
        &self.0
    }

    /// Returns longitude.
    pub fn lon(&self) -> &T {
        &self.1
    }

    /// Returns self with function `f` applied to both lat and lon.
    ///
    /// # Examples
    ///
    /// Convert from seconds to degrees:
    ///
    /// ```
    /// use jgd::LatLon;
    ///
    /// let seconds = LatLon(126000, 486000);
    /// let degrees = seconds.map(|x| f64::from(x) / 3_600.);
    /// # assert_eq!(degrees, LatLon(35., 135.));
    /// ```
    ///
    /// Convert from degrees to seconds:
    ///
    /// ```
    /// # use jgd::{LatLon, Tokyo};
    /// #
    /// # let degrees = LatLon::<f64>(35.0, 135.0);
    /// let seconds = degrees.map(|x| (x * 3_600.).round() as i32);
    /// # assert_eq!(seconds, LatLon(126000, 486000));
    /// ```
    pub fn map<U>(self, f: impl Fn(T) -> U) -> LatLon<U> {
        let LatLon(lat, lon) = self;
        LatLon(f(lat), f(lon))
    }
}
impl LatLon<f64> {
    /// Converts from degrees to [`Dms`].
    ///
    /// # Examples
    ///
    /// ```
    /// use jgd::LatLon;
    ///
    /// # let degrees = LatLon(35.0, 135.0);
    /// let LatLon(lat, lon) = degrees.to_dms();
    /// # assert_eq!(lat, jgd::Dms(35, 0, 0.0));
    /// # assert_eq!(lon, jgd::Dms(135, 0, 0.0));
    /// ```
    pub fn to_dms(self) -> LatLon<Dms> {
        self.map(Dms::from_degrees)
    }

    pub(crate) fn validate_degrees(self) -> Result<(), DegreesError> {
        fn is_in_degrees_range(lat: f64, lon: f64) -> bool {
            lat.abs() <= 90. && lon.abs() <= 180.
        }

        let LatLon(lat, lon) = self;
        if is_in_degrees_range(lat, lon) {
            return Ok(());
        }
        let possibly_reversed = is_in_degrees_range(lon, lat);
        Err(DegreesError { possibly_reversed })
    }
}
impl LatLon<Dms> {
    /// Converts from [`Dms`] to degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use jgd::{Dms, LatLon};
    ///
    /// let dms = LatLon(Dms(35, 0, 0.0), Dms(135, 0, 0.0));
    /// let degrees = dms.to_degrees();
    /// # assert_eq!(degrees, LatLon(35., 135.0));
    /// ```
    pub fn to_degrees(self) -> LatLon<f64> {
        self.map(Dms::to_degrees)
    }
}
impl Add<LatLon> for LatLon {
    type Output = Self;
    fn add(mut self, rhs: LatLon) -> Self::Output {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self
    }
}
impl Sub<LatLon> for LatLon {
    type Output = Self;
    fn sub(mut self, rhs: LatLon) -> Self::Output {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self
    }
}
impl Mul<f64> for LatLon {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|x| x * rhs)
    }
}
impl Div<f64> for LatLon {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self.map(|x| x / rhs)
    }
}

/// Degrees minutes seconds.
///
/// # Examples
///
/// ```
/// # use jgd::Dms;
/// let lat = Dms(35, 0, 0.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Dms(
    /// Degrees.
    pub i32,
    /// Minutes.
    pub i32,
    /// Seconds.
    pub f64,
);
impl Dms {
    /// Returns degrees.
    pub fn d(self) -> i32 {
        self.0
    }

    /// Returns minutes.
    pub fn m(self) -> i32 {
        self.1
    }

    /// Returns seconds.
    pub fn s(self) -> f64 {
        self.2
    }

    /// Constructs from decimal degrees.
    fn from_degrees(deg: f64) -> Self {
        let d = deg as i32;
        let m = (deg * 60. % 60.) as i32;
        let s = (deg * 3600.) % 60.;
        Dms(d, m, s)
    }

    /// Converts to decimal degrees.
    fn to_degrees(self) -> f64 {
        let Dms(d, m, s) = self;
        f64::from(d) + f64::from(m) / 60. + s / 3_600.
    }
}

/// Errors in input [LatLon].
#[derive(Debug, PartialEq, Eq)]
pub struct DegreesError {
    /// The [LatLon] may be in the order of lon, lat.
    possibly_reversed: bool,
}
impl fmt::Display for DegreesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "degrees out of range")?;

        if self.possibly_reversed {
            write!(f, "; may be lat and lon reversed?")?;
        }

        Ok(())
    }
}
impl std::error::Error for DegreesError {}
