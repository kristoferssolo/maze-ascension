use crate::{
    create_color_scheme,
    theme::{colorscheme::ColorScheme, palette::rgb_u8},
};
use bevy::prelude::*;
use strum::EnumIter;

create_color_scheme!(
    pub RosePine, {
        Base: "#191724",
        Surface: "#1f1d2e",
        Overlay: "#26233a",
        Muted: "#6e6a86",
        Subtle: "#908caa",
        Text: "#e0def4",
        Love: "#eb6f92",
        Gold: "#f6c177",
        Rose: "#ebbcba",
        Pine: "#31748f",
        Foam: "#9ccfd8",
        Iris: "#c4a7e7",
        HighlightLow: "#21202e",
        HighlightMed: "#403d52",
        HighlightHigh: "#524f67"
    }
);

create_color_scheme!(
    pub RosePineMoon, {
        Base: "#232136",
        Surface: "#2a273f",
        Overlay: "#393552",
        Muted: "#6e6a86",
        Subtle: "#908caa",
        Text: "#e0def4",
        Love: "#eb6f92",
        Gold: "#f6c177",
        Rose: "#ea9a97",
        Pine: "#3e8fb0",
        Foam: "#9ccfd8",
        Iris: "#c4a7e7",
        HighlightLow: "#2a283e",
        HighlightMed: "#44415a",
        HighlightHigh: "#56526e"
    }
);

create_color_scheme!(
    pub RosePineDawn, {
        Base: "#faf4ed",
        Surface: "#fffaf3",
        Overlay: "#f2e9e1",
        Muted: "#9893a5",
        Subtle: "#797593",
        Text: "#575279",
        Love: "#b4637a",
        Gold: "#ea9d34",
        Rose: "#d7827e",
        Pine: "#286983",
        Foam: "#56949f",
        Iris: "#907aa9",
        HighlightLow: "#f4ede8",
        HighlightMed: "#dfdad9",
        HighlightHigh: "#cecacd"
    }
);

#[macro_export]
macro_rules! create_color_scheme {
    ($(#[$meta:meta])* $vis:vis $name:ident, {
        Base: $base:expr,
        Surface: $surface:expr,
        Overlay: $overlay:expr,
        Muted: $muted:expr,
        Subtle: $subtle:expr,
        Text: $text:expr,
        Love: $love:expr,
        Gold: $gold:expr,
        Rose: $rose:expr,
        Pine: $pine:expr,
        Foam: $foam:expr,
        Iris: $iris:expr,
        HighlightLow: $hl_low:expr,
        HighlightMed: $hl_med:expr,
        HighlightHigh: $hl_high:expr
    }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
        $vis enum $name {
            Base,
            Surface,
            Overlay,
            Muted,
            Subtle,
            Text,
            Love,
            Gold,
            Rose,
            Pine,
            Foam,
            Iris,
            HighlightLow,
            HighlightMed,
            HighlightHigh,
        }

        impl $name {
            fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
                let hex = if hex.starts_with('#') { &hex[1..] } else { hex };
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                (r, g, b)
            }
        }

        impl ColorScheme for $name {
            fn to_color(&self) -> Color {
                match self {
                    Self::Base => {
                        let (r, g, b) = Self::hex_to_rgb($base);
                        rgb_u8(r, g, b)
                    },
                    Self::Surface => {
                        let (r, g, b) = Self::hex_to_rgb($surface);
                        rgb_u8(r, g, b)
                    },
                    Self::Overlay => {
                        let (r, g, b) = Self::hex_to_rgb($overlay);
                        rgb_u8(r, g, b)
                    },
                    Self::Muted => {
                        let (r, g, b) = Self::hex_to_rgb($muted);
                        rgb_u8(r, g, b)
                    },
                    Self::Subtle => {
                        let (r, g, b) = Self::hex_to_rgb($subtle);
                        rgb_u8(r, g, b)
                    },
                    Self::Text => {
                        let (r, g, b) = Self::hex_to_rgb($text);
                        rgb_u8(r, g, b)
                    },
                    Self::Love => {
                        let (r, g, b) = Self::hex_to_rgb($love);
                        rgb_u8(r, g, b)
                    },
                    Self::Gold => {
                        let (r, g, b) = Self::hex_to_rgb($gold);
                        rgb_u8(r, g, b)
                    },
                    Self::Rose => {
                        let (r, g, b) = Self::hex_to_rgb($rose);
                        rgb_u8(r, g, b)
                    },
                    Self::Pine => {
                        let (r, g, b) = Self::hex_to_rgb($pine);
                        rgb_u8(r, g, b)
                    },
                    Self::Foam => {
                        let (r, g, b) = Self::hex_to_rgb($foam);
                        rgb_u8(r, g, b)
                    },
                    Self::Iris => {
                        let (r, g, b) = Self::hex_to_rgb($iris);
                        rgb_u8(r, g, b)
                    },
                    Self::HighlightLow => {
                        let (r, g, b) = Self::hex_to_rgb($hl_low);
                        rgb_u8(r, g, b)
                    },
                    Self::HighlightMed => {
                        let (r, g, b) = Self::hex_to_rgb($hl_med);
                        rgb_u8(r, g, b)
                    },
                    Self::HighlightHigh => {
                        let (r, g, b) = Self::hex_to_rgb($hl_high);
                        rgb_u8(r, g, b)
                    },
                }
            }
        }
    };
}
