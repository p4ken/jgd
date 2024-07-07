use crate::{
    coord::{MICRO_SECS, SECS},
    LatLon,
};

/// 日本測地系から世界測地系への座標変換パラメータ。
///
/// たとえ陸地であっても、無人島や、後年に埋め立てられた沿岸部などには、パラメータが存在しない。
///
/// 出典: 国土地理院 [TKY2JGD.par](https://www.gsi.go.jp/sokuchikijun/tky2jgd_download.html) (Ver.2.1.2, 2003年公開) をもとに形式を変換して作成。
#[cfg(feature = "tky2jgd")]
pub const TKY2JGD: Grid = crate::par::TKY2JGD.to_grid();

/// 平成23年(2011年)東北地方太平洋沖地震の座標補正パラメータ。
///
/// 3月11日以降の地震で複雑な地殻変動が生じた地域には、パラメータが存在しない。
///
/// 出典: 国土地理院 [touhokutaiheiyouoki2011.par](https://www.gsi.go.jp/sokuchikijun/sokuchikijun41012.html) (Ver.4.0.0, 2017年公開) をもとに形式を変換して作成。
#[cfg(feature = "patchjgd")]
pub const TOUHOKUTAIHEIYOUOKI2011: Grid = crate::par::TOUHOKUTAIHEIYOUOKI2011.to_grid();

/// Parameters grid.
pub struct Grid<'a> {
    dots: &'a [Dot],
}
impl<'a> Grid<'a> {
    #[allow(dead_code)]
    pub(crate) const fn new(dots: &'a [Dot]) -> Self {
        Self { dots }
    }

    /// Get a shift parameter for coordinate in degrees with bilinear interpolation.
    ///
    /// Every parameters at four corners of the mesh that `coord` belongs must exist in the `Grid`.
    /// Otherwise `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use jgd::{LatLon, TKY2JGD};
    ///
    /// let coord = LatLon(35.0, 135.0);
    /// let shift = TKY2JGD.bilinear(coord);
    /// # assert!(shift.is_some());
    /// ```
    pub fn bilinear(&self, degrees: LatLon) -> Option<LatLon> {
        // > 地域毎の変換パラメータの格子点は, 3 次メッシュの中央ではなく, 南西隅に対応する (飛田, 2001)
        let mesh = Mesh3::floor(degrees);
        let i = self.search_after(0, mesh)?;
        let sw_shift = self.dots[i].shift;

        let i = self.search_at(i + 1, mesh.east())?;
        let se_shift = self.dots[i].shift;

        let i = self.search_after(i + 1, mesh.north())?;
        let nw_shift = self.dots[i].shift;

        let i = self.search_at(i + 1, mesh.north().east())?;
        let ne_shift = self.dots[i].shift;

        let LatLon(n_weight, e_weight) = mesh.diagonal_weight(degrees);
        let LatLon(s_weight, w_weight) = mesh.north().east().diagonal_weight(degrees);

        // weighted mean
        let shift = sw_shift.to_degree() * s_weight * w_weight
            + se_shift.to_degree() * s_weight * e_weight
            + nw_shift.to_degree() * n_weight * w_weight
            + ne_shift.to_degree() * n_weight * e_weight;

        Some(shift)
    }

    fn search_after(&self, first: usize, query: Mesh3) -> Option<usize> {
        self.dots
            .get(first..)?
            .binary_search_by_key(&query, |dot| dot.mesh)
            .ok()
            .map(|i| i + first)
    }

    fn search_at(&self, index: usize, query: Mesh3) -> Option<usize> {
        (self.dots.get(index)?.mesh == query).then_some(index)
    }

    /// Nearest-neighbor interpolation.
    ///
    /// 最近傍補間。
    fn _nearest(&self, _degrees: LatLon, _limit: f64) -> LatLon {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Dot {
    mesh: Mesh3,
    shift: MicroSecond,
}

/// Serial number of Japanese MESH3 grids starting from 0 degree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
struct Mesh3 {
    lat: i16,
    lon: i16,
}
impl Mesh3 {
    const LAT_SEC: f64 = 30.;
    const LON_SEC: f64 = 45.;

    /// Evaluate the southwest of the mesh containing `p`.
    fn floor(degrees: LatLon) -> Self {
        // "saturating cast" since Rust 1.45.0
        // https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html#fixing-unsoundness-in-casts
        let lat = (degrees.lat() * 120.) as i16;
        let lon = (degrees.lon() * 80.) as i16;
        Self { lat, lon }
    }
    fn diagonal_weight(self, p: LatLon) -> LatLon {
        let diff_secs = (p - self.to_degree()).map(|x| x.abs() * SECS);
        let weight_lat = diff_secs.lat() / Self::LAT_SEC;
        let weight_lon = diff_secs.lon() / Self::LON_SEC;
        LatLon(weight_lat, weight_lon)
    }
    fn north(mut self) -> Self {
        self.lat += 1;
        self
    }
    fn east(mut self) -> Self {
        self.lon += 1;
        self
    }
    fn to_degree(self) -> LatLon {
        let lat = f64::from(self.lat) * Self::LAT_SEC;
        let lon = f64::from(self.lon) * Self::LON_SEC;
        LatLon(lat, lon) / SECS
    }
}

/// Shift amount in microseconds.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MicroSecond {
    lat: i32,
    lon: i32,
}
impl MicroSecond {
    fn to_degree(self) -> LatLon {
        LatLon(self.lat, self.lon).map(f64::from) / MICRO_SECS
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;

    use crate::{
        coord::{MICRO_SECS, SECS},
        Grid, LatLon,
    };

    use super::{Dot, Mesh3, MicroSecond};

    #[cfg(feature = "tky2jgd")]
    #[test]
    fn tky2jgd_dots() {
        use super::TKY2JGD;

        let records = TKY2JGD.dots;
        assert_eq!(records.len(), 392323);

        let r = records.last().unwrap();
        assert_eq!(r.mesh.lat, 5463);
        assert_eq!(r.mesh.lon, 11356);
        assert_eq!(r.shift.lat, 7875320);
        assert_eq!(r.shift.lon, -13995610);
    }

    #[test]
    fn micro_second() {
        let deg = MicroSecond {
            lat: 3600,
            lon: 7200,
        }
        .to_degree();
        assert_eq!(deg, LatLon(0.000_001, 0.000_002));
    }

    //         45"
    //  (0, 0) -- (6, 6)
    //    |         | 30"
    //  (0,-6) -- (6, 0)
    const SMALLEST: &[Dot] = &[
        Dot {
            mesh: Mesh3 { lon: 0, lat: 0 },
            shift: MicroSecond { lon: 0, lat: -6 },
        },
        Dot {
            mesh: Mesh3 { lon: 1, lat: 0 },
            shift: MicroSecond { lon: 6, lat: 0 },
        },
        Dot {
            mesh: Mesh3 { lon: 0, lat: 1 },
            shift: MicroSecond { lon: 0, lat: 0 },
        },
        Dot {
            mesh: Mesh3 { lon: 1, lat: 1 },
            shift: MicroSecond { lon: 6, lat: 6 },
        },
    ];

    #[test]
    fn interpolate_corner() {
        let sut = Grid::new(&SMALLEST);
        let ret = sut.bilinear(LatLon::new(0.0, 0.0)).unwrap();
        assert_eq!(ret.lon(), 0.0);
        assert_eq!(ret.lat(), -6. / MICRO_SECS);
    }

    #[test]
    fn interpolate_middle() {
        let sut = Grid::new(&SMALLEST);
        let exp = LatLon(-2., 2.) / MICRO_SECS;
        let ret = sut.bilinear(LatLon(10., 15.) / SECS).unwrap();
        assert_ulps_eq!(exp.lat(), ret.lat());
        assert_ulps_eq!(exp.lon(), ret.lon());
    }

    #[test]
    fn interpolate_out_of_grid() {
        let sut = Grid::new(&SMALLEST);
        let ret = sut.bilinear(LatLon(30.001, 45.001) / SECS);
        assert_eq!(ret, None);
    }

    #[test]
    fn interpolate_almost_out_of_grid() {
        let sut = Grid::new(&SMALLEST);
        let ret = sut.bilinear(LatLon(29.999, 44.999) / SECS);
        assert_ne!(ret, None);
    }
}
