use crate::solution::brain_luck;

pub mod solution;
pub mod tests;

fn main() {
    println!(
        "{:?}",
        brain_luck(",>,<[>[->+>+<<]>[-<<-[>]>>>[<[-<->]<[>]>>[[-]>>+<]>-<]<<]>>>+<<[-<<+>>]<<<]>>>>>[-<<<<<+>>>>>]<<<<<.", vec![12, 2])
    )
}
