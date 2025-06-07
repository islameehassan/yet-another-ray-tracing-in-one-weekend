use std::rc::Rc;
use std::sync::Arc;
//use std::sync::Arc;

use crate::ray::ray::Ray;
use crate::utils::interval::Interval;
use crate::vec3::vec3::*;
use crate::world::hit_record::HitRecord;
use crate::world::material::*;

#[derive(Debug, Clone)]
pub enum SceneObject {
    Sphere {
        radius: f32,
        center: Point3,
        material: Arc<Material>,
    },
    Cylinder {
        radius: f32,
        center: Point3,
        height: f32,
        material: Arc<Material>,
    },
}

impl SceneObject {
    #[inline]
    fn hit_sphere(
        radius: f32,
        center: Point3,
        ray: &Ray,
        ray_t: Interval,
        rec: &mut HitRecord,
    ) -> bool {
        let a = ray.direction().length_squared();
        let center_origin_vec = center - ray.origin();
        let h = Vec3::dot_product(ray.direction(), center_origin_vec);
        let c = center_origin_vec.length_squared() - radius * radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        if ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(root);
        let outward_normal = (rec.p - center) / radius;
        rec.set_face_normal(ray, outward_normal);
        true
    }

    #[inline]
    fn hit_cylinder(
        radius: f32,
        center: Point3,
        height: f32,
        ray: &Ray,
        ray_t: Interval,
        rec: &mut HitRecord,
    ) -> bool {
        let oc = center - ray.origin();
        let a = ray.direction().x * ray.direction().x + ray.direction().z * ray.direction().z;
        let h = ray.direction().x * oc.x + ray.direction().z * oc.z;
        let c = oc.x * oc.x + oc.z * oc.z - radius * radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let half_height = height / 2.0;
        let y_min = center.y - half_height;
        let y_max = center.y + half_height;

        let mut root = (h - sqrtd) / a;
        let mut hit_point = ray.at(root);

        if ray_t.surrounds(root) || hit_point.y < y_min || hit_point.y > y_max {
            root = (h + sqrtd) / a;
            hit_point = ray.at(root);
            if ray_t.surrounds(root) || hit_point.y < y_min || hit_point.y > y_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = hit_point;
        let outward_normal = Vec3::new(
            (rec.p.x - center.x) / radius,
            0.0, // Y component is 0 for cylinder sides
            (rec.p.z - center.z) / radius,
        );
        rec.set_face_normal(ray, outward_normal);
        true
    }

    #[inline]
    pub fn hit(&self, r: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        match self {
            Self::Sphere {
                radius,
                center,
                material,
            } => {
                if Self::hit_sphere(*radius, *center, r, interval, rec) {
                    rec.material = Arc::clone(&material);
                    true
                } else {
                    false
                }
            }
            Self::Cylinder {
                radius,
                center,
                height,
                material,
            } => {
                if Self::hit_cylinder(*radius, *center, *height, r, interval, rec) {
                    rec.material = Arc::clone(&material);
                    true
                } else {
                    false
                }
            }
        }
    }
}
