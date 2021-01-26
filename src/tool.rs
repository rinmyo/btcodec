use std::ops::Range;

//convert binary number string into type T: number
pub(crate) fn bin_str2int<T>(bin_str: &str, range: Range<usize>) -> T {
    u128::from_str_radix(&bin_str[range], 2).unwrap() as T
}

//counter
pub(crate) fn get_range(start: usize) -> Box<dyn Fn(usize) -> Range<usize>> {
    let mut counter = start;
    Box::new(
        |step: usize| {
            counter = counter + step;
            counter - step..counter
        }
    )
}