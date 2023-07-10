use super::serde_array::arrays;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gene<const ISize: usize, const OSize: usize> {
    #[serde(with = "arrays")]
    Factor([f64; ISize]),
    #[serde(with = "arrays")]
    Constant([f64; OSize]),
    Abs(Box<Gene<ISize, OSize>>),
    And(Box<Gene<ISize, OSize>>, Box<Gene<ISize, OSize>>),
    Or(Box<Gene<ISize, OSize>>, Box<Gene<ISize, OSize>>),
    Xor(Box<Gene<ISize, OSize>>, Box<Gene<ISize, OSize>>),
    Plus(Box<Gene<ISize, OSize>>, Box<Gene<ISize, OSize>>),
    Minus(Box<Gene<ISize, OSize>>, Box<Gene<ISize, OSize>>),
    Multiply(Box<Gene<ISize, OSize>>, Box<Gene<ISize, OSize>>),
    Divide(Box<Gene<ISize, OSize>>, Box<Gene<ISize, OSize>>),
    Sin(Box<Gene<ISize, OSize>>),
    Cos(Box<Gene<ISize, OSize>>),
}
pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

pub trait Index {
    type Output;
    fn get_item(&self, i: usize) -> &Self::Output;
}

pub trait GeneGenerate<const M: usize, const N: usize> {
    fn generate_coord(&self, coord: [f64; M], gene: &Gene<N, M>) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let gene: Gene<2, 3> = Gene::Sin(Box::new(Gene::Plus(
            Box::new(Gene::Factor([0.2, 0.3])),
            Box::new(Gene::Constant([0.1, 0.2, 0.3])),
        )));
        let str = serde_json::to_string(&gene).unwrap();
        println!("{}", &str);
        let obj: Gene<2, 3> = serde_json::from_str(&str).unwrap();
        assert_eq!(gene, obj)
    }
}
