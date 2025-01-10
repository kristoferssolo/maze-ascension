use hexx::Hex;

use super::errors::RadiusError;

pub trait Coordinates {
    fn get_coords(&self) -> (i32, i32);
}

impl Coordinates for (i32, i32) {
    fn get_coords(&self) -> (i32, i32) {
        *self
    }
}

impl Coordinates for Hex {
    fn get_coords(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn is_within_radius<R, C>(radius: R, coords: &C) -> Result<bool, RadiusError>
where
    R: Into<i32>,
    C: Coordinates,
{
    let radius = radius.into();

    if radius < 0 {
        return Err(RadiusError::NegativeRadius(radius));
    }

    let (q, r) = coords.get_coords();
    let s = -q - r; // Calculate third axial coordinate (q + r + s = 0)

    Ok(q.abs().max(r.abs()).max(s.abs()) <= radius)
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::*;
    use rstest::*;

    #[rstest]
    // Original test cases
    #[case(0, (0, 0), true)] // Center point
    #[case(1, (1, 0), true)] // Point at radius 1
    #[case(1, (2, 0), false)] // Point outside radius 1
    #[case(2, (2, 0), true)] // East
    #[case(2, (0, 2), true)] // Southeast
    #[case(2, (-2, 2), true)] // Southwest
    #[case(2, (-2, 0), true)] // West
    #[case(2, (0, -2), true)] // Northwest
    #[case(2, (2, -2), true)] // Northeast
    #[case(2, (3, 0), false)] // Just outside radius 2
    // Large radius test cases
    #[case(6, (6, 0), true)] // East at radius 6
    #[case(6, (0, 6), true)] // Southeast at radius 6
    #[case(6, (-6, 6), true)] // Southwest at radius 6
    #[case(6, (-6, 0), true)] // West at radius 6
    #[case(6, (0, -6), true)] // Northwest at radius 6
    #[case(6, (6, -6), true)] // Northeast at radius 6
    #[case(6, (7, 0), false)] // Just outside radius 6 east
    #[case(6, (4, 4), false)] // Outside radius 6 diagonal
    #[case(6, (5, 5), false)] // Outside radius 6 diagonal
    // Edge cases with large radius
    #[case(6, (6, -3), true)] // Complex position within radius 6
    #[case(6, (-3, 6), true)] // Complex position within radius 6
    #[case(6, (3, -6), true)] // Complex position within radius 6
    #[case(6, (7, -7), false)] // Outside radius 6 corner
    fn valid_radius_tuple(#[case] radius: i32, #[case] pos: (i32, i32), #[case] expected: bool) {
        let result = is_within_radius(radius, &pos);
        assert_ok_eq!(result, expected);
    }

    #[rstest]
    // Large radius test cases for Hex struct
    #[case(6, (6, 0), true)] // East at radius 6
    #[case(6, (0, 6), true)] // Southeast at radius 6
    #[case(6, (-6, 6), true)] // Southwest at radius 6
    #[case(6, (-6, 0), true)] // West at radius 6
    #[case(6, (0, -6), true)] // Northwest at radius 6
    #[case(6, (6, -6), true)] // Northeast at radius 6
    #[case(6, (4, 4), false)] // Outside radius 6 diagonal
    #[case(6, (5, 5), false)] // Outside radius 6 diagonal
    fn valid_radius_hex(#[case] radius: i32, #[case] pos: (i32, i32), #[case] expected: bool) {
        let hex = Hex::from(pos);
        let result = is_within_radius(radius, &hex);
        assert_ok_eq!(result, expected);
    }

    #[rstest]
    #[case(-1)]
    #[case(-2)]
    #[case(-5)]
    fn negative_radius(#[case] radius: i32) {
        let result = is_within_radius(radius, &(0, 0));
        assert_err!(&result);
    }

    #[test]
    fn boundary_points() {
        let radius = 3;
        // Test points exactly on the boundary of radius 3
        assert_ok_eq!(is_within_radius(radius, &(3, 0)), true); // East boundary
        assert_ok_eq!(is_within_radius(radius, &(0, 3)), true); // Southeast boundary
        assert_ok_eq!(is_within_radius(radius, &(-3, 3)), true); // Southwest boundary
        assert_ok_eq!(is_within_radius(radius, &(-3, 0)), true); // West boundary
        assert_ok_eq!(is_within_radius(radius, &(0, -3)), true); // Northwest boundary
        assert_ok_eq!(is_within_radius(radius, &(3, -3)), true); // Northeast boundary
    }

    #[test]
    fn large_boundary_points() {
        let radius = 6;
        // Test points exactly on the boundary of radius 6
        assert_ok_eq!(is_within_radius(radius, &(6, 0)), true); // East boundary
        assert_ok_eq!(is_within_radius(radius, &(0, 6)), true); // Southeast boundary
        assert_ok_eq!(is_within_radius(radius, &(-6, 6)), true); // Southwest boundary
        assert_ok_eq!(is_within_radius(radius, &(-6, 0)), true); // West boundary
        assert_ok_eq!(is_within_radius(radius, &(0, -6)), true); // Northwest boundary
        assert_ok_eq!(is_within_radius(radius, &(6, -6)), true); // Northeast boundary

        // Test points just outside the boundary
        assert_ok_eq!(is_within_radius(radius, &(7, 0)), false); // Just outside east
        assert_ok_eq!(is_within_radius(radius, &(0, 7)), false); // Just outside southeast
        assert_ok_eq!(is_within_radius(radius, &(-7, 7)), false); // Just outside southwest
    }

    #[test]
    fn different_coordinate_types() {
        // Test with tuple coordinates
        assert_ok_eq!(is_within_radius(2, &(1, 1)), true);

        // Test with Hex struct
        let hex = Hex { x: 1, y: 1 };
        assert_ok_eq!(is_within_radius(2, &hex), true);
    }
}
