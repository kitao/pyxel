use std::fs::File;
use std::io::Read;

use serde::Deserialize;

use crate::settings::TILE_SIZE;
use crate::tilemap::{ImageSource, Tilemap};
use crate::SharedTilemap;

#[derive(Debug, Deserialize)]
struct TilesetImage {
    width: u32,
}

#[derive(Debug, Deserialize)]
struct Tileset {
    tilewidth: u32,
    tileheight: u32,
    image: TilesetImage,
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
    #[serde(rename = "tileset", default)]
    tilesets: Vec<Tileset>,
    #[serde(rename = "layer", default)]
    layers: Vec<Layer>,
}

impl Tilemap {
    pub fn from_tmx(filename: &str, layer_index: u32) -> SharedTilemap {
        let mut file = File::open(filename).unwrap();
        let mut tmx_text = String::new();
        file.read_to_string(&mut tmx_text).unwrap();
        let tmx: TiledMapFile = serde_xml_rs::from_str(&tmx_text).unwrap();
        let tileset = &tmx.tilesets[0];
        assert!(
            tileset.tilewidth == TILE_SIZE && tileset.tileheight == TILE_SIZE,
            "Tile size of TMX file must be {}x{}",
            TILE_SIZE,
            TILE_SIZE
        );
        let image_width = tileset.image.width / TILE_SIZE;
        let layer = &tmx.layers[layer_index as usize];
        assert!(
            layer.data.encoding == "csv",
            "Encoding of TMX file must be CSV"
        );
        let layer_data: Vec<u32> = layer
            .data
            .tiles
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let tilemap = Tilemap::new(layer.width, layer.height, ImageSource::Index(0));
        {
            let mut tilemap = tilemap.lock();
            for (i, tile_id) in layer_data.iter().enumerate() {
                let x = i % layer.width as usize;
                let y = i / layer.width as usize;
                let tile_x = (tile_id % image_width) as u8;
                let tile_y = (tile_id / image_width) as u8;
                tilemap.canvas.write_data(x, y, (tile_x, tile_y));
            }
        }
        tilemap
    }
}
