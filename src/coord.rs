use std::ops::{Add, Div, Mul, Sub};

pub(crate) const DEGREES: f64 = 1.;
pub(crate) const MINUTES: f64 = DEGREES * 60.;
pub(crate) const SECS: f64 = MINUTES * 60.;
pub(crate) const MILLI_SECS: f64 = 3_600_000.;
pub(crate) const MICRO_SECS: f64 = MILLI_SECS * 1_000.;

/// Latitude and longitude of a coordinate.
///
/// 緯度と経度のペア。
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
/// # assert_eq!(degrees.lat(), 35.);
/// # assert_eq!(degrees.lon(), 135.);
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
    pub fn lat(self) -> T {
        self.0
    }

    /// Returns longitude.
    pub fn lon(self) -> T {
        self.1
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
    /// # assert_eq!(degrees.lat(), 35.);
    /// # assert_eq!(degrees.lon(), 135.);
    /// ```
    ///
    /// Convert from degrees to seconds:
    /// ```
    /// use jgd::LatLon;
    ///
    /// let degrees = LatLon::new(35.0, 135.0);
    /// let seconds = degrees.map(|x| (x * 3_600.).round() as i32);
    /// # assert_eq!(seconds.lat(), 126000);
    /// # assert_eq!(seconds.lon(), 486000);
    /// ```
    pub fn map<U>(self, f: impl Fn(T) -> U) -> LatLon<U> {
        let [lat, lon] = self.as_array().map(f);
        LatLon(lat, lon)
    }

    fn as_array(self) -> [T; 2] {
        [self.0, self.1]
    }
}
impl LatLon<f64> {
    /// Constructs with latitude and longitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use jgd::LatLon;
    ///
    /// let degrees = LatLon::new(35.0, 135.0);
    /// ```
    pub fn new(lat: f64, lon: f64) -> Self {
        Self(lat.into(), lon.into())
    }

    /// Converts from degrees to [`Dms`].
    ///
    /// # Examples
    ///
    /// ```
    /// use jgd::LatLon;
    /// #
    /// # let degrees = LatLon(35.0, 135.0);
    /// let LatLon(lat, lon) = degrees.to_dms();
    /// # assert_eq!(lat, jgd::Dms(35, 0, 0.0));
    /// # assert_eq!(lon, jgd::Dms(135, 0, 0.0));
    /// ```
    pub fn to_dms(&self) -> LatLon<Dms> {
        self.map(Dms::from_degrees)
    }
}
impl LatLon<Dms> {
    /// 度分秒から度に変換する。
    /// Converts from [`Dms`] to degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use jgd::{Dms, LatLon};
    ///
    /// let dms = LatLon(Dms(35, 0, 0.0), Dms(135, 0, 0.0));
    /// let degrees = dms.to_degrees();
    /// # assert_eq!(degrees.lat(), 35.0);
    /// # assert_eq!(degrees.lon(), 135.0);
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

/// Degrees, minutes and seconds.
///
/// 度分秒。
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
    /// Constructs with degrees, minutes and seconds.
    pub fn new(d: i32, m: i32, s: f64) -> Self {
        Self(d, m, s)
    }

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
        Self(d, m, s)
    }

    /// Converts to decimal degrees.
    fn to_degrees(self) -> f64 {
        let Self(d, m, s) = self;
        f64::from(d) + f64::from(m) / 60. + s / 3_600.
    }
}

/// Earth-centered, Earth-fixed coordinate.
///
/// 三次元直交座標。
#[derive(Debug, Clone, Copy)]
pub struct ECEF {
    x: f64,
    y: f64,
    z: f64,
}
impl ECEF {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
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
