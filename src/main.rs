mod password_generator;

fn main() {
    let a = password_generator::generate(24);

    println!("{}", a);
}
