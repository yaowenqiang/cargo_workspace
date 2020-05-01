trait Sawtoolth {
    fn sawtooth(&self) -> Self;
}

impl Sawtoolth for f64 {
    fn sawtooth(&self) -> f64 {
        self - self.floor()
    }
}

fn main() {
    println!("{}", 2.34f64.sawtooth());
}
