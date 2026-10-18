use std::fmt::Debug;

fn get_items<T: Ord + Clone + Debug>(v: &Vec<T>, pivot: &T) -> (Vec<T>, Vec<T>) {
    let mut less: Vec<T> = Vec::new();
    let mut greater: Vec<T> = Vec::new();

    for item in v {
        if item == pivot {
            continue;
        }

        if item < pivot {
            less.push(item.clone());
        } else {
            greater.push(item.clone());
        }
    }

    (less, greater)
}

pub fn quicksort<T: Ord + Clone + Debug>(items: Vec<T>) -> Vec<T> {
    if items.len() < 2 {
        items
    } else {
        let pivot = items.first().unwrap();
        let (less, greater) = get_items(&items, pivot);

        [quicksort(less), vec![pivot.clone()], quicksort(greater)].concat()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quicksort_vector_integer() {
        let v = vec![3, 2, 1];
        let result = vec![1, 2, 3];

        assert_eq!(quicksort(v), result);
    }

    #[test]
    fn vector_str_false() {
        let v = vec!["lucas", "zebra", "lu", "wonder"];
        let result = vec!["lu", "lucas", "wonder", "zebra"];

        assert_eq!(quicksort(v), result);
    }
}
