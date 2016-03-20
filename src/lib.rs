mod insertion_sort;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        use super::insertion_sort;
        let vec = vec![3, 1, 2, 8, 4, 7, 5];
        let sorted = insertion_sort::sort(vec);
        assert_eq!(sorted.len(), 7);
        assert_eq!(sorted[0], 1);
        assert_eq!(sorted[1], 2);
        assert_eq!(sorted[2], 3);
        assert_eq!(sorted[3], 4);
        assert_eq!(sorted[4], 5);
        assert_eq!(sorted[5], 7);
        assert_eq!(sorted[6], 8);
    }
}
