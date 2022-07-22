use rayon;

fn quicksort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    rayon::join(|| quicksort(lo), || quicksort(hi));
}

fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_sorting_base_case() {
        let mut list: Vec<usize> = Vec::new();
        quicksort(&mut list[..]);
        assert_eq!(list, Vec::new());

        let mut list2: Vec<usize> = vec![1];
        quicksort(&mut list2[..]);
        assert_eq!(list2, vec![1]);

        let mut sorted_list = vec![1, 2, 3];
        quicksort(&mut sorted_list[..]);
        assert_eq!(sorted_list, vec![1, 2, 3]);
    }

    #[test]
    fn test_sort_small_list() {
        let mut list = vec![2, 1];
        quicksort(&mut list[..]);
        assert_eq!(list, vec![1, 2]);
    }

    #[test]
    fn test_random_list_sorting() {
        let mut list = gen_random_list(10000);
        quicksort(&mut list);
        assert_eq!(list.len(), list.len());

        for i in 1..list.len() - 1 {
            assert!(list[i] >= list[i - 1]);
        }
    }

    fn gen_random_list(length: usize) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        (0..length).map(|_| rng.gen_range(0..length)).collect()
    }
}
