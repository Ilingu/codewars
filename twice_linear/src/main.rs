use solution::dbl_linear;

mod solution;
mod tests;

fn main() {
    for (i, u) in dbl_linear(100).iter().enumerate() {
        println!("{i} {u}");
    }
}
