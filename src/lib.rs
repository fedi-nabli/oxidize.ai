pub mod math;

#[cfg(test)]
mod tests {
    use super::*;

    use math::{matrix::Matrix, vector::Vector};

    #[test]
    fn test_vector_add() {
        let v1 = Vector { data: vec![1.2, 1.3, 1.5] };
        let v2 = Vector { data: vec![1.8, 1.7, 1.5] };
        let result = v1 + v2;
        assert_eq!(result.data, vec![3.0, 3.0, 3.0]);
    }

    #[test]
    fn test_vector_sub() {
        let v1: Vector<i32> = Vector { data: vec![1, 2] };
        let v2: Vector<i32> = Vector { data: vec![1, 1] };
        let result = v1 - v2;
        assert_eq!(result.data, vec![0, 1]);
    }

    #[test]
    fn test_vector_mul() {
        let v1 = Vector { data: vec![1.0] };
        let v2 = Vector { data: vec![2.0] };
        let result = v1 * v2;
        assert_eq!(result.data, vec![2.0]);
    }

    #[test]
    fn test_vector_div() {
        let v1 = Vector { data: vec![1.0] };
        let v2 = Vector { data: vec![2.0] };
        let result = v1 / v2;
        assert_eq!(result.data, vec![0.5]);
    }

    #[test]
    fn test_vector_empty() {
        let mut v: Vector<i32> = Vector { data: vec![] };
        assert_eq!(v.is_empty(), true);
        v.data = vec![1];
        assert_eq!(v.is_empty(), false);
    }

    #[test]
    fn test_vector_len() {
        let v = Vector { data: vec![1.0] };
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn test_vector_normalize() {
        let vector = Vector { data: vec![3.0, 4.0] }; // A 2D vector
        let normalized = vector.normalize();
        
        // Expected length of the vector [3, 4] is 5
        let expected_length = 1.0; // Length of normalized vector should be 1.0
        
        // Calculate length of normalized vector
        let norm_length = normalized.dot(&normalized).sqrt();
        
        // Assert that the normalized vector has length 1
        assert!((norm_length - expected_length).abs() < f64::EPSILON, "Normalized vector length is not 1");
        
        // Optional: Check the values of the normalized vector
        assert_eq!(normalized.data, vec![0.6, 0.8]); // Expected normalized values for [3, 4]
    }

    #[test]
    fn test_vector_min_max() {
        let vec = Vector::from(vec![1.0, 3.0, 3.1]);
        assert_eq!(vec.min().unwrap(), 1.0);
        assert_eq!(vec.max().unwrap(), 3.1);
    }

    #[test]
    fn test_matrix() {
        let mat: Matrix<f64> = Matrix::identity(3);
        let v = Vector { data: vec![1.0, 2.1, 3.5, 4.6] };
        let mat2: Matrix<f64> = Matrix::from_vec(2, 2, v.data).unwrap();
        let mat3 = mat2.transpose();
        println!("{}", mat);
        println!("{}", mat2);
        println!("{}", mat3);
    }
}
