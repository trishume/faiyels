use particle_renderer::{Instance};
pub fn layout_file_at(x: f32, y: f32, v: &mut Vec<Instance>) {
  let offset = 1.0 + 0.4; // size + gap

  let begin = 0.5;
  let mut translate = [x+begin, y+begin];

  let length = 1000 as usize;
  v.reserve(length*length);
  for x in 0..length {
      for y in 0..length {
          v.push(Instance {
              translate: translate,
              color: (((x*5) << 8) | ((y*5) << 16) | 0xFF) as u32
          });
          translate[1] += offset;
      }
      translate[1] = begin;
      translate[0] += offset;
  }
}
