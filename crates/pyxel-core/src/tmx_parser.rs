use std::fs::File;
use std::io::Read;

use serde::Deserialize;

use crate::settings::TILE_SIZE;
use crate::tilemap::{ImageSource, ImageTileCoord, RcTilemap, Tilemap};
use crate::utils::remove_whitespace;

const TMX_TILE_FLAG_MASK: u32 = 0xf000_0000;

#[derive(Debug, Deserialize)]
struct Tileset {
    #[serde(rename = "@firstgid")]
    firstgid: u32,
    #[serde(rename = "@columns")]
    columns: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct LayerData {
    #[serde(rename = "@encoding")]
    encoding: String,
    #[serde(rename = "#text")]
    tiles: String,
}

#[derive(Debug, Deserialize)]
struct Layer {
    #[serde(rename = "@width")]
    width: u32,
    #[serde(rename = "@height")]
    height: u32,
    data: LayerData,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "map")]
struct TmxMap {
    #[serde(rename = "@tilewidth")]
    tilewidth: u32,
    #[serde(rename = "@tileheight")]
    tileheight: u32,
    #[serde(rename = "tileset", default)]
    tilesets: Vec<Tileset>,
    #[serde(rename = "layer", default)]
    layers: Vec<Layer>,
}

pub fn parse_tmx(path: &str, layer_index: u32) -> Result<RcTilemap, String> {
    let err = |msg| format!("{msg} '{path}'");

    let mut file = File::open(path).map_err(|_| err("Failed to open file"))?;
    let mut tmx_text = String::new();
    file.read_to_string(&mut tmx_text)
        .map_err(|_| err("Failed to read file"))?;

    let tmx: TmxMap = serde_xml_rs::from_str(&tmx_text).map_err(|_| err("Failed to parse file"))?;

    if tmx.tilewidth != TILE_SIZE || tmx.tileheight != TILE_SIZE {
        return Err(err("Invalid tile size in file"));
    }

    let tileset = tmx
        .tilesets
        .first()
        .ok_or_else(|| err("No tileset found in file"))?;
    let columns = tileset
        .columns
        .ok_or_else(|| err("No embedded tileset in file"))?;
    if columns == 0 {
        return Err(err("Invalid tileset columns in file"));
    }

    let layer = tmx
        .layers
        .get(layer_index as usize)
        .ok_or_else(|| format!("Layer {layer_index} not found in file '{path}'"))?;
    if layer.width == 0 || layer.height == 0 {
        return Err(err("Invalid layer dimensions in file"));
    }
    if layer.data.encoding != "csv" {
        return Err(err("Unsupported encoding in file"));
    }

    let tile_ids: Vec<u32> = remove_whitespace(&layer.data.tiles)
        .split(',')
        .map(|s| s.parse::<u32>().map_err(|_| err("Failed to parse file")))
        .collect::<Result<_, _>>()?;
    let expected_tile_count = usize::try_from(u64::from(layer.width) * u64::from(layer.height))
        .map_err(|_| err("Layer dimensions are too large in file"))?;
    if tile_ids.len() != expected_tile_count {
        return Err(err("Layer data size does not match dimensions in file"));
    }

    // Pyxel tiles store only image coordinates, so discard TMX transform flags.
    let tilemap = Tilemap::try_new(layer.width, layer.height, ImageSource::Index(0))
        .map_err(|_| err("Layer dimensions are too large in file"))?;
    let mut tilemap_ref = rc_mut!(tilemap);
    for (y, row) in tile_ids.chunks(layer.width as usize).enumerate() {
        for (x, &id) in row.iter().enumerate() {
            let id = (id & !TMX_TILE_FLAG_MASK).saturating_sub(tileset.firstgid);
            tilemap_ref.canvas.write_data(
                x,
                y,
                (
                    (id % columns) as ImageTileCoord,
                    (id / columns) as ImageTileCoord,
                ),
            );
        }
    }
    drop(tilemap_ref);
    Ok(tilemap)
}
