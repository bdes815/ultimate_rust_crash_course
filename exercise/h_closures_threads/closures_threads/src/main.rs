// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam_channel::unbounded;
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    // In the closures for both the .filter() and .map() the argument will be a reference, so you'll
    // either need to dereference the argument once in the parameter list like this: `|&x|` or you
    // will need to dereference it each time you use it in the expression like this: `*x`
    v.iter()
        .filter(|x| *x % 2 == 0) //keeps every even number
        .map(|&x| x.pow(2))
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    let handle = thread::spawn(||{expensive_sum(my_vector)});

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }

    let sum = handle.join().unwrap_or_default();//join waits for the thread to finish, and unwrap gets the result from it
    println!("The child thread's expensive sum is {}", sum);


    let (tx, rx) = crossbeam_channel::unbounded();
    // Cloning just the sender of the channels makes both senders have the same receiver
    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        pause_ms(300);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed. A Receiver channel is automatically closed once all Sender channels have been
    // closed. Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    // Join the child threads for good hygiene.
    handle_a.join().unwrap();
    handle_b.join().unwrap();

    let (tx_c, rx_c) = crossbeam_channel::unbounded();
    let (tx_c2, rx_c2) = crossbeam_channel::unbounded();

    let handle_c = thread::spawn(move || {
        for msg in rx_c {
            println!("handle_c thread: Received {}", msg);
        }
    });

    let handle_d = thread::spawn(move || {
        for msg in rx_c2 {
            println!("handle_d thread: Received {}", msg);
        }
    });

    for letter in vec!["a", "b", "c", "d", "e", "f", "g", "h", "i"] {
        pause_ms(200);
        println!("Main thread: Sending {}", letter);
        tx_c.send(letter).unwrap();
        tx_c2.send(letter).unwrap();
        println!("Main thread: Sent {}", letter);
    }
    drop(tx_c);
    drop(tx_c2);

    handle_c.join().unwrap();
    handle_d.join().unwrap();

    println!("Main thread: Exiting.")
}
