trait property {
    fn get_property(&self) -> f32;
}

struct S1 {
    val: f32,
}

impl S1 {
    fn get_val(&self) -> f32 {
        return self.val;
    }
}

impl property for S1 {
    fn get_property(&self) -> f32 {
        return 0.0;
    }
}

struct S2 {
    val: f32,
}
impl S2 {
    fn get_val(&self) -> f32 {
        return self.val;
    }
}
impl property for S2 {
    fn get_property(&self) -> f32 {
        return 0.0;
    }
}

fn main() {
    let s2 = S2 { val: 10.0 };
    let s1 = S1 { val: 20.0 };
    let mut A: Box<dyn property> = Box::new(s1);
}
