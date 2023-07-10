use super::gene::{Gene, GeneGenerate, Index};

pub type Coord = [f64; 2];
pub type Rgb = [f64; 3];

impl<const M: usize, const N: usize> GeneGenerate<M, N> for Rgb {
    fn generate_coord(&self, coord: [f64; M], gene: &Gene<N, M>) -> Self {
        todo!()
    }
}
