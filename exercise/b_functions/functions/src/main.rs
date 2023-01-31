// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width: i32 = 4;
    let height: i32 = 7;
    let depth: i32 = 10;

    let area: i32 = area_of(width, height);
    println!("Area is {}", area);

    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 {
    // return x * y;
    x*y /* Normally how it's done in Rust */
}

fn volume(width: i32, height: i32, depth: i32) -> i32 {
    return  width*height*depth;
}