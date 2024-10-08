mod rod_cutting;

fn main() {
    let price = [1, 5, 8, 9, 10, 17, 17, 20];
    println!("{}", rod_cutting::rod_cutting(price.to_vec(), 8));
}
