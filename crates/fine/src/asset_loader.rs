// use crate::render::prelude::{GeometryData, IndiceValues, VertexValues};
// use crate::transform::Transform;
// use glam::Mat4;
// use gltf::{buffer::Source, Gltf};
use std::path::Path;

pub fn load_gltf<P>(_path: P)
where
    P: AsRef<Path>,
{
    // let gltf = Gltf::open(path).expect("[gltf] Does not exist.");

    // // let mut geometries: Vec<GeometryData> = Vec::new();
    // let mut buffers_data: Vec<Vec<u8>> = Vec::new();

    // for buffer in gltf.buffers() {
    //     match buffer.source() {
    //         Source::Bin => {
    //             if let Some(blob) = gltf.blob.as_deref() {
    //                 buffers_data.push(blob.into());
    //             }
    //         }
    //         _ => unimplemented!(),
    //     }
    // }

    // for mesh in gltf.meshes() {
    //     for primitive in mesh.primitives() {
    //         geometries.push(create_geometry(&buffers_data, primitive));
    //     }
    // }

    // let mut world = World::new();
    // for scene in gltf.scenes() {
    //     let root = world.reserve_entity();
    //     world.insert_one(root, Transform::new()).unwrap();

    //     for node in scene.nodes() {
    //         let entity = world.reserve_entity();

    //         world
    //             .insert_one(
    //                 entity,
    //                 Transform::from_matrix4(Mat4::from_cols_array_2d(&node.transform().matrix())),
    //             )
    //             .unwrap();

    //         if let Some(mesh) = node.mesh() {
    //             let geometries: Vec<GeometryData> = mesh
    //                 .primitives()
    //                 .map(|primitive| create_geometry(&buffers_data, primitive))
    //                 .collect();
    //             world.insert_one(entity, geometries).unwrap();
    //         }
    //     }
    // }

    // world
}

// fn create_geometry(buffers_data: &Vec<Vec<u8>>, primitive: gltf::Primitive) -> GeometryData {
//     let reader = primitive.reader(|buffer| Some(&buffers_data[buffer.index()]));

//     let mut data = GeometryData::new();

//     if let Some(values) = reader
//         .read_positions()
//         .map(|v| VertexValues::Float3(v.collect()))
//     {
//         data.set_attribute(GeometryData::ATTRIBUTE_POSITION, values);
//     }

//     if let Some(values) = reader
//         .read_normals()
//         .map(|v| VertexValues::Float3(v.collect()))
//     {
//         data.set_attribute(GeometryData::ATTRIBUTE_NORMAL, values);
//     }

//     if let Some(values) = reader
//         .read_tex_coords(0)
//         .map(|v| VertexValues::Float2(v.into_f32().collect()))
//     {
//         data.set_attribute(GeometryData::ATTRIBUTE_TEX_COORD, values);
//     }

//     if let Some(values) = reader
//         .read_indices()
//         .map(|v| IndiceValues::U32(v.into_u32().collect()))
//     {
//         data.set_indices(Some(values));
//     }

//     data
// }

// fn create_node(buffers_data: &Vec<Vec<u8>>, node: gltf::Node, parent: &Entity, world: &mut World) {
//     let entity = world.reserve_entity();

//     world
//         .insert_one(
//             entity,
//             Transform::from_matrix4(Mat4::from_cols_array_2d(&node.transform().matrix())),
//         )
//         .unwrap();

//     if let Some(mesh) = node.mesh() {
//         let geometries: Vec<GeometryData> = mesh
//             .primitives()
//             .map(|primitive| create_geometry(&buffers_data, primitive))
//             .collect();
//         world.insert_one(entity, geometries).unwrap();
//     }

//     for node in node.children() {
//         create_node(buffers_data, node, &entity, world);
//     }
// }