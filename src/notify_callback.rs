use std::{thread, time};

// Define a function that takes a callback (aka closure or functions) as input.
// The callback is defined by generic type F, which needs to satisfy the Fn(i32) signature.
// Our function then pretends to do some complicated calculations and periodically triggers
// the callback with the current state of progress.
fn process<F>(notify_progress_callback: F)
where
    F: Fn(i32),
{
    let one_second = time::Duration::from_secs(1);
    for progress_percentage in (0..101).step_by(10) {
        /*
            Pretend to do a lot of complicated calculations
        */
        thread::sleep(one_second);

        notify_progress_callback(progress_percentage);
    }
}

#[allow(dead_code)]
pub fn notify_callback_example() {
    // Define a closure that handles the current progress percentage by simply printing it out.
    let notify_callback = |progress_percentage: i32| println!("{progress_percentage}%");

    process(notify_callback);
}
