use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, Grid, Label};
use random_legend_picker::randomizer;
use gtk::Image;

fn main() {
    
    let application =
        gtk::Application::new(Some("org.example.HelloWorld"), Default::default());

    application.connect_activate(build_ui);

    application.run();

}

fn build_ui(application: &gtk::Application) {
    //take the ui file to read
    let glade_src = include_str!("interface.ui");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    window.set_default_size(350, 70);
    window.set_position(gtk::WindowPosition::Center);
    window.set_border_width(20);

    let grid: gtk::Grid = builder.object("grid").expect("Couldn't get grid");
    //grid.set_column_spacing(10);

    grid.set_column_homogeneous(true);
    let legend_db = randomizer::random_pick();

    //label conf
    let legend_name: Label = builder.object("legend_name").expect("Couldn't get legend_name");
    legend_name.set_text(&legend_db.0);
    
    //image conf
    let legend_portrait: Image = builder.object("portrait").expect("Couldn't get legend_portrait");
    legend_portrait.set_from_file(&legend_db.1);


    //button conf
    let randomizer_button: Button = builder.object("button").expect("Couldn't get button");
    randomizer_button.connect_clicked(move |_| {
        let legend_db = randomizer::random_pick();
        legend_name.set_text(&legend_db.0);
        legend_portrait.set_from_file(&legend_db.1);
    });


    window.show_all();
}   