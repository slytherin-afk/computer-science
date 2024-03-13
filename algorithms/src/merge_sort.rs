fn merge<T>(array: &mut [T], left: usize, mid: usize, right: usize)
where
    T: Copy + PartialOrd,
{
    let sub_array_one = mid - left + 1;
    let sub_array_two = right - mid;

    let mut left_array = vec![];
    let mut right_array = vec![];

    let mut i = 0;
    while i < sub_array_one {
        left_array.push(array[left + i]);
        i += 1;
    }

    let mut j = 0;
    while j < sub_array_two {
        right_array.push(array[mid + 1 + j]);
        j += 1;
    }

    let mut index_of_sub_array_one = 0;
    let mut index_of_sub_array_two = 0;
    let mut index_of_merged_array = left;

    while (index_of_sub_array_one < sub_array_one) && (index_of_sub_array_two < sub_array_two) {
        if left_array[index_of_sub_array_one] <= right_array[index_of_sub_array_two] {
            array[index_of_merged_array] = left_array[index_of_sub_array_one];
            index_of_sub_array_one += 1;
        } else {
            array[index_of_merged_array] = right_array[index_of_sub_array_two];
            index_of_sub_array_two += 1;
        }
        index_of_merged_array += 1;
    }

    while index_of_sub_array_one < sub_array_one {
        array[index_of_merged_array] = left_array[index_of_sub_array_one];
        index_of_sub_array_one += 1;
        index_of_merged_array += 1;
    }

    while index_of_sub_array_two < sub_array_two {
        array[index_of_merged_array] = right_array[index_of_sub_array_two];
        index_of_sub_array_two += 1;
        index_of_merged_array += 1;
    }
}

pub fn merge_sort<T>(array: &mut [T], begin: usize, end: usize)
where
    T: Copy + PartialOrd,
{
    if begin >= end {
        return;
    }

    let mid = begin + (end - begin) / 2;

    merge_sort(array, begin, mid);
    merge_sort(array, mid + 1, end);

    merge(array, begin, mid, end);
}

#[cfg(test)]
mod test {
    use super::merge_sort;

    #[test]
    fn sort_numbers_array() {
        let mut value = [100, 230, 0, 1, 32, 2];
        let expected = [0, 1, 2, 32, 100, 230];
        let end = value.len() - 1;

        merge_sort(&mut value, 0, end);

        assert_eq!(value, expected);
    }

    #[test]
    fn sort_numbers_vector() {
        let mut value = vec![100, 230, 0, 1, 32, 2];
        let expected = vec![0, 1, 2, 32, 100, 230];
        let end = value.len() - 1;

        merge_sort(&mut value, 0, end);

        assert_eq!(value, expected);
    }

    #[test]
    fn sort_char_array() {
        let mut value = ['z', 'd', 'a', 'e'];
        let expected = ['a', 'd', 'e', 'z'];
        let end = value.len() - 1;

        merge_sort(&mut value, 0, end);

        assert_eq!(value, expected);
    }

    #[test]
    fn sort_char_vector() {
        let mut value = vec!['z', 'd', 'a', 'e'];
        let expected = vec!['a', 'd', 'e', 'z'];
        let end = value.len() - 1;

        merge_sort(&mut value, 0, end);

        assert_eq!(value, expected);
    }
}
