// ------------------------------------ Temperature ------------------------------------
pub trait CelsiusToFahrenheit {
    fn h_celsius_to_fahrenheit(&self) -> f64;
}
impl<T> CelsiusToFahrenheit for T
where
    T: Copy + Into<f64>,
{
    fn h_celsius_to_fahrenheit(&self) -> f64 {
        (*self).into() * 9.0 / 5.0 + 32.0
    }
}
pub trait FahrenheitToCelsius {
    fn h_fahrenheit_to_celsius(&self) -> f64;
}
impl<T> FahrenheitToCelsius for T
where
    T: Copy + Into<f64>,
{
    fn h_fahrenheit_to_celsius(&self) -> f64 {
        ((*self).into() - 32.0) * 5.0 / 9.0
    }
}   

pub trait CelsiusToKelvin {
    fn h_celsius_to_kelvin(&self) -> f64;
}
impl<T> CelsiusToKelvin for T
where
    T: Copy + Into<f64>,
{
    fn h_celsius_to_kelvin(&self) -> f64 {
        (*self).into() + 273.15
    }
}
pub trait KelvinToCelsius {
    fn h_kelvin_to_celsius(&self) -> f64;
}
impl<T> KelvinToCelsius for T
where
    T: Copy + Into<f64>,
{
    fn h_kelvin_to_celsius(&self) -> f64 {
        (*self).into() - 273.15
    }
}   

pub trait FahrenheitToKelvin {
    fn h_fahrenheit_to_kelvin(&self) -> f64;
}
impl<T> FahrenheitToKelvin for T
where
    T: Copy + Into<f64>,
{
    fn h_fahrenheit_to_kelvin(&self) -> f64 {   
        ((*self).into() - 32.0) * 5.0 / 9.0 + 273.15
    }
}
pub trait KelvinToFahrenheit {
    fn h_kelvin_to_fahrenheit(&self) -> f64;
}
impl<T> KelvinToFahrenheit for T
where
    T: Copy + Into<f64>,
{
    fn h_kelvin_to_fahrenheit(&self) -> f64 {
        ((*self).into() - 273.15) * 9.0 / 5.0 + 32.0
    }
}       


// ------------------------------------ Length conversions ------------------------------------
pub trait MetersToKilometers {
    fn h_meters_to_kilometers(&self) -> f64;
}
impl<T> MetersToKilometers for T
where
    T: Copy + Into<f64>,
{
    fn h_meters_to_kilometers(&self) -> f64 {
        (*self).into() / 1000.0
    }
}
pub trait KilometersToMeters {
    fn h_kilometers_to_meters(&self) -> f64;
}
impl<T> KilometersToMeters for T
where
    T: Copy + Into<f64>,
{
    fn h_kilometers_to_meters(&self) -> f64 {
        (*self).into() * 1000.0
    }
}       

pub trait CentimetersToMeters {
    fn h_centimeters_to_meters(&self) -> f64;
}
impl<T> CentimetersToMeters for T
where
    T: Copy + Into<f64>,
{
    fn h_centimeters_to_meters(&self) -> f64 {
        (*self).into() / 100.0
    }
}
pub trait MetersToCentimeters {
    fn h_meters_to_centimeters(&self) -> f64;
}
impl<T> MetersToCentimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_meters_to_centimeters(&self) -> f64 {
        (*self).into() * 100.0
    }
}

pub trait CentimetersToMillimeters {
    fn h_centimeters_to_millimeters(&self) -> f64;
}
impl<T> CentimetersToMillimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_centimeters_to_millimeters(&self) -> f64 {
        (*self).into() * 10.0
    }
}
pub trait MillimetersToCentimeters {
    fn h_millimeters_to_centimeters(&self) -> f64;
}
impl<T> MillimetersToCentimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_millimeters_to_centimeters(&self) -> f64 {
        (*self).into() / 10.0
    }
}

pub trait KilometersToMiles {
    fn h_kilometers_to_miles(&self) -> f64;
}
impl<T> KilometersToMiles for T
where
    T: Copy + Into<f64>,
{
    fn h_kilometers_to_miles(&self) -> f64 {
        (*self).into() * 0.621371
    }
}
pub trait MilesToKilometers {
    fn h_miles_to_kilometers(&self) -> f64;
}
impl<T> MilesToKilometers for T
where
    T: Copy + Into<f64>,
{
    fn h_miles_to_kilometers(&self) -> f64 {
        (*self).into() / 0.621371
    }
}

pub trait InchesToCentimeters {
    fn h_inches_to_centimeters(&self) -> f64;
}
impl<T> InchesToCentimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_inches_to_centimeters(&self) -> f64 {
        (*self).into() * 2.54
    }
}

pub trait CentimetersToInches {
    fn h_centimeters_to_inches(&self) -> f64;
}
impl<T> CentimetersToInches for T
where
    T: Copy + Into<f64>,
{
    fn h_centimeters_to_inches(&self) -> f64 {
        (*self).into() / 2.54
    }
}

pub trait CentimetersToDecimeters {
    fn h_centimeters_to_decimeters(&self) -> f64;
}
impl<T> CentimetersToDecimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_centimeters_to_decimeters(&self) -> f64 {
        (*self).into() / 10.0
    }
}
pub trait DecimetersToCentimeters {
    fn h_decimeters_to_centimeters(&self) -> f64;
}
impl<T> DecimetersToCentimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_decimeters_to_centimeters(&self) -> f64 {
        (*self).into() * 10.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(0.0.h_celsius_to_fahrenheit(), 32.0);
        assert_eq!(100.0.h_celsius_to_fahrenheit(), 212.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(32.0.h_fahrenheit_to_celsius(), 0.0);
        assert_eq!(212.0.h_fahrenheit_to_celsius(), 100.0);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        assert_eq!(0.0.h_celsius_to_kelvin(), 273.15);
    }

    #[test]
    fn test_kelvin_to_celsius() {
        assert_eq!(273.15.h_kelvin_to_celsius(), 0.0);
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        assert!((32.0.h_fahrenheit_to_kelvin() - 273.15).abs() < 0.01);
    }

    #[test]
    fn test_kelvin_to_fahrenheit() {
        assert!((273.15.h_kelvin_to_fahrenheit() - 32.0).abs() < 0.01);
    }

    #[test]
    fn test_meters_to_kilometers() {
        assert_eq!(1000.0.h_meters_to_kilometers(), 1.0);
    }

    #[test]
    fn test_kilometers_to_meters() {
        assert_eq!(1.0.h_kilometers_to_meters(), 1000.0);
    }

    #[test]
    fn test_centimeters_to_meters() {
        assert_eq!(100.0.h_centimeters_to_meters(), 1.0);
    }

    #[test]
    fn test_meters_to_centimeters() {
        assert_eq!(1.0.h_meters_to_centimeters(), 100.0);
    }

    #[test]
    fn test_centimeters_to_millimeters() {
        assert_eq!(1.0.h_centimeters_to_millimeters(), 10.0);
    }

    #[test]
    fn test_millimeters_to_centimeters() {
        assert_eq!(10.0.h_millimeters_to_centimeters(), 1.0);
    }

    #[test]
    fn test_kilometers_to_miles() {
        assert!((1.0.h_kilometers_to_miles() - 0.621371).abs() < 0.0001);
    }

    #[test]
    fn test_miles_to_kilometers() {
        assert!((1.0.h_miles_to_kilometers() - 1.60934).abs() < 0.0001);
    }

    #[test]
    fn test_inches_to_centimeters() {
        assert_eq!(1.0.h_inches_to_centimeters(), 2.54);
    }

    #[test]
    fn test_centimeters_to_inches() {
        assert_eq!(2.54.h_centimeters_to_inches(), 1.0);
    }

    #[test]
    fn test_centimeters_to_decimeters() {
        assert_eq!(10.0.h_centimeters_to_decimeters(), 1.0);
    }

    #[test]
    fn test_decimeters_to_centimeters() {
        assert_eq!(1.0.h_decimeters_to_centimeters(), 10.0);
    }
}
