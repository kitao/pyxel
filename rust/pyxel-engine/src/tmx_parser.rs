use std::fs::File;
use std::io::Read;

use serde::Deserialize;

use crate::settings::TILE_SIZE;
use crate::tilemap::{ImageSource, Tilemap};
use crate::utils::remove_whitespace;
use crate::SharedTilemap;

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
struct TiledMapFile {
    #[serde(rename = "@tilewidth")]
    tilewidth: u32,
    #[serde(rename = "@tileheight")]
    tileheight: u32,
    #[serde(rename = "tileset", default)]
    tilesets: Vec<Tileset>,
    #[serde(rename = "layer", default)]
    layers: Vec<Layer>,
}

pub fn parse_tmx(filename: &str, layer_index: u32) -> Result<SharedTilemap, String> {
    let mut file = File::open(filename).map_err(|_| format!("Failed to open file '{filename}'"))?;

    let mut tmx_text = String::new();
    file.read_to_string(&mut tmx_text)
        .map_err(|_| "Failed to read TMX file".to_string())?;

    let tmx: TiledMapFile =
        serde_xml_rs::from_str(&tmx_text).map_err(|_| "Failed to parse TMX file".to_string())?;

    if tmx.tilewidth != TILE_SIZE || tmx.tileheight != TILE_SIZE {
        return Err(format!(
            "TMX file's tile size is not {TILE_SIZE}x{TILE_SIZE}"
        ));
    }

    if tmx.tilesets.is_empty() {
        return Err("Tileset not found in TMX file".to_string());
    }
    let tileset = &tmx.tilesets[0];
    let tileset_columns = tileset
        .columns
        .ok_or_else(|| "Tileset is not embedded in TMX file".to_string())?;

    if layer_index >= tmx.layers.len() as u32 {
        return Err(format!("Layer {layer_index} not found in TMX file"));
    }
    let layer = &tmx.layers[layer_index as usize];
    if layer.data.encoding != "csv" {
        return Err("TMX file's encoding is not CSV".to_string());
    }

    let layer_data: Vec<u32> = remove_whitespace(&layer.data.tiles)
        .split(',')
        .map(|s| {
            s.parse::<u32>()
                .map_err(|_| "Failed to parse CSV tile data".to_string())
        })
        .collect::<Result<_, _>>()?;

    let tilemap = Tilemap::new(layer.width, layer.height, ImageSource::Index(0));
    {
        let mut tilemap = tilemap.lock();
        for (i, tile_id) in layer_data.iter().enumerate() {
            let x = i % layer.width as usize;
            let y = i / layer.width as usize;
            let tile_id = if *tile_id > tileset.firstgid {
                tile_id - tileset.firstgid
            } else {
                0
            };
            let tile_x = (tile_id % tileset_columns) as u8;
            let tile_y = (tile_id / tileset_columns) as u8;
            tilemap.canvas.write_data(x, y, (tile_x, tile_y));
        }
    }

    Ok(tilemap)
}
