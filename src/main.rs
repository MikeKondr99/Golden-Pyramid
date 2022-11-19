use pyramid::*;
use rand::prelude::*;
#[allow(unused)]
fn main() {
    let size = 10_000_000;
    println!("Размер {size} элементов");
    test::<Simple>("Простой", size);
    test::<Vectorization>("Векторизация", size);
    test::<ParallelSplit<2>>("Параллельный/2", size);
}

fn test<T: LayerCalc>(name: &str, size: usize) {
    let layer = gen_data(size);
    let mut rest = gen_data(size - 1);

    let start = std::time::Instant::now();
    T::algorithm(&layer, &mut rest, size);
    println!("{}:\t{} нано", name, start.elapsed().as_nanos());
}

fn gen_data(size: usize) -> Vec<u32> {
    (0..size)
        .map(|_| rand::thread_rng().gen_range(0..500))
        .collect()
}
