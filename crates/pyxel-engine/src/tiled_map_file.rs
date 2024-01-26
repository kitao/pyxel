use std::fs::File;
use std::io::Read;

use serde::Deserialize;

use crate::settings::TILE_SIZE;
use crate::tilemap::{ImageSource, Tilemap};
use crate::utils::remove_whitespace;
use crate::SharedTilemap;

#[derive(Debug, Deserialize)]
struct Tileset {
    firstgid: u32,
    columns: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct LayerData {
    #[serde(rename = "encoding")]
    encoding: String,
    #[serde(rename = "$value")]
    tiles: String,
}

#[derive(Debug, Deserialize)]
struct Layer {
    width: u32,
    height: u32,
    data: LayerData,
}

#[derive(Debug, Deserialize)]
struct TiledMapFile {
    tilewidth: u32,
    tileheight: u32,
    #[serde(rename = "tileset", default)]
    tilesets: Vec<Tileset>,
    #[serde(rename = "layer", default)]
    layers: Vec<Layer>,
}

impl Tilemap {
    pub fn from_tmx(filename: &str, layer_index: u32) -> SharedTilemap {
        let file = File::open(filename);
        if file.is_err() {
            println!("Failed to open '{filename}'");
            return Self::new(1, 1, ImageSource::Index(0));
        }
        let mut file = file.unwrap();
        let mut tmx_text = String::new();
        let blank_tilemap = Self::new(1, 1, ImageSource::Index(0));
        if file.read_to_string(&mut tmx_text).is_err() {
            println!("Failed to read TMX file");
            return blank_tilemap;
        }
        let tmx = serde_xml_rs::from_str(&tmx_text);
        if tmx.is_err() {
            println!("Failed to parse TMX file");
            return blank_tilemap;
        }
        let tmx: TiledMapFile = tmx.unwrap();
        if tmx.tilewidth != TILE_SIZE || tmx.tileheight != TILE_SIZE {
            println!("TMX file's tile size is not {TILE_SIZE}x{TILE_SIZE}");
            return blank_tilemap;
        }
        if tmx.tilesets.is_empty() {
            println!("Tileset not found in TMX file");
            return blank_tilemap;
        }
        let tileset = &tmx.tilesets[0];
        if tileset.columns.is_none() {
            println!("Tileset is not embedded in TMX file");
            return blank_tilemap;
        }
        let tileset_columns = tileset.columns.unwrap();
        if layer_index >= tmx.layers.len() as u32 {
            println!("Layer {layer_index} not found in TMX file");
            return blank_tilemap;
        };
        let layer = &tmx.layers[layer_index as usize];
        if layer.data.encoding != "csv" {
            println!("TMX file's encoding is not CSV");
            return blank_tilemap;
        }
        let layer_data: Vec<u32> = remove_whitespace(&layer.data.tiles)
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let tilemap = Self::new(layer.width, layer.height, ImageSource::Index(0));
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
        tilemap
    }
}
