/// # Temperature Unit Conversion Module
/// This module provides functionality to convert temperatures between Celsius, 
/// Fahrenheit, and Kelvin.

/// Temperature Units Enum
///
/// Represents the different temperature units available for conversion
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

/// Converts a temperature from a source unit to a target unit.
///
/// # Arguments
/// * `value` - The temperature value to convert.
/// * `source_unit` - The unit to convert from.
/// * `target_unit` - The unit to convert to.
///
/// # Returns
///
/// The converted temperature value as an `f64`.
pub fn convert_temperature(
    value: f64,
    source_unit: TemperatureUnit,
    target_unit: TemperatureUnit,
) -> f64 {
    match (source_unit, target_unit) {
        (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => value * 9.0 / 5.0 + 32.0,
        (TemperatureUnit::Celsius, TemperatureUnit::Kelvin) => value + 273.15,
        (TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius) => (value - 32.0) * 5.0 / 9.0,
        (TemperatureUnit::Fahrenheit, TemperatureUnit::Kelvin) => {
            (value - 32.0) * 5.0 / 9.0 + 273.15
        }
        (TemperatureUnit::Kelvin, TemperatureUnit::Celsius) => value - 273.15,
        (TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit) => {
            (value - 273.15) * 9.0 / 5.0 + 32.0
        }
        _ => value, // If source and target units are the same, return value
    }
}

// --- Test Suite for TemperatureUnit Conversions ---

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.0001; // Tolerance for floating-point comparisons

    // --- Tests for same-unit conversions ---

    #[test]
    fn test_celsius_to_celsius_is_identity() {
        let result = convert_temperature(25.0, TemperatureUnit::Celsius, TemperatureUnit::Celsius);
        assert!((result - 25.0).abs() < EPSILON);
    }

    #[test]
    fn test_fahrenheit_to_fahrenheit_is_identity() {
        let result = convert_temperature(
            77.0,
            TemperatureUnit::Fahrenheit,
            TemperatureUnit::Fahrenheit,
        );
        assert!((result - 77.0).abs() < EPSILON);
    }

    #[test]
    fn test_kelvin_to_kelvin_is_identity() {
        let result = convert_temperature(298.15, TemperatureUnit::Kelvin, TemperatureUnit::Kelvin);
        assert!((result - 298.15).abs() < EPSILON);
    }

    // --- Tests for Celsius conversions ---

    #[test]
    fn test_celsius_to_fahrenheit() {
        // Freezing point of water
        let result =
            convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit);
        assert!((result - 32.0).abs() < EPSILON);
        // Boiling point of water
        let result =
            convert_temperature(100.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit);
        assert!((result - 212.0).abs() < EPSILON);
        // Room temperature
        let result =
            convert_temperature(20.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit);
        assert!((result - 68.0).abs() < EPSILON);
        // Negative value
        let result =
            convert_temperature(-40.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit);
        assert!((result - -40.0).abs() < EPSILON);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        // Freezing point of water
        let result = convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Kelvin);
        assert!((result - 273.15).abs() < EPSILON);
        // Absolute zero
        let result =
            convert_temperature(-273.15, TemperatureUnit::Celsius, TemperatureUnit::Kelvin);
        assert!((result - 0.0).abs() < EPSILON);
    }

    // --- Tests for Fahrenheit conversions ---

    #[test]
    fn test_fahrenheit_to_celsius() {
        // Freezing point of water
        let result =
            convert_temperature(32.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius);
        assert!((result - 0.0).abs() < EPSILON);
        // Boiling point of water
        let result =
            convert_temperature(212.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius);
        assert!((result - 100.0).abs() < EPSILON);
        // Room temperature
        let result =
            convert_temperature(68.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius);
        assert!((result - 20.0).abs() < EPSILON);
        // Negative value
        let result =
            convert_temperature(-40.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius);
        assert!((result - -40.0).abs() < EPSILON);
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        // Freezing point of water
        let result =
            convert_temperature(32.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Kelvin);
        assert!((result - 273.15).abs() < EPSILON);
        // Absolute zero
        let result = convert_temperature(
            -459.67,
            TemperatureUnit::Fahrenheit,
            TemperatureUnit::Kelvin,
        );
        assert!((result - 0.0).abs() < EPSILON);
    }

    // --- Tests for Kelvin conversions ---

    #[test]
    fn test_kelvin_to_celsius() {
        // Freezing point of water
        let result = convert_temperature(273.15, TemperatureUnit::Kelvin, TemperatureUnit::Celsius);
        assert!((result - 0.0).abs() < EPSILON);
        // Absolute zero
        let result = convert_temperature(0.0, TemperatureUnit::Kelvin, TemperatureUnit::Celsius);
        assert!((result - -273.15).abs() < EPSILON);
    }

    #[test]
    fn test_kelvin_to_fahrenheit() {
        // Freezing point of water
        let result =
            convert_temperature(273.15, TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit);
        assert!((result - 32.0).abs() < EPSILON);
        // Absolute zero
        let result = convert_temperature(0.0, TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit);
        assert!((result - -459.67).abs() < EPSILON);
    }
}
