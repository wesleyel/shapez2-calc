use rand::prelude::Distribution;

pub const SHAPEZ2_DEMENTION: usize = 4;
pub const SHAPEZ2_LAYER: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EColor {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
    Black,
    Uncolored,
    Empty,
}

impl EColor {
    pub fn to_string(&self) -> String {
        match self {
            EColor::Red => "r".to_string(),
            EColor::Green => "g".to_string(),
            EColor::Blue => "b".to_string(),
            EColor::Yellow => "y".to_string(),
            EColor::Magenta => "m".to_string(),
            EColor::Cyan => "c".to_string(),
            EColor::White => "w".to_string(),
            EColor::Black => "k".to_string(),
            EColor::Uncolored => "u".to_string(),
            EColor::Empty => "-".to_string(),
        }
    }

    pub fn try_from_string(s: &str) -> Option<EColor> {
        match s.to_lowercase().as_str() {
            "r" => Some(EColor::Red),
            "g" => Some(EColor::Green),
            "b" => Some(EColor::Blue),
            "y" => Some(EColor::Yellow),
            "m" => Some(EColor::Magenta),
            "c" => Some(EColor::Cyan),
            "w" => Some(EColor::White),
            "k" => Some(EColor::Black),
            "u" => Some(EColor::Uncolored),
            "-" => Some(EColor::Empty),
            _ => None,
        }
    }
}

impl Distribution<EColor> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> EColor {
        match rng.gen_range(0..=9) {
            0 => EColor::Red,
            1 => EColor::Green,
            2 => EColor::Blue,
            3 => EColor::Yellow,
            4 => EColor::Magenta,
            5 => EColor::Cyan,
            6 => EColor::White,
            7 => EColor::Black,
            8 => EColor::Uncolored,
            _ => EColor::Empty,
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
    Empty,
}

impl EShape {
    pub fn to_string(&self) -> String {
        match self {
            EShape::Circle => "C".to_string(),
            EShape::Rectangle => "R".to_string(),
            EShape::Windmill => "W".to_string(),
            EShape::Star => "S".to_string(),
            EShape::Pin => "P".to_string(),
            EShape::Empty => "-".to_string(),
        }
    }

    pub fn try_from_string(s: &str) -> Option<EShape> {
        match s.to_uppercase().as_str() {
            "C" => Some(EShape::Circle),
            "R" => Some(EShape::Rectangle),
            "W" => Some(EShape::Windmill),
            "S" => Some(EShape::Star),
            "P" => Some(EShape::Pin),
            "-" => Some(EShape::Empty),
            _ => None,
        }
    }
}

impl Distribution<EShape> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> EShape {
        match rng.gen_range(0..=4) {
            0 => EShape::Circle,
            1 => EShape::Rectangle,
            2 => EShape::Windmill,
            3 => EShape::Star,
            _ => EShape::Pin,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleItem {
    color: EColor,
    shape: EShape,
}

impl SingleItem {
    pub fn new() -> SingleItem {
        SingleItem {
            color: EColor::Empty,
            shape: EShape::Empty,
        }
    }

    pub fn to_string(&self) -> String {
        match (self.shape, self.color) {
            (EShape::Empty, EColor::Empty) => "--".to_string(),
            _ => format!("{}{}", self.shape.to_string(), self.color.to_string()),
        }
    }

    pub fn try_from_string(s: &str) -> Option<SingleItem> {
        if s.len() != 2 {
            return None;
        }
        let shape_code = &s[0..1];
        let color_code = &s[1..2];

        Some(SingleItem {
            shape: EShape::try_from_string(shape_code)?,
            color: EColor::try_from_string(color_code)?,
        })
    }
}

impl Distribution<SingleItem> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> SingleItem {
        SingleItem {
            color: rng.gen(),
            shape: rng.gen(),
        }
    }
}
/// Shape is a 4x4 matrix of SingleItem
///
/// ```plaintext
///  Layer 0
///   3 | 0
///   -----
///   2 | 1
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    pub items: [[SingleItem; SHAPEZ2_DEMENTION]; SHAPEZ2_LAYER],
}

impl Shape {
    pub fn new() -> Shape {
        Shape {
            items: [[SingleItem::new(); SHAPEZ2_DEMENTION]; SHAPEZ2_LAYER],
        }
    }

    pub fn new_random() -> Shape {
        let mut shape = Shape::new();
        let shape_layer = rand::random::<usize>() % SHAPEZ2_LAYER;
        for i in 0..=shape_layer {
            for j in 0..SHAPEZ2_DEMENTION {
                shape.items[i][j] = match rand::random::<usize>() % 2 {
                    0 => SingleItem {
                        color: EColor::Empty,
                        shape: EShape::Empty,
                    },
                    _ => SingleItem {
                        color: rand::random(),
                        shape: rand::random(),
                    },
                };

                // if the shape is not empty, the color should not be empty
                if shape.items[i][j].shape != EShape::Empty {
                    while shape.items[i][j].color == EColor::Empty {
                        shape.items[i][j].color = rand::random();
                    }
                }
            }
        }
        shape
    }

    pub fn to_string(&self) -> String {
        let mut result = self.to_raw_string();
        const EMPTY_LAYER: &str = ":--------";
        const EMPTY_ITEM: &str = "--";
        // search from right to left, when meet ":--------" remove it
        loop {
            if result.ends_with(EMPTY_LAYER) {
                result = result[0..result.len() - EMPTY_LAYER.len()].to_string();
            } else {
                break;
            }
        }
        // if : at the end, remove it
        if result.ends_with(":") {
            result = result[0..result.len() - 1].to_string();
        }
        // if : not present, search from left to right, when meet "--" remove it
        if !result.contains(":") {
            loop {
                if result.ends_with(EMPTY_ITEM) {
                    result = result[0..result.len() - EMPTY_ITEM.len()].to_string();
                } else {
                    break;
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_color_to_string() {
        assert_eq!(EColor::Red.to_string(), "r");
        assert_eq!(EColor::Green.to_string(), "g");
        assert_eq!(EColor::Blue.to_string(), "b");
        assert_eq!(EColor::Yellow.to_string(), "y");
        assert_eq!(EColor::Magenta.to_string(), "m");
        assert_eq!(EColor::Cyan.to_string(), "c");
        assert_eq!(EColor::White.to_string(), "w");
        assert_eq!(EColor::Uncolored.to_string(), "u");
        assert_eq!(EColor::Empty.to_string(), "-");
    }

    #[test]
    fn test_color_try_from_string() {
        assert_eq!(EColor::try_from_string("r"), Some(EColor::Red));
        assert_eq!(EColor::try_from_string("g"), Some(EColor::Green));
        assert_eq!(EColor::try_from_string("b"), Some(EColor::Blue));
        assert_eq!(EColor::try_from_string("y"), Some(EColor::Yellow));
        assert_eq!(EColor::try_from_string("m"), Some(EColor::Magenta));
        assert_eq!(EColor::try_from_string("c"), Some(EColor::Cyan));
        assert_eq!(EColor::try_from_string("w"), Some(EColor::White));
        assert_eq!(EColor::try_from_string("u"), Some(EColor::Uncolored));
        assert_eq!(EColor::try_from_string("-"), Some(EColor::Empty));
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
            color: EColor::Red,
            shape: EShape::Circle,
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
            color: EColor::Red,
            shape: EShape::Circle,
        };
        assert_eq!(shape.to_string(), "Cr".to_string());
    }

    #[test]
    fn test_shape_try_from_string() {
        let s1 = SingleItem {
            color: EColor::Red,
            shape: EShape::Circle,
        };
        let s2 = SingleItem {
            color: EColor::Green,
            shape: EShape::Rectangle,
        };
        let s3 = SingleItem {
            color: EColor::Empty,
            shape: EShape::Empty,
        };
        let shape = Shape {
            items: [[s1, s2, s3, s3]; SHAPEZ2_LAYER],
        };

        assert_eq!(
            Shape::try_from_string("CrRg----:CrRg----:CrRg----:CrRg----"),
            Some(shape)
        );
    }

    #[test]
    fn test_new_random_shape() {
        let shape = Shape::new_random();
        assert_eq!(
            shape.to_raw_string().len(),
            2 * SHAPEZ2_LAYER * SHAPEZ2_DEMENTION + SHAPEZ2_LAYER - 1
        );
    }

    #[test]
    fn test_ramdom_shape_loopback_raw_string() {
        let shape = Shape::new_random();
        let shape_str = shape.to_raw_string();
        assert_eq!(Shape::try_from_string(&shape_str), Some(shape));
    }

    #[test]
    fn test_ramdom_shape_loopback_minify_string() {
        let shape = Shape::new_random();
        let shape_str = shape.to_string();
        assert_eq!(Shape::try_from_string(&shape_str), Some(shape));
    }
}
