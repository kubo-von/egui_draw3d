use egui::{self, Response};
use glam::{Vec3, Mat4, Quat};

use super::Shape3d;
use crate::camera::Camera;

const phi: f32 = std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct Vector { 
    pub size: f32,
    pub dir: Vec3, 
    pub xform: Mat4,
    pub color: egui::Color32
    }
impl Vector {
    pub fn new(size: f32, dir: Vec3, xform: Mat4, color: egui::Color32)->Vector{
        Vector { 
            size: size, 
            dir:dir,
            xform: xform,
            color: color
            }
    }
}

impl Shape3d for Vector{
    fn draw(&self,ui: &mut egui::Ui, cam: &dyn Camera, painter: &egui::Painter, response: &egui::Response) {
        let viewport_size = response.rect.size(); //egui::Vec2::new(ui.available_width()-50.0, ui.available_height()-50.0)
        // Get the relative position of our "canvas"
let to_screen = egui::emath::RectTransform::from_to(
    egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.size()),
    response.rect,
);
        
 
    let pivot = self.xform.transform_point3(Vec3::ZERO);
    let pivot_projected = cam.project_point( pivot );
    let pivot_screen = egui::Pos2::new( pivot_projected.x * viewport_size.x, pivot_projected.y * viewport_size.y)  + viewport_size/2.0;
    
    let end = self.xform.transform_point3(self.dir * self.size);
    let end_projected = cam.project_point( end );
    let end_screen = egui::Pos2::new( end_projected.x * viewport_size.x, end_projected.y * viewport_size.y)  + viewport_size/2.0;
    
    let cam_pos = cam.get_center();
    let mut stroke_width = 4.0;
    let attenuate = (1.0 - (cam_pos-pivot).length() / cam.get_far() ).clamp(0.0, 1.0);
    stroke_width *= attenuate+0.1; //attenuate by distance from camera
    let fill_alpha = 0.5 * attenuate;
    
    //paint the main line
    painter.add(egui::Shape::LineSegment {
        points: [to_screen.transform_pos(pivot_screen), to_screen.transform_pos(end_screen)],
        stroke: egui::Stroke {
            width: stroke_width,
            color: self.color,
        },
    });
    let arrow_size = 0.07;
    // Paint the arrow tip
    let mut tip_pts: Vec<egui::Pos2> = Vec::with_capacity(63);
    for r in 0..600{
        let u =  r as f32 / 600.0;
        let cu = u * phi * 80.0;
        let a_width = arrow_size * (1.0-u);
        let x_axis = self.dir.cross(Vec3::new(phi, phi, phi)).normalize();
        let z_axis = x_axis.cross(self.dir).normalize();
        let base_p = self.xform.transform_point3( 
            self.dir * self.size * (1.0-arrow_size + arrow_size * u * 2.2) 
            + x_axis * cu.sin() * a_width
            + z_axis * cu.cos() * a_width
        );
        let base_p_projected = cam.project_point( base_p );
        let base_p_screen = egui::Pos2::new( base_p_projected.x * viewport_size.x, base_p_projected.y * viewport_size.y)  + viewport_size/2.0;
    tip_pts.push(to_screen.transform_pos(base_p_screen));
    }
    
    painter.add(egui::Shape::Path(
        egui::epaint::PathShape::line(tip_pts, egui::Stroke::new(stroke_width*1.0, self.color) ))
    );
        
    
    }
}