use std::sync::{OnceLock, RwLock, RwLockReadGuard};
use std::thread;
use std::time::Duration;
use slint::{Color, SharedString};

// Using the expanded macro, so the IDE can find and parse the generated source.
include!(env!("SLINT_INCLUDE_GENERATED"));

unsafe impl Send for MainWindow {}

unsafe impl Sync for MainWindow {}

pub(crate) fn main_window() -> RwLockReadGuard<'static, MainWindow> {
    static MAIN_WINDOW: OnceLock<RwLock<MainWindow>> = OnceLock::new();
    let try_lock = MAIN_WINDOW.get_or_init(|| {
        RwLock::new(MainWindow::new().unwrap())
    })
        .read();

    if let Ok(try_lock) = try_lock {
        try_lock
    } else {
        let err = try_lock.err().unwrap();
        panic!("Main Window locking error: {}", err);
    }
}

fn animation_update_timer() {
    const ANIMATION_UPDATE_INTERVAL: u64 = 100;

    thread::spawn(move || {
        let duration = Duration::from_millis(ANIMATION_UPDATE_INTERVAL);

        let mut i: i32 = 0;
        loop {
            if i == 0 {
                thread::sleep(Duration::from_secs(1));
            }
            // fast_tick.recv().unwrap();
            thread::sleep(duration);
            slint::invoke_from_event_loop(move || unsafe {
                main_window().set_killer_text(SharedString::from(bug_text(i)));
                main_window().set_text_color(Color::from_argb_u8(255, 255, 0, 0));
            }).expect("Couldn't call main UI thread");

            i += 1;
            println!("Text {}", i);
        }
    });
}

fn bug_text(mut i: i32) -> String {
    let result = if i > 120 {
        format!("The injustice of the dark bug! {}", i)
    } else if i > 100 {
        format!("The dark bug rises the windows to the front! {}", i)
    } else if i > 80 {
        format!("The dark bug! {}", i)
    } else if i > 60 {
        format!("Bug continues! {}", i)
    } else if i > 40 {
        format!("Bugs 4 eva! {}", i)
    } else if i > 20 {
        format!("Bug returns! {}", i)
    } else {
        format!("Holy Bugfest Batman! {}", i)
    };
    format!("{}\nI challenge you: Push the window to the background and let it stay there!", result)
}

fn main() {
    animation_update_timer();

    main_window().run().unwrap();
}
