use gtk4 as gtk;
use gtk::glib;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct CustomButton;

impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for CustomButton {
    
}

impl WidgetImpl for CustomButton {
    
}

impl ButtonImpl for CustomButton {
    
}
