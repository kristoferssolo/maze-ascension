use bevy::prelude::*;
use std::ops::Deref;

/// A trait for types that can be converted to a Bevy `Color`.
///
/// Implementing this trait allows a type to be easily converted to various Bevy color types.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use maze_ascension::theme::prelude::ColorScheme;
///
/// struct MyColor(u8, u8, u8);
///
/// impl ColorScheme for MyColor {
///     fn to_color(&self) -> Color {
///         Color::srgb(
///             self.0 as f32 / 255.0,
///             self.1 as f32 / 255.0,
///             self.2 as f32 / 255.0
///         )
///     }
/// }
///
/// let my_color = MyColor(255, 0, 0);
/// let bevy_color: Color = my_color.to_color();
/// assert_eq!(bevy_color, Color::srgb(1., 0., 0.));
/// ```
pub trait ColorScheme {
    /// Converts the implementing type to a Bevy `Color`.
    fn to_color(&self) -> Color;

    /// Converts the implementing type to a Bevy `LinearRgba`.
    ///
    /// This method provides a default implementation based on `to_color()`.
    fn to_linear_rgba(&self) -> LinearRgba {
        self.to_color().to_linear()
    }

    /// Converts the implementing type to a Bevy `StandardMaterial`.
    ///
    /// This method provides a default implementation that sets the emissive color.
    fn to_standart_material(&self) -> StandardMaterial {
        StandardMaterial {
            emissive: self.to_linear_rgba(),
            ..default()
        }
    }
}

/// A wrapper type that implements `From` traits for types implementing `ColorScheme`.
///
/// This wrapper allows for easy conversion from `ColorScheme` types to Bevy color types.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use maze_ascension::theme::prelude::{ColorScheme, ColorSchemeWrapper};
///
/// struct MyColor(u8, u8, u8);
///
/// impl ColorScheme for MyColor {
///     fn to_color(&self) -> Color {
///         Color::srgb(
///             self.0 as f32 / 255.0,
///             self.1 as f32 / 255.0,
///             self.2 as f32 / 255.0
///         )
///     }
/// }
///
/// let my_color = MyColor(0, 255, 0);
/// let wrapper = ColorSchemeWrapper(my_color);
/// let bevy_color: Color = wrapper.into();
/// assert_eq!(bevy_color, Color::srgb(0., 1., 0.));
/// ```
pub struct ColorSchemeWrapper<T: ColorScheme>(pub T);

impl<T: ColorScheme> From<T> for ColorSchemeWrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: ColorScheme> Deref for ColorSchemeWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ColorScheme> From<ColorSchemeWrapper<T>> for Color {
    fn from(value: ColorSchemeWrapper<T>) -> Self {
        value.to_color()
    }
}

impl<T: ColorScheme> From<ColorSchemeWrapper<T>> for LinearRgba {
    fn from(value: ColorSchemeWrapper<T>) -> Self {
        value.to_linear_rgba()
    }
}
