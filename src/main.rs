use minreq::get;
use std::{thread::sleep, time::Duration};

const URL: &str = "http://rust-lang.org";
const ONE_MINUTE: Duration = Duration::from_secs(60);

fn main() {
    loop_get(URL, ONE_MINUTE);
}

fn loop_get(url: &str, dur: Duration) {
    let mut i = 0;

    loop {
        i += 1;
        println!("{} GET {}", i, url);
        let _ = get(url).send();
        sleep(dur);
    }
}
