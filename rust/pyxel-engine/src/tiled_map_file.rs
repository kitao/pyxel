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
        macro_rules! assert_or_break {
            ($condition:expr, $fmt:expr $(,$arg:tt)*) => {
                if !$condition {
                    println!($fmt, $($arg)*);
                    break;
                }
            };
        }
        #[allow(clippy::never_loop)]
        loop {
            let file = File::open(filename);
            assert_or_break!(file.is_ok(), "Failed to open file '{filename}'");
            let mut file = file.unwrap();
            let mut tmx_text = String::new();
            assert_or_break!(
                file.read_to_string(&mut tmx_text).is_ok(),
                "Failed to read TMX file"
            );
            let tmx = serde_xml_rs::from_str(&tmx_text);
            assert_or_break!(tmx.is_ok(), "Failed to parse TMX file");
            let tmx: TiledMapFile = tmx.unwrap();
            assert_or_break!(
                tmx.tilewidth == TILE_SIZE && tmx.tileheight == TILE_SIZE,
                "TMX file's tile size is not {TILE_SIZE}x{TILE_SIZE}"
            );
            assert_or_break!(!tmx.tilesets.is_empty(), "Tileset not found in TMX file");
            let tileset = &tmx.tilesets[0];
            assert_or_break!(
                tileset.columns.is_some(),
                "Tileset is not embedded in TMX file"
            );
            let tileset_columns = tileset.columns.unwrap();
            assert_or_break!(
                layer_index < tmx.layers.len() as u32,
                "Layer {layer_index} not found in TMX file"
            );
            let layer = &tmx.layers[layer_index as usize];
            assert_or_break!(
                layer.data.encoding == "csv",
                "TMX file's encoding is not CSV"
            );
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
            return tilemap;
        }
        // Return a blank tilemap due to an error
        Self::new(1, 1, ImageSource::Index(0))
    }
}
