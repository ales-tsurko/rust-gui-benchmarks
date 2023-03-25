use std::time::Instant;
use once_cell::sync::OnceCell;

use slint::ComponentHandle;

static INSTANCE: OnceCell<Instant> = OnceCell::new();

slint::include_modules!();
fn main() {
    let component = MainWindow::new();
    let _ = component.as_weak();
    component.on_start(move || {
        println!("start");
        INSTANCE.set(Instant::now()).unwrap();
    });
    component.on_stop(move || {
        let time = INSTANCE.get().unwrap().elapsed().as_nanos();
        println!("Time: {} ns", time);
    });

    component.run();
}