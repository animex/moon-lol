use bevy::math::Mat4;

pub fn neg_mat_z(mat: &mut Mat4) {
    mat.x_axis.z = -mat.x_axis.z;
    mat.y_axis.z = -mat.y_axis.z;
    mat.z_axis.z = -mat.z_axis.z;
    mat.w_axis.z = -mat.w_axis.z;
}
