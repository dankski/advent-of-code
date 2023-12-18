use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Line {
  pub dst: u64,
  pub src: u64,
  pub range: u64,
}

#[derive(Debug, Clone)]
pub struct CategoryMap {
    pub name: String,
    pub lines: Vec<Line>,
}

impl CategoryMap {

  pub fn generate_mapping(&self) -> Vec<u64> {

    let mut max_source_dim: (u64, u64) = (0, 0); // (index, range)

    for line in &self.lines {
      if line.src  > max_source_dim.0 {
        max_source_dim = (line.src, line.range);
      }
    }

    let max_mapping_dim: u64 = max_source_dim.0 + max_source_dim.1;
    let mut mapping: Vec<u64> = (0.. max_mapping_dim).collect();
    
    for line in &self.lines {
      let dst_end_index: u64 = line.dst + line.range;
      let dst_values: Vec<u64> = (line.dst..dst_end_index).collect();
      let src_start_index: usize = line.src as usize;
      let src_end_index: usize = (line.src + line.range) as usize;
      mapping.splice(src_start_index..src_end_index, dst_values);
    }

    return mapping;
  }

  pub fn generate_map(&self) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    

    for line in &self.lines {
      let start_src = line.src;
      let end_src = line.src + line.range;
      let start_dst = line.dst;
      let end_dst = line.dst + line.range;
      let keys: Vec<u64> = (start_src..end_src).collect();
      let values: Vec<u64> = (start_dst..end_dst).collect();
      let keys_values: Vec<(u64,u64)> = keys.into_iter().zip(values.into_iter()).collect();

      for &(k,v) in &keys_values {
        map.insert(k, v);
      }
    }
    
    map
  }

}

