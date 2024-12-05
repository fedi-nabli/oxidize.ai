pub mod math;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    use math::matrix::Matrix;

    #[test]
    fn matrix_test() {
        let ones: Matrix<f64> = Matrix::ones(3, 3);
        let zeros: Matrix<f64> = Matrix::zeroes(3, 3);
        let mut id: Matrix<f64> = Matrix::identity(3);
        println!("Ones matrix\n{}", ones);
        println!("Zeroes matrix\n{}", zeros);
        println!("Identity matrix\n{}", id);
        println!("{:?}", id.get(1, 2));
        id.set(1, 2, 3.5).unwrap();
        println!("{:?}", id.get(1, 2));
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
