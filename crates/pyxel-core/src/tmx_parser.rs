use std::fs::File;
use std::io::Read;

use serde::Deserialize;

use crate::settings::TILE_SIZE;
use crate::tilemap::{ImageSource, ImageTileCoord, Tilemap};
use crate::utils::remove_whitespace;

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

pub fn parse_tmx(path: &str, layer_index: u32) -> Result<*mut Tilemap, String> {
    let err = |msg| format!("{msg} '{path}'");

    let mut file = File::open(path).map_err(|_| err("Failed to open file"))?;
    let mut tmx_text = String::new();
    file.read_to_string(&mut tmx_text)
        .map_err(|_| err("Failed to read file"))?;

    let tmx: TmxMap =
        serde_xml_rs::from_str(&tmx_text).map_err(|_| err("Failed to parse file"))?;

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

    let layer = tmx
        .layers
        .get(layer_index as usize)
        .ok_or_else(|| format!("Layer {layer_index} not found in file '{path}'"))?;
    if layer.data.encoding != "csv" {
        return Err(err("Unsupported encoding in file"));
    }

    let tile_ids: Vec<u32> = remove_whitespace(&layer.data.tiles)
        .split(',')
        .map(|s| s.parse::<u32>().map_err(|_| err("Failed to parse file")))
        .collect::<Result<_, _>>()?;

    let tilemap = Tilemap::new(layer.width, layer.height, ImageSource::Index(0));
    let tilemap_ref = unsafe { &mut *tilemap };
    for (y, row) in tile_ids.chunks(layer.width as usize).enumerate() {
        for (x, &id) in row.iter().enumerate() {
            let id = id.saturating_sub(tileset.firstgid);
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
    Ok(tilemap)
}
