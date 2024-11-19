pub fn run() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Task 2 - Tracker: {}", tracker);
    };
    update();
    update();
}
