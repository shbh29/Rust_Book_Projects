
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}



struct Inventory {
    shirtStock: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, up : Option<ShirtColor>) -> ShirtColor {
        up.unwrap_or_else(|| self.fromStocked())
    }
    fn fromStocked(&self) -> ShirtColor {
        let mut redStock = 0;
        let mut blueStock  = 0;

        for sc in self.shirtStock.iter() {
            match sc {
                ShirtColor::Blue => blueStock += 1,
                ShirtColor::Red => redStock += 1,
            }
        }

        if redStock > blueStock {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn main() {
    let shirtStock  = vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red];

    let inv = Inventory {
        shirtStock: shirtStock,
    };

    let up = Some(ShirtColor::Blue);
    let shirtColor = inv.giveaway(up);
    println!("user pref: {:?} and got shirt color: {:?}", up, shirtColor);

    let up = None;
    let shirtColor = inv.giveaway(up);
    println!("user pref: {:?} and got shirt color: {:?}", up, shirtColor);

}