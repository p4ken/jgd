use std::ops::{Add, Div, Mul, Sub};

pub(crate) const DEGREES: f64 = 1.;
pub(crate) const MINUTES: f64 = DEGREES * 60.;
pub(crate) const SECS: f64 = MINUTES * 60.;
pub(crate) const MILLI_SECS: f64 = 3_600_000.;
pub(crate) const MICRO_SECS: f64 = MILLI_SECS * 1_000.;

/// 緯度経度。
/// Latitude and longitude of a coordinate.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct LatLon<T = f64>(pub T, pub T);
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
    pub fn map<U>(self, f: impl Fn(T) -> U) -> LatLon<U> {
        let [lat, lon] = self.as_array().map(f);
        LatLon(lat, lon)
    }

    fn as_array(self) -> [T; 2] {
        [self.0, self.1]
    }
}
impl LatLon {
    /// コンストラクタ。
    /// Constructs with latitude and longitude.
    pub fn new(lat: f64, lon: f64) -> Self {
        Self(lat.into(), lon.into())
    }

    /// 度分秒に変換する。
    /// Converts to degrees, minutes, seconds.
    pub fn to_dms(&self) -> (Dms, Dms) {
        [self.lat(), self.lon()].map(Dms::from_degrees).into()
    }
}
impl LatLon<Dms> {
    /// 度分秒から度に変換する。
    /// Converts from [`Dms`] to degrees.
    ///
    /// # Examples
    ///
    /// 東経 139°44'28.8869" 北緯 35°39'29.1572" (日本緯経度原点)
    ///
    /// ```
    /// use jgd::{Dms, LatLon};
    ///
    /// let degrees = LatLon(Dms(35, 39, 29.1572), Dms(139, 44, 28.8869)).to_degrees();
    /// ```
    pub fn to_degrees(self) -> LatLon {
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

/// 度分秒。
/// A tuple of degrees, minutes and seconds.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Dms(pub i32, pub i32, pub f64);
impl Dms {
    /// Constructs with degrees, minutes and seconds.
    pub fn new(d: i32, m: i32, s: f64) -> Self {
        // It's not const fn because s may be Into<f64> in the future.
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

/// 三次元直交座標。
/// Earth-centered, Earth-fixed coordinate.
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
