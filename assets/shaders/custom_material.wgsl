#import bevy_pbr::{
    mesh_functions,
    forward_io::{Vertex},
    view_transformations::position_world_to_clip,
    mesh_view_bindings::globals,
}

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var texture_texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;
@group(2) @binding(3) var particle_color_texture_texture: texture_2d<f32>;
@group(2) @binding(4) var particle_color_texture_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let base_color = textureSample(texture_texture, texture_sampler, mesh.uv);

    let sample_color = textureSample(particle_color_texture_texture, particle_color_texture_sampler, vec2(globals.time / 3, 0));

    let time_1 = globals.time ;

    let t_1 = cos((base_color.r * 50) + time_1);

    // let t_2 = cos((base_color.r * 100) + time_1 + 0.8);

    return (base_color.a) * t_1 * vec4<f32>(1.0, 1.0, 1.0, 1.0);
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

// -- 辅助函数：从四元数创建 4x4 旋转矩阵 --
// 输入的四元数 rotation: vec4<f32> 应为 (x, y, z, w) 格式
fn mat4x4_from_quat(rotation: vec4<f32>) -> mat4x4<f32> {
    let x = rotation.x;
    let y = rotation.y;
    let z = rotation.z;
    let w = rotation.w;

    let x2 = x * 2.0;
    let y2 = y * 2.0;
    let z2 = z * 2.0;

    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;

    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;

    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;

    // WGSL 的 mat4x4 构造函数是按列优先的
    // 所以我们按列来构建矩阵
    return mat4x4<f32>(
        // Column 0
        vec4<f32>(1.0 - (yy + zz), xy + wz, xz - wy, 0.0),
        // Column 1
        vec4<f32>(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0),
        // Column 2
        vec4<f32>(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0),
        // Column 3
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );
}


// -- 主函数：从缩放、旋转（四元数）、平移创建变换矩阵 --
fn from_scale_rotation_translation(
    scale: vec3<f32>,
    rotation: vec4<f32>,
    translation: vec3<f32>
) -> mat4x4<f32> {
    // 1. 从四元数创建旋转矩阵
    let rotation_mat = mat4x4_from_quat(rotation);

    // 2. 创建一个带有平移和缩放信息的矩阵
    // 我们直接将缩放因子放在对角线上，并将平移向量放在最后一列
    // 这样做可以减少一次矩阵乘法，效率更高
    let translation_and_scale_mat = mat4x4<f32>(
        // Column 0
        vec4<f32>(scale.x, 0.0, 0.0, 0.0),
        // Column 1
        vec4<f32>(0.0, scale.y, 0.0, 0.0),
        // Column 2
        vec4<f32>(0.0, 0.0, scale.z, 0.0),
        // Column 3
        vec4<f32>(translation.x, translation.y, translation.z, 1.0)
    );

    // 3. 将它们组合起来
    // 最终的变换矩阵 M = M_translate * M_rotate * M_scale
    // 由于我们已经将平移和缩放组合在一个矩阵中，我们可以通过以下方式高效地得到相同的结果
    return translation_and_scale_mat * rotation_mat;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // 1. 获取完整的 world_from_local 变换矩阵
    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    

    #ifdef IS_LOCAL_ORIENTATION
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(vertex.position, 1.0));
    #else
    let translation = world_from_local[3].xyz;
    var translation_only_matrix = from_scale_rotation_translation(vec3(1.0, 1.0, 1.0), vec4(0.0, 1.0, 0.0, 0.0), translation);
    out.world_position = translation_only_matrix * vec4(vertex.position, 1.0);
    #endif

    // 5. 将世界位置转换为裁剪空间位置
    out.clip_position = position_world_to_clip(out.world_position.xyz);
    
    // 6. 传递其他数据
    out.uv = vertex.uv;

    return out;

}