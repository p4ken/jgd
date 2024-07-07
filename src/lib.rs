// release build fails on warnings
#![cfg_attr(not(debug_assertions), deny(warnings))]
// doc requres nightly
#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

//! 日本国内の測地系を変換する。
//! Transform geodetic datums used in Japan.
//!
//! # Examples
//!
//! 旧日本測地系 [`Tokyo`] の北緯35度・東経135度を、世界測地系 [`Jgd2011`] に変換する。
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
//! <br>
//!
//! [`geo`](https://docs.rs/geo/latest/geo/index.html#types) の形状を測地系変換する。
//!
//! ```
//! use geo::{Coord, LineString, MapCoords};
//! use jgd::{LatLon, Tokyo};
//!
//! let tokyo_datum = LineString::from(vec![(135.0, 35.0), (135.1, 35.1)]);
//! let jgd2011 = tokyo_datum.map_coords(|Coord { x, y }| {
//!     // 順序に注意: lat, lon <=> y, x
//!     let LatLon(y, x) = Tokyo::new(LatLon(y, x)).to_jgd2000().to_jgd2011().degrees();
//!     Coord { x, y }
//! });
//! ```
//!
//! # Limitations
//!
//! 国内の陸地を対象としている。海上や国外の座標には適さない。
//!
//! 一般に、測地系変換によって、ある測地系で測量・作成された座標を、あたかも別の測地系かのように模擬できる。
//! 異なる測地系で整備された座標同士のズレを低減できても、ズレが消滅することはない。
//! 変換メソッド毎に精度や制約が異なり、詳細はそれぞれのドキュメントに記載されている。
//!
//! 緯度経度で表される地理座標のみ変換可能。平面直角座標系(XY)は未対応。
//!
//! # Compatibility
//!
//! パラメータグリッドによる変換は、国土地理院の `TKY2JGD` および `PatchJGD` を独自に再現したもの。
//!
//! 3パラメータによる変換は、`QGIS` などで使われる `Proj` と同等の実装。
//!
//! # Features
//!
//! 有効にすると、ビルド後のバイナリサイズが増える。
//!
//! - `tky2jgd` - [TKY2JGD] を使用する。デフォルトで有効。
//! - `patchjgd` - [TOUHOKUTAIHEIYOUOKI2011] を使用する。デフォルトで有効。
//!
//! # References
//!
//! - 飛田幹男 [最近の測地座標系と座標変換についての考察](https://www.jstage.jst.go.jp/article/sokuchi1954/43/4/43_4_231/_pdf) (測地学会誌 43巻 4号 (1997) pp231-235)
//! - 飛田幹男 [世界測地系移行のための座標変換ソフトウェア "TKY2JGD"](https://www.gsi.go.jp/common/000063173.pdf) (国土地理院時報 97集 (2001) pp31-51)
//! - 飛田幹男ほか [日本測地系における離島位置の補正量](https://www.jstage.jst.go.jp/article/sokuchi1954/49/3/49_3_181/_pdf) (測地学会誌 49巻 3号 (2003) pp181-192)
//! - 飛田幹男 [地震時地殻変動に伴う座標値の変化を補正するソフトウェア "PatchJGD"](https://www.jstage.jst.go.jp/article/sokuchi/55/4/55_4_355/_pdf/-char/ja) (測地学会誌 55巻 4号 (2009) pp355-367)

mod coord;
mod crs;
mod earth;
mod grid;
mod island;
#[cfg(any(feature = "tky2jgd", feature = "patchjgd"))]
mod par;

pub use coord::{Dms, LatLon};
pub use crs::{Jgd2000, Jgd2011, Tokyo, Tokyo97};
pub use grid::Grid;
#[cfg(feature = "tky2jgd")]
pub use grid::TKY2JGD;
#[cfg(feature = "patchjgd")]
pub use grid::TOUHOKUTAIHEIYOUOKI2011;
