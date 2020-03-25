use std::fmt;

#[derive(Debug)]
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

#[derive(Debug)]
enum TupleVect {
    Heart(&'static str),
    Diamond(&'static str,&'static str),
    Spade(&'static str,&'static str,&'static str),
    Club(&'static str,&'static str,&'static str,&'static str)
}


impl fmt::Display for Suit {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Heart => write!(f, "my heart"),
            Suit::Diamond => write!(f, "my diamond"),
            Suit::Spade => write!(f, "my spade"),
            Suit::Club => write!(f, "my club")
        }
    }
}
fn main() {
    let heart = Suit::Heart;
    println!("{}", heart);
    let value = TupleVect::Heart("abc");

    let vecs = vec![
        TupleVect::Heart("abc"),
        TupleVect::Diamond("abc","def"),
        TupleVect::Spade("abc","def", "high"),
        TupleVect::Club("abc","def","hi", "opq"),
    ];
    if let  TupleVect::Heart(a) = value {
        println!("{} ", a);
    };

    for i in vecs {
        match &i {
            TupleVect::Heart(a) =>  println!("{}", a),
            TupleVect::Diamond(a, b) =>  println!("{} {}", a, b),
            TupleVect::Spade(a, b, c) =>  println!("{} {} {}", a, b, c),
            TupleVect::Club(a, b, c, d) =>  println!("{} {} {} {} ", a, b, c, d),
        }

    }

}
