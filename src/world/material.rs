use crate::ray::ray::Ray;
use crate::utils::helpers::random_f32;
use crate::vec3::color3::Color3;
use crate::vec3::vec3::*;
use crate::world::hit_record::*;

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian { albedo: Color3 },
    Metal { albedo: Color3, fuzz: f32 },
    Dielectric { refraction_index: f32 },
}

impl Material {
    #[inline]
    pub fn scatter(
        &self,
        ray: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Color3,
        scattered: &mut Ray,
    ) -> bool {
        match *self {
            Self::Lambertian { albedo } => {
                *attenuation = albedo;
                self.scatter_lambertian(ray, hit_rec, scattered)
            }
            Self::Metal { albedo, fuzz } => {
                *attenuation = albedo;
                self.scatter_metal(ray, hit_rec, scattered, fuzz)
            }
            Self::Dielectric { refraction_index } => {
                *attenuation = Color3::new(1.0, 1.0, 1.0);
                self.scatter_dielectric(ray, hit_rec, scattered, refraction_index)
            }
        }
    }

    #[inline]
    fn scatter_lambertian(&self, ray: &Ray, hit_rec: &HitRecord, scattered: &mut Ray) -> bool {
        let dir = hit_rec.normal + Vec3::random_unit_vector_on_hemisphere(hit_rec.normal);
        *scattered = Ray::new(hit_rec.p, dir);
        true
    }

    #[inline]
    fn scatter_metal(
        &self,
        ray: &Ray,
        hit_rec: &HitRecord,
        scattered: &mut Ray,
        fuzz: f32,
    ) -> bool {
        let mut dir = Vec3::reflect(ray.direction(), hit_rec.normal);
        dir = dir + (fuzz * Vec3::random_unit_vector_on_hemisphere(hit_rec.normal));
        *scattered = Ray::new(hit_rec.p, dir);
        true
    }

    #[inline]
    fn scatter_dielectric(
        &self,
        ray: &Ray,
        hit_rec: &HitRecord,
        scattered: &mut Ray,
        refraction_index: f32,
    ) -> bool {
        let source_medium_ref_index = if hit_rec.front_face {
            1.0
        } else {
            refraction_index
        };
        let dest_medium_ref_index = if source_medium_ref_index == 1.0 {
            refraction_index
        } else {
            1.0
        };

        let unit_vector = Vec3::unit(ray.direction());
        let cos_theta: f32 = f32::min(Vec3::dot_product(-unit_vector, hit_rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = sin_theta * source_medium_ref_index / dest_medium_ref_index > 1.0;
        let reflectance = || {
            let r0 = (source_medium_ref_index - dest_medium_ref_index)
                / (source_medium_ref_index + dest_medium_ref_index);
            r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
        };

        let mut direction = Vec3::default();
        if cannot_refract || reflectance() > random_f32() {
            direction = Vec3::reflect(unit_vector, hit_rec.normal);
        } else {
            direction = Vec3::refract(
                unit_vector,
                hit_rec.normal,
                source_medium_ref_index,
                dest_medium_ref_index,
            )
        }
        *scattered = Ray::new(hit_rec.p, direction);
        true
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Color3::new(0.0, 0.0, 0.0),
        }
    }
}
