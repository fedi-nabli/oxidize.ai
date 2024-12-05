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
        let ones: Matrix<f64> = Matrix::ones(2, 3);
        let id: Matrix<f64> = Matrix::identity(3);
        let add = (ones * id).unwrap();
        println!("{}", add);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
