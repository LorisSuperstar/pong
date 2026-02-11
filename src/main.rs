use std::sync::WaitTimeoutResult;

slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();

    let weak = ui.as_weak();

    let x_border_left: f32 = 0.0;
    let y_border_upp: f32 = 0.0;
    let x_border_right: f32 = 1450.0;
    let y_border_bottom: f32 = 950.0;

    ui.on_button_clicked(move || {
        let ui:MainWindow = weak.upgrade().unwrap();
       
    });
    

    ui.run().unwrap();
}
