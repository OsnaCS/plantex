use base::math::*;


pub struct Camera {
    pub position: Point3f, // PRP
    // look_at_point: Point3<f32>, // VRP
    pub view_up_vector: Vector3f, // VUV
    pub theta: f32,
    pub phi: f32, // theta_phi: (f32, f32),
}

impl Camera {
    /// Returns the projection matrix
    pub fn proj_matrix(&self) -> Matrix4<f32> {
        perspective(deg(60.0), 16.0 / 9.0, 0.1, 100.0)
    }

    /// Returs view matrix
    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.position, self.get_look_at_point(), self.view_up_vector)
    }

    /// Calculates the look_at_point by using theta and phi on the unit sphere
    pub fn get_look_at_point(&self) -> Point3f {
        // let lookatvector = self.get_look_at_vector();
        self.position + self.get_look_at_vector()
        // Point3f::new(lookatvector.x + self.position.x,
        //              lookatvector.y + self.position.y,
        //              lookatvector.z + self.position.z)
    }

    pub fn get_look_at_vector(&self) -> Vector3f {
        Vector3f::new(self.theta.sin() * self.phi.cos(),
                      self.theta.sin() * self.phi.sin(),
                      self.theta.cos())
    }

    pub fn move_by(&mut self, pos_diff: Vector3f) {
        self.position += pos_diff;
        // self.position.add_assign(pos_diff);
    }

    /// method to call when forward or backward movement is needed
    /// `factor` is a factor to scale the movement speed
    /// `factor` has to be positive for foward movement
    /// `factor` has to be negative for backward movement
    pub fn move_forward_backward(&mut self, factor: f32) {
        // maybe implement the function so the factor is a parameter (to regulate speed
        // with shift)
        let mut lookatvector = self.get_look_at_vector();
        lookatvector.z = 0.0;
        lookatvector.normalize();
        lookatvector *= factor;
        self.move_by(lookatvector); //does the "self" have to be here?
    }

    /// method to call when left or right movement is needed
    /// `factor` is a factor to scale the movement speed
    /// `factor` has to be positive for left movement
    /// `factor` has to be negative for right movement
    // pub fn move_left_right(&mut self, factor: f32) {
    //     let mut lookatvector = self.get_look_at_vector();
    //     lookatvector.z = 0.0;
    //     lookatvector.normalize();
    //     // Get the orthogonal 2d-vector, which is 90 degrees to the left
    // let mut move_dir = Vector3f::new(-lookatvector.y, lookatvector.x,
    // 0.0);
    //     move_dir *= factor;
    //     self.move_by(move_dir);
    //
    // }
    /// method to call when upward or downward movement is needed
    /// `factor` is a factor to scale the movement speed
    /// `factor` has to be positive for upward movement
    /// `factor` has to be negative for downward movement
    // pub fn move_up_down(&mut self, factor: f32) {
    //     self.move_by(Vector3f::new(0.0, 0.0, factor));
    // }


    pub fn change_dir(&mut self, theta_diff: f32, phi_diff: f32) {
        self.theta += theta_diff;
        self.phi += phi_diff;
        // new matrix will be calculated
    }
}
