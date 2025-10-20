mod banker;

fn main() {
    const NRES: usize = 3;
    const NTH: usize = 5;

    let available = [3, 3, 2];
    let max = [
        [7, 5, 3],
        [3, 2, 2],
        [9, 0, 2],
        [2, 2, 2],
        [4, 3, 3],
    ];

    let resource = banker::Resource::<NRES, NTH>::new(available, max);

    println!("Is system safe? {}", resource.is_safe());
}
