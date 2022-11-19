use std::thread;

use rand::prelude::*;

macro_rules! zip {
    ($iter:expr) => {
        $iter.into_iter()
    };
    ($first:expr, $second:expr) => {
        $first.into_iter().zip($second.into_iter())
    };
    ($first:expr,$second:expr, $($rest:expr),*) => {
        zip!($first,$second)
            .zip(zip!($($rest),*))
            .map(|((a,b),c)| (a,b,c))

    };
}

pub fn get_size(layers: usize) -> u64 {
    (layers as u64 * (layers as u64 + 1)) / 2
}

fn gen_data(size: usize) -> Vec<u32> {
    (0..size)
        .map(|_| rand::thread_rng().gen_range(0..500))
        .collect()
}

#[allow(unused)]
fn main() {
    let size = 30000;
    let layer = gen_data(size);
    let mut rest = gen_data(size - 1);
    //
    let start = std::time::Instant::now();
    Vectorization::algorithm(&layer, &mut rest, size);
    println!("{} наносекунд", start.elapsed().as_nanos());
}

///
///   0
///  1 2
/// 3 4 5
///
pub fn pyramid<T: LayerCalc>(mut input: &mut [u32], size: usize) -> u32 {
    if input.len() != (size * (size + 1)) / 2 {
        panic!("Размер данных пирамиды должен соответствовать указному количеству слоёв");
    }
    for i in (2..=size).rev() {
        let (layer, rest) = input.split_at_mut(i);
        T::algorithm(layer, &mut rest[0..i - 1], i);
        input = rest;
    }
    input[0]
}

///
///   0 1 2
///  3 4 5
/// 6 7 8
///
pub fn rectangle<T: LayerCalc>(mut input: &mut [u32], size: usize) -> u32 {
    if input.len() != size * size {
        panic!("Размер данных прямоугольника должен соответствовать указному количеству слоёв");
    }
    while input.len() > size {
        let (layer, rest) = input.split_at_mut(size);
        T::algorithm(layer, &mut rest[0..size - 1], size);
        rest[size - 1] += layer[size - 1];
        input = rest;
    }
    *input.iter().max().expect("ввод не должен быть пустым")
}

pub trait LayerCalc {
    fn algorithm(layer: &[u32], rest: &mut [u32], n: usize);
}

pub struct Simple;
impl LayerCalc for Simple {
    fn algorithm(layer: &[u32], rest: &mut [u32], _n: usize) {
        for i in 0..layer.len() - 1 {
            rest[i] += layer[i].max(layer[i + 1]);
        }
    }
}
pub struct Vectorization;
impl LayerCalc for Vectorization {
    fn algorithm(l: &[u32], r: &mut [u32], n: usize) {
        zip!(l[0..n - 1], l[1..], &mut r[0..n - 1]).for_each(|(a, b, r)| *r += a.max(b));
    }
}

pub struct ParallelSplit<const N: usize>;
impl<const N: usize> LayerCalc for ParallelSplit<N> {
    fn algorithm(layer: &[u32], rest: &mut [u32], n: usize) {
        let size = n / N + 1;
        let chunks = zip!(
            layer[0..n - 1].chunks(size),
            layer[1..].chunks(size),
            rest[0..n - 1].chunks_mut(size)
        );
        thread::scope(|s| {
            chunks.for_each(|(l1, l2, r)| {
                s.spawn(|| {
                    zip!(l1, l2, r).for_each(|(a, b, r)| *r += a.max(b));
                });
            });
        });
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case::one_layer(&mut [5],1,5)]
    #[case::two_layers(&mut [5, 6, 3],2,11)]
    #[case::example(&mut [7, 3, 8, 8, 1, 0, 2, 7, 4, 4, 4, 5, 2, 6, 5],5,30)]
    #[case::example2(&mut [7, 2, 3, 3, 3, 1, 3, 1, 5, 4, 3, 1, 3, 1, 3, 2, 2, 2, 2, 2, 2, 5, 6, 4, 5, 6, 4, 3], 7, 29)]
    fn simple_tests(
        #[values(pyramid::<Simple>,
            pyramid::<Vectorization>,
            pyramid::<ParallelSplit<2>>,
            pyramid::<ParallelSplit<3>>,
            pyramid::<ParallelSplit<4>>,
        )]
        version: impl Fn(&mut [u32], usize) -> u32,
        #[case] input: &mut [u32],
        #[case] size: usize,
        #[case] expected: u32,
    ) {
        input.reverse();
        let answer = version(input, size);
        assert_eq!(answer, expected)
    }

    #[rstest]
    #[case::one_layer(&mut [5],1,5)]
    ///  5 6
    ///   3 4
    ///   *10
    #[case::two_layers(&mut [5, 6, 3, 4],2,10)]
    ///
    /// 1   2   3   4\
    ///  5   6   7   8\
    ///   9  10  11   12\
    ///    13  14  15   16\
    ///              *40
    ///
    #[case::example(&mut [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],4,40)]
    ///
    /// 1   2   3   100\
    ///  5   6   100   8\
    ///   9  100  11   12\
    ///    100  14  15   16\
    ///              *400
    ///
    #[case::example2(&mut [1,2,3,100,5,6,100,8,9,100,11,12,100,14,15,16],4,400)]
    fn rect_tests(
        #[values(rectangle::<Simple>,
            rectangle::<Vectorization>,
            rectangle::<ParallelSplit<2>>,
            rectangle::<ParallelSplit<4>>,
        )]
        version: impl Fn(&mut [u32], usize) -> u32,
        #[case] input: &mut [u32],
        #[case] size: usize,
        #[case] expected: u32,
    ) {
        input.reverse();
        let answer = version(input, size);
        assert_eq!(answer, expected)
    }
}
