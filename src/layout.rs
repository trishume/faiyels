use particle_renderer::{Instance};
use std::io::prelude::*;
use std::path::{Path};
use walkdir::WalkDir;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Theme, Color};
use syntect::easy::HighlightFile;

fn color_to_u32(c: Color) -> u32 {
    ((c.r as u32) << (3*8)) |
    ((c.g as u32) << (2*8)) |
    ((c.b as u32) << (1*8)) |
    ((c.a as u32) << (0*8))
}

pub fn layout_file_at(path: &Path, x: f32, y: f32, v: &mut Vec<Instance>, ss: &SyntaxSet, theme: &Theme) {
    let offset = 1.0 + 0.4; // size + gap

    let x_begin = x+0.5;
    let mut translate = [x_begin, y+0.5];

    let mut highlighter = HighlightFile::new(path, &ss, theme).unwrap();
    for maybe_line in highlighter.reader.lines() {
        let line = maybe_line.unwrap();
        let regions = highlighter.highlight_lines.highlight(&line);
        let mut column = 0;
        for &(ref style, s) in &regions {
            for c in s.chars() {
                if !c.is_whitespace() {
                    v.push(Instance {
                        translate: translate,
                        color: color_to_u32(style.foreground),
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
        }
        translate[0] = x_begin;
        translate[1] -= offset;
    }
}

pub fn layout_dir(path: &Path) -> Vec<Instance> {
    let ss = SyntaxSet::load_defaults_nonewlines();
    let ts = ThemeSet::load_defaults();
    let mut v = Vec::new();
    let mut x = 0.0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
        if entry.file_type().is_file() {
            layout_file_at(entry.path(), x, 0.0, &mut v, &ss, &ts.themes["base16-ocean.dark"]);
            x += 142.0;
        }
    }
    return v;
}
