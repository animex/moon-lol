use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use league_utils::neg_array_z;

use crate::core::particle::{ATTRIBUTE_LIFETIME, ATTRIBUTE_UV_FRAME, ATTRIBUTE_WORLD_POSITION};

#[derive(Default)]
pub struct ParticleMeshQuad {}

impl From<ParticleMeshQuad> for Mesh {
    fn from(_value: ParticleMeshQuad) -> Self {
        // let mut mesh = Mesh::from(Cuboid::new(1.0, 1.0, 1.0));
        let mut mesh: Mesh = Plane3d::new(Vec3::NEG_Z, Vec2::splat(1.0)).into();

        let transform = Transform::from_rotation(Quat::from_rotation_z(PI / 2.));

        let VertexAttributeValues::Float32x3(values) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
        else {
            panic!();
        };

        let values = values
            .into_iter()
            .map(|v| transform.transform_point(Vec3::from_array(*v)))
            .collect::<Vec<_>>();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, values.clone());

        let VertexAttributeValues::Float32x3(values) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
        else {
            panic!();
        };

        let values = values
            .into_iter()
            .map(|v| neg_array_z(v))
            .collect::<Vec<_>>();

        mesh.insert_attribute(ATTRIBUTE_WORLD_POSITION, values.clone());

        let indices = mesh.indices_mut().unwrap();

        match indices {
            Indices::U16(items) => items.reverse(),
            Indices::U32(items) => items.reverse(),
        }

        let VertexAttributeValues::Float32x3(values) =
            mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap()
        else {
            panic!();
        };

        let values = values
            .into_iter()
            .map(|v| neg_array_z(v))
            .collect::<Vec<_>>();

        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, values.clone());

        let VertexAttributeValues::Float32x2(uv_values) =
            mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap()
        else {
            panic!();
        };

        let values = uv_values
            .into_iter()
            .map(|&v| [v[0], v[1], 0.0, 0.0])
            .collect::<Vec<_>>();

        mesh.insert_attribute(ATTRIBUTE_UV_FRAME, values);

        let values = Vec::from([[0.0; 2]; 4]);
        mesh.insert_attribute(ATTRIBUTE_LIFETIME, values);

        let values = Vec::from([[1.0; 4]; 4]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, values);

        mesh
    }
}
