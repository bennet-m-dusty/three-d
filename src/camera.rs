use gust::*;

pub trait Camera
{
    fn get_view(&self) -> &Mat4;
    fn get_projection(&self) -> &Mat4;
    fn position(&self) -> &Vec3;
    fn target(&self) -> &Vec3;
    fn set_view(&mut self, position: Vec3, target: Vec3);
}

struct BaseCamera {
    position: Vec3,
    target: Vec3,
    view: Mat4
}

impl BaseCamera
{
    pub fn new(position: Vec3, target: Vec3) -> BaseCamera
    {
        let mut camera = BaseCamera {position, target, view: Mat4::identity()};
        camera.set_view(position, target);
        camera
    }

    pub fn set_view(&mut self, position: Vec3, target: Vec3)
    {
        self.position = position;
        self.target = target;
        let up = (vec3(1.0, 0.0, 0.0).cross(&(self.target - self.position))).normalize(); // TODO
        self.view = Mat4::look_at_rh(&na::Point::from_coordinates(self.position), &na::Point::from_coordinates(self.target), &up);
    }

    pub fn get_view(&self) -> &Mat4
    {
        &self.view
    }
}

pub struct PerspectiveCamera {
    base: BaseCamera,
    projection: Mat4
}

impl PerspectiveCamera
{
    pub fn new(position: Vec3, target: Vec3, aspect: f32, z_near: f32, z_far: f32) -> PerspectiveCamera
    {
        let mut camera = PerspectiveCamera { base: BaseCamera::new(position, target), projection: Mat4::identity() };
        camera.set_extent(aspect, z_near, z_far);
        camera
    }

    fn set_extent(&mut self, aspect: f32, z_near: f32, z_far: f32)
    {
        if z_near < 0.0 || z_near > z_far { panic!("Wrong perspective camera parameters") };
        self.projection = Mat4::new_perspective(aspect, 0.25 * ::std::f32::consts::PI, z_near, z_far);
    }
}

impl Camera for PerspectiveCamera
{
    fn get_view(&self) -> &Mat4
    {
        self.base.get_view()
    }

    fn get_projection(&self) -> &Mat4
    {
        &self.projection
    }

    fn position(&self) -> &Vec3
    {
        &self.base.position
    }

    fn target(&self) -> &Vec3
    {
        &self.base.target
    }

    fn set_view(&mut self, position: Vec3, target: Vec3)
    {
        self.base.set_view(position, target);
    }
}

pub struct OrthographicCamera {
    base: BaseCamera,
    projection: Mat4
}

impl OrthographicCamera
{
    pub fn new(position: Vec3, target: Vec3, width: f32, height: f32, depth: f32) -> OrthographicCamera
    {
        let mut camera = OrthographicCamera { base: BaseCamera::new(position, target), projection: Mat4::identity() };
        camera.set_extent(width, height, depth);
        camera
    }

    fn set_extent(&mut self, width: f32, height: f32, depth: f32)
    {
        self.projection = Mat4::new_orthographic(-0.5 * width, 0.5 * width, -0.5 * height, 0.5 * height, -0.5 * depth, 0.5 * depth);
    }
}

impl Camera for OrthographicCamera
{
    fn get_view(&self) -> &Mat4
    {
        self.base.get_view()
    }

    fn get_projection(&self) -> &Mat4
    {
        &self.projection
    }

    fn position(&self) -> &Vec3
    {
        &self.base.position
    }

    fn target(&self) -> &Vec3
    {
        &self.base.target
    }

    fn set_view(&mut self, position: Vec3, target: Vec3)
    {
        self.base.set_view(position, target);
    }
}