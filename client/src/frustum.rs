use base::math::*;
use std::f64::consts::PI;
enum SIDE {
    Top,
    Bottom,
    Left,
    Right,
    NearP,
    FarP,
}
pub enum LOCATION {
    Inside,
    Outside,
    Intersect,
}

// f64 for high accuracy #makef64greatagain
const ANG2RAD: f64 = (PI / 180.);
const RAD2ANG: f64 = (180. / PI);

struct Plane {
    points: [Point3f; 3],
    d: f32,
    normal: Vector3f,
}

impl Plane {
    // is used to create plane with dummy values
    // use setPoint3fs for right initialization
    pub fn new() -> Plane {
        Plane {
            points: [Point3f::new(0., 0., 0.), Point3f::new(1., 1., 1.), Point3f::new(3., 2., 2.)],
            d: 0.,
            normal: Vector3f::new(0., 1., -1.),
        }
    }

    pub fn set_point3fs(&mut self, x: Vector3f, y: Vector3f, z: Vector3f) {
        self.points[0] = Point3f {
            x: x.x,
            y: x.y,
            z: x.z,
        };
        self.points[1] = Point3f {
            x: y.x,
            y: y.y,
            z: y.z,
        };
        self.points[2] = Point3f {
            x: z.x,
            y: z.y,
            z: z.z,
        };

        let xy = y - x;
        let xz = z - x;

        self.normal = xy.cross(xz).normalize();
        self.d = -(self.normal.x * self.points[0].x + self.normal.y * self.points[0].y +
                   self.normal.z * self.points[0].z);
    }

    pub fn distance(&self, p: &Point3f) -> f32 {
        let top = self.normal.x * p.x + self.normal.y * p.y + self.normal.z * p.z + self.d;
        let c = |x| x * x;
        let bottom = (c(self.normal.x) + c(self.normal.y) + c(self.normal.z)).sqrt();
        // info!("distance {:?}", top / bottom);
        top / bottom
    }
}

pub struct Frustum {
    planes: [Plane; 6],
    angle: f32,
    ratio: f32,
    near: f32,
    far: f32,
    nearheight: f32,
    nearwidth: f32,
    farheight: f32,
    farwidth: f32,
}

impl Frustum {
    // initialization stuff contains dummys use
    // set_cam_internals and set_cam_def to initialize the frustum
    pub fn new() -> Frustum {
        let ps =
            [Plane::new(), Plane::new(), Plane::new(), Plane::new(), Plane::new(), Plane::new()];
        Frustum {
            planes: ps,
            angle: 0.0,
            ratio: 0.75,
            near: 10.0,
            far: 300.0,
            nearheight: 800.0,
            nearwidth: 600.0,
            farheight: 1920.0,
            farwidth: 1080.0,
        }
    }

    pub fn set_cam_internals(&mut self, angle: f32, ratio: f32, near: f32, far: f32) {
        self.angle = angle;
        self.ratio = ratio;
        self.near = near;
        self.far = far;

        let tan: f32 = ((ANG2RAD * self.angle as f64 * 0.5).tan()) as f32;

        self.nearheight = self.near * tan;
        self.nearwidth = self.nearheight * ratio;
        self.farheight = self.far * tan;
        self.farwidth = self.farheight * ratio;
    }

    // camera pos is a vector to use operators like
    // -/+ (no ops for point3f - vector3f)
    // note replace with more efficent later on
    pub fn set_cam_def(&mut self, pos: Point3f, look_at: Vector3f, up: Vector3f) {
        // axis cause we all want them
        let camera_pos = Vector3f::new(pos.x, pos.y, pos.z);
        let z = (camera_pos - look_at).normalize();
        let x = (up.cross(z)).normalize();
        let y = (z.cross(x)).normalize();

        // calc center of [insert near/far plane joke here]
        let c = {
            |x: f32| (camera_pos - z) * (x)
        };

        let nc = c(self.near);
        let fc = c(self.far);

        // 4 corners of Frustum near plane
        let ntl = nc + y * self.nearheight - x * self.nearwidth;
        let ntr = nc + y * self.nearheight + x * self.nearwidth;
        let nbl = nc - y * self.nearheight - x * self.nearwidth;
        let nbr = nc - y * self.nearheight + x * self.nearwidth;
        // 4 corners of Frustum far plane
        let ftl = fc + y * self.farheight - x * self.farwidth;
        let ftr = fc + y * self.farheight + x * self.farwidth;
        let fbl = fc - y * self.farheight - x * self.farwidth;
        let fbr = fc - y * self.farheight + x * self.farwidth;

        // set planes points are given counter clockwise
        let mut c = |s, x, y, z| self.planes[s as usize].set_point3fs(x, y, z);
        c(SIDE::Top, ntr, ntl, ftl);
        c(SIDE::Bottom, nbl, nbr, fbr);
        c(SIDE::Left, ntl, nbl, fbl);
        c(SIDE::Right, nbr, ntr, fbr);
        c(SIDE::NearP, ntl, ntr, nbr);
        c(SIDE::FarP, ftr, ftl, fbl);
    }

    pub fn point_in_frustum(&self, p: &Point3f) -> LOCATION {
        // If a point is inside the frustum it must be on the right
        // side of every plane.
        for i in 0..6 {
            if self.planes[i].distance(p) < 0.0 {
                return LOCATION::Outside;
            }
        }
        LOCATION::Inside
    }

    pub fn sphere_in_frustum(&self) {
        // TODO if needed
    }

    pub fn box_in_frustum(&self, points: [&Point3f; 8]) -> LOCATION {
        // If one of the corner points is inside we box needs to be rendered so INSIDE
        let mut ins;
        let mut out;
        for i in 0..6 {
            ins = 0;
            out = 0;
            for k in 0..8 {
                if self.planes[i].distance(points[k]) < 0.0 {
                    out += 1;
                    if ins != 0 {
                        break;
                    }
                } else {
                    ins += 1;
                    if out != 0 {
                        break;
                    }
                }
            }
            // no point inside plane ... OUTSIDE
            if ins == 0 {
                return LOCATION::Outside;
            } else if out != 0 {
                return LOCATION::Intersect;
            }
        }
        LOCATION::Inside
    }
}

// simple culling struct
pub struct SimpleCull {
    cam_pos: Point3f,
    look_at: Vector3f,
    fov: f32,
}

impl SimpleCull {
    pub fn new() -> SimpleCull {
        SimpleCull {
            cam_pos: Point3f::new(0.0, 0.0, 0.0),
            look_at: Vector3f::new(0.0, 0.0, 0.0),
            fov: 60.0,
        }
    }
    // sets values of struct
    pub fn set_up(&mut self, cam_pos: Point3f, look_at: Vector3f, fov: f32) {
        self.fov = fov;
        let fake_look = look_at.normalize() * -15.;
        self.cam_pos = cam_pos + fake_look;
        self.look_at = look_at.normalize();
    }
    // checks if box is visible
    pub fn is_vis(&self, points: [&Point3f; 8]) -> LOCATION {
        for i in 0..8 {
            let mut look_at_point = points[i] - self.cam_pos;
            look_at_point = look_at_point.normalize();
            let res = look_at_point.dot(self.look_at).acos() * RAD2ANG as f32;
            if res <= self.fov {
                return LOCATION::Inside;
            }
        }
        LOCATION::Outside
    }
}
