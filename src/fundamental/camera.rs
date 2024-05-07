use glam::{Mat4, Quat, Vec3, Vec3A};

pub struct Camera {
    view: Mat4,
    projection: Mat4,
    view_projection: Mat4,
    position: Vec3A,
    right: Vec3A,
    direction: Vec3A,
    up: Vec3A,
    yaw: f32  ,
    pitch: f32  ,
    near_plane: f32  ,
    far_plane: f32  ,
    field_of_view_y: f32  ,
    aspect_ratio: f32  ,
    zoom: f32  ,
    viewport_width: f32  ,
    viewport_height: f32  ,
    perspective: bool ,
    update_projection: bool ,
}
impl Camera {
    fn init_perpective(&mut self, near_plane : f32, far_plane: f32, fov_y : f32, aspect_ratio : f32 ) { // TODO : Make Builder Pattern
        self.perspective = true;
        self.near_plane = near_plane;
        self.far_plane = far_plane;
        self.field_of_view_y = fov_y;
        self.aspect_ratio = aspect_ratio;
    
        self.reset();
    }
    fn init_orthographic(&mut self, near_plane : f32, far_plane : f32, viewport_width : f32 , viewport_height : f32, zoom : f32 ) {
        self.perspective = false;
        self.near_plane = near_plane;
        self.far_plane = far_plane;
        self.viewport_width = viewport_width;
        self.viewport_height = viewport_height;
        self.zoom = zoom;
    
        self.reset();
    }
    fn reset(&mut self){
        self.position = Vec3A::ZERO;
        self.yaw = 0.0;
        self.pitch = 0.0;
        self.view = Mat4::IDENTITY;
        self.projection = Mat4::IDENTITY;
        self.update_projection = true;
    }
    fn set_viewport_size(&mut self, width: f32, height : f32 ) {
        self.viewport_width = width;
        self.viewport_height = height;
        self.update_projection = true;
    }
    fn set_zoom(&mut self, zoom: f32 ) {
        self.zoom = zoom;
        self.update_projection = true;
    }
    fn set_aspect_ratio(&mut self, aspect_ratio : f32 ) {
        self.aspect_ratio = aspect_ratio;
        self.update_projection = true;
    }
    fn set_fov_y(&mut self, fov_y : f32 ) {
        self.field_of_view_y = fov_y;
        self.update_projection = true;
    }
    fn update(&mut self) {
        let pitch_rotation = Quat::from_axis_angle(glam::Vec3::X, self.pitch);
        let yaw_rotation = Quat::from_axis_angle(glam::Vec3::Y, self.yaw);
        let rotation = pitch_rotation * yaw_rotation;

        let translation = Mat4::from_translation(self.position.into());
        let view = Mat4::from_quat(rotation) * translation;

        self.right = Vec3A::new(view.x_axis.x, view.y_axis.x, view.z_axis.x);
        self.up = Vec3A::new(view.x_axis.y, view.y_axis.y, view.z_axis.y);
        self.direction = Vec3A::new(view.x_axis.z, view.y_axis.z, view.z_axis.z);

        if self.update_projection {
            self.update_projection = false;
            self.calculate_projection_matrix();
            
        }
        self.calculate_view_projection();

    }

    fn rotate(&mut self, delta_pitch : f32, delta_yaw : f32 ) {
        self.pitch += delta_pitch;
        self.yaw += delta_yaw;
    }
    fn calculate_projection_matrix(&mut self){
        if self.perspective {
            self.projection = Mat4::perspective_rh(self.field_of_view_y.to_radians(), self.aspect_ratio, self.near_plane, self.far_plane);
            return
        }
        self.projection = Mat4::orthographic_rh(
            self.zoom * (-self.viewport_width/2.0), 
            self.zoom * (self.viewport_width/2.0),
            self.zoom * (-self.viewport_height/2.0),
            self.zoom * (self.viewport_height/2.0),
            self.near_plane,
            self.far_plane
        );
        
    }
    fn calculate_view_projection(&mut self) {
        self.view_projection = self.projection * self.view;
    }
    fn get_projection_ortho_2d( out_matrix : &mut Mat4 ) {
        *out_matrix = Mat4::orthographic_rh(0.0, 1.0, 0.0, 1.0, -1.0, 1.0);
    }
    fn yaw_pitch_from_direction( direction : &Vec3A, yaw : &mut f32, pitch : &mut f32 ) {
        *yaw = direction.z.atan2(direction.x).to_degrees();
        *pitch = direction.y.asin().to_degrees();
    }

}