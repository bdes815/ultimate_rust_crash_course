// Silence some warnings that could distract from the exercise
#![allow(unused_variables, unused_mut, dead_code)]

// Someone is shooting arrows at a target. Here are the classification of the shots
enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot {
    fn points(self) -> i32 {
        // match self {
        //     Self::Bullseye => return 5,
        //     Self::Hit(num) => {
        //         if num > 3.0 {
        //             return 2;
        //         } else {
        //             return 1;
        //         }
        //     }
        //     Self::Miss => return 0,
        // }
        match self {
            Self::Bullseye => 5,
            Self::Hit(num) if num < 3.0 => 2,
            Self::Hit(num) => 1, 
            Self::Miss => 0,
        }
    }
}

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    // for coordinate in arrow_coords {
    //     coordinate.print_description();
    //     let dist = coordinate.distance_from_center();
    //     if dist <= 1.0 {
    //         shots.push(Shot::Bullseye);
    //     } else if dist <= 5.0 {
    //         shots.push(Shot::Hit(dist))
    //     } else {
    //         shots.push(Shot::Miss);
    //     }
    // }

    for coordinate in arrow_coords {
        coordinate.print_description();
        let shot = match coordinate.distance_from_center() {
            x if x < 1.0 => Shot::Bullseye,
            x if x < 5.0 => Shot::Hit(x),
            _ => Shot::Miss,
        };
        shots.push(shot);
        
    }

    let mut total = 0;
    for shot in shots {
        total += shot.points();
    }

    println!("Final point total is: {}", total);
}

// A coordinate of where an Arrow hit
#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y
        );
    }
}

// Generate some random coordinates
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}
