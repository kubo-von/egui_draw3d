use egui::{self, Response};
use glam::{Vec3, Mat4, Quat};

use super::Shape3d;
use crate::camera::Camera;

#[derive( Clone)]
pub struct Polymesh {
    pub name: Option<String>,
    pub xform: Mat4,
    pub line_width: f32,
    pub color: egui::Color32,
    pub points: Vec<Vec3>,
    pub indices: Vec<usize>,
    pub counts: Vec<usize>, 
    }
impl Polymesh {
    pub fn new( name: Option<String>,xform: Mat4, line_width: f32, color: egui::Color32, points: Vec<Vec3>, indices: Vec<usize>, counts: Vec<usize>)->Polymesh{
        Polymesh {
            name: name,
            xform: xform,
            line_width: line_width,
            color: color,
            points: points,
            indices: indices,
            counts: counts,
            }
    }
}

impl Shape3d for Polymesh{
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
    
    let cam_pos = cam.get_center();
    let attenuate = (1.0 - (cam_pos-pivot).length() / cam.get_far() ).clamp(0.0, 1.0);
    let fill_alpha = 0.5 * attenuate;
    let color = egui::Color32::from_rgba_unmultiplied(self.color.r(), self.color.g(), self.color.b(), ( (self.color.a() as f32 / 256.0 ) * fill_alpha*255.0 ) as u8 );
    let stroke = egui::Stroke {
            width: self.line_width,
            color: color,
        };
        
    let mut current_index = 0;
    // for each face
    for c in self.counts.iter(){
        let mut path_pts = Vec::with_capacity(*c);
        // for each vtx of the face
        for rel_i in current_index..(current_index+*c){
            let vtx_i = self.indices[rel_i];
            let vtx_P = self.points[vtx_i];
            let p_xformed = self.xform.transform_point3(vtx_P);
            let p_projected = cam.project_point( p_xformed );
            let p_screen = egui::Pos2::new( p_projected.x * viewport_size.x, p_projected.y * viewport_size.y)  + viewport_size/2.0;
            path_pts.push(p_screen);
        }
        //calculate the face normal
        let C_P = self.points[self.indices[current_index + *c -1]];
        let B_P = self.points[self.indices[current_index + *c -2]];
        let A_P = self.points[self.indices[current_index + *c -3]];
        let dir_BC = (C_P - B_P).normalize();
        let dir_BA = (A_P - B_P).normalize();
        let N = dir_BA.cross(dir_BC);
        
        let cam_pos = cam.get_center();
        let V = ( (C_P+A_P+B_P)/3.0 - cam_pos ).normalize();
        let N_dot_V = - N.dot(V);
        
    
        // draw the face wire
        let mut stroke_local = stroke.clone();
        stroke_local.color = egui::Color32::from_rgba_unmultiplied( stroke_local.color.r(),stroke_local.color.g(),stroke_local.color.b(), (N_dot_V.max(0.0) * 255.0) as u8 );
        painter.add(
            egui::epaint::PathShape::line(path_pts, stroke_local)
        );
        
        current_index += *c;
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