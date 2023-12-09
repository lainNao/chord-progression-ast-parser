/*
 Generated by typeshare 1.7.0
*/

export type SectionMeta = 
	| { type: "section", value: string }
	| { type: "repeat", value: number };

export type ChordInfoMeta = 
	| { type: "key", value: Key };

export type ChordExpression = 
	| { type: "chord", value: Chord }
	| { type: "unIdentified", value?: undefined }
	| { type: "noChord", value?: undefined }
	| { type: "same", value?: undefined };

export interface ChordInfo {
	metaInfos: ChordInfoMeta[];
	chordExpression: ChordExpression;
	denominator?: string;
}

export type ChordBlock = ChordInfo[];

export interface Section {
	metaInfos: SectionMeta[];
	chordBlocks: ChordBlock[];
}

export type Ast = Section[];

export enum Base {
	A = "A",
	B = "B",
	C = "C",
	D = "D",
	E = "E",
	F = "F",
	G = "G",
}

export enum Accidental {
	Sharp = "#",
	Flat = "b",
}

export enum ChordType {
	Minor = "m",
	Major = "M",
	Augmented = "aug",
	Diminished = "dim",
}

export enum Extension {
	Two = "2",
	Three = "3",
	FlatThree = "b3",
	Four = "4",
	FlatFive = "b5",
	Five = "5",
	SharpFive = "#5",
	FlatSix = "b6",
	Six = "6",
	Seven = "7",
	FlatNine = "b9",
	Nine = "9",
	SharpNine = "#9",
	FlatEleven = "b11",
	Eleven = "11",
	SharpEleven = "#11",
	FlatThirteen = "b13",
	Thirteen = "13",
	SharpThirteen = "#13",
	MajorSeven = "M7",
	MajorNine = "M9",
	MajorEleven = "M11",
	MajorThirteen = "M13",
	Add9 = "add9",
	Add11 = "add11",
	Add13 = "add13",
	Sus2 = "sus2",
	Sus4 = "sus4",
	HalfDiminish = "o",
}

export interface ChordDetailed {
	base: Base;
	accidental?: Accidental;
	chordType: ChordType;
	extensions: Extension[];
}

export interface Chord {
	plain: string;
	detailed: ChordDetailed;
}

export enum Key {
	Cb_M = "Cb",
	Cb_m = "Cbm",
	C_M = "C",
	C_m = "Cm",
	Cs_M = "C#",
	Cs_m = "C#m",
	Db_M = "Db",
	Db_m = "Dbm",
	D_M = "D",
	D_m = "Dm",
	Ds_M = "D#",
	Ds_m = "D#m",
	Eb_M = "Eb",
	Eb_m = "Ebm",
	E_M = "E",
	E_m = "Em",
	Fb_M = "E#",
	Fb_m = "E#m",
	F_M = "F",
	F_m = "Fm",
	Fs_M = "F#",
	Fs_m = "F#m",
	Gb_M = "Gb",
	Gb_m = "Gbm",
	G_M = "G",
	G_m = "Gm",
	Gs_M = "G#",
	Gs_m = "G#m",
	Ab_M = "Ab",
	Ab_m = "Abm",
	A_M = "A",
	A_m = "Am",
	As_M = "A#",
	As_m = "A#m",
	Bb_M = "Bb",
	Bb_m = "Bbm",
	B_M = "B",
	B_m = "Bm",
}
