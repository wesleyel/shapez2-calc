const SHAPEZ2_DEMENTION: usize = 4;
const SHAPEZ2_LAYER: usize = 4;

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
            "u" => Some(EColor::Uncolored),
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
    color: Option<EColor>,
    shape: Option<EShape>,
}

impl SingleItem {
    pub fn to_string(&self) -> String {
        match (self.shape, self.color) {
            (Some(shape), Some(color)) => format!("{}{}", shape.to_string(), color.to_string()),
            (Some(EShape::Pin), None) => format!("{}-", EShape::Pin.to_string()),
            _ => "--".to_string(),
        }
    }

    pub fn try_from_string(s: &str) -> Option<SingleItem> {
        if s.len() != 2 {
            return None;
        }
        let shape_code = &s[0..1];
        let color_code = &s[1..2];

        Some(SingleItem {
            shape: EShape::try_from_string(shape_code),
            color: EColor::try_from_string(color_code),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    items: [[SingleItem; SHAPEZ2_DEMENTION]; SHAPEZ2_LAYER],
}

impl Shape {
    pub fn new() -> Shape {
        Shape {
            items: [[SingleItem {
                color: None,
                shape: None,
            }; SHAPEZ2_DEMENTION]; SHAPEZ2_LAYER],
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = self.to_raw_string();
        const EMPTY_LAYER: &str = ":--------";
        const EMPTY_ITEM: &str = "--";
        // search from right to left, when meet ":--------" remove it
        let mut index = result.len();
        while index > 2 * SHAPEZ2_DEMENTION {
            if &result[index - EMPTY_LAYER.len()..index] == EMPTY_LAYER {
                result = result[0..index - EMPTY_LAYER.len()].to_string();
                index -= EMPTY_LAYER.len();
            } else {
                index -= 1;
            }
        }
        // if : at the end, remove it
        if result.ends_with(":") {
            result = result[0..result.len() - 1].to_string();
        }
        // if : not present, search from left to right, when meet "--" remove it
        if !result.contains(":") {
            let mut index = 0;
            while index < result.len() {
                if &result[index..index + EMPTY_ITEM.len()] == EMPTY_ITEM {
                    result = result[0..index].to_string();
                    break;
                } else {
                    index += 1;
                }
            }
        }
        result
    }

    pub fn to_raw_string(&self) -> String {
        let mut result = String::new();
        for i in 0..SHAPEZ2_LAYER {
            for j in 0..SHAPEZ2_DEMENTION {
                result.push_str(&self.items[i][j].to_string());
            }
            if i != SHAPEZ2_LAYER - 1 {
                result.push_str(":");
            }
        }
        result
    }

    pub fn try_from_string(s: &str) -> Option<Shape> {
        let mut shape = Shape::new();

        let layer_strings: Vec<&str> = s.split(':').collect();

        if layer_strings.len() > SHAPEZ2_LAYER {
            return None;
        }

        for (layer_index, layer_str) in layer_strings.iter().enumerate() {
            if layer_str.len() != SHAPEZ2_DEMENTION * 2 {
                return None;
            }
            for quadrant_index in 0..SHAPEZ2_DEMENTION {
                let code = &layer_str[quadrant_index * 2..quadrant_index * 2 + 2];
                if let Some(item) = SingleItem::try_from_string(code) {
                    shape.items[layer_index][quadrant_index] = item;
                } else {
                    return None;
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
    fn test_shape_to_raw_string() {
        let mut shape = Shape::new();
        assert_eq!(
            shape.to_raw_string(),
            "--------:--------:--------:--------".to_string()
        );

        shape.items[0][0] = SingleItem {
            color: Some(EColor::Red),
            shape: Some(EShape::Circle),
        };
        assert_eq!(
            shape.to_raw_string(),
            "Cr------:--------:--------:--------".to_string()
        );
    }

    #[test]
    fn test_shape_to_string() {
        let mut shape = Shape::new();
        assert_eq!(shape.to_string(), "".to_string());

        shape.items[0][0] = SingleItem {
            color: Some(EColor::Red),
            shape: Some(EShape::Circle),
        };
        assert_eq!(shape.to_string(), "Cr".to_string());
    }

    #[test]
    fn test_shape_try_from_string() {
        let s1 = SingleItem {
            color: Some(EColor::Red),
            shape: Some(EShape::Circle),
        };
        let s2 = SingleItem {
            color: Some(EColor::Green),
            shape: Some(EShape::Rectangle),
        };
        let s3 = SingleItem {
            color: None,
            shape: None,
        };
        let shape = Shape {
            items: [[s1, s2, s3, s3]; SHAPEZ2_LAYER],
        };

        assert_eq!(
            Shape::try_from_string("CrRg----:CrRg----:CrRg----:CrRg----"),
            Some(shape)
        );
    }
}
