use std::cmp::Ordering;

/// Given a sorted list and a target number, returns the index of the number if found, otherwise return None
/// 
/// Examples
/// ```
/// use algorithms::search::binary_search::search;
/// 
/// let list = vec![1, 2, 3, 4, 5];
/// let target = 5;
/// let result = search(&list, target);
/// assert_eq!(result, Some(4));
/// ```
pub fn search(list: &[i32], target: i32) -> Option<usize> {
    if list.is_empty() { return None }
    let mid = list.len() / 2;

    match list[mid].cmp(&target) {
        Ordering::Greater => search(&list[..mid], target),
        Ordering::Less => match search(&list[mid+1..], target) {
            Some(i) => Some(mid + i + 1),
            None => None
        },
        Ordering::Equal => Some(mid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_empty_list() {
        let list = Vec::new(); 
        let target = 0;
        let result = search(&list, target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_search_value_not_found() {
        let list = vec![1, 2, 3, 4, 5];
        let target = 30;
        let result = search(&list, target);
        assert_eq!(result, None);
    }


    #[test]
    fn test_search_value_middle() {
        let list = vec![1, 2, 3, 4, 5];
        let target = 3;
        let result = search(&list, target);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_search_value_beginning_of_list() {
        let list = vec![1, 2, 3, 4, 5];
        let target = 1;
        let result = search(&list, target);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_search_value_end_of_list() {
        let list = vec![1, 2, 3, 4, 5];
        let target = 5;
        let result = search(&list, target);
        assert_eq!(result, Some(4));
    }
}