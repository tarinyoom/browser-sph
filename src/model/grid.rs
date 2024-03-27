use std::hash::Hash;

use itertools::izip;
use rustc_hash::FxHashMap;

use super::{Globals, Particle};

pub struct Grid<E> {
    pub anchor: Vec<f64>,
    pub cell_dims: Vec<f64>,
    pub n_cells: Vec<usize>,
    pub cells: Vec<Vec<E>>,
    pub neighbor_offsets: Vec<i32>,
    pub map: FxHashMap<E, Particle>,
}

impl<E: Eq + Hash + Copy> Grid<E> {
    pub fn new(g: &Globals) -> Self {
        let sz = izip!(&g.bounds_min, &g.bounds_max).map(|(min, max)| (max - min));
        let n_cells: Vec<usize> = sz
            .clone()
            .map(|len| f64::floor(len / g.radius) as usize)
            .collect();
        let cells = vec![vec![]; n_cells.iter().fold(1, |a, b| a * b)];
        let cell_dims = izip!(sz, &n_cells).map(|(a, b)| a / *b as f64).collect();
        let mut grid = Grid::<E> {
            anchor: g.bounds_min.clone(),
            cell_dims: cell_dims,
            n_cells: n_cells,
            cells: cells,
            neighbor_offsets: vec![],
            map: FxHashMap::default(),
        };
        grid.set_neighbor_offsets();
        grid
    }

    fn set_neighbor_offsets(&mut self) {
        self.neighbor_offsets = calculate_neighbor_idxs(self.anchor.len())
            .into_iter()
            .map(|c| self.flatten(c))
            .collect();
    }

    fn flatten(&self, coords: Vec<i32>) -> i32 {
        izip!(coords, &self.n_cells).fold(0, |acc, (len, n)| acc * *n as i32 + len)
    }

    fn hash(&self, p: &Particle) -> Vec<usize> {
        izip!(&p.position, &self.anchor, &self.cell_dims)
            .map(|(x, anchor, width)| get_region(*x, *anchor, *width))
            .collect()
    }

    pub fn fill<It: Iterator<Item = (E, Particle)>>(&mut self, iter: It) {
        self.map = iter.collect();
        self.cells = self.cells.iter().map(|_| vec![]).collect();
        for (id, p) in &self.map {
            let coords = self.hash(p).into_iter().map(|v| v as i32).collect();
            let i = self.flatten(coords);
            self.cells[i as usize].push(*id);
        }
    }

    pub fn neighbors(&self, id: E) -> Vec<&Particle> {
        let p = self.map.get(&id).unwrap();
        let coords = self.hash(p).into_iter().map(|v| v as i32).collect();
        let i = self.flatten(coords);
        self.neighbor_offsets
            .iter()
            .map(|n| *n + i)
            .filter(|n| 0 <= *n && *n < self.cells.len() as i32)
            .map(|i| self.cells[i as usize].clone())
            .flatten()
            .filter(|e_| *e_ != id)
            .map(|e| self.map.get(&e).unwrap())
            .collect()
    }
}

fn calculate_neighbor_idxs(dim: usize) -> Vec<Vec<i32>> {
    if dim == 0 {
        vec![vec![]]
    } else {
        let proj = calculate_neighbor_idxs(dim - 1);
        let mut ret: Vec<Vec<i32>> = vec![];
        for i in -1..=1 {
            let p = proj.iter().map(|v| {
                let mut prefix = v.clone();
                prefix.push(i);
                prefix
            });
            ret = ret.into_iter().chain(p).collect();
        }
        ret
    }
}

fn get_region(x: f64, anchor: f64, width: f64) -> usize {
    if x - anchor == 0.0 {
        0
    } else {
        f64::ceil((x - anchor) / width) as usize - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_grid() -> Grid<u32> {
        let globals = Globals {
            bounds_min: vec![0.0, 0.0],
            bounds_max: vec![50.0, 100.0],
            radius: 14.0,
            n: 100,
        };
        Grid::new(&globals)
    }

    #[test]
    fn test_discretization() {
        let g = example_grid();
        assert_eq!(g.anchor, vec![0.0, 0.0]);
        assert_eq!(g.cell_dims, vec![50.0 / 3.0, 100.0 / 7.0]);
        assert_eq!(g.cells.len(), 21);
    }

    fn assert_hash(g: &Grid<u32>, p: Vec<f64>, exp: i32) {
        let h = g.hash(&p.into()).into_iter().map(|v| v as i32).collect();
        let i = g.flatten(h);
        assert_eq!(i, exp);
    }

    #[test]
    fn test_hash() {
        let g = example_grid();
        assert_hash(&g, vec![0.0, 0.0], 0);
        assert_hash(&g, vec![50.0, 0.0], 14);
        assert_hash(&g, vec![50.0, 100.0], 20);
        assert_hash(&g, vec![0.0, 100.0], 6);
    }

    #[test]
    fn test_neighbor_idxs() {
        let idxs = calculate_neighbor_idxs(2);
        assert_eq!(
            idxs,
            vec![
                vec![-1, -1],
                vec![0, -1],
                vec![1, -1],
                vec![-1, 0],
                vec![0, 0],
                vec![1, 0],
                vec![-1, 1],
                vec![0, 1],
                vec![1, 1]
            ]
        );
        for i in 0..=5 {
            let idxs = calculate_neighbor_idxs(i);
            assert_eq!(idxs.len(), (3 as usize).pow(i as u32));
        }
    }

    #[test]
    fn test_flatten() {
        let g = example_grid();
        assert_eq!(g.flatten(vec![-1, 0]), -7);
        assert_eq!(g.flatten(vec![0, 0]), 0);
        assert_eq!(g.flatten(vec![1, 0]), 7);
        assert_eq!(g.flatten(vec![2, 0]), 14);
        assert_eq!(g.flatten(vec![3, 0]), 21);
        assert_eq!(g.flatten(vec![0, 6]), 6);
        assert_eq!(g.flatten(vec![0, 7]), 7);
        assert_eq!(g.flatten(vec![-1, -1]), -8);
        assert_eq!(g.flatten(vec![3, 7]), 28);
    }

    #[test]
    fn test_neighbor_offsets() {
        let g = example_grid();
        assert_eq!(g.neighbor_offsets, vec![-8, -1, 6, -7, 0, 7, -6, 1, 8]);
    }
}
