pub struct Auxil {
    pub name: char,
    pub ratio: f64
    }

impl Auxil {
    pub const fn new(name: char, ratio: f64) -> Self {
        Auxil { name, ratio }
        }
    }