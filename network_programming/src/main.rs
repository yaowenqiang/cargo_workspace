extern crate term;
fn main() {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::GREEN).unwrap();
    write!(t, "Hello, ").unwrap();
    t.fg(term::color::RED).unwrap();
    writeln!(t," World!").unwrap();
    t.reset().unwrap();
}
