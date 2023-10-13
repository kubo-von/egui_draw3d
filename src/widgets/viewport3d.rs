use std::ops::Deref;

use glam::Mat4;

use crate::{Shape3d};
use crate::camera::{*};
use crate::shapes;
pub struct Viewport3d {
    width: usize,
    height: usize,
    scene: Vec<Box<dyn Shape3d>>,
    camera: Box<dyn Camera>
}
impl Viewport3d {
    pub fn with_scene(mut self, scene: Vec<Box<dyn Shape3d>>)->Self{
        self.scene = scene;
        self
    }
    pub fn with_camera(mut self, camera: Box<dyn Camera>)->Self{
        self.camera = camera;
        self
    }
    pub fn with_size(mut self, width: usize,height: usize)->Self{
        self.width = width;
        self.height = height;
        self
    }
}

impl Default for Viewport3d {
    fn default() -> Self {
        Self {
            width: 640,
            height: 480,
            scene: Vec::new(),
            camera: Box::new( Perspective::new(0.35, Mat4::IDENTITY, 1.0, 0.01, 20.0 ))
        }
    }
}

impl eframe::egui::Widget for Viewport3d {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(self.width as f32, self.height as f32),
            egui::Sense::click_and_drag(),
        );
        
        let cam = self.camera.deref().clone();
        
        for sh in self.scene{
            sh.draw(ui,cam,&painter, &response);
        }
        if response.clicked(){
            println!("clicked {:?}", response.ctx.pointer_interact_pos());
        }
        
        response 
    }
    
    }