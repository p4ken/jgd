# Change log

## v0.3.0

Breaking Changes:

* The constructor of datums like `Tokyo::new()` now returns `Result`.

Added:

* Made `ECEF` and `Ellipsoid` public.

## v0.2.0

Breaking Changes:

* Removed crate level functions `from_jgd2000()`, `from_tokyo()` and `from_tokyo97()`. Use constructors like `Tokyo::new()` instead.
* `LatLon::from_dms()` is replaced to `LatLon<Dms>::to_degrees()`.
* `LatLon` functions `from_secs()`, `from_milli_secs()` and `from_micro_secs()` are replaced to `LatLon::map()` and operatorions like `/ 3600`.
* Changed `LatLon` and `Dms` to tuple structs.

## v0.1.1

Initial release.
