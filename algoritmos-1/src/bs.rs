pub fn binary_search<T: Ord>(items: &[T], target: T) -> bool {
    let mut l = 0;
    let mut r = items.len();

    while l < r {
        let c = l + (r - l) / 2; 

        if items[c] == target {
            return true;
        }
        
        else if items[c] < target {
            l = c + 1;
        } else {
            r = c;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vector_integer_true() {
        let v = vec![1,2,3];
        let target = 3;

        assert!(binary_search(&v, target))
    }

    #[test]
    fn vector_integer_false() {
        let v = vec![1,2,3];
        let target = 5;

        assert!(!binary_search(&v, target))
    }


    #[test]
    fn vector_str_false() {
        let v = vec!["lu", "lucas", "wonder", "zebra"];

        assert!(!binary_search(&v, "well"))
    }

    #[test]
    fn vector_str_true() {
        let v = vec!["lu", "lucas", "wonder", "zebra"];

        assert!(binary_search(&v, "lu"));
        assert!(binary_search(&v, "zebra"));
    }
}
