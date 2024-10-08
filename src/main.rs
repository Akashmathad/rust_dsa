mod rod_cutting;
mod lc_2696;

fn main() {
    let s:String = String::from("ACBBD");
    println!("{}", lc_2696::min_length(s));
}
