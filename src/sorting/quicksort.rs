use rand::Rng;

pub fn quicksort(list: &[usize]) -> Vec<usize>{
	if list.len() == 0 { return Vec::new(); }
	if list.len() == 1 { return vec![list[0]]; }
	let mut smaller_list = Vec::new();
	let mut bigger_list = Vec::new();

	let mut rng = rand::thread_rng();
	let pivot = rng.gen_range(0..list.len());

	for i in 0..list.len() {
		if i == pivot { continue }
		if list[i] <= list[pivot] {
			smaller_list.push(list[i]);
		} else {
			bigger_list.push(list[i]);
		}
	}

	let mut left = quicksort(&smaller_list);
	let mut right = quicksort(&bigger_list);

	left.push(list[pivot]);
	left.append(&mut right);

	left

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sorting_base_case() {
		let list: Vec<usize> = Vec::new();
		quicksort(&list[..]);
		assert_eq!(list, Vec::new());

		let list2: Vec<usize> = vec![1];
		quicksort(&list2[..]);
		assert_eq!(list2, vec![1]);

		let sorted_list = vec![1, 2, 3];
		quicksort(&sorted_list[..]);
		assert_eq!(sorted_list, vec![1, 2, 3]);
	}

	#[test]
	fn test_sort_small_list() {
		let list = vec![2, 1];
		let result = quicksort(&list[..]);
		assert_eq!(result, vec![1, 2]);
	}

	#[test]
	fn test_random_list_sorting() {
		let mut list = gen_random_list(1000);
		let result = quicksort(&list);
		assert_eq!(result.len(), list.len());

		// Default sort the input list to compare with the quicksort result
		list.sort();

		assert_eq!(result, list);
	}

	fn gen_random_list(length: usize) -> Vec<usize> {
		let mut rng = rand::thread_rng();
		(0..length).map(|_| { rng.gen_range(0..length) }).collect()
	}

}