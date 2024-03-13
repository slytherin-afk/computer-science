pub fn bubble_sort<T>(value: &mut [T])
where
    T: PartialOrd + Copy,
{
    let mut i = 0;
    let n = value.len();

    while i < (n - 1) {
        let mut j = 0;
        let mut swapped = false;

        while j < (n - i - 1) {
            if value[j] > value[j + 1] {
                (value[j], value[j + 1]) = (value[j + 1], value[j]);

                swapped = true;
            }

            j += 1;
        }

        if !swapped {
            break;
        }

        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::bubble_sort;

    #[test]
    fn sort_numbers_array() {
        let mut value = [100, 230, 0, 1, 32, 2];
        let expected = [0, 1, 2, 32, 100, 230];

        bubble_sort(&mut value);

        assert_eq!(value, expected);
    }

    #[test]
    fn sort_numbers_vector() {
        let mut value = vec![100, 230, 0, 1, 32, 2];
        let expected = vec![0, 1, 2, 32, 100, 230];

        bubble_sort(&mut value);

        assert_eq!(value, expected);
    }

    #[test]
    fn sort_char_array() {
        let mut value = ['z', 'd', 'a', 'e'];
        let expected = ['a', 'd', 'e', 'z'];

        bubble_sort(&mut value);

        assert_eq!(value, expected);
    }

    #[test]
    fn sort_char_vector() {
        let mut value = vec!['z', 'd', 'a', 'e'];
        let expected = vec!['a', 'd', 'e', 'z'];

        bubble_sort(&mut value);

        assert_eq!(value, expected);
    }
}
