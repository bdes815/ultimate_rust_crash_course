/*In here the const have a global scope */
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // const STARTING_MISSILES: i32 = 8;
    // const READY_AMOUNT: i32 = 2;
    // let mut missiles: i32 = STARTING_MISSILES;
    // let ready: i32 = READY_AMOUNT;
    let _a: i32 = 2; /* Using just a will make the compiler throw a warning */
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready; /* Without the mutable in missiles it fails */
    println!("{} missiles left...", missiles);
}
