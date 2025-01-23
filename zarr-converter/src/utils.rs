/// Find the middle divisor of the first dimension of the shape
///
/// # Arguments
/// - `shape` : A vector of numbers
///
/// # Returns
/// The middle divisor of the first dimension of the shape
pub fn find_middle_divisor(shape: &[usize]) -> usize {
    let first_dim = shape[0];
    let mut divisors = vec![];

    for i in 1..=first_dim {
        if first_dim % i == 0 {
            divisors.push(i);
        }
    }
    let len = divisors.len();
    if len > 0 {
        divisors[len / 2]
    } else {
        shape[0]
    }
}
