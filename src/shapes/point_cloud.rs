use egui::{self, Response};
use glam::{Vec3, Mat4, Quat};

use super::Shape3d;
use crate::camera::Camera;

#[derive( Clone)]
pub struct PointCloud {
    pub name: Option<String>,
    pub xform: Mat4,
    pub size: f32, 
    pub color: egui::Color32,
    pub points: Vec<Vec3>,
    }
impl PointCloud {
    pub fn new(name: Option<String>,xform: Mat4, size: f32, color: egui::Color32, points: Vec<Vec3>)->PointCloud{
        PointCloud {
            name: name,
            xform: xform,
            size: size,
            color: color,
            points: points,
            }
    }
}

impl Shape3d for PointCloud{
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
    
    let radius_p = self.xform.transform_point3(Vec3::ZERO)+self.size * cam.get_xform().x_axis.truncate();
    let radius_p_projected = cam.project_point( radius_p );
    let radius_p_screen = egui::Pos2::new( radius_p_projected.x * viewport_size.x, radius_p_projected.y * viewport_size.y)  + viewport_size/2.0;
    let circle_screen_size = (pivot_screen-radius_p_screen).length();
    
    let cam_pos = cam.get_center();
    let attenuate = (1.0 - (cam_pos-pivot).length() / cam.get_far() ).clamp(0.0, 1.0);
    let fill_alpha = 0.5 * attenuate;
    let color = egui::Color32::from_rgba_unmultiplied(self.color.r(), self.color.g(), self.color.b(), ( (self.color.a() as f32 / 256.0 ) * fill_alpha*255.0 ) as u8 );
        
    // Paint the points
    for p in &self.points{
        let p_x = self.xform.transform_point3(*p);
        let p_projected = cam.project_point( p_x );
        let p_screen = egui::Pos2::new( p_projected.x * viewport_size.x, p_projected.y * viewport_size.y)  + viewport_size/2.0;
        painter.add(egui::Shape::Circle(egui::epaint::CircleShape{
            center: to_screen.transform_pos(p_screen),
            radius: circle_screen_size,
            fill:  color,   
            stroke:  egui::Stroke::NONE 
        }));
    }

    match &self.name{
        Some(n) => {
            let text_pos = pivot_screen;
            painter.text(text_pos, egui::Align2::CENTER_CENTER, n.clone(), egui::FontId::monospace(16.0), color);
            },
        None => {}
    }
        
    }
}