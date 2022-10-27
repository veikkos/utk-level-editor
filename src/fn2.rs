use std::fs::{metadata, File};
use std::io::Read;

#[derive(Debug)]
pub struct Line {
    pub x: u8,
    pub y: u8,
    pub width: u8,
}

#[derive(Debug)]
pub struct Character {
    pub width: u32,
    pub height: u32,
    pub lines: Vec<Line>,
}

pub type FN2 = Vec<Character>;

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let metadata = metadata(filename).expect("unable to read metadata");
    let size = metadata.len() as usize;
    let mut buffer = vec![0; size];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

pub fn load_font(filename: &str) -> FN2 {
    let data = get_file_as_byte_vec(filename);
    let mut font: FN2 = Vec::new();
    let mut offset: usize = 0x027D;
    let number_of_chars_to_parse = 92;
    'parsing: loop {
        let width = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let height = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let color_bytes = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        let line_bytes = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        offset += color_bytes as usize;

        let mut lines: Vec<Line> = Vec::new();
        for _line in 0..(line_bytes / 3) {
            let line = Line {
                x: data[offset],
                y: data[offset + 1],
                width: data[offset + 2],
            };
            if line.width > 0 {
                lines.push(line);
            }
            offset += 3;
        }

        font.push(Character {
            width,
            height,
            lines,
        });

        if font.len() == number_of_chars_to_parse {
            break 'parsing;
        }
    }
    font
}
