
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirt_stock: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, up : Option<ShirtColor>) -> ShirtColor {
        up.unwrap_or_else(|| self.from_stocked())
    }
    fn from_stocked(&self) -> ShirtColor {
        let mut red_stock = 0;
        let mut blue_stock  = 0;

        for sc in self.shirt_stock.iter() {
            match sc {
                ShirtColor::Blue => blue_stock += 1,
                ShirtColor::Red => red_stock += 1,
            }
        }

        if red_stock > blue_stock {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn main() {
    let shirt_stock  = vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red];

    let inv = Inventory {
        shirt_stock: shirt_stock,
    };

    let up = Some(ShirtColor::Blue);
    let shirt_color = inv.giveaway(up);
    println!("user pref: {:?} and got shirt color: {:?}", up, shirt_color);

    let up = None;
    let shirt_color = inv.giveaway(up);
    println!("user pref: {:?} and got shirt color: {:?}", up, shirt_color);

}