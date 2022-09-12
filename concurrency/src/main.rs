use std::thread;
use std::time::Duration;

fn main() {
    let vec = vec![1, 2, 3];
    let handles = (1..10).map(|el| {
        thread::spawn(move || {
            for i in 1..10 {
                println!("Thread number: {i}, {:#?}", vec);
                thread::sleep(Duration::from_millis(1));
            }
        })
    });

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handles.for_each(|handle| handle.join().unwrap());

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();

    println!("After calling closure: {:?}", list);
}
