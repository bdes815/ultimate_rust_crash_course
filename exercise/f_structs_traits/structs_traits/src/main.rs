trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

#[derive(Debug)] // include this line right before your struct definition
struct Grapes {
    grapes_left: u32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.grapes_left -= 1;
    }
}

fn main() {
    let mut carrot = Carrot { percent_left: 100.0 };

    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { grapes_left: 100 };

    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);

    bunny_nibbles(&mut grapes);
    println!("Bunny nibbles for awhile: {:?}", grapes);
}

fn bunny_nibbles<T: Bite>(biteable: &mut T) {
    for bites in 0..4 {
        biteable.bite();
        println!("Bite number {}", bites);
    }
}