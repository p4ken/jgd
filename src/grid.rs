use crate::LatLon;

/// 日本測地系から世界測地系への座標変換パラメータ。
///
/// たとえ陸地であっても、無人島や、後年に埋め立てられた沿岸部などで、パラメータグリッドが存在しない。
///
/// 出典: 国土地理院 [TKY2JGD.par](https://www.gsi.go.jp/sokuchikijun/tky2jgd_download.html) (Ver.2.1.2, 2003年公開) をもとに形式を変換して作成。
#[cfg(feature = "tky2jgd")]
pub const TKY2JGD: Grid = crate::par::TKY2JGD.to_grid();

/// 平成23年(2011年)東北地方太平洋沖地震の座標補正パラメータ。
///
/// 3月11日以降に複雑な地殻変動をともなう地震の発生した地域では、パラメータが存在しない。
///
/// 出典: 国土地理院 [touhokutaiheiyouoki2011.par](https://www.gsi.go.jp/sokuchikijun/sokuchikijun41012.html) (Ver.4.0.0, 2017年公開) をもとに形式を変換して作成。
#[cfg(feature = "patchjgd")]
pub const TOUHOKUTAIHEIYOUOKI2011: Grid = crate::par::TOUHOKUTAIHEIYOUOKI2011.to_grid();

/// パラメータグリッド。
/// Parameters grid.
pub struct Grid<'a> {
    dots: &'a [Dot],
}
impl<'a> Grid<'a> {
    #[allow(dead_code)]
    pub(crate) const fn new(dots: &'a [Dot]) -> Self {
        Self { dots }
    }

    /// バイリニア補間。
    /// Bilinear interpolation.
    ///
    /// 指定された座標が属する3次メッシュの四隅すべてのパラメータがグリッド内に存在しなければならない。
    /// 一つでも欠けていた場合は `None` を返す。
    pub fn bilinear(&self, p: LatLon) -> Option<LatLon> {
        // > 地域毎の変換パラメータの格子点は，3 次メッシュの中央ではなく，南西隅に対応する (飛田, 2001)
        let mesh = Mesh3::floor(p);
        let i = self.search_after(0, mesh)?;
        let sw_shift = self.dots[i].shift;

        let i = self.search_at(i + 1, mesh.east())?;
        let se_shift = self.dots[i].shift;

        let i = self.search_after(i + 1, mesh.north())?;
        let nw_shift = self.dots[i].shift;

        let i = self.search_at(i + 1, mesh.north().east())?;
        let ne_shift = self.dots[i].shift;

        let LatLon(n_weight, e_weight) = mesh.diagonal_weight(p);
        let LatLon(s_weight, w_weight) = mesh.north().east().diagonal_weight(p);

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

    /// 最近傍補間。
    /// Nearest-neighbor interpolation.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
/// Serial number of Japanese MESH3 grids starting from 0 degree.
struct Mesh3 {
    lat: i16,
    lon: i16,
}
impl Mesh3 {
    const LAT_SEC: f64 = 30.;
    const LON_SEC: f64 = 45.;

    /// Evaluate the southwest of the mesh containing `p`.
    fn floor(degree: LatLon) -> Self {
        // "saturating cast" since Rust 1.45.0
        // https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html#fixing-unsoundness-in-casts
        let lat = (degree.lat() * 120.) as i16;
        let lon = (degree.lon() * 80.) as i16;
        Self { lat, lon }
    }
    fn diagonal_weight(self, p: LatLon) -> LatLon {
        let min_degree = self.to_degree();
        let weight_lat = (p.lat() - min_degree.lat()).abs() * 3_600. / Self::LAT_SEC;
        let weight_lon = (p.lon() - min_degree.lon()).abs() * 3_600. / Self::LON_SEC;
        LatLon::new(weight_lat, weight_lon)
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
        LatLon::from_secs(lat, lon)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MicroSecond {
    lat: i32,
    lon: i32,
}
impl MicroSecond {
    fn to_degree(self) -> LatLon {
        let lat = f64::from(self.lat) / 3_600_000_000.;
        let lon = f64::from(self.lon) / 3_600_000_000.;
        LatLon::new(lat, lon)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;

    use super::*;

    #[cfg(feature = "tky2jgd")]
    #[test]
    fn tky2jgd() {
        let records = TKY2JGD.dots;
        assert_eq!(records.len(), 392323);

        let r = records.last().unwrap();
        assert_eq!(r.mesh.lat, 5463);
        assert_eq!(r.mesh.lon, 11356);
        assert_eq!(r.shift.lat, 7875320);
        assert_eq!(r.shift.lon, -13995610);
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
        assert_eq!(ret.lat(), -6. / 3_600_000_000.);
    }

    #[test]
    fn interpolate_middle() {
        let sut = Grid::new(&SMALLEST);
        let exp = LatLon::from_micro_secs(-2, 2);
        let ret = sut.bilinear(LatLon::from_secs(10., 15.)).unwrap();
        assert_ulps_eq!(exp.lat(), ret.lat());
        assert_ulps_eq!(exp.lon(), ret.lon());
    }
}
