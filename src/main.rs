#[derive(Debug, Clone, Copy)]
pub enum EColor {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Cyan,
    White,
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
            EColor::Uncolored => "u".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EShape {
    Circle,
    Square,
    Windmill,
    Star,
}

impl EShape {
    pub fn to_string(&self) -> String {
        match self {
            EShape::Circle => "o".to_string(),
            EShape::Square => "s".to_string(),
            EShape::Windmill => "w".to_string(),
            EShape::Star => "t".to_string(),
        }
    }

    pub fn new_from_string(s: &str) -> EShape {
        match s {
            "o" => EShape::Circle,
            "s" => EShape::Square,
            "w" => EShape::Windmill,
            "t" => EShape::Star,
            _ => panic!("Invalid shape string"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SingleItem {
    color: EColor,
    shape: EShape,
}

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    items: [[Option<SingleItem>; 4]; 4],
}

impl Shape {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for i in 0..4 {
            for j in 0..4 {
                match &self.items[i][j] {
                    Some(item) => {
                        result.push_str(&item.color.to_string());
                        result.push_str(&item.shape.to_string());
                    }
                    None => {
                        result.push_str("--");
                    }
                }
            }
        }
        result
    }
    
}

fn main() {
    let mut shape = Shape {
        items: [[None; 4]; 4],
    };
    shape.items[0][0] = Some(SingleItem {
        color: EColor::Red,
        shape: EShape::Circle,
    });
    shape.items[0][1] = Some(SingleItem {
        color: EColor::Green,
        shape: EShape::Square,
    });
    shape.items[0][2] = Some(SingleItem {
        color: EColor::Blue,
        shape: EShape::Windmill,
    });
    shape.items[0][3] = Some(SingleItem {
        color: EColor::Yellow,
        shape: EShape::Star,
    });
    println!("{}", shape.to_string());
}

