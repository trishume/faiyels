use particle_renderer::{Instance};
use std::io::prelude::*;
use std::path::{Path};
use std::fs::File;
use std::io::BufReader;
use walkdir::WalkDir;

pub fn layout_file_at<T: BufRead>(input: T, x: f32, y: f32, v: &mut Vec<Instance>) {
    let offset = 1.0 + 0.4; // size + gap

    let x_begin = x+0.5;
    let mut translate = [x_begin, y+0.5];

    for line in input.lines() {
        let s = line.unwrap();
        let mut column = 0;
        for c in s.chars() {
            if !c.is_whitespace() {
                v.push(Instance {
                    translate: translate,
                    color: ((column << 8) | 0xFF00FF) as u32
                });
            }

            translate[0] += offset;
            column += 1;

            if column >= 100 {
                translate[0] = x_begin;
                translate[1] -= offset;
                column = 0;
            }
        }
        translate[0] = x_begin;
        translate[1] -= offset;
    }
}

pub fn layout_dir(path: &Path) -> Vec<Instance> {
    let mut v = Vec::new();
    let mut x = 0.0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
        if entry.file_type().is_file() {
            let f = BufReader::new(File::open(entry.path()).unwrap());
            layout_file_at(f, x, -(entry.depth() as f32)*5.0, &mut v);
            x += 140.0;
        }
    }
    return v;
}
