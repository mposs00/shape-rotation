use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

union PackedFloat {
    packed: std::arch::x86_64::__m128,
    unpacked: [f32; 4]
}

struct HardFloat;

use std::arch::x86_64::*;
impl HardFloat {
    #[cfg(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            not(target_feature = "sse4.1")
        )
    )]
    #[cfg(target_arch = "x86_64")]
    fn dot_product3(lhs: Vec3f, rhs: Vec3f) -> f32 {
        unsafe {
            let lhs_packed = _mm_set_ps(lhs.x, lhs.y, lhs.z, 0.);
            let rhs_packed = _mm_set_ps(rhs.x, rhs.y, rhs.z, 0.);

            let r1 = _mm_mul_ps(lhs_packed, rhs_packed);
            let r2 = _mm_hadd_ps(r1, r1);
            let r3 = _mm_hadd_ps(r2, r2);
            let mut result: f32 = 0.;
            let ptr_result: *mut f32 = &mut result;
            _mm_store_ss(ptr_result, r3);
            result
        }
    }

    #[cfg(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse4.1"
        )
    )]
    #[cfg(target_arch = "x86_64")]
    fn dot_product3(lhs: Vec3f, rhs: Vec3f) -> f32 {
        unsafe {
            let lhs_packed = _mm_set_ps(lhs.x, lhs.y, lhs.z, 0.);
            let rhs_packed = _mm_set_ps(rhs.x, rhs.y, rhs.z, 0.);

            let result = PackedFloat { packed: _mm_dp_ps::<0xff>(lhs_packed, rhs_packed) };

            result.unpacked[0]
        }
    }

    fn add3(lhs: Vec3f, rhs: Vec3f) -> Vec3f {
        unsafe {
            let lhs_packed = _mm_set_ps(lhs.x, lhs.y, lhs.z, 0.);
            let rhs_packed = _mm_set_ps(rhs.x, rhs.y, rhs.z, 0.);
            
            let result_packed = PackedFloat { packed: _mm_add_ps(lhs_packed, rhs_packed) };

            Vec3f {
                x: result_packed.unpacked[3],
                y: result_packed.unpacked[2],
                z: result_packed.unpacked[1]
            }
        }
    }

    fn mul3(lhs: Vec3f, rhs: Vec3f) -> Vec3f {
        unsafe {
            let lhs_packed = _mm_set_ps(lhs.x, lhs.y, lhs.z, 0.);
            let rhs_packed = _mm_set_ps(rhs.x, rhs.y, rhs.z, 0.);
            
            let result_packed = PackedFloat { packed: _mm_mul_ps(lhs_packed, rhs_packed) };

            Vec3f {
                x: result_packed.unpacked[3],
                y: result_packed.unpacked[2],
                z: result_packed.unpacked[1]
            }
        }
    }

    fn div3(lhs: Vec3f, rhs: Vec3f) -> Vec3f {
        unsafe {
            let lhs_packed = _mm_set_ps(lhs.x, lhs.y, lhs.z, 0.);
            let rhs_packed = _mm_set_ps(rhs.x, rhs.y, rhs.z, 0.);
            
            let result_packed = PackedFloat { packed: _mm_div_ps(lhs_packed, rhs_packed) };

            Vec3f {
                x: result_packed.unpacked[3],
                y: result_packed.unpacked[2],
                z: result_packed.unpacked[1]
            }
        }
    }

    fn sub3(lhs: Vec3f, rhs: Vec3f) -> Vec3f {
        unsafe {
            let lhs_packed = _mm_set_ps(lhs.x, lhs.y, lhs.z, 0.);
            let rhs_packed = _mm_set_ps(rhs.x, rhs.y, rhs.z, 0.);
            
            let result_packed = PackedFloat { packed: _mm_sub_ps(lhs_packed, rhs_packed) };

            Vec3f {
                x: result_packed.unpacked[3],
                y: result_packed.unpacked[2],
                z: result_packed.unpacked[1]
            }
        }
    }
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn zero() -> Vec3f {
        Vec3f {
            x: 0.,
            y: 0.,
            z: 0.
        }
    }

    pub fn reflect(i: Vec3f, n: Vec3f) -> Vec3f {
        i - (n * ((i * n) * 2.))            
    }

    pub fn normalize(self) -> Vec3f {
        let norm = self.norm();
        HardFloat::div3(self, Vec3f::new(norm, norm, norm))
    }

    pub fn norm(self) -> f32 {
        f32::sqrt(HardFloat::dot_product3(self, self))
    }

    pub fn to_vec(self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }
}

impl ops::Sub<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Vec3f) -> Vec3f {
        HardFloat::sub3(self, rhs)
    }
}

impl ops::Add<Vec3f> for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Vec3f) -> Vec3f {
        HardFloat::add3(self, rhs)
    }
}

impl ops::Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, rhs: f32) -> Vec3f {
        HardFloat::mul3(self, Vec3f::new(rhs, rhs, rhs))
    }
}

impl ops::Mul<Vec3f> for Vec3f {
    type Output = f32;

    fn mul(self, rhs: Vec3f) -> f32 {
        HardFloat::dot_product3(self, rhs)
    }
}