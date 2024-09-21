pub mod math;

#[cfg(test)]
mod tests {
    use super::*;

    use math::vector::Vector;

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
}
