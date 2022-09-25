use std::{time::Duration, str::FromStr, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{DefaultMapNote, vulnus};

#[derive(Debug)]
pub struct Map {
    pub(crate) song_id: u64,
    pub(crate) notes : Vec<MapNote>
}

pub enum ParseSoundSpaceMapErr {
    InvalidSyntax,
    UnknownFailure
}

impl Display for ParseSoundSpaceMapErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseSoundSpaceMapErr::InvalidSyntax => write!(f, "Invalid Syntax"),
            ParseSoundSpaceMapErr::UnknownFailure => write!(f, "Unknown Failure")
        }
    }
}


impl FromStr for Map {
    type Err = ParseSoundSpaceMapErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut each_note = s.split(',');
        let song_id = each_note.next().unwrap().parse::<u64>().or(Err(ParseSoundSpaceMapErr::InvalidSyntax))?;
        let notes : Vec<MapNote> = each_note.map(|n| -> Result<MapNote,ParseSoundSpaceMapErr> {
            let nn = n.split('|');
            let mut take_3 = nn.take(3);
            let x = take_3.next().ok_or(ParseSoundSpaceMapErr::InvalidSyntax)?.parse::<f32>().or(Err(ParseSoundSpaceMapErr::InvalidSyntax))?;
            let y = take_3.next().ok_or(ParseSoundSpaceMapErr::InvalidSyntax)?.parse::<f32>().or(Err(ParseSoundSpaceMapErr::InvalidSyntax))?;
            let t = take_3.next().ok_or(ParseSoundSpaceMapErr::InvalidSyntax)?.parse::<f64>().or(Err(ParseSoundSpaceMapErr::InvalidSyntax))?;
            Ok(MapNote {
                x,
                y,
                time:t
            })
        } )
        .flatten() // unwrap
        .collect();

        Ok(Map {
            song_id,
            notes
        })

        // Err(ParseSoundSpaceMapErr::UnknownFailure)
    }
}
impl From<vulnus::Map> for Map {
    fn from(m: vulnus::Map) -> Self {
        Map {
            song_id: 0,
            notes: m.notes.into_iter().map(|n| MapNote::from(n)).collect()
        }
    }
}

impl Map {

    pub fn set_offset(&mut self, offset: Duration) {
        self.notes.iter_mut().for_each(|note| note.set_offset(offset));
    }
    pub fn resize(&mut self, by: i32) {
        self.notes.iter_mut().for_each(|note| note.resize(by));
    }
    pub fn set_song_id(&mut self, song_id: u64) {
        self.song_id = song_id;
    }
    pub fn get_song_id(&self) -> u64 {
        self.song_id
    }
    pub fn get_notes(&self) -> &Vec<MapNote> {
        &self.notes
    }
    
}
impl ToString for Map {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.song_id.to_string());
        s.push(',');
        s.push_str(&self.notes.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","));
        s
    }
}

#[derive(Debug)]

pub struct MapNote {
    pub(crate) time: f64,
    pub(crate) x:f32,
    pub(crate) y:f32
}

impl From<vulnus::MapNote> for MapNote {
    fn from(n: vulnus::MapNote) -> Self {
        MapNote {
            time: n.time,
            x: n.x,
            y: n.y
        }
    }
}
impl ToString for MapNote {
    fn to_string(&self) -> String {
        format!("{:.2}|{:.2}|{:.2}", self.x, self.y, self.time)
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