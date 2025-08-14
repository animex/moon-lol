use bevy::math::{Mat4, Quat, Vec3};

pub fn neg_mat_z(mat: &Mat4) -> Mat4 {
    let (scale, rotation, translation) = mat.to_scale_rotation_translation();

    Mat4::from_scale_rotation_translation(scale, neg_rotation_z(&rotation), neg_vec_z(&translation))
}

pub fn neg_vec_z(vec: &Vec3) -> Vec3 {
    Vec3::new(vec.x, vec.y, -vec.z)
}

pub fn neg_array_z(array: &[f32; 3]) -> [f32; 3] {
    [array[0], array[1], -array[2]]
}

pub fn neg_rotation_z(rotation: &Quat) -> Quat {
    Quat::from_xyzw(-rotation.x, -rotation.y, rotation.z, rotation.w)
}

pub fn reverse_indices(indices: &Vec<u16>) -> Vec<u16> {
    let mut reversed = indices.clone();
    reversed.reverse();
    reversed
}
