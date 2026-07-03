pub mod main_menu;
pub mod room;

pub trait Page {
    fn update(&mut self) {
        
    }
    fn draw(&self) {}
}