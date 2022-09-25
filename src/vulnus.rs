use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{DefaultMapNote, soundspace};

#[derive(Debug, Deserialize,Serialize)]
pub struct Map {
    #[serde(rename = "_approachDistance")] 
    pub(crate) approach_distance: f32,
    #[serde(rename = "_approachTime")] 
    pub(crate) approach_time: f32,
    #[serde(rename = "_name")] 
    pub(crate) name: String,
    #[serde(rename = "_notes")] 
    pub(crate) notes : Vec<MapNote>,
    #[serde(skip)]
    pub(crate) meta_data: Option<MetaData>

}
#[derive(Debug, Deserialize,Serialize)]
pub struct MetaData {
    #[serde(rename = "_artist")] 
    pub(crate) artist: String,
    #[serde(rename = "_difficulties")] 
    pub(crate) difficulties: Vec<String>,
    #[serde(rename = "_mappers")] 
    pub(crate) name: Vec<String>,
    #[serde(rename = "_music")] 
    pub(crate) music : String,
    #[serde(rename = "_title")] 
    pub(crate) title : String,
    #[serde(rename = "_version")] 
    pub(crate) version : u32,
}

impl MetaData {
    pub fn set_artist(&mut self, artist: String) {
        self.artist = artist;
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_music(&mut self, music: String) {
        self.music = music;
    }
    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }
    pub fn set_difficulties(&mut self, difficulties: Vec<String>) {
        self.difficulties = difficulties;
    }
    pub fn set_mappers(&mut self, mappers: Vec<String>) {
        self.name = mappers;
    }
    pub fn get_artist(&self) -> &String {
        &self.artist
    }
    pub fn get_title(&self) -> &String {
        &self.title
    }
    pub fn get_music(&self) -> &String {
        &self.music
    }
    pub fn get_version(&self) -> &u32 {
        &self.version
    }
    pub fn get_difficulties(&self) -> &Vec<String> {
        &self.difficulties
    }
    pub fn get_mappers(&self) -> &Vec<String> {
        &self.name
    }

}


impl Map {
    pub fn set_offset(&mut self, offset: Duration) {
        self.notes.iter_mut().for_each(|note| note.set_offset(offset));
    }
    pub fn resize(&mut self, by: i32) {
        self.notes.iter_mut().for_each(|note| note.resize(by));
    }
    pub fn set_ad(&mut self, ad: f32) {
        self.approach_distance = ad;
    }
    pub fn set_at(&mut self, at: f32) {
        self.approach_time = at;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_notes(&self) -> &Vec<MapNote> {
        &self.notes
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_ad(&self) -> f32 {
        self.approach_distance
    }
    pub fn get_at(&self) -> f32 {
        self.approach_time
    }

    // meta data
    pub fn add_metadata(&mut self, meta_data: MetaData) {
        self.meta_data = Some(meta_data);
    }

}
impl From<soundspace::Map> for Map {
    fn from(m: soundspace::Map) -> Self {
        Map {
            approach_distance: 0.0,
            approach_time: 0.0,
            name: String::from(""),
            notes: m.notes.into_iter().map(|n| MapNote::from(n)).collect(),
            meta_data: None
        }
    }
}

#[derive(Debug, Deserialize,Serialize)]

pub struct MapNote {
    #[serde(rename = "_time")] 
    pub(crate) time: f64,
    #[serde(rename = "_x")] 
    pub(crate) x:f32,
    #[serde(rename = "_y")] 
    pub(crate) y:f32
}

impl From<soundspace::MapNote> for MapNote {
    fn from(n: soundspace::MapNote) -> Self {
        MapNote {
            time: n.time,
            x: n.x,
            y: n.y
        }
    }
}

impl DefaultMapNote for MapNote {
    fn y_as_ref(&self) -> &f32 {
        &self.y
    }
    fn x_as_ref(&self) -> &f32 {
        &self.x
    }
    fn time_as_ref(&self) -> &f64 {
        &self.time
    }
    fn y_as_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    fn x_as_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    fn time_as_mut(&mut self) -> &mut f64 {
        &mut self.time
    }
}