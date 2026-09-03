use std::thread;
use std::time::Duration;

pub fn sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

pub fn sleep_secs(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}

pub fn spawn<F>(f: F) -> thread::JoinHandle<()>
where
    F: FnOnce() + Send + 'static,
{
    thread::spawn(f)
}

pub fn spawn_with_name<F>(name: &str, f: F) -> thread::JoinHandle<()>
where
    F: FnOnce() + Send + 'static,
{
    thread::Builder::new()
        .name(name.to_string())
        .spawn(f)
        .unwrap()
}

pub fn current_thread_id() -> u64 {
    let id = thread::current().id();
    format!("{:?}", id).bytes().fold(0, |acc, b| acc ^ b as u64)
}

pub fn is_main_thread() -> bool {
    thread::current().name() == Some("main")
}

pub fn yield_now() {
    thread::yield_now();
}

pub fn park() {
    thread::park();
}

pub fn park_timeout(ms: u64) {
    thread::park_timeout(Duration::from_millis(ms));
}

pub fn unpark(thread: &thread::Thread) {
    thread.unpark();
}
