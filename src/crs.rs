use crate::{
    coord::ECEF,
    earth::{BESSEL, GRS80},
    DegreeRangeError, LatLon,
};

#[cfg(feature = "tky2jgd")]
use crate::TKY2JGD;

#[cfg(feature = "patchjgd")]
use crate::TOUHOKUTAIHEIYOUOKI2011;

/// Tokyo Datum, The older Japanese Datum.
///
/// 旧日本測地系。
///
/// EPSG: 4301
pub struct Tokyo {
    degrees: LatLon,
}
impl Tokyo {
    /// Constructs a [`Tokyo`] with a coordinate in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{LatLon, Tokyo};
    /// #
    /// # fn main() -> anyhow::Result<()> {
    /// let jgd2011 = Tokyo::new(LatLon(35.0, 135.0))?.to_jgd2000().to_jgd2011();
    /// #   Ok(())
    /// # }
    /// ```
    pub fn new(degrees: LatLon) -> Result<Self, DegreeRangeError> {
        degrees.validate_degrees()?;
        Ok(Self { degrees })
    }

    /// Transforms to [`Jgd2000`].
    ///
    /// [`TKY2JGD`] を用いて変換される。精度は、一定の条件下で
    /// 「緯度, 経度の標準偏差はそれぞれ9cm, 8cm」[(飛田, 2001)](crate#references)。
    ///
    /// ただし、[`TKY2JGD`] の範囲外では [`Tokyo97::to_jgd2000`] によって変換され、精度が大きく下がる。
    ///
    /// 日本国内の地表面の座標のみに使用可能。地中や空中ではズレが大きくなる。
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{Tokyo, LatLon};
    /// #
    /// # let tokyo = Tokyo::new(LatLon(35.0, 135.0)).unwrap();
    /// let LatLon(lat, lon) = tokyo.to_jgd2000().degrees();
    /// ```
    #[cfg(feature = "tky2jgd")]
    pub fn to_jgd2000(&self) -> Jgd2000 {
        match TKY2JGD.bilinear(self.degrees) {
            Some(shift) => Jgd2000::new(self.degrees + shift),
            None => Tokyo97::new_unchecked(self.degrees).to_jgd2000(),
        }
    }

    /// Transforms to [`Tokyo97`].
    fn _to_tokyo97(&self) {}

    /// Returnes coordinate in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{Tokyo, LatLon};
    /// #
    /// # let tokyo = Tokyo::new(LatLon(35.0, 135.0)).unwrap();
    /// let LatLon(lat, lon) = tokyo.degrees();
    /// ```
    pub fn degrees(&self) -> LatLon {
        self.degrees
    }
}

/// Tokyo 97, The older Japanese Datum.
///
/// 世界測地系を基準に、3パラメータによる変換式で定義された測地系 [(飛田, 1997)](crate#references)。
///
/// 旧日本測地系で測量された座標には [`Tokyo`] の方が適している。
pub struct Tokyo97 {
    degrees: LatLon,
}
impl Tokyo97 {
    const TO_ITRF94: ECEF = ECEF::new(-146.414, 507.337, 680.507);

    /// Constructs a [`Tokyo97`] with a coordinate in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{LatLon, Tokyo97};
    /// #
    /// # fn main() -> anyhow::Result<()> {
    /// let jgd2000 = Tokyo97::new(LatLon(35.0, 135.0))?.to_jgd2000();
    /// #   Ok(())
    /// # }
    /// ```
    pub fn new(degrees: LatLon) -> Result<Self, DegreeRangeError> {
        degrees.validate_degrees()?;
        Ok(Self { degrees })
    }

    fn new_unchecked(degrees: LatLon) -> Self {
        Self { degrees }
    }

    /// Transforms to [`Jgd2000`].
    ///
    /// 3パラメータ [(飛田, 2001)](crate#references) を用いて変換される。
    /// このパラメータは東京を基準に算出された。北海道や九州のように遠くへ行くほどズレが大きくなる傾向がある。
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{Tokyo97, LatLon};
    /// #
    /// # let tokyo97 = Tokyo97::new(LatLon(35.0, 135.0)).unwrap();
    /// let LatLon(lat, lon) = tokyo97.to_jgd2000().degrees();
    /// ```
    pub fn to_jgd2000(&self) -> Jgd2000 {
        // https://www.gsi.go.jp/LAW/G2000-g2000faq-1.htm
        // > 測地成果2000での経度・緯度は、世界測地系であるITRF94座標系とGRS80の楕円体を使用して表します
        let itrf94 = BESSEL.to_ecef(self.degrees) + Self::TO_ITRF94;
        Jgd2000::new(GRS80.to_geodetic(itrf94))
    }

    /// Inverse of [`Tokyo::to_tokyo97`].
    fn _to_tokyo(&self) {}

    /// Returnes coordinate in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{LatLon, Tokyo97};
    /// #
    /// # let tokyo97 = Tokyo97::new(LatLon(35.0, 135.0)).unwrap();
    /// let LatLon(lat, lon) = tokyo97.degrees();
    /// ```
    pub fn degrees(&self) -> LatLon {
        self.degrees
    }
}

/// Japanese Geodetic Datum 2000 (JGD2000).
///
/// 世界測地系。
///
/// EPSG: 4612
pub struct Jgd2000 {
    degrees: LatLon,
}
impl Jgd2000 {
    /// Constructs a [`Jgd2000`] with a coordinate in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{LatLon, Jgd2000};
    /// #
    /// let to_jgd2011 = Jgd2000::new(LatLon(35.0, 135.0)).to_jgd2011();
    pub fn new(degrees: LatLon) -> Self {
        // TODO: 度単位の範囲チェック
        Self { degrees }
    }

    /// Transforms to [`Jgd2011`].
    ///
    /// [`TOUHOKUTAIHEIYOUOKI2011`] を用いて変換される。
    /// ただし、パラメータが存在しない地域では何も行われない。
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{Jgd2000, LatLon};
    /// #
    /// # let jgd2000 = Jgd2000::new(LatLon(35.0, 135.0));
    /// let LatLon(lat, lon) = jgd2000.to_jgd2011().degrees();
    /// ```
    #[cfg(feature = "patchjgd")]
    pub fn to_jgd2011(&self) -> Jgd2011 {
        let shift = TOUHOKUTAIHEIYOUOKI2011
            .bilinear(self.degrees)
            .unwrap_or_default();
        Jgd2011::new(self.degrees + shift)
    }

    /// Inverse of [`Tokyo::to_jgd2000`].
    fn _to_tokyo(&self) {
        // グリッドのキーは日本測地系だが、求めたいのも日本測地系なので、矛盾している。
        // オリジナルの実装 modTky2jgd.bas:1108 は、精度や対応範囲を割り切っている。
    }

    /// Inverse of [`Tokyo97::to_jgd2000`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{Jgd2000, LatLon};
    /// #
    /// # let jgd2000 = Jgd2000::new(LatLon(35.0, 135.0));
    /// let LatLon(lat, lon) = jgd2000.to_tokyo97().degrees();
    /// ```
    pub fn to_tokyo97(&self) -> Tokyo97 {
        let itrf94 = GRS80.to_ecef(self.degrees) - Tokyo97::TO_ITRF94;
        Tokyo97::new_unchecked(BESSEL.to_geodetic(itrf94))
    }

    /// Returnes coordinate in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{LatLon, Jgd2000};
    /// #
    /// # let jgd2000 = Jgd2000::new(LatLon(35.0, 135.0));
    /// let LatLon(lat, lon) = jgd2000.degrees();
    /// ```
    pub fn degrees(&self) -> LatLon {
        self.degrees
    }
}

/// Japanese Geodetic Datum 2011 (JGD2011).
///
/// 世界測地系。
///
/// EPSG: 6668
pub struct Jgd2011 {
    degrees: LatLon,
}
impl Jgd2011 {
    #[allow(dead_code)]
    fn new(degrees: LatLon) -> Self {
        // TODO: 度単位の範囲チェック
        Self { degrees }
    }

    /// Inverse of [`Jgd2000::to_jgd2011`].
    ///
    /// [`TOUHOKUTAIHEIYOUOKI2011`] を用いて逆変換される。
    fn _to_jgd2000(&self) {
        // Jgd2000::_to_tokyo() と同様の課題あり
    }

    /// Returnes coordinate in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{LatLon, Jgd2000};
    /// #
    /// # let jgd2011 = Jgd2000::new(LatLon(35.0, 135.0)).to_jgd2011();
    /// let LatLon(lat, lon) = jgd2011.degrees();
    /// ```
    pub fn degrees(&self) -> LatLon {
        self.degrees
    }
}

/// 平面直角座標系
// https://vldb.gsi.go.jp/sokuchi/surveycalc/surveycalc/algorithm/xy2bl/xy2bl.htm
// https://sw1227.hatenablog.com/entry/2018/11/30/200702
struct _PlaneRectangular {}
