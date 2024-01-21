use glam::{Vec3, Mat4};

pub trait Camera {
    fn project_point(&self, p: Vec3) -> Vec3;
    fn dist_to_point(&self, p: Vec3) -> f32;
    fn get_center(&self) -> Vec3;
    fn get_xform(&self) -> Mat4;
    fn get_far(&self) -> f32;
}
#[derive(Copy, Clone)]
pub struct Perspective { 
    pub focal_lenght: f32, 
    pub xform: Mat4,
    pub aspect_ratio :f32,
    pub near: f32,
    pub far: f32,
    }
impl Perspective  {
    pub fn new(focal_lenght: f32, xform: Mat4, aspect_ratio: f32, near: f32 , far: f32 )->Self{
        Perspective { 
            focal_lenght: focal_lenght, 
            xform: xform ,
            aspect_ratio: aspect_ratio,
            near: near,
            far: far,
        }
    }
    
}

//camera Y translation has to be flipped to match 
impl Camera for Perspective {
    fn project_point(&self, p: Vec3) -> Vec3 
    {
        let h_aperature = 0.209549993277;
        let fov_x: f32 = 2.0 * f32::tan((h_aperature/2.0) / self.focal_lenght); //TODO calculate fov_y by takign aspect ratio into account
        //println!("fov: {:?}", fov_x);
        let prj_mtx = Mat4::perspective_rh(fov_x, self.aspect_ratio, self.near, self.far);
        let p_xformed =  self.xform.inverse().transform_point3(p * Vec3::new(1.0, -1.0, 1.0)); // apply the camera xform
        if p_xformed.z >= 0.0 {return Vec3::NAN;}
        let p_projected = prj_mtx.project_point3(p_xformed); // apply the perspective projection
        if p_projected.z <= self.near{
            return Vec3::NAN;
        }
        p_projected
    }
    fn dist_to_point(&self, p: Vec3) -> f32{
        (self.get_center() - p ).length()
    }
    fn get_center(&self) -> Vec3{
        self.xform.transform_point3(Vec3::ZERO) * Vec3::new(1.0, -1.0, 1.0)
    }
    fn get_xform(&self) -> Mat4{
        self.xform
    }
    fn get_far(&self) -> f32{
        self.far
    }
}