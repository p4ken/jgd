use crate::{
    coord::ECEF,
    earth::{BESSEL, GRS80},
    LatLon,
};

#[cfg(feature = "tky2jgd")]
use crate::TKY2JGD;

#[cfg(feature = "patchjgd")]
use crate::TOUHOKUTAIHEIYOUOKI2011;

/// 旧日本測地系。Tokyo Datum, The older Japanese Datum.
///
/// EPSG: 4301
pub struct Tokyo {
    degrees: LatLon,
}
impl Tokyo {
    /// [`Tokyo`] から変換する。
    /// Transform from a coordinate in Tokyo Datum.
    ///
    /// # Examples
    ///
    /// ```
    /// use jgd::{LatLon, Tokyo};
    ///
    /// let tokyo = LatLon::from_dms((35, 0, 0.0), (135, 0, 0.0));
    /// let jgd2000 = Tokyo::new(tokyo).to_jgd2000().degrees();
    /// ```
    pub fn new(degrees: LatLon) -> Self {
        Self { degrees }
    }

    /// [`TKY2JGD`] を用いて [`Jgd2000`] へ変換する。
    /// Transform to JGD2000.
    ///
    /// # Limitations
    ///
    /// 日本国内の地表面の座標のみに使用できる。地中や空中ではズレが大きくなる。
    ///
    /// パラメータグリッドの範囲外の座標は、[`Tokyo97`] を経由し、一律の数式による変換にフォールバックされる。
    /// 複数の座標で表される形状が、パラメータグリッドの範囲内外をまたがっていると、変換後の形状が大きく変わる可能性がある。
    ///
    /// 国土地理院によるオリジナルの実装の精度は、一定条件下で「緯度, 経度の標準偏差はそれぞれ9cm, 8cm」[(飛田, 2001)](crate#references) とされている。
    #[cfg(feature = "tky2jgd")]
    pub fn to_jgd2000(&self) -> Jgd2000 {
        match TKY2JGD.bilinear(self.degrees) {
            Some(shift) => Jgd2000::new(self.degrees + shift),
            None => self.to_tokyo97().to_jgd2000(),
        }
    }

    /// 離島位置の補正量 [(飛田, 2003)](crate#references) を用いて [`Tokyo97`] へ変換する。
    #[allow(dead_code)]
    fn to_tokyo97(&self) -> Tokyo97 {
        // TODO
        Tokyo97::new(self.degrees)
    }

    /// 緯度経度。
    /// Latitude and longitude.
    pub fn degrees(&self) -> LatLon {
        self.degrees
    }
}

/// 旧日本測地系。Tokyo 97, The older Japanese Datum.
///
/// 世界測地系を基準に、3パラメータによる変換式で定義された測地系 [(飛田, 1997)](crate#references)。
///
/// 旧日本測地系で測量された座標を世界測地系へ変換するには [`Tokyo`] の方が高精度となる。
pub struct Tokyo97 {
    degrees: LatLon,
}
impl Tokyo97 {
    const TO_ITRF94: ECEF = ECEF::new(-146.414, 507.337, 680.507);

    /// [`Tokyo97`] から変換する。
    /// Entry point to transform a coordinate from Tokyo97 to other.
    ///
    /// # Examples
    ///
    /// ```
    /// use jgd::{LatLon, Tokyo97};
    ///
    /// let jgd2000 = Tokyo97::new(LatLon(35.0, 135.0)).to_jgd2000().degrees();
    /// ```
    pub fn new(degrees: LatLon) -> Self {
        Self { degrees }
    }

    /// 3パラメータ [(飛田, 2001)](crate#references) を用いて [`Jgd2000`] へ変換する。
    /// Transform to JGD2000.
    pub fn to_jgd2000(&self) -> Jgd2000 {
        // https://www.gsi.go.jp/LAW/G2000-g2000faq-1.htm
        // > 測地成果2000での経度・緯度は、世界測地系であるITRF94座標系とGRS80の楕円体を使用して表します
        let itrf94 = BESSEL.to_ecef(self.degrees) + Self::TO_ITRF94;
        Jgd2000::new(GRS80.to_geodetic(itrf94))
    }

    /// 離島位置の補正量 [(飛田, 2003)](crate#references) を用いて [`Tokyo`] へ逆変換する。
    /// Inverse of [`Tokyo::to_tokyo97`].
    fn _to_tokyo(&self) -> Tokyo {
        // TODO
        Tokyo::new(self.degrees)
    }

    /// 度単位の緯度経度。
    /// Latitude and longitude in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jgd::{LatLon, Tokyo97};
    /// #
    /// # let tokyo97 = Tokyo97::new(LatLon(35.0, 135.0));
    /// let LatLon(lat, lon) = tokyo97.degrees();
    /// ```
    pub fn degrees(&self) -> LatLon {
        self.degrees
    }
}

/// 世界測地系。Japanese Geodetic Datum 2000 (JGD2000).
///
/// EPSG: 4612
pub struct Jgd2000 {
    degrees: LatLon,
}
impl Jgd2000 {
    /// [`Jgd2000`] から変換する。
    /// Transform from a coordinate in JGD2000.
    ///
    /// # Examples
    ///
    /// ```
    /// use jgd::{Jgd2000, LatLon};
    ///
    /// let jgd2000 = LatLon::from_dms((35, 0, 0.0), (135, 0, 0.0));
    /// let jgd2011 = Jgd2000::new(jgd2000).to_jgd2011().degrees();
    /// ```
    pub fn new(degrees: LatLon) -> Self {
        Self { degrees }
    }

    /// [`TOUHOKUTAIHEIYOUOKI2011`] を用いて [`Jgd2011`] へ変換する。
    ///
    /// パラメータが存在しない地域では、何も行われない。
    /// 複数の座標で表される形状が、パラメータグリッドの範囲内外をまたがっていると、形状が大きく変わる可能性がある。
    #[cfg(feature = "patchjgd")]
    pub fn to_jgd2011(&self) -> Jgd2011 {
        let shift = TOUHOKUTAIHEIYOUOKI2011
            .bilinear(self.degrees)
            .unwrap_or_default();
        Jgd2011::new(self.degrees + shift)
    }

    /// [`TKY2JGD`] を用いて [`Tokyo`] へ逆変換する。
    /// Inverse of [`Tokyo::to_jgd2000`].
    fn _to_tokyo(&self) {}

    /// 3パラメータを用いて [`Tokyo97`] へ逆変換する。
    /// Inverse of [`Tokyo97::to_jgd2000`].
    pub fn to_tokyo97(&self) -> Tokyo97 {
        let itrf94 = GRS80.to_ecef(self.degrees) - Tokyo97::TO_ITRF94;
        Tokyo97::new(BESSEL.to_geodetic(itrf94))
    }

    /// 緯度経度。
    /// Latitude and longitude.
    pub fn degrees(&self) -> LatLon {
        self.degrees
    }
}

/// 世界測地系。Japanese Geodetic Datum 2011 (JGD2011).
///
/// EPSG: 6668
pub struct Jgd2011 {
    degrees: LatLon,
}
impl Jgd2011 {
    #[allow(dead_code)]
    fn new(degrees: LatLon) -> Self {
        Self { degrees }
    }

    /// [`TOUHOKUTAIHEIYOUOKI2011`] を用いて [`Jgd2000`] へ逆変換する。
    fn _to_jgd2000(&self) {}

    /// 緯度経度。
    /// Latitude and longitude.
    pub fn degrees(&self) -> LatLon {
        self.degrees
    }
}

// /// 平面直角座標系
// // https://vldb.gsi.go.jp/sokuchi/surveycalc/surveycalc/algorithm/xy2bl/xy2bl.htm
// // https://sw1227.hatenablog.com/entry/2018/11/30/200702
// struct _PlaneRectangular<T>(T);

// /// Webメルカトル座標系
// struct _WebMercator<T>(T);
