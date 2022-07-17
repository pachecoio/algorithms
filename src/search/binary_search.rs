/// Return the index of the number if found, otherwise return None
fn search(list: &[i32], target: i32) -> Option<usize> {
    if list.len() == 0 { return None }
    let mid = list.len() / 2;
    if list[mid] > target { search(&list[..mid], target) }
    else if list[mid] < target { 
        let result = search(&list[mid+1..], target);
        match result {
            Some(i) => Some(mid + i + 1),
            None => None
        }
    }
    else { Some(mid) }
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