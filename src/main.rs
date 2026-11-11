use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Button;
use gtk4::prelude::*;

fn main() {
    let app = Application::builder()
        .application_id("com.rvrsdev.waycalc")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Waycalc")
            .default_height(200)
            .default_width(300)
            .build();

        let button = Button::with_label("Click me");
        button.connect_clicked(|_button| {
            eprintln!("Clicked!");
        });

        let layout = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

        // & - референс к объекту (поинтер)
        layout.append(&button);

        window.set_child(Some(&layout));

        window.show();
    });

    app.run();
}
