pub mod cube;
pub mod grid;
pub mod point_light;
pub mod point_cloud;
pub mod polymesh;
pub mod vector;
use egui;
use crate::camera::Camera;


pub trait Shape3d {
    fn draw(&self, ui: &mut egui::Ui,  cam: &dyn Camera, painter: &egui::Painter, response: &egui::Response);
}