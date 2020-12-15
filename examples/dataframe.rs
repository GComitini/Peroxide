#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;
use peroxide::structure::dataframe::*;

fn main() {
    let x = c!(1,2,3,4);
    let a = Series::new(x);
    a.print();
    println!("");

    let s = a.at(0);
    s.print();
    println!("");

    let b = Series::new(vec!['a', 'b', 'c', 'd']);

    let mut df = DataFrame::new(vec![a, b]);

    df.print();
    println!("");

    df["1"] = Series::new(c!(5,6,7,8));

    df.print();
    println!("");

    df.push("a", Series::new(vec!['a', 'b', 'c', 'd']));

    df.print();
    println!("");
    df.row(1).print();
    println!("");

    let ch: char = df.row(1)["a"].at(0).unwrap();
    ch.print();

    df[0].as_type(DType::USIZE);

    df.print();
}
