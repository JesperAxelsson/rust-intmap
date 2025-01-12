# Changelog

## [Unreleased]

## [3.0.1] 2025-01-12
### Changed
- Avoid re-hashing (twice!) in VacantEntry::insert

## [3.0.0] 2024-12-09
### Changed
- `IntMap::new` is now const and creates an instance with zero capacity, thus will not allocate. The latter is also true for `IntMap::default`. Previously the initial capacity was 4. If you want to restore the old behavior, you can use `IntMap::with_capacity(4)`.
- The prime for hashing `u64` keys has changed. The previous one was `11400714819323198549u64`. If you want to restore the old prime, you can create a wrapper type for the `u64` key and implement `IntKey` for it.
- Make all iterator structs public.

### Changed Breaking!
- `IntMap` and co. have now a new type parameter `K` that represents the key type. The key can be any primitive integer. Custom types that wrap primitive integers are also supported if they implement `IntKey`.
- The iterator structs return the key by value instead of by reference.

## [2.0.0] 2022-07-17

### Added Breaking!
- `insert` now matches behavior of built-in `insert`
- Old `insert` has been renamed to `insert_checked`

## [1.1.0] 2022-06-11

### Added 
- Implement Default for IntMap [@roman-kashitsyn](https://github.com/roman-kashitsyn) 
- Added `set_load_factor`

### Fixed
- PartialEq implementation checked inclusion, not equality [@roman-kashitsyn](https://github.com/roman-kashitsyn) 

### Changed
- Change default load factor to 90.9

## [1.0.0] 2022-06-10
- Bump to 1.0 as we are feature complete!

### Added 
- Added Entry api [@jakoschiko](https://github.com/jakoschiko) 


## [0.8.0] 2022-06-09

### Added 
- Added changelog!
- Add support for Serde behind "serde" feature flag, disabled by default [@roman-kashitsyn](https://github.com/roman-kashitsyn) 

### Changed
- Improve documentation of `insert` [@jakoschiko](https://github.com/jakoschiko)
- Various code modernizations [@chinoto](https://github.com/chinoto)

