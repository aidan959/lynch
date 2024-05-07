use glam::{Vec3A, Mat4};

struct Camera {
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
    fn init_perpective(&mut self, near_plane : f32, far_plane: f32, fov_y : f32, aspect_ratio : f32 ) {

    }
    fn init_orthographic(&mut self, near_plane : f32, far_plane : f32, viewport_width : f32 , f32 viewport_height, f32 zoom ) {

    }
    fn reset(&mut self){

    }
    fn set_viewport_size(&mut self, width: f32, height : f32 ) {

    }
    fn set_zoom(&mut self, zoom: f32 ) {

    }
    fn set_aspect_ratio(&mut self, aspect_ratio : f32 ) {

    }
    fn set_fov_y(&mut self, fov_y : f32 ) {

    }
    fn update(&mut self) {

    }
    fn rotate(&mut self, delta_pitch : f32, delta_yaw : f32 ) {

    }
    fn calculate_projection_matrix(&mut self){

    }
    fn calculate_view_projection(&mut self) {
        
    }
    fn unproject_inverted_y( screen_coordinates : &Vec3A ) -> Vec3A { 
        todo!("Implement unproject_inverted_y")
    }
    fn get_projection_ortho_2d( out_matrix : &mut Mat4 ) {

    }
    fn yaw_pitch_from_direction( direction : &Vec3A, yaw : f32, pitch : f32 ) {

    }

}