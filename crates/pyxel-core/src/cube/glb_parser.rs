use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;

use crate::cube::mat4::{Mat4, RcMat4};
use crate::cube::mesh::{ColImage, Material, Mesh, RcMesh};
use crate::cube::motion::{
    CubicQuatKey, CubicVec3Key, Motion, MotionChannel, MotionInterpolation, MotionTarget,
    MotionValues,
};
use crate::cube::primitive::{Primitive, RcPrimitive, MODE_TRIANGLES};
use crate::cube::quat::Quat;
use crate::cube::vec3::Vec3;
use crate::image::{Color, Image, RcImage, Rgb24};
use crate::settings::MAX_COLORS;

type TextureTint = [f32; 3];
type TextureTintKey = (u32, u32, u32);
type ImageCacheKey = (usize, TextureTintKey, Option<(i32, u32)>);

pub(super) fn parse_glb(filename: &str, colkey: Option<i32>, fps: f32) -> Result<RcMesh, String> {
    if !fps.is_finite() || fps <= 0.0 {
        return Err("GLB animation fps must be greater than 0".to_string());
    }

    let bytes = fs::read(filename).map_err(|_| format!("Failed to open file '{filename}'"))?;
    validate_glb_header(&bytes)?;
    let skip_animations = warn_glb_pre_import_features(&bytes)?;
    let import_bytes = sanitize_glb_for_import(&bytes)?;

    let (document, buffers, images) = gltf::import_slice(import_bytes.as_ref())
        .map_err(|e| format!("Failed to read GLB '{filename}': {e}"))?;
    validate_document(&document, images.len())?;
    let (materials, resolved_colkey) = import_materials(&document, &images, colkey)?;

    let mesh = Mesh::new();
    {
        let m = rc_mut!(&mesh);
        m.colkey = resolved_colkey;
        if let Some(material) = materials.first() {
            m.col_img = material.col_img.clone();
        }
        m.materials = materials;
    }

    let scene = document
        .default_scene()
        .or_else(|| document.scenes().next())
        .ok_or_else(|| "GLB has no scene".to_string())?;

    let mut node_parts = HashMap::<usize, usize>::new();
    for node in scene.nodes() {
        import_node(&mesh, &buffers, &mut node_parts, &node, -1)?;
    }

    if !skip_animations {
        import_animations(&mesh, &buffers, &node_parts, &document, fps)?;
    }
    rc_ref!(&mesh).validate()?;
    Ok(mesh)
}

fn warn_glb(message: &str) {
    eprintln!("Pyxel warning: {message}");
}

// Image conversion helpers

fn rgba8_to_pyxel_image(
    width: u32,
    height: u32,
    rgba: &[u8],
    colors: &[Rgb24],
    color_factor: TextureTint,
    alpha_mask: Option<(Color, f32)>,
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
    validate_palette(colors)?;

    let rc = Image::new(width, height);
    {
        let image = rc_mut!(rc);
        let mut color_table = HashMap::<(u8, u8, u8), Color>::with_capacity(256);

        for y in 0..height_usize {
            for x in 0..width_usize {
                let base = (y * width_usize + x) * 4;
                if let Some((color, cutoff)) = alpha_mask {
                    if is_masked_alpha(rgba[base + 3], cutoff) {
                        image.canvas.write_data(x, y, color);
                        continue;
                    }
                }
                let src_rgb = tinted_rgb(rgba, base, color_factor);
                let color = if let Some(color) = color_table.get(&src_rgb) {
                    *color
                } else {
                    let color = rgb_to_palette_color(src_rgb, colors)?;
                    color_table.insert(src_rgb, color);
                    color
                };
                image.canvas.write_data(x, y, color);
            }
        }
    }

    Ok(rc)
}

fn tinted_rgb(rgba: &[u8], base: usize, color_factor: TextureTint) -> (u8, u8, u8) {
    (
        (rgba[base] as f32 * color_factor[0]).round() as u8,
        (rgba[base + 1] as f32 * color_factor[1]).round() as u8,
        (rgba[base + 2] as f32 * color_factor[2]).round() as u8,
    )
}

// BLEND falls back to opaque silently here; validate_texture_usage warns
// once per material before import reaches this resolver.
fn mask_alpha_cutoff(material: &gltf::Material) -> Result<Option<f32>, String> {
    match material.alpha_mode() {
        gltf::material::AlphaMode::Opaque | gltf::material::AlphaMode::Blend => return Ok(None),
        gltf::material::AlphaMode::Mask => {}
    }

    let cutoff = material.alpha_cutoff().unwrap_or(0.5);
    if !cutoff.is_finite() || !(0.0..=1.0).contains(&cutoff) {
        return Err("GLB material alphaCutoff must be between 0.0 and 1.0".to_string());
    }
    Ok(Some(cutoff))
}

fn is_masked_alpha(alpha: u8, cutoff: f32) -> bool {
    (alpha as f32 / 255.0) < cutoff
}

fn validate_palette(colors: &[Rgb24]) -> Result<(), String> {
    if colors.is_empty() {
        return Err("Palette must contain at least one color".to_string());
    }
    if colors.len() > MAX_COLORS as usize {
        return Err(format!("Palette must contain at most {MAX_COLORS} colors"));
    }
    Ok(())
}

fn resolve_mask_colkey(
    colkey: Option<i32>,
    used_colors: &[bool],
    needs_colkey: bool,
) -> Option<i32> {
    let Some(colkey) = colkey else {
        if !needs_colkey {
            return None;
        }
        if let Some(index) = used_colors.iter().position(|used| !*used) {
            return Some(index as i32);
        }
        warn_glb("GLB alpha mask requires an unused colkey color; alpha mask is ignored");
        return None;
    };

    if !needs_colkey {
        return Some(colkey);
    }

    let max_colors = MAX_COLORS as i32;
    if !(0..max_colors).contains(&colkey) {
        warn_glb(&format!(
            "GLB alpha mask requires colkey between 0 and {}; selecting a fallback colkey",
            max_colors - 1
        ));
        return select_fallback_mask_colkey(used_colors);
    }
    if used_colors.get(colkey as usize).copied().unwrap_or(false) {
        warn_glb(
            "GLB alpha mask colkey collides with an opaque texture color; selecting a fallback colkey",
        );
        return select_fallback_mask_colkey(used_colors);
    }
    Some(colkey)
}

fn select_fallback_mask_colkey(used_colors: &[bool]) -> Option<i32> {
    if let Some(index) = used_colors.iter().position(|used| !*used) {
        return Some(index as i32);
    }
    warn_glb("GLB alpha mask requires an unused colkey color; alpha mask is ignored");
    None
}

fn mark_texture_palette_usage(
    width: u32,
    height: u32,
    rgba: &[u8],
    colors: &[Rgb24],
    color_factor: TextureTint,
    alpha_cutoff: Option<f32>,
    used_colors: &mut [bool],
) -> Result<bool, String> {
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

    let mut has_alpha_mask_pixels = false;
    for pixel_index in 0..pixel_count {
        let base = pixel_index * 4;
        if let Some(cutoff) = alpha_cutoff {
            if is_masked_alpha(rgba[base + 3], cutoff) {
                has_alpha_mask_pixels = true;
                continue;
            }
        }
        let color = rgb_to_palette_color(tinted_rgb(rgba, base, color_factor), colors)?;
        used_colors[color as usize] = true;
    }

    Ok(has_alpha_mask_pixels)
}

fn color_distance_sq(a: (u8, u8, u8), b: (u8, u8, u8)) -> f32 {
    let dr = a.0 as f32 - b.0 as f32;
    let dg = a.1 as f32 - b.1 as f32;
    let db = a.2 as f32 - b.2 as f32;
    dr * dr + dg * dg + db * db
}

fn rgb_to_palette_color(rgb: (u8, u8, u8), colors: &[Rgb24]) -> Result<Color, String> {
    validate_palette(colors)?;

    let mut closest_color: Color = 0;
    let mut closest_dist: f32 = f32::MAX;
    for (i, pal_color) in colors.iter().enumerate() {
        let pal_rgb = (
            (pal_color >> 16) as u8,
            (pal_color >> 8) as u8,
            *pal_color as u8,
        );
        let dist = color_distance_sq(rgb, pal_rgb);
        if dist < closest_dist {
            closest_color = i as Color;
            closest_dist = dist;
        }
    }
    Ok(closest_color)
}

// The factor-alpha warning lives in validate_texture_usage; these
// converters run in both material passes and stay silent.
fn base_color_factor_to_rgb(factor: [f32; 4]) -> Result<(u8, u8, u8), String> {
    if factor.iter().any(|component| !component.is_finite()) {
        return Err("GLB material baseColorFactor must be finite".to_string());
    }
    Ok((
        (factor[0].clamp(0.0, 1.0) * 255.0).round() as u8,
        (factor[1].clamp(0.0, 1.0) * 255.0).round() as u8,
        (factor[2].clamp(0.0, 1.0) * 255.0).round() as u8,
    ))
}

fn base_color_factor_to_tint(factor: [f32; 4]) -> Result<(TextureTint, TextureTintKey), String> {
    if factor.iter().any(|component| !component.is_finite()) {
        return Err("GLB material baseColorFactor must be finite".to_string());
    }

    let tint = [
        factor[0].clamp(0.0, 1.0),
        factor[1].clamp(0.0, 1.0),
        factor[2].clamp(0.0, 1.0),
    ];
    let key = (tint[0].to_bits(), tint[1].to_bits(), tint[2].to_bits());
    Ok((tint, key))
}

fn import_materials(
    document: &gltf::Document,
    images: &[gltf::image::Data],
    colkey: Option<i32>,
) -> Result<(Vec<Material>, Option<i32>), String> {
    let colors = crate::pyxel::colors();
    validate_palette(colors)?;

    let mut used_colors = vec![false; colors.len()];
    let mut needs_mask_colkey = false;
    for material in document.materials() {
        let pbr = material.pbr_metallic_roughness();
        let alpha_cutoff = mask_alpha_cutoff(&material)?;
        if let Some(texture_info) = pbr.base_color_texture() {
            let (tint, _) = base_color_factor_to_tint(pbr.base_color_factor())?;
            let image_index = texture_info.texture().source().index();
            if let Some(img) = images.get(image_index) {
                let rgba = image_to_rgba8(img)?;
                needs_mask_colkey |= mark_texture_palette_usage(
                    img.width,
                    img.height,
                    &rgba,
                    colors,
                    tint,
                    alpha_cutoff,
                    &mut used_colors,
                )?;
            } else {
                warn_glb("GLB base color texture image is missing; using flat material color");
                let rgb = base_color_factor_to_rgb(pbr.base_color_factor())?;
                let color = rgb_to_palette_color(rgb, colors)?;
                used_colors[color as usize] = true;
            }
        } else {
            let rgb = base_color_factor_to_rgb(pbr.base_color_factor())?;
            let color = rgb_to_palette_color(rgb, colors)?;
            used_colors[color as usize] = true;
        }
    }

    let resolved_colkey = resolve_mask_colkey(colkey, &used_colors, needs_mask_colkey);
    let mut image_cache = HashMap::<ImageCacheKey, RcImage>::new();
    let mut materials = Vec::new();

    // Resolve materials while reusing identical decoded images
    for material in document.materials() {
        let pbr = material.pbr_metallic_roughness();
        let alpha_cutoff = mask_alpha_cutoff(&material)?;
        let col_img = if let Some(texture_info) = pbr.base_color_texture() {
            let (tint, tint_key) = base_color_factor_to_tint(pbr.base_color_factor())?;
            let image_index = texture_info.texture().source().index();
            let mask_colkey = resolved_colkey.zip(alpha_cutoff);
            let cache_key = (
                image_index,
                tint_key,
                mask_colkey.map(|(colkey, cutoff)| (colkey, cutoff.to_bits())),
            );
            if let Some(img) = image_cache.get(&cache_key) {
                ColImage::Image(img.clone())
            } else if let Some(img) = images.get(image_index) {
                let rgba = image_to_rgba8(img)?;
                let image = rgba8_to_pyxel_image(
                    img.width,
                    img.height,
                    &rgba,
                    colors,
                    tint,
                    mask_colkey.map(|(colkey, cutoff)| (colkey as Color, cutoff)),
                )?;
                image_cache.insert(cache_key, image.clone());
                ColImage::Image(image)
            } else {
                warn_glb("GLB base color texture image is missing; using flat material color");
                let rgb = base_color_factor_to_rgb(pbr.base_color_factor())?;
                ColImage::Color(i32::from(rgb_to_palette_color(rgb, colors)?))
            }
        } else {
            let rgb = base_color_factor_to_rgb(pbr.base_color_factor())?;
            ColImage::Color(i32::from(rgb_to_palette_color(rgb, colors)?))
        };
        materials.push(Material {
            col_img,
            colkey: resolved_colkey,
        });
    }

    Ok((materials, resolved_colkey))
}

// Validation helpers

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

fn warn_glb_pre_import_features(bytes: &[u8]) -> Result<bool, String> {
    let Some(json_bytes) = glb_json_chunk(bytes)? else {
        return Ok(false);
    };
    let has_animation_pointer = json_bytes
        .windows(br#""KHR_animation_pointer""#.len())
        .any(|window| window == br#""KHR_animation_pointer""#);
    if has_animation_pointer {
        warn_glb(
            "GLB animation pointer/material animation is not supported; animations are ignored",
        );
    }
    Ok(has_animation_pointer)
}

fn glb_json_chunk(bytes: &[u8]) -> Result<Option<&[u8]>, String> {
    if bytes.len() < 20 {
        return Ok(None);
    }
    let json_len = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]) as usize;
    let json_type = [bytes[16], bytes[17], bytes[18], bytes[19]];
    if json_type != *b"JSON" {
        return Err("GLB first chunk must be JSON".to_string());
    }
    let json_end = 20_usize
        .checked_add(json_len)
        .ok_or_else(|| "GLB JSON chunk length overflows".to_string())?;
    if json_end > bytes.len() {
        return Err("GLB JSON chunk is truncated".to_string());
    }
    Ok(Some(&bytes[20..json_end]))
}

fn sanitize_glb_for_import(bytes: &[u8]) -> Result<Cow<'_, [u8]>, String> {
    let Some(json_bytes) = glb_json_chunk(bytes)? else {
        return Ok(Cow::Borrowed(bytes));
    };
    if !json_bytes
        .windows(br#""KHR_animation_pointer""#.len())
        .any(|window| window == br#""KHR_animation_pointer""#)
    {
        return Ok(Cow::Borrowed(bytes));
    }

    let json_len = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]) as usize;
    let json_start = 20;
    let json_end = json_start + json_len;
    let json = std::str::from_utf8(json_bytes)
        .map_err(|_| "GLB JSON chunk must be UTF-8".to_string())?
        .trim_end_matches([' ', '\0']);
    let json = remove_top_level_json_property(json, "animations")?;
    let json = remove_top_level_json_property(&json, "extensionsUsed")?;
    let json = remove_top_level_json_property(&json, "extensionsRequired")?;
    let mut new_json = json.into_bytes();
    let pad = (4 - new_json.len() % 4) % 4;
    new_json.extend(std::iter::repeat_n(b' ', pad));

    let total_len = 12 + 8 + new_json.len() + bytes.len() - json_end;
    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(&bytes[0..8]);
    out.extend_from_slice(&(total_len as u32).to_le_bytes());
    out.extend_from_slice(&(new_json.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&new_json);
    out.extend_from_slice(&bytes[json_end..]);
    Ok(Cow::Owned(out))
}

fn remove_top_level_json_property(json: &str, key: &str) -> Result<String, String> {
    let Some((start, end)) = top_level_json_property_span(json, key)? else {
        return Ok(json.to_string());
    };
    let mut out = String::with_capacity(json.len() - (end - start));
    out.push_str(&json[..start]);
    out.push_str(&json[end..]);
    Ok(out)
}

fn top_level_json_property_span(json: &str, key: &str) -> Result<Option<(usize, usize)>, String> {
    let bytes = json.as_bytes();
    let key_literal = format!("\"{key}\"");
    let key_bytes = key_literal.as_bytes();
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escape = false;
    let mut i = 0;

    // Scan top-level JSON tokens while respecting strings and nesting
    while i < bytes.len() {
        let b = bytes[i];
        if in_string {
            if escape {
                escape = false;
            } else if b == b'\\' {
                escape = true;
            } else if b == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        match b {
            b'"' => {
                if depth == 1 && bytes[i..].starts_with(key_bytes) {
                    let mut colon = skip_json_ws(bytes, i + key_bytes.len());
                    if bytes.get(colon) != Some(&b':') {
                        i += 1;
                        continue;
                    }
                    colon += 1;
                    let value_start = skip_json_ws(bytes, colon);
                    let value_end = skip_json_value(bytes, value_start)?;
                    let mut end = skip_json_ws(bytes, value_end);
                    let mut start = i;
                    let prev = previous_json_non_ws(bytes, start);
                    if let Some(prev) = prev.filter(|&prev| bytes[prev] == b',') {
                        start = prev;
                    } else {
                        let next = skip_json_ws(bytes, end);
                        if bytes.get(next) == Some(&b',') {
                            end = next + 1;
                        }
                    }
                    return Ok(Some((start, end)));
                }
                in_string = true;
            }
            b'{' | b'[' => depth += 1,
            b'}' | b']' => depth -= 1,
            _ => {}
        }
        i += 1;
    }
    Ok(None)
}

fn skip_json_ws(bytes: &[u8], mut i: usize) -> usize {
    while matches!(bytes.get(i), Some(b' ' | b'\n' | b'\r' | b'\t')) {
        i += 1;
    }
    i
}

fn previous_json_non_ws(bytes: &[u8], i: usize) -> Option<usize> {
    let mut i = i.checked_sub(1)?;
    while matches!(bytes[i], b' ' | b'\n' | b'\r' | b'\t') {
        i = i.checked_sub(1)?;
    }
    Some(i)
}

fn skip_json_value(bytes: &[u8], start: usize) -> Result<usize, String> {
    let Some(&first) = bytes.get(start) else {
        return Err("GLB JSON property is missing a value".to_string());
    };
    if first == b'"' {
        return skip_json_string(bytes, start);
    }
    if first != b'{' && first != b'[' {
        let mut i = start;
        while i < bytes.len() && !matches!(bytes[i], b',' | b'}' | b']') {
            i += 1;
        }
        return Ok(i);
    }

    let mut depth = 0i32;
    let mut in_string = false;
    let mut escape = false;
    for (i, &b) in bytes.iter().enumerate().skip(start) {
        if in_string {
            if escape {
                escape = false;
            } else if b == b'\\' {
                escape = true;
            } else if b == b'"' {
                in_string = false;
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'{' | b'[' => depth += 1,
            b'}' | b']' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(i + 1);
                }
            }
            _ => {}
        }
    }
    Err("GLB JSON property value is truncated".to_string())
}

fn skip_json_string(bytes: &[u8], start: usize) -> Result<usize, String> {
    let mut escape = false;
    for (i, &b) in bytes.iter().enumerate().skip(start + 1) {
        if escape {
            escape = false;
        } else if b == b'\\' {
            escape = true;
        } else if b == b'"' {
            return Ok(i + 1);
        }
    }
    Err("GLB JSON string value is truncated".to_string())
}

fn validate_document(document: &gltf::Document, image_count: usize) -> Result<(), String> {
    if document
        .extensions_used()
        .chain(document.extensions_required())
        .any(|extension| extension == "KHR_animation_pointer")
    {
        warn_glb(
            "GLB animation pointer/material animation is not supported; animations are ignored",
        );
    }
    if document.skins().next().is_some() {
        warn_glb("GLB skins are not supported; skinning is ignored");
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

    validate_mesh_features(document);
    validate_texture_usage(document, image_count);
    Ok(())
}

fn validate_mesh_features(document: &gltf::Document) {
    for mesh in document.meshes() {
        if mesh.weights().is_some() {
            warn_glb("GLB mesh morph targets are not supported; base mesh is used");
        }
        for primitive in mesh.primitives() {
            if primitive.material().index().is_none() {
                warn_glb("GLB primitive material is missing; default mesh material is used");
            }
            validate_primitive_attributes(&primitive);
            if primitive.morph_targets().next().is_some() {
                warn_glb("GLB mesh morph targets are not supported; base mesh is used");
            }
        }
    }

    for node in document.nodes() {
        if let gltf::scene::Transform::Matrix { .. } = node.transform() {
            warn_glb("GLB matrix node transforms are not supported; transform is decomposed");
        }
        if node.skin().is_some() {
            warn_glb("GLB skins are not supported; skinning is ignored");
        }
        if node.weights().is_some() {
            warn_glb("GLB node morph target weights are not supported; base mesh is used");
        }
    }
}

fn validate_primitive_attributes(primitive: &gltf::Primitive) {
    for (semantic, _) in primitive.attributes() {
        match semantic {
            gltf::mesh::Semantic::Positions
            | gltf::mesh::Semantic::Normals
            | gltf::mesh::Semantic::TexCoords(0) => {}
            _ => {
                warn_glb(&format!(
                    "GLB unsupported vertex attribute: {semantic:?}; attribute is ignored"
                ));
            }
        }
    }
}

fn validate_texture_usage(document: &gltf::Document, image_count: usize) {
    for material in document.materials() {
        if !matches!(
            material.alpha_mode(),
            gltf::material::AlphaMode::Opaque | gltf::material::AlphaMode::Mask
        ) {
            warn_glb("GLB material alpha mode is not supported; alpha is ignored");
        }
        let factor = material.pbr_metallic_roughness().base_color_factor();
        if factor[3].is_finite() && (factor[3] - 1.0).abs() > f32::EPSILON {
            warn_glb("GLB material baseColorFactor alpha is not supported; alpha is ignored");
        }
        if material.normal_texture().is_some()
            || material.occlusion_texture().is_some()
            || material.emissive_texture().is_some()
            || material
                .pbr_metallic_roughness()
                .metallic_roughness_texture()
                .is_some()
        {
            warn_glb("GLB unsupported texture usage; non-base-color textures are ignored");
        }
        if let Some(texture) = material.pbr_metallic_roughness().base_color_texture() {
            if texture.texture().source().index() >= image_count {
                warn_glb("GLB base color texture image is missing; using flat material color");
            }
        }
    }
}

// Image conversion helpers (continued)

fn image_to_rgba8(img: &gltf::image::Data) -> Result<Vec<u8>, String> {
    match img.format {
        gltf::image::Format::R8G8B8A8 => Ok(img.pixels.clone()),
        gltf::image::Format::R8G8B8 => {
            let mut rgba = Vec::with_capacity((img.width as usize) * (img.height as usize) * 4);
            for rgb in img.pixels.as_chunks::<3>().0 {
                rgba.extend_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
            }
            Ok(rgba)
        }
        gltf::image::Format::R8G8 => {
            let mut rgba = Vec::with_capacity((img.width as usize) * (img.height as usize) * 4);
            for rg in img.pixels.as_chunks::<2>().0 {
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

// Scene import

fn import_node(
    mesh: &RcMesh,
    buffers: &[gltf::buffer::Data],
    node_parts: &mut HashMap<usize, usize>,
    node: &gltf::Node,
    parent: i32,
) -> Result<(), String> {
    let node_name = node.name().unwrap_or("").to_string();
    let node_part = add_part(
        mesh,
        None,
        node_transform(node),
        parent,
        node_name.clone(),
        None,
    );
    node_parts.insert(node.index(), node_part);

    if let Some(gltf_mesh) = node.mesh() {
        for primitive in gltf_mesh.primitives() {
            let primitive_index = primitive.index();
            let name = if node_name.is_empty() {
                format!("primitive_{primitive_index}")
            } else {
                format!("{node_name}_primitive_{primitive_index}")
            };
            let material_index = primitive.material().index();
            let primitive = import_primitive(&primitive, buffers)?;
            add_part(
                mesh,
                Some(primitive),
                Mat4::identity(),
                node_part as i32,
                name,
                material_index,
            );
        }
    }

    for child in node.children() {
        import_node(mesh, buffers, node_parts, &child, node_part as i32)?;
    }
    Ok(())
}

fn add_part(
    mesh: &RcMesh,
    primitive: Option<RcPrimitive>,
    transform: RcMat4,
    parent: i32,
    name: String,
    material_index: Option<usize>,
) -> usize {
    let m = rc_mut!(mesh);
    let index = m.primitives.len();
    m.primitives.push(primitive);
    m.transforms.push(transform);
    m.parents.push(parent);
    m.names.push(name);
    m.material_indices.push(material_index);
    index
}

fn node_transform(node: &gltf::Node) -> RcMat4 {
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

// Primitive import

fn import_primitive(
    primitive: &gltf::Primitive,
    buffers: &[gltf::buffer::Data],
) -> Result<RcPrimitive, String> {
    if primitive.mode() != gltf::mesh::Mode::Triangles {
        return Err("GLB only triangle primitives are supported".to_string());
    }

    let has_texture = primitive
        .material()
        .pbr_metallic_roughness()
        .base_color_texture()
        .is_some();

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
    let vertex_normals = match reader.read_normals() {
        Some(normals) => {
            let normals = normals
                .flat_map(std::iter::IntoIterator::into_iter)
                .collect::<Vec<f32>>();
            if normals.len() % 3 != 0 || normals.len() / 3 != vertex_count {
                return Err("GLB NORMAL and POSITION count mismatch".to_string());
            }
            if normals.iter().any(|normal| !normal.is_finite()) {
                return Err("GLB NORMAL values must be finite".to_string());
            }
            Some(normals)
        }
        None => None,
    };

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
        if let Some(vertex_normals) = vertex_normals {
            p.normals = authored_normals_to_flat_normals(&vertex_normals, &p.indices, &p.normals)?;
        }
    }
    Ok(prim)
}

fn authored_normals_to_flat_normals(
    vertex_normals: &[f32],
    indices: &[i32],
    fallback_normals: &[f32],
) -> Result<Vec<f32>, String> {
    let vertex_count = vertex_normals.len() / 3;
    let face_count = if indices.is_empty() {
        vertex_count / 3
    } else {
        indices.len() / 3
    };
    let mut out = Vec::with_capacity(face_count * 3);

    for face_index in 0..face_count {
        let (a, b, c) = if indices.is_empty() {
            (face_index * 3, face_index * 3 + 1, face_index * 3 + 2)
        } else {
            (
                indices[face_index * 3] as usize,
                indices[face_index * 3 + 1] as usize,
                indices[face_index * 3 + 2] as usize,
            )
        };
        if a >= vertex_count || b >= vertex_count || c >= vertex_count {
            return Err("GLB primitive index exceeds NORMAL count".to_string());
        }

        let nx = vertex_normals[a * 3] + vertex_normals[b * 3] + vertex_normals[c * 3];
        let ny = vertex_normals[a * 3 + 1] + vertex_normals[b * 3 + 1] + vertex_normals[c * 3 + 1];
        let nz = vertex_normals[a * 3 + 2] + vertex_normals[b * 3 + 2] + vertex_normals[c * 3 + 2];
        let len = (nx * nx + ny * ny + nz * nz).sqrt();

        if len > f32::EPSILON {
            out.extend_from_slice(&[nx / len, ny / len, nz / len]);
        } else {
            let base = face_index * 3;
            out.extend_from_slice(&fallback_normals[base..base + 3]);
        }
    }

    Ok(out)
}

// Animation import

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

    // Import each animation and its target-property channels
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
                    gltf::animation::Interpolation::CubicSpline => MotionInterpolation::CubicSpline,
                    gltf::animation::Interpolation::Step => MotionInterpolation::Step,
                    gltf::animation::Interpolation::Linear => MotionInterpolation::Linear,
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
                // Property dispatch: read each channel's outputs as the value
                // kind its glTF target property declares.
                let (target, values) = match channel.target().property() {
                    gltf::animation::Property::Translation => {
                        let values = match reader.read_outputs() {
                            Some(gltf::animation::util::ReadOutputs::Translations(values)) => {
                                let values = values
                                    .map(|v| Vec3 {
                                        x: v[0],
                                        y: v[1],
                                        z: v[2],
                                    })
                                    .collect::<Vec<_>>();
                                if interpolation == MotionInterpolation::CubicSpline {
                                    MotionValues::CubicTranslations(cubic_vec3_keys(&values, fps)?)
                                } else {
                                    MotionValues::Translations(values)
                                }
                            }
                            _ => {
                                return Err(
                                    "GLB animation translation values are missing".to_string()
                                );
                            }
                        };
                        (MotionTarget::Translation, values)
                    }
                    gltf::animation::Property::Rotation => {
                        let values = match reader.read_outputs() {
                            Some(gltf::animation::util::ReadOutputs::Rotations(values)) => {
                                let values = values
                                    .into_f32()
                                    .map(|v| Quat {
                                        x: v[0],
                                        y: v[1],
                                        z: v[2],
                                        w: v[3],
                                    })
                                    .collect::<Vec<_>>();
                                if interpolation == MotionInterpolation::CubicSpline {
                                    MotionValues::CubicRotations(cubic_quat_keys(&values, fps)?)
                                } else {
                                    MotionValues::Rotations(values)
                                }
                            }
                            _ => {
                                return Err("GLB animation rotation values are missing".to_string());
                            }
                        };
                        (MotionTarget::Rotation, values)
                    }
                    gltf::animation::Property::Scale => {
                        let values = match reader.read_outputs() {
                            Some(gltf::animation::util::ReadOutputs::Scales(values)) => {
                                let values = values
                                    .map(|v| Vec3 {
                                        x: v[0],
                                        y: v[1],
                                        z: v[2],
                                    })
                                    .collect::<Vec<_>>();
                                if interpolation == MotionInterpolation::CubicSpline {
                                    MotionValues::CubicScales(cubic_vec3_keys(&values, fps)?)
                                } else {
                                    MotionValues::Scales(values)
                                }
                            }
                            _ => return Err("GLB animation scale values are missing".to_string()),
                        };
                        (MotionTarget::Scale, values)
                    }
                    gltf::animation::Property::MorphTargetWeights => {
                        return Err("GLB morph target animation is not supported".to_string());
                    }
                };
                let value_count = value_len(&values);
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

fn value_len(values: &MotionValues) -> usize {
    match values {
        MotionValues::Translations(values) | MotionValues::Scales(values) => values.len(),
        MotionValues::Rotations(values) => values.len(),
        MotionValues::CubicTranslations(values) | MotionValues::CubicScales(values) => values.len(),
        MotionValues::CubicRotations(values) => values.len(),
    }
}

fn cubic_vec3_keys(values: &[Vec3], fps: f32) -> Result<Vec<CubicVec3Key>, String> {
    let (chunks, remainder) = values.as_chunks::<3>();
    if !remainder.is_empty() {
        return Err("GLB cubic spline animation values must be tangent/value triples".to_string());
    }

    Ok(chunks
        .iter()
        .map(|chunk| CubicVec3Key {
            in_tangent: vec3_per_frame(&chunk[0], fps),
            value: chunk[1],
            out_tangent: vec3_per_frame(&chunk[2], fps),
        })
        .collect())
}

fn cubic_quat_keys(values: &[Quat], fps: f32) -> Result<Vec<CubicQuatKey>, String> {
    let (chunks, remainder) = values.as_chunks::<3>();
    if !remainder.is_empty() {
        return Err("GLB cubic spline animation values must be tangent/value triples".to_string());
    }

    Ok(chunks
        .iter()
        .map(|chunk| CubicQuatKey {
            in_tangent: quat_per_frame(&chunk[0], fps),
            value: chunk[1],
            out_tangent: quat_per_frame(&chunk[2], fps),
        })
        .collect())
}

fn vec3_per_frame(v: &Vec3, fps: f32) -> Vec3 {
    Vec3 {
        x: v.x / fps,
        y: v.y / fps,
        z: v.z / fps,
    }
}

fn quat_per_frame(q: &Quat, fps: f32) -> Quat {
    Quat {
        x: q.x / fps,
        y: q.y / fps,
        z: q.z / fps,
        w: q.w / fps,
    }
}
