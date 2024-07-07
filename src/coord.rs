use std::ops::{Add, Div, Mul, Sub};

/// 緯度経度。
/// Latitude and longitude of a coordinate.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct LatLon<T = f64>(pub T, pub T);
impl<T> LatLon<T> {
    pub fn lat(self) -> T {
        self.0
    }

    pub fn lon(self) -> T {
        self.1
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> LatLon<U> {
        let [lat, lon] = self.as_array().map(f);
        LatLon(lat, lon)
    }

    fn as_array(self) -> [T; 2] {
        [self.0, self.1]
    }
}
impl LatLon {
    pub(crate) fn new<T: Into<f64>>(lat: T, lon: T) -> Self {
        Self(lat.into(), lon.into())
    }

    /// 秒から変換する。
    /// Converts from seconds.
    pub fn from_secs<T: Into<f64>>(lat: T, lon: T) -> Self {
        Self::new(lat.into(), lon.into()) / 3_600.
    }

    /// ミリ秒から度に変換する。
    /// Converts from milliseconds.
    pub fn from_milli_secs<T: Into<f64>>(lat: T, lon: T) -> Self {
        Self::from_secs(lat, lon) / 1_000.
    }

    /// マイクロ秒から度に変換する。
    /// Converts from microseconds.
    pub fn from_micro_secs<T: Into<f64>>(lat: T, lon: T) -> Self {
        Self::from_milli_secs(lat, lon) / 1_000.
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
/// Degrees, minutes, seconds.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Dms(pub i32, pub i32, pub f64);
impl Dms {
    // fn new<D: Into<i32>, M: Into<i32>, S: Into<f64>>(d: D, m: M, s: S) -> Self {
    //     let d = d.into();
    //     let m = m.into();
    //     let s = s.into();
    //     Self { d, m, s }
    // }
    pub fn d(self) -> i32 {
        self.0
    }
    pub fn m(self) -> i32 {
        self.1
    }
    pub fn s(self) -> f64 {
        self.2
    }
    fn from_degrees(deg: f64) -> Self {
        let d = deg as i32;
        let m = (deg * 60. % 60.) as i32;
        let s = (deg * 3600.) % 60.;
        Self(d, m, s)
    }
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
