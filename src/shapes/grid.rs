use egui::{self, Response};
use glam::{Vec3, Mat4, Quat};

use super::Shape3d;
use crate::camera::Camera;

#[derive(Copy, Clone)]
pub struct Grid { 
    pub size: f32, 
    pub xform: Mat4,
    pub width: f32,
    }
impl Grid {
    pub fn new(size: f32, xform: Mat4, width: f32)->Grid{
        Grid { 
            size: size,
            xform: xform,
            width: width
            
            }
    }
}

impl Shape3d for Grid{
    fn draw(&self,ui: &mut egui::Ui, cam: &dyn Camera, painter: &egui::Painter, response: &egui::Response) {
        let viewport_size = response.rect.size(); //egui::Vec2::new(ui.available_width()-50.0, ui.available_height()-50.0)
        let n_lines = 20;
        let line_dist = 1.0;
        let size = n_lines as f32 * line_dist;
        
        // Get the relative position of our "canvas"
        let to_screen = egui::emath::RectTransform::from_to(
            egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.size()),
            response.rect,
        );
        
        for d in 0..2{ // we do the lines two times, rotate by 90 degrees fro the 2nd one
            for x in 0..n_lines{
                for y in 0..n_lines{
                    let rot_m = match d{
                        0 => Mat4::IDENTITY,
                        _ => Mat4::from_rotation_y(90_f32.to_radians())
                    };
                    let offset = Vec3::new(line_dist * y as f32,0.0,line_dist * x as f32) - Vec3::new(line_dist * (n_lines as f32 - 1.0) / 2.0,0.0,line_dist * (n_lines as f32 - 1.0) as f32 / 2.0 );
                    let vtx1 = Vec3::new(0.5, 0.0, 0.0) * line_dist + offset;
                    let vtx1_projected = cam.project_point( self.xform.transform_point3( rot_m.transform_point3(vtx1)));
                    let p1 = egui::Pos2::new( vtx1_projected.x * viewport_size.x, vtx1_projected.y * viewport_size.y)  + viewport_size/2.0;
                    let vtx2_projected = Vec3::new(-0.5, 0.0, 0.0)  * line_dist + offset;
                    let vtx2 = cam.project_point( self.xform.transform_point3( rot_m.transform_point3(vtx2_projected)));
                    let p2 = egui::Pos2::new( vtx2.x * viewport_size.x, vtx2.y * viewport_size.y) + viewport_size/2.0;
                    let a = 1.0 - offset.length()/(n_lines as f32 * 0.5 * line_dist); // alpha from distance to the grid center
                    if vtx1_projected.is_nan() || vtx2_projected.is_nan(){ continue;};
                    // Paint the line!
                    painter.add(egui::Shape::LineSegment {
                        points: [to_screen.transform_pos(p1), to_screen.transform_pos(p2)],
                        stroke: egui::Stroke {
                            width: 2.0,
                            color: egui::Color32::from_rgba_unmultiplied(40, 40, 40, (a * 255.0) as u8),
                        },
                    });
                }
            }
        }
        
    }
}