#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use egui_draw3d::widgets::viewport3d::Viewport3d;
use egui_draw3d::shapes;
use egui_draw3d::camera;
use glam::{Vec3, Mat4, Quat};


fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([1300.0, 800.0].into()),
        min_window_size: Some([300.0, 300.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "egui_draw3d example",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    )
}


pub struct TemplateApp {
    sensitivity: f32,
    camera_xform: Mat4 //stores current xform of camera so we can modify it by panning etc 
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            sensitivity: 0.01,
            camera_xform:  Mat4::from_scale_rotation_translation(Vec3::ONE, Quat::IDENTITY, Vec3::new(0.0, -1.0, 5.0) ) //camera Y translation has to be flipped to match 
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        
         //creating some points we will use later in poincloud shape
        let example_pointcloud: Vec<Vec3> = (0..90).into_iter().map(|pi| {let u = pi as f32/90.0; Vec3::new((u*10.0).sin(), (u*10.0).cos(), u) }).collect();

        egui::CentralPanel::default().show(ctx, |ui| {
            let camera = camera::Perspective::new(0.35, self.camera_xform, 1280.0/720.0, 0.01, 30.0);
            
            // add all shapes we wanna render
            let scene: Vec<Box<dyn shapes::Shape3d>> = vec!(
                Box::new( shapes::grid::Grid::new(1.0, Mat4::IDENTITY, 0.04) ),
                Box::new( shapes::cube::Cube::new(Vec3::ONE,  Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)) * Mat4::from_scale(Vec3::new(1.5, 0.2, 1.5)) )  ),
                Box::new( shapes::cube::Cube::new(Vec3::ONE,  Mat4::from_translation(Vec3::new(4.0, 0.0, 0.0)) * Mat4::from_scale(Vec3::new(0.5, 0.5, 0.5)) )  ),
                Box::new( shapes::cube::Cube::new(Vec3::ONE,  Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0)) * Mat4::from_scale(Vec3::new(0.5, 0.5, 0.5)) )  ),
                Box::new( shapes::cube::Cube::new(Vec3::ONE, Mat4::from_translation(Vec3::new(0.0, 0.5, 2.0))) ),
                Box::new( shapes::cube::Cube::new(Vec3::ONE, Mat4::from_translation(Vec3::new(0.0, 0.5, 10.0))) ),
                Box::new( shapes::cube::Cube::new(Vec3::ONE, Mat4::from_translation(Vec3::new(1.0, 1.5, 1.0)) * Mat4::from_rotation_x(30_f32.to_radians())) ),
                
                Box::new( shapes::point_light::PointLight::new(0.3, Mat4::from_translation(Vec3::new(1.0, 3.5, 1.0)) * Mat4::from_rotation_x(30_f32.to_radians())) ),
                
                Box::new( shapes::vector::Vector::new(1.0, Vec3::new(1.0, 0.0, 0.0).normalize(), Mat4::from_translation(Vec3::new(2.0, 0.0, 0.0)), egui::Color32::RED) ),
                Box::new( shapes::vector::Vector::new(1.0, Vec3::new(0.0, 1.0, 0.0).normalize(), Mat4::from_translation(Vec3::new(2.0, 0.0, 0.0)), egui::Color32::GREEN) ),
                Box::new( shapes::vector::Vector::new(1.0, Vec3::new(0.0, 0.0, 1.0).normalize(), Mat4::from_translation(Vec3::new(2.0, 0.0, 0.0)), egui::Color32::BLUE) ),
                
                Box::new( shapes::point_cloud::PointCloud::new(example_pointcloud, 0.02, Mat4::from_translation(Vec3::new(-2.0, 0.5, 0.0)) * Mat4::from_rotation_x(30_f32.to_radians())) ),
                 );     
            
            let view_response = ui.add(
                Viewport3d::default()
                .with_scene(scene)
                .with_camera(Box::new(camera))
                .with_size(1280, 720)
            );
                
            // handle camera rotation by mouse dragging 
            if ctx.input(|i|{ i.modifiers.alt && !i.pointer.middle_down()}){
                let cam_rot_y = Mat4::from_rotation_y(view_response.drag_delta().x * self.sensitivity);
                let x_axis = self.camera_xform.x_axis.truncate();
                let cam_rot_x = Mat4::from_axis_angle(x_axis, view_response.drag_delta().y * self.sensitivity);
                self.camera_xform = cam_rot_y * cam_rot_x * camera.xform;
            }
            
            // handle camera dolly by scrolling and paning 
            ctx.input(|i|{
                //dolly
                let scroll = i.scroll_delta.y;
                if scroll != 0.0{
                    let dolly_center = Vec3::ZERO;
                    let z_axis = (self.camera_xform.transform_point3(Vec3::ZERO)-dolly_center).normalize();//self.camera_xform.z_axis.truncate();
                    let dolly_m = Mat4::from_translation(z_axis * scroll * self.sensitivity);
                    self.camera_xform = dolly_m * self.camera_xform ;
                    }
                //pan
                if ctx.input(|i|{ i.modifiers.alt && i.pointer.middle_down()}){
                    let drag = view_response.drag_delta();
                    let pan_m_x = Mat4::from_translation(self.camera_xform.x_axis.truncate() * -drag.x * self.sensitivity);
                    let pan_m_y = Mat4::from_translation(self.camera_xform.y_axis.truncate() * -drag.y * self.sensitivity);
                    self.camera_xform = pan_m_x * pan_m_y * self.camera_xform ;
                }
                });
            ui.separator();


        });
    }
}
