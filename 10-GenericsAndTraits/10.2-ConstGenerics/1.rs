struct Array<T, const N: usize> {
    data: [T; N],
}
fn main() {
    let _arrays: [Array<i32, 3>; 3] = [
        Array { data: [1, 2, 3] },
        Array { data: [1, 4, 5] },
        Array { data: [4, 5, 7] },
    ];

    let floats: [Array<f64, 2>; 3] = [
        Array { data: [1.0, 2.0] },
        Array { data: [3.0, 3.0] },
        Array { data: [5.0, 9.0] },
    ];

    println!("suc");
}
