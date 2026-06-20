use std::collections::HashMap;
use std::fs;

use crate::cube::mat4::Mat4;
use crate::cube::mesh::{ColImage, Mesh, RcMesh};
use crate::cube::motion::{Motion, MotionChannel, MotionInterpolation, MotionTarget, MotionValues};
use crate::cube::primitive::{Primitive, MODE_TRIANGLES};
use crate::cube::quat::Quat;
use crate::cube::vec3::Vec3;
use crate::image::{Color, Image, RcImage, Rgb24};
use crate::settings::MAX_COLORS;

pub(super) fn parse_glb(filename: &str, colkey: Option<i32>, fps: f32) -> Result<RcMesh, String> {
    if !fps.is_finite() || fps <= 0.0 {
        return Err("GLB animation fps must be greater than 0".to_string());
    }

    let bytes = fs::read(filename).map_err(|_| format!("Failed to open file '{filename}'"))?;
    validate_glb_header(&bytes)?;

    let (document, buffers, images) = gltf::import_slice(bytes.as_slice())
        .map_err(|e| format!("Failed to read GLB '{filename}': {e}"))?;
    validate_document(&document, images.len())?;

    let mesh = Mesh::new();
    {
        let m = rc_mut!(&mesh);
        m.colkey = colkey;
        if let Some(img) = images.first() {
            let rgba = image_to_rgba8(img)?;
            m.col_img = ColImage::Image(rgba8_to_pyxel_image(
                img.width,
                img.height,
                &rgba,
                crate::pyxel::colors(),
            )?);
        }
    }

    let scene = document
        .default_scene()
        .or_else(|| document.scenes().next())
        .ok_or_else(|| "GLB has no scene".to_string())?;

    let mut node_parts = HashMap::<usize, usize>::new();
    for node in scene.nodes() {
        import_node(
            &mesh,
            &buffers,
            &mut node_parts,
            &node,
            -1,
            images.len() == 1,
        )?;
    }

    import_animations(&mesh, &buffers, &node_parts, &document, fps)?;
    rc_ref!(&mesh).validate()?;
    Ok(mesh)
}

fn rgba8_to_pyxel_image(
    width: u32,
    height: u32,
    rgba: &[u8],
    colors: &[Rgb24],
) -> Result<RcImage, String> {
    let width_usize = width as usize;
    let height_usize = height as usize;
    let pixel_count = width_usize
        .checked_mul(height_usize)
        .ok_or_else(|| "GLB texture dimensions overflow".to_string())?;
    let expected_len = pixel_count
        .checked_mul(4)
        .ok_or_else(|| "GLB texture dimensions overflow".to_string())?;

    if rgba.len() != expected_len {
        return Err("GLB texture buffer length does not match image dimensions".to_string());
    }
    if colors.is_empty() {
        return Err("Palette must contain at least one color".to_string());
    }
    if colors.len() > MAX_COLORS as usize {
        return Err(format!("Palette must contain at most {MAX_COLORS} colors"));
    }
    if rgba.chunks_exact(4).any(|p| p[3] != 255) {
        return Err(
            "GLB texture alpha is not supported; paint a visible colkey color instead".to_string(),
        );
    }

    let rc = Image::new(width, height);
    {
        let image = rc_mut!(rc);
        let mut color_table = HashMap::<(u8, u8, u8), Color>::with_capacity(256);

        for y in 0..height_usize {
            for x in 0..width_usize {
                let base = (y * width_usize + x) * 4;
                let src_rgb = (rgba[base], rgba[base + 1], rgba[base + 2]);
                let color = if let Some(color) = color_table.get(&src_rgb) {
                    *color
                } else {
                    let mut closest_color: Color = 0;
                    let mut closest_dist: f32 = f32::MAX;
                    for (i, pal_color) in colors.iter().enumerate() {
                        let pal_rgb = (
                            (pal_color >> 16) as u8,
                            (pal_color >> 8) as u8,
                            *pal_color as u8,
                        );
                        let dist = color_distance_sq(src_rgb, pal_rgb);
                        if dist < closest_dist {
                            closest_color = i as Color;
                            closest_dist = dist;
                        }
                    }
                    color_table.insert(src_rgb, closest_color);
                    closest_color
                };
                image.canvas.write_data(x, y, color);
            }
        }
    }

    Ok(rc)
}

fn color_distance_sq(a: (u8, u8, u8), b: (u8, u8, u8)) -> f32 {
    let dr = a.0 as f32 - b.0 as f32;
    let dg = a.1 as f32 - b.1 as f32;
    let db = a.2 as f32 - b.2 as f32;
    dr * dr + dg * dg + db * db
}

fn validate_glb_header(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() < 4 || &bytes[0..4] != b"glTF" {
        return Err("GLB binary header is required".to_string());
    }
    if bytes.len() >= 8 {
        let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        if version != 2 {
            return Err("GLB version 2 is required".to_string());
        }
    }
    Ok(())
}

fn validate_document(document: &gltf::Document, image_count: usize) -> Result<(), String> {
    if document.textures().count() > 1 {
        return Err("GLB multiple textures are not supported".to_string());
    }
    if document.materials().count() > 1 {
        return Err("GLB multiple materials are not supported".to_string());
    }
    if document.skins().next().is_some() {
        return Err("GLB skins are not supported".to_string());
    }
    if image_count > 1 || document.images().count() > 1 {
        return Err("GLB multiple textures/images are not supported".to_string());
    }

    for buffer in document.buffers() {
        if let gltf::buffer::Source::Uri(_) = buffer.source() {
            return Err("GLB external buffers are not supported".to_string());
        }
    }
    for image in document.images() {
        match image.source() {
            gltf::image::Source::View { .. } => {}
            gltf::image::Source::Uri { .. } => {
                return Err("GLB external images are not supported".to_string());
            }
        }
    }

    validate_mesh_features(document)?;
    validate_texture_usage(document, image_count)?;
    Ok(())
}

fn validate_mesh_features(document: &gltf::Document) -> Result<(), String> {
    for mesh in document.meshes() {
        if mesh.weights().is_some() {
            return Err("GLB mesh morph targets are not supported".to_string());
        }
        for primitive in mesh.primitives() {
            if primitive.morph_targets().next().is_some() {
                return Err("GLB mesh morph targets are not supported".to_string());
            }
        }
    }

    for node in document.nodes() {
        if node.skin().is_some() {
            return Err("GLB skins are not supported".to_string());
        }
        if node.weights().is_some() {
            return Err("GLB node morph target weights are not supported".to_string());
        }
    }

    Ok(())
}

fn validate_texture_usage(document: &gltf::Document, image_count: usize) -> Result<(), String> {
    if document.textures().count() == 0 && image_count == 0 {
        return Ok(());
    }
    if document.textures().count() != 1 || image_count != 1 {
        return Err("GLB texture/image usage is not supported".to_string());
    }

    let Some(material) = document.materials().next() else {
        return Err("GLB embedded image requires a base color texture material".to_string());
    };
    if material.normal_texture().is_some()
        || material.occlusion_texture().is_some()
        || material.emissive_texture().is_some()
        || material
            .pbr_metallic_roughness()
            .metallic_roughness_texture()
            .is_some()
    {
        return Err("GLB unsupported texture usage".to_string());
    }
    if material
        .pbr_metallic_roughness()
        .base_color_texture()
        .is_none()
    {
        return Err(
            "GLB embedded image requires pbrMetallicRoughness.baseColorTexture".to_string(),
        );
    }

    Ok(())
}

fn image_to_rgba8(img: &gltf::image::Data) -> Result<Vec<u8>, String> {
    match img.format {
        gltf::image::Format::R8G8B8A8 => Ok(img.pixels.clone()),
        gltf::image::Format::R8G8B8 => {
            let mut rgba = Vec::with_capacity((img.width as usize) * (img.height as usize) * 4);
            for rgb in img.pixels.chunks_exact(3) {
                rgba.extend_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
            }
            Ok(rgba)
        }
        gltf::image::Format::R8G8 => {
            let mut rgba = Vec::with_capacity((img.width as usize) * (img.height as usize) * 4);
            for rg in img.pixels.chunks_exact(2) {
                rgba.extend_from_slice(&[rg[0], rg[0], rg[0], rg[1]]);
            }
            Ok(rgba)
        }
        gltf::image::Format::R8 => {
            let mut rgba = Vec::with_capacity((img.width as usize) * (img.height as usize) * 4);
            for &r in &img.pixels {
                rgba.extend_from_slice(&[r, r, r, 255]);
            }
            Ok(rgba)
        }
        _ => Err("GLB unsupported image format".to_string()),
    }
}

fn import_node(
    mesh: &RcMesh,
    buffers: &[gltf::buffer::Data],
    node_parts: &mut HashMap<usize, usize>,
    node: &gltf::Node,
    parent: i32,
    has_texture: bool,
) -> Result<(), String> {
    let node_name = node.name().unwrap_or("").to_string();
    let node_part = add_part(mesh, None, node_transform(node), parent, node_name.clone());
    node_parts.insert(node.index(), node_part);

    if let Some(gltf_mesh) = node.mesh() {
        for primitive in gltf_mesh.primitives() {
            let primitive_index = primitive.index();
            let name = if node_name.is_empty() {
                format!("primitive_{primitive_index}")
            } else {
                format!("{node_name}_primitive_{primitive_index}")
            };
            let primitive = import_primitive(&primitive, buffers, has_texture)?;
            add_part(
                mesh,
                Some(primitive),
                Mat4::identity(),
                node_part as i32,
                name,
            );
        }
    }

    for child in node.children() {
        import_node(
            mesh,
            buffers,
            node_parts,
            &child,
            node_part as i32,
            has_texture,
        )?;
    }
    Ok(())
}

fn add_part(
    mesh: &RcMesh,
    primitive: Option<crate::cube::primitive::RcPrimitive>,
    transform: crate::cube::mat4::RcMat4,
    parent: i32,
    name: String,
) -> usize {
    let m = rc_mut!(mesh);
    let index = m.primitives.len();
    m.primitives.push(primitive);
    m.transforms.push(transform);
    m.parents.push(parent);
    m.names.push(name);
    index
}

fn node_transform(node: &gltf::Node) -> crate::cube::mat4::RcMat4 {
    let (translation, rotation, scale) = node.transform().decomposed();
    let pos = Vec3 {
        x: translation[0],
        y: translation[1],
        z: translation[2],
    };
    let rot = Quat {
        x: rotation[0],
        y: rotation[1],
        z: rotation[2],
        w: rotation[3],
    };
    let scale = Vec3 {
        x: scale[0],
        y: scale[1],
        z: scale[2],
    };
    Mat4::compose(&pos, &rot, &scale)
}

fn import_primitive(
    primitive: &gltf::Primitive,
    buffers: &[gltf::buffer::Data],
    has_texture: bool,
) -> Result<crate::cube::primitive::RcPrimitive, String> {
    if primitive.mode() != gltf::mesh::Mode::Triangles {
        return Err("GLB only triangle primitives are supported".to_string());
    }

    if has_texture
        && primitive
            .material()
            .pbr_metallic_roughness()
            .base_color_texture()
            .is_none()
    {
        return Err("GLB textured primitive is missing base color texture material".to_string());
    }

    let reader =
        primitive.reader(|buffer| buffers.get(buffer.index()).map(|data| data.0.as_slice()));
    let positions = reader
        .read_positions()
        .ok_or_else(|| "GLB primitive is missing POSITION".to_string())?
        .flat_map(std::iter::IntoIterator::into_iter)
        .collect::<Vec<f32>>();
    if positions.len() % 3 != 0 {
        return Err("GLB primitive POSITION length is not divisible by 3".to_string());
    }
    let vertex_count = positions.len() / 3;
    if vertex_count == 0 {
        return Err("GLB primitive POSITION count is zero".to_string());
    }

    let uvs = match reader.read_tex_coords(0) {
        Some(tex_coords) => tex_coords
            .into_f32()
            .flat_map(std::iter::IntoIterator::into_iter)
            .collect::<Vec<f32>>(),
        None if has_texture => return Err("GLB primitive is missing TEXCOORD_0".to_string()),
        None => Vec::new(),
    };
    if has_texture && (uvs.len() % 2 != 0 || uvs.len() / 2 != vertex_count) {
        return Err("GLB TEXCOORD_0 and POSITION count mismatch".to_string());
    }

    let indices = reader
        .read_indices()
        .map(|indices| indices.into_u32().map(|i| i as i32).collect::<Vec<i32>>())
        .unwrap_or_default();
    if indices.iter().any(|&index| index as usize >= vertex_count) {
        return Err("GLB primitive index exceeds POSITION count".to_string());
    }
    if indices.is_empty() {
        if !vertex_count.is_multiple_of(3) {
            return Err("GLB triangle vertex count must be a multiple of 3".to_string());
        }
    } else if !indices.len().is_multiple_of(3) {
        return Err("GLB triangle indices count must be a multiple of 3".to_string());
    }

    let prim = Primitive::new();
    {
        let p = rc_mut!(&prim);
        p.positions = positions;
        p.uvs = uvs;
        p.indices = indices;
        p.mode = MODE_TRIANGLES;
        p.compute_normals();
    }
    Ok(prim)
}

fn import_animations(
    mesh: &RcMesh,
    buffers: &[gltf::buffer::Data],
    node_parts: &HashMap<usize, usize>,
    document: &gltf::Document,
    fps: f32,
) -> Result<(), String> {
    let base_transforms = rc_ref!(mesh)
        .transforms
        .iter()
        .map(|transform| *rc_ref!(transform))
        .collect::<Vec<_>>();

    for animation in document.animations() {
        let motion = Motion::new(
            animation.name().unwrap_or("").to_string(),
            0.0,
            base_transforms.clone(),
        );
        {
            let m = rc_mut!(&motion);
            for channel in animation.channels() {
                let interpolation = match channel.sampler().interpolation() {
                    gltf::animation::Interpolation::Step => MotionInterpolation::Step,
                    gltf::animation::Interpolation::Linear => MotionInterpolation::Linear,
                    gltf::animation::Interpolation::CubicSpline => {
                        return Err("GLB cubic spline animation is not supported".to_string());
                    }
                };
                let reader = channel
                    .reader(|buffer| buffers.get(buffer.index()).map(|data| data.0.as_slice()));
                let inputs = reader
                    .read_inputs()
                    .ok_or_else(|| "GLB animation channel is missing input times".to_string())?
                    .map(|seconds| seconds * fps)
                    .collect::<Vec<f32>>();
                if inputs.is_empty() {
                    return Err("GLB animation channel has empty input keys".to_string());
                }
                if let Some(&last) = inputs.last() {
                    m.length = m.length.max(last);
                }

                let node_index = channel.target().node().index();
                let part_index = *node_parts.get(&node_index).ok_or_else(|| {
                    format!("GLB animation targets node {node_index} outside imported scene")
                })?;
                let (target, values) = match channel.target().property() {
                    gltf::animation::Property::Translation => {
                        let values = match reader.read_outputs() {
                            Some(gltf::animation::util::ReadOutputs::Translations(values)) => {
                                values
                                    .map(|v| Vec3 {
                                        x: v[0],
                                        y: v[1],
                                        z: v[2],
                                    })
                                    .collect()
                            }
                            _ => {
                                return Err(
                                    "GLB animation translation values are missing".to_string()
                                );
                            }
                        };
                        (
                            MotionTarget::Translation,
                            MotionValues::Translations(values),
                        )
                    }
                    gltf::animation::Property::Rotation => {
                        let values = match reader.read_outputs() {
                            Some(gltf::animation::util::ReadOutputs::Rotations(values)) => values
                                .into_f32()
                                .map(|v| Quat {
                                    x: v[0],
                                    y: v[1],
                                    z: v[2],
                                    w: v[3],
                                })
                                .collect(),
                            _ => {
                                return Err("GLB animation rotation values are missing".to_string());
                            }
                        };
                        (MotionTarget::Rotation, MotionValues::Rotations(values))
                    }
                    gltf::animation::Property::Scale => {
                        let values = match reader.read_outputs() {
                            Some(gltf::animation::util::ReadOutputs::Scales(values)) => values
                                .map(|v| Vec3 {
                                    x: v[0],
                                    y: v[1],
                                    z: v[2],
                                })
                                .collect(),
                            _ => return Err("GLB animation scale values are missing".to_string()),
                        };
                        (MotionTarget::Scale, MotionValues::Scales(values))
                    }
                    gltf::animation::Property::MorphTargetWeights => {
                        return Err("GLB morph target animation is not supported".to_string());
                    }
                };
                let value_count = motion_value_len(&values);
                if value_count != inputs.len() {
                    return Err(format!(
                        "GLB animation input/output counts mismatch: inputs={}, outputs={}",
                        inputs.len(),
                        value_count
                    ));
                }
                m.channels.push(MotionChannel {
                    part_index,
                    target,
                    inputs,
                    values,
                    interpolation,
                });
            }
        }
        rc_mut!(mesh).motions.push(motion);
    }
    Ok(())
}

fn motion_value_len(values: &MotionValues) -> usize {
    match values {
        MotionValues::Translations(values) | MotionValues::Scales(values) => values.len(),
        MotionValues::Rotations(values) => values.len(),
    }
}
