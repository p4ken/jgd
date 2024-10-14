// release build fails on warnings
#![cfg_attr(not(debug_assertions), deny(warnings))]
// doc requres nightly
#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

//! Transform geodetic datums used in Japan.
//!
//! # Examples
//!
//! ```
//! use jgd::{LatLon, Tokyo};
//!
//! let LatLon(lat, lon) = Tokyo::new(LatLon(35.0, 135.0))
//!     .to_jgd2000()
//!     .to_jgd2011()
//!     .degrees();
//! ```
//!
//! Transform coordinates of [`geo`](https://docs.rs/geo/latest/geo/index.html#types) crate:
//!
//! ```
//! use geo::{Coord, LineString, MapCoords};
//! use jgd::{LatLon, Tokyo};
//!
//! let tokyo_datum = LineString::from(vec![(135.0, 35.0), (135.1, 35.1)]);
//! let jgd2011 = tokyo_datum.map_coords(|Coord { x, y }| {
//!     // lat, lon <=> y, x
//!     let LatLon(y, x) = Tokyo::new(LatLon(y, x)).to_jgd2000().to_jgd2011().degrees();
//!     Coord { x, y }
//! });
//! ```
//!
//! # Features
//!
//! Each feature increases the size of the build binary.
//!
//! - `tky2jgd` - [TKY2JGD] is used. Enabled by default.
//! - `patchjgd` - [TOUHOKUTAIHEIYOUOKI2011] is used. Enabled by default.
//!
//! # Limitations
//!
//! 対象地域は日本国内の陸地のみ。海上や国外の座標には適さない。
//!
//! 一般に、測地系変換によって、ある測地系で測量・作成された座標を、あたかも別の測地系かのように模擬できる。
//! 異なる測地系で整備された座標同士のズレを低減できても、ズレが消滅することはない。
//! 変換方法によって精度や制約が異なり、詳細はメソッド毎のドキュメントに記載されている。
//!
//! 緯度経度で表される地理座標のみが対応されている。平面直角座標系などの投影座標は対応されていない。
//!
//! # Compatibility
//!
//! パラメータグリッドによる変換は、国土地理院の `TKY2JGD` および `PatchJGD` と同等。
//!
//! 3パラメータによる変換は、`QGIS` などで使われる `Proj` と同等。
//!
//! オリジナルの実装との差異が 1mm 以内となるようにテストされている。
//!
//! # References
//!
//! - 飛田幹男 [最近の測地座標系と座標変換についての考察](https://www.jstage.jst.go.jp/article/sokuchi1954/43/4/43_4_231/_pdf) (測地学会誌 43巻 4号 (1997) pp231-235)
//! - 飛田幹男 [世界測地系移行のための座標変換ソフトウェア "TKY2JGD"](https://www.gsi.go.jp/common/000063173.pdf) (国土地理院時報 97集 (2001) pp31-51)
//! - 飛田幹男 [地震時地殻変動に伴う座標値の変化を補正するソフトウェア "PatchJGD"](https://www.jstage.jst.go.jp/article/sokuchi/55/4/55_4_355/_pdf/-char/ja) (測地学会誌 55巻 4号 (2009) pp355-367)

mod coord;
mod crs;
mod earth;
mod grid;
#[cfg(any(feature = "tky2jgd", feature = "patchjgd"))]
mod par;

pub use coord::{Dms, LatLon};
pub use crs::{Jgd2000, Jgd2011, Tokyo, Tokyo97};
pub use grid::Grid;
#[cfg(feature = "tky2jgd")]
pub use grid::TKY2JGD;
#[cfg(feature = "patchjgd")]
pub use grid::TOUHOKUTAIHEIYOUOKI2011;
