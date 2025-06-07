use crate::{ray::ray::Ray, utils::interval::Interval, world::scene_object::SceneObject};

use super::hit_record::HitRecord;

pub struct SceneObjectList {
    pub objects: Vec<SceneObject>,
}

impl SceneObjectList {
    pub fn new() -> Self {
        SceneObjectList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: SceneObject) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval, hit_rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut temp_interval = ray_t;

        let mut temp_rec = HitRecord::default();
        for object in &self.objects {
            if object.hit(ray, temp_interval, &mut temp_rec) {
                hit_anything = true;
                temp_interval.max = temp_rec.t;
                *hit_rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
