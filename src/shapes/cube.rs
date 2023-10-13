use egui::{self, Response};
use glam::{Vec3, Mat4, Quat};

use super::Shape3d;
use crate::camera::Camera;

#[derive(Copy, Clone)]
pub struct Cube { 
    pub size: Vec3, 
    pub xform: Mat4 
    }
impl Cube {
    pub fn new(size: Vec3, xform: Mat4)->Cube{
        Cube { 
            size: size, 
            xform: xform
            }
    }
}

impl Shape3d for Cube{
    fn draw(&self,ui: &mut egui::Ui, cam: &dyn Camera, painter: &egui::Painter, response: &egui::Response) {
        let viewport_size = response.rect.size(); //egui::Vec2::new(ui.available_width()-50.0, ui.available_height()-50.0)

        //   7.+------+ 4    
        //  .' |    .'|     
        //6+------+'5 |   
        // |   |  |   |     
        // | 3,+--|---+ 0 
        // |.'    | .'    
        //2+------+'1        
        let vertices = [
            Vec3::new(0.5, -0.5, -0.5), //0
            Vec3::new(0.5, -0.5, 0.5), //1
            Vec3::new(-0.5, -0.5, 0.5), //2
            Vec3::new(-0.5, -0.5, -0.5), //3 
            Vec3::new(0.5, 0.5, -0.5), //4
            Vec3::new(0.5, 0.5, 0.5), //5
            Vec3::new(-0.5, 0.5, 0.5), //6
            Vec3::new(-0.5, 0.5, -0.5), //7
            ];
        let indices = [ 
            (0,1),(1,2),(2,3),(3,0), //bottom
            (0,4),(1,5),(2,6),(3,7), // lines up
            (4,5),(5,6),(6,7),(7,4), // top
            ];
        
        // Get the relative position of our "canvas"
let to_screen = egui::emath::RectTransform::from_to(
    egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.size()),
    response.rect,
);
        
        for line in indices{
            let vtx1 = self.xform.transform_point3( vertices[line.0] * self.size);
            let vtx1_projected = cam.project_point( vtx1 );
            let p1 = egui::Pos2::new( vtx1_projected.x * viewport_size.x, vtx1_projected.y * viewport_size.y)  + viewport_size/2.0;
            let vtx2 = self.xform.transform_point3( vertices[line.1] * self.size);
            let vtx2_projected = cam.project_point( vtx2 );
            let p2 = egui::Pos2::new( vtx2_projected.x * viewport_size.x, vtx2_projected.y * viewport_size.y) + viewport_size/2.0;
            if vtx1_projected.is_nan() || vtx2_projected.is_nan(){ continue;}; // don't draw if any of the points if behind the camera 
            let cube_center = self.xform.transform_point3(Vec3::ZERO);
            let cam_pos = cam.get_center();
            let line_center_n = ( (vtx1+vtx2)/2.0 - cube_center ).normalize();
            let mut line_width =  line_center_n.dot((cam_pos-cube_center).normalize()) + 1.0 + 0.1;
            line_width *= (1.0 - (cam_pos-cube_center).length() / cam.get_far() ).clamp(0.0, 1.0)+0.1; //attenuate by distance from camera
            // Paint the line!
            painter.add(egui::Shape::LineSegment {
                points: [to_screen.transform_pos(p1), to_screen.transform_pos(p2)],
                stroke: egui::Stroke {
                    width: line_width,
                    color: egui::Color32::LIGHT_GRAY,
                },
            });
        }
    }
}