/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord + Clone>(array: &mut [T]) {
    //TODO
    let len = array.len();
    let mut vecs: Vec<T> = Vec::with_capacity(len);
    vecs.push(array[0].clone());

    for i in 1..len {
        for j in 0..vecs.len() {
            if array[i] > vecs[j] {
                if j == vecs.len() - 1 {
                    vecs.push(array[i].clone());
                    break;
                }
            } else {
                vecs.insert(j, array[i].clone());
                break;
            }
        }
    }

    for (i, j) in array.iter_mut().zip(vecs.iter()) {
        *i = j.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
