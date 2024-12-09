#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EColor {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Cyan,
    White,
    Black,
    Uncolored,
}

impl EColor {
    pub fn to_string(&self) -> String {
        match self {
            EColor::Red => "r".to_string(),
            EColor::Green => "g".to_string(),
            EColor::Blue => "b".to_string(),
            EColor::Yellow => "y".to_string(),
            EColor::Purple => "p".to_string(),
            EColor::Cyan => "c".to_string(),
            EColor::White => "w".to_string(),
            EColor::Black => "k".to_string(),
            EColor::Uncolored => "u".to_string(),
        }
    }

    pub fn try_from_string(s: &str) -> Option<EColor> {
        match s.to_lowercase().as_str() {
            "r" => Some(EColor::Red),
            "g" => Some(EColor::Green),
            "b" => Some(EColor::Blue),
            "y" => Some(EColor::Yellow),
            "p" => Some(EColor::Purple),
            "c" => Some(EColor::Cyan),
            "w" => Some(EColor::White),
            "k" => Some(EColor::Black),
            "u" | "-" => Some(EColor::Uncolored),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EShape {
    Circle,
    Rectangle,
    Windmill,
    Star,
    Pin,
}

impl EShape {
    pub fn to_string(&self) -> String {
        match self {
            EShape::Circle => "C".to_string(),
            EShape::Rectangle => "R".to_string(),
            EShape::Windmill => "W".to_string(),
            EShape::Star => "S".to_string(),
            EShape::Pin => "P".to_string(),
        }
    }

    pub fn try_from_string(s: &str) -> Option<EShape> {
        match s.to_uppercase().as_str() {
            "C" => Some(EShape::Circle),
            "R" => Some(EShape::Rectangle),
            "W" => Some(EShape::Windmill),
            "S" => Some(EShape::Star),
            "P" => Some(EShape::Pin),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleItem {
    color: EColor,
    shape: EShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    items: [[Option<SingleItem>; 4]; 4],
}

impl Shape {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        // Find the highest non-empty layer
        let mut highest_non_empty_layer = None;
        for i in (0..4).rev() {
            if self.items[i].iter().any(|item| item.is_some()) {
                highest_non_empty_layer = Some(i);
                break;
            }
        }
        if let Some(max_layer) = highest_non_empty_layer {
            for i in 0..=max_layer {
                for j in 0..4 {
                    match &self.items[i][j] {
                        Some(item) => {
                            result.push_str(&item.shape.to_string());
                            result.push_str(&item.color.to_string());
                        }
                        None => {
                            result.push_str("--");
                        }
                    }
                }
                if i != max_layer {
                    result.push_str(":");
                }
            }
        }
        result
    }

    pub fn try_from_string(s: &str) -> Option<Shape> {
        let mut shape = Shape {
            items: [[None; 4]; 4],
        };

        let layer_strings: Vec<&str> = s.split(':').collect();

        if layer_strings.len() > 4 {
            return None;
        }

        for (layer_index, layer_str) in layer_strings.iter().enumerate() {
            if layer_str.len() != 8 {
                return None;
            }
            for quadrant_index in 0..4 {
                let code = &layer_str[quadrant_index * 2..quadrant_index * 2 + 2];
                if code == "--" {
                    shape.items[layer_index][quadrant_index] = None;
                } else {
                    let shape_code = &code[0..1];
                    let color_code = &code[1..2];

                    let shape_enum = EShape::try_from_string(shape_code)?;
                    let color_enum = EColor::try_from_string(color_code)?;

                    shape.items[layer_index][quadrant_index] = Some(SingleItem {
                        color: color_enum,
                        shape: shape_enum,
                    });
                }
            }
        }
        Some(shape)
    }

    pub fn to_shapez2_shape_viewer(&self) -> String {
        let binding = self.to_string();
        let encoded = urlencoding::encode(&binding);
        format!(
            "https://community-vortex.shapez2.com/shape?identifier={}&extend=false&expand=false",
            encoded
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_to_string() {
        assert_eq!(EColor::Red.to_string(), "r");
        assert_eq!(EColor::Green.to_string(), "g");
        assert_eq!(EColor::Blue.to_string(), "b");
        assert_eq!(EColor::Yellow.to_string(), "y");
        assert_eq!(EColor::Purple.to_string(), "p");
        assert_eq!(EColor::Cyan.to_string(), "c");
        assert_eq!(EColor::White.to_string(), "w");
        assert_eq!(EColor::Uncolored.to_string(), "u");
    }

    #[test]
    fn test_color_try_from_string() {
        assert_eq!(EColor::try_from_string("r"), Some(EColor::Red));
        assert_eq!(EColor::try_from_string("g"), Some(EColor::Green));
        assert_eq!(EColor::try_from_string("b"), Some(EColor::Blue));
        assert_eq!(EColor::try_from_string("y"), Some(EColor::Yellow));
        assert_eq!(EColor::try_from_string("p"), Some(EColor::Purple));
        assert_eq!(EColor::try_from_string("c"), Some(EColor::Cyan));
        assert_eq!(EColor::try_from_string("w"), Some(EColor::White));
        assert_eq!(EColor::try_from_string("u"), Some(EColor::Uncolored));
        assert_eq!(EColor::try_from_string("x"), None);
    }

    #[test]
    fn test_shape_to_string() {
        let shape = Shape {
            items: [
                [
                    Some(SingleItem {
                        color: EColor::Red,
                        shape: EShape::Circle,
                    }),
                    Some(SingleItem {
                        color: EColor::Green,
                        shape: EShape::Rectangle,
                    }),
                    None,
                    None,
                ],
                [
                    None,
                    None,
                    Some(SingleItem {
                        color: EColor::Blue,
                        shape: EShape::Windmill,
                    }),
                    None,
                ],
                [
                    None,
                    None,
                    None,
                    Some(SingleItem {
                        color: EColor::Yellow,
                        shape: EShape::Star,
                    }),
                ],
                [None, None, None, None],
            ],
        };

        assert_eq!(
            shape.to_string(),
            "CrRg----:----Wb--:------Sy".to_string(),
        );
    }

    #[test]
    fn test_shape_try_from_string() {
        let shape = Shape {
            items: [
                [
                    Some(SingleItem {
                        color: EColor::Red,
                        shape: EShape::Circle,
                    }),
                    Some(SingleItem {
                        color: EColor::Green,
                        shape: EShape::Rectangle,
                    }),
                    None,
                    None,
                ],
                [
                    None,
                    None,
                    Some(SingleItem {
                        color: EColor::Blue,
                        shape: EShape::Windmill,
                    }),
                    None,
                ],
                [
                    None,
                    None,
                    None,
                    Some(SingleItem {
                        color: EColor::Yellow,
                        shape: EShape::Star,
                    }),
                ],
                [None, None, None, None],
            ],
        };

        assert_eq!(
            Shape::try_from_string("CrRg----:----Wb--:------Sy"),
            Some(shape)
        );
    }
}
