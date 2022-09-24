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
    pub(crate) notes : Vec<MapNote>
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
}
impl From<soundspace::Map> for Map {
    fn from(m: soundspace::Map) -> Self {
        Map {
            approach_distance: 0.0,
            approach_time: 0.0,
            name: String::from(""),
            notes: m.notes.into_iter().map(|n| MapNote::from(n)).collect()
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