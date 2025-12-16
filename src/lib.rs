#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

/// Underscore argument macro.
///
/// See library documentation for more information.
#[macro_export]
macro_rules! args {($x:ident ( $($y:ident : $z:expr),* $(,)*)) =>
    {paste::paste!([<$x _ $(_ $y)*>])($($z),*)}}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        #[allow(non_snake_case)]
        fn clear__color(_color: [f32; 4]) {}
        #[allow(non_snake_case)]
        fn line__color_from_to(_color: [f32; 4], _from: [f32; 2], _to: [f32; 2]) {}
        #[allow(non_snake_case)]
        fn person__fname_lname_age_city_country_planet(
            _first_name: &str,
            _last_name: &str,
            _age: u8,
            _city: &str,
            _country: &str,
            _planet: &str,
        ) {}

        args!(clear(color: [1.0; 4]));
        args!(clear(color: [1.0; 4],));
        args!(line(color: [1.0; 4], from: [0.0, 0.0], to: [100.0, 100.0]));
        args!(person(
            fname: "Homer",
            lname: "Simpson",
            age: 42,
            city: "Springfield",
            country: "US",
            planet: "Earth"
        ));
    }
}
