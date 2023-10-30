pub mod sort_integers_by_the_number_of_1_bits;

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn nr_1356_sort_by_bits_ex_01() {
        assert_eq!(vec![0,1,2,4,8,3,5,6,7], Solution::sort_by_bits(vec![0,1,2,3,4,5,6,7,8]));
    }

    #[test]
    fn nr_1356_sort_by_bits_ex_02() {
        assert_eq!(vec![1,2,4,8,16,32,64,128,256,512,1024], Solution::sort_by_bits(vec![1024,512,256,128,64,32,16,8,4,2,1]));
    }
}
