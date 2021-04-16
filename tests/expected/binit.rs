use std::collections;

fn bisect_right(data: &Vec<i32>, item: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high: _ = data.len();
    while low < high {
        let middle: _ = i32::from(((low + high) / 2));
        if item < data[middle] {
            high = middle;
        } else {
            low = (middle + 1);
        }
    }
    return low;
}

fn bin_it(limits: &Vec<i32>, data: &Vec<i32>) -> Vec<i32> {
    let mut bins = vec![0];
    for _x in limits.iter() {
        bins.push(0);
    }
    for d in data {
        bins[bisect_right(&limits, d) as usize] += 1;
    }
    return bins;
}

fn main() {
    let limits = vec![23, 37, 43, 53, 67, 83];
    let data = vec![
        95, 21, 94, 12, 99, 4, 70, 75, 83, 93, 52, 80, 57, 5, 53, 86, 65, 17, 92, 83, 71, 61, 54,
        58, 47, 16, 8, 9, 32, 84, 7, 87, 46, 19, 30, 37, 96, 6, 98, 40, 79, 97, 45, 64, 60, 29, 49,
        36, 43, 55,
    ];
    assert!(bin_it(&limits, &data) == vec![11, 4, 2, 6, 9, 5, 13]);
}
