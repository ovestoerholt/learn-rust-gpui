use gpui::{Global, Rgba, rgb};

/// Material Design-inspired theme system with semantic color names
#[derive(Clone, Debug)]

#[allow(dead_code)]
pub struct Theme {
    // Surface colors (backgrounds)
    pub surface_primary: Rgba,      // Main app background
    pub surface_secondary: Rgba,    // Card/panel backgrounds
    pub surface_tertiary: Rgba,     // Nested components
    pub surface_elevated: Rgba,     // Elevated elements (dialogs, menus)
    
    // Text colors
    pub text_primary: Rgba,         // Main text
    pub text_secondary: Rgba,       // Supporting text
    pub text_disabled: Rgba,        // Disabled text
    pub text_on_accent: Rgba,       // Text on accent backgrounds
    
    // Accent/brand colors
    pub accent_primary: Rgba,       // Main brand color
    pub accent_secondary: Rgba,     // Secondary accent
    pub accent_hover: Rgba,         // Hover state for accent elements
    
    // Semantic colors
    pub success: Rgba,              // Success states
    pub warning: Rgba,              // Warning states
    pub error: Rgba,                // Error states
    pub info: Rgba,                 // Info states
    
    // Border colors
    pub border_subtle: Rgba,        // Subtle borders/dividers
    pub border_strong: Rgba,        // Strong borders
}

impl Global for Theme {}

impl Theme {
    #[allow(dead_code)]
    pub fn light() -> Self {
        Self {
            // Light surfaces
            surface_primary: rgb(0xFFFFFF),      // Pure white
            surface_secondary: rgb(0xF5F5F5),    // Light gray
            surface_tertiary: rgb(0xEEEEEE),     // Slightly darker
            surface_elevated: rgb(0xFFFFFF),     // White with shadow
            
            // Light text
            text_primary: rgb(0x1A1A1A),         // Near black
            text_secondary: rgb(0x666666),       // Gray
            text_disabled: rgb(0xAAAAAA),        // Light gray
            text_on_accent: rgb(0xFFFFFF),       // White
            
            // Accents
            accent_primary: rgb(0x1976D2),       // Material Blue 700
            accent_secondary: rgb(0x388E3C),     // Material Green 700
            accent_hover: rgb(0x1565C0),         // Material Blue 800
            
            // Semantic
            success: rgb(0x4CAF50),              // Material Green
            warning: rgb(0xFF9800),              // Material Orange
            error: rgb(0xF44336),                // Material Red
            info: rgb(0x2196F3),                 // Material Blue
            
            // Borders
            border_subtle: rgb(0xE0E0E0),        // Light border
            border_strong: rgb(0xBDBDBD),        // Medium border
        }
    }

    #[allow(dead_code)]
    pub fn dark() -> Self {
        Self {
            // Dark surfaces
            surface_primary: rgb(0x121212),      // Material dark background
            surface_secondary: rgb(0x1E1E1E),    // Elevated background
            surface_tertiary: rgb(0x2A2A2A),     // Card background
            surface_elevated: rgb(0x2C2C2C),     // Dialog background
            
            // Dark text
            text_primary: rgb(0xFFFFFF),         // White
            text_secondary: rgb(0xB3B3B3),       // Light gray
            text_disabled: rgb(0x666666),        // Darker gray
            text_on_accent: rgb(0xFFFFFF),       // White
            
            // Accents
            accent_primary: rgb(0x90CAF9),       // Material Blue 200
            accent_secondary: rgb(0xA5D6A7),     // Material Green 200
            accent_hover: rgb(0xBBDEFB),         // Material Blue 100
            
            // Semantic
            success: rgb(0x81C784),              // Material Green 300
            warning: rgb(0xFFB74D),              // Material Orange 300
            error: rgb(0xE57373),                // Material Red 300
            info: rgb(0x64B5F6),                 // Material Blue 300
            
            // Borders
            border_subtle: rgb(0x383838),        // Subtle dark border
            border_strong: rgb(0x4F4F4F),        // Strong dark border
        }
    }
}
