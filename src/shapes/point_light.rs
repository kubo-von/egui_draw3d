use egui::{self, Response};
use glam::{Vec3, Mat4, Quat};

use super::Shape3d;
use crate::camera::Camera;

#[derive( Clone)]
pub struct PointLight {
    pub name: Option<String>,
    pub xform: Mat4,
    pub size: f32,
    pub color: egui::Color32,
    }
impl PointLight {
    pub fn new( name: Option<String>, xform: Mat4, size: f32, color: egui::Color32)->PointLight{
        PointLight {
            name: name,
            xform: xform,
            size: size,
            color: color,
            }
    }
}

impl Shape3d for PointLight{
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
    let mut stroke_width = 2.0;
    let attenuate = (1.0 - (cam_pos-pivot).length() / cam.get_far() ).clamp(0.0, 1.0);
    stroke_width *= attenuate+0.1; //attenuate by distance from camera
    let fill_alpha = 0.7 * attenuate;
    let stroke_color = egui::Color32::from_rgba_unmultiplied(255, 255, 160, 255 );
    
    //paint the circle
    painter.add(egui::Shape::Circle(egui::epaint::CircleShape{
        center: to_screen.transform_pos(pivot_screen),
        radius: circle_screen_size,
        fill:  egui::Color32::from_rgba_unmultiplied(255, 255, 0, (fill_alpha*255.0) as u8 ),   
        stroke:  egui::Stroke::new(stroke_width, stroke_color) 
        }));
        
    // Paint the rays
    for r in 0..17{
        let u =  r as f32 / 17.0 * std::f32::consts::PI *2.0;
        let dir = egui::Vec2::new(u.sin(), u.cos());
        let start_p = pivot_screen+(dir*circle_screen_size*1.1);
        let end_p = pivot_screen+(dir*circle_screen_size*1.5);
        // Paint the line!
        painter.add(egui::Shape::LineSegment {
            points: [to_screen.transform_pos(start_p), to_screen.transform_pos(end_p)],
            stroke: egui::Stroke {
                width: stroke_width,
                color: stroke_color,
            },
        });
    }
    
    }
}