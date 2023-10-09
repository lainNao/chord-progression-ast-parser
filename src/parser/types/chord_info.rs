use super::chord::Chord;
use super::chord_info_meta::ChordInfoMeta;

#[derive(Debug, PartialEq, Clone)]
pub struct ChordInfo {
    pub meta_infos: Vec<ChordInfoMeta>,
    pub chord: ChordOrUnidentified,
    pub denominator: Option<String>, // 曖昧で扱いようが無いのでstring
}

#[derive(Debug, PartialEq, Clone)]
pub enum ChordOrUnidentified {
    Chord(Chord),
    Unidentified,
}