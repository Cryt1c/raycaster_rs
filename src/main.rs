use std::cmp::max;

use glam::DVec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = max((image_width as f64 / aspect_ratio) as i32, 1);

    let world = HittableList {
        objects: vec![
            Box::new(Sphere {
                center: DVec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: DVec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = DVec3::new(0.0, 0.0, 0.0);

    let viewport_u = DVec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = DVec3::new(0.0, -viewport_height, 0.0);
    eprintln!("{} {} {}", viewport_u.x, viewport_u.y, viewport_u.z);
    eprintln!("{} {} {}", viewport_v.x, viewport_v.y, viewport_v.z);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;
    eprintln!(
        "{} {} {}",
        pixel_delta_u.x, pixel_delta_u.y, pixel_delta_u.z
    );
    eprintln!(
        "{} {} {}",
        pixel_delta_v.x, pixel_delta_v.y, pixel_delta_v.z
    );

    let viewport_upper_left =
        camera_center - DVec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{} {}\n255", image_width, image_height);

    for (j, y) in (0..image_height).enumerate() {
        eprintln!("\rScanlines remaining: {}", y);
        for (i, _x) in (0..image_width).enumerate() {
            let pixel_center =
                pixel00_loc + ((i as f64) * pixel_delta_u) + ((j as f64) * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray {
                origin: camera_center,
                dir: ray_direction,
            };

            let pixel_color = ray_color(ray, &world);
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}

fn write_color(pixel_color: DVec3) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x).floor(),
        (255.999 * pixel_color.y).floor(),
        (255.999 * pixel_color.z).floor()
    )
}

fn ray_color(ray: Ray, world: &HittableList) -> DVec3 {
    let mut rec = HitRecord {
        p: DVec3::new(0.0, 0.0, 0.0),
        normal: DVec3::new(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: false,
    };
    if world.hit(&ray, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * DVec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
    }

    let unit_direction = ray.dir.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
}

struct Ray {
    origin: DVec3,
    dir: DVec3,
}

impl Ray {
    fn at(&self, t: f64) -> DVec3 {
        return self.origin + t * self.dir;
    }
}

struct HitRecord {
    p: DVec3,
    normal: DVec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: DVec3) {
        self.front_face = ray.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

struct Sphere {
    center: DVec3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrt = discriminant.sqrt();

        let root = (-half_b - sqrt) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrt) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        return true;
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: DVec3::new(0.0, 0.0, 0.0),
            normal: DVec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
            }
        }

        return hit_anything;
    }
}
