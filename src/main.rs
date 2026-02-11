use rand::RngExt;
use slint::ComponentHandle;

slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();

    let timer = slint::Timer::default();
    let weak = ui.as_weak();

    let _x_border_left: f32 = 0.0;
    let y_border_top: f32 = 0.0;
    let x_border_right: f32 = 1450.0;
    let y_border_bottom: f32 = 950.0;
    let x_middle = x_border_right / 2.0;

    ui.on_button_clicked(move || {
    let ui = weak.upgrade().unwrap();
    ui.set_show_button(false);

    
    ui.set_square_x(x_middle);
    ui.set_square_y(y_border_top);

    let mut rng = rand::rng(); 

    let timer_weak = ui.as_weak();

    let mut isTop = true;

    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(1000), move || {
        if let Some(ui) = timer_weak.upgrade(){

                let random_x = rng.random_range(0.0..x_border_right);
                let _random_y = rng.random_range(0.0..y_border_bottom); 

            ui.set_square_x(random_x);
            if !isTop{
                ui.set_square_y(y_border_top);
                isTop = true;
            }
            else {
                ui.set_square_y(y_border_bottom);
                isTop = false;
            }
        }
    })

    
});
    

    ui.run().unwrap();
}
