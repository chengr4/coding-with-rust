pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    arr.sort();

    let min_diff = arr
        .windows(2)
        .map(|tuple| tuple[1] - tuple[0])
        .min()
        .unwrap_or(i32::MAX);

    arr.windows(2)
        .filter(|tuple| tuple[1] - tuple[0] == min_diff)
        .map(|tuple| vec![tuple[0], tuple[1]])
        .collect()
}
