///! Problem:
///! - Suppose you are a farmer and you want to devide your farm evenly into square plots.
///! - You want the plots to be as big as possible.
///!
///!
///!           1680 m
///! _______________________
///! |                     |
///! |                     | 640 m
///! |                     |
///! |_____________________|
///!
///! To solve this problem using D&C, there are two steps:
///!
///! 1. Figure out the base case. This should be the simplest possible case.
///! 2. Devide or decrease your problem until it becomes the base case.
///!
///! Algorith step by step:
///!
///! 1. Pick the smallest side of the land, which in this case 640m related to the height of the land.
///! 2. 640m is less than half of the land length, so we decrease the length of the land by 2*640m.
///!         1680 - (640 * 2)
///! 3. The remaining land would be a 400x640
///! 4. We repeat the steps with the remaining land until one of the sides is equal to the half of the other side.
///!         In this example, repeating the steps we would get to the base case of 80x160.
///!         As 80 is half of 160, we found our base case and we know that 80x80 is the biggest land size that would devide this whole land evenly.
///!

pub struct Land {
    pub height: usize,
    pub length: usize,
}

impl Land {
    pub fn new(height: usize, length: usize) -> Land {
        Land {
            height: height,
            length: length,
        }
    }
}

/// Given a Land with height and length, return the biggest square land plot definition corresponding to the evenly distribution of the total land area.
/// 
/// # Examples
/// 
/// ```
/// use algorithms::devide_and_conquer::evenly_square_plots::*;
/// 
/// let land = Land::new(1680, 640);
/// let result = get_square_plots(&land);
/// assert_eq!(result.length, 80);
/// assert_eq!(result.height, 80);
/// ```
pub fn get_square_plots(land: &Land) -> Land {
    let land_sizes = vec![land.height, land.length];
    let smallest_size = land_sizes.iter().min().unwrap();
    let largest_size = land_sizes.iter().max().unwrap();
    let mut size = usize::from(*largest_size);

    while size > *smallest_size {
        size -= smallest_size;
    }
    
    if size == *smallest_size { Land::new(size, size) }
    else {
        let reduced_land = Land::new(*smallest_size, size);
        get_square_plots(&reduced_land)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devide_base_case_land() {
        let land = Land::new(50, 25);
        let result = get_square_plots(&land);
        assert_eq!(result.height, 25);
        assert_eq!(result.length, 25);
    }

    #[test]
    fn test_devide_unbalanced_land() {
        let land = Land::new(1680, 640);
        let result = get_square_plots(&land);
        assert_eq!(result.height, 80);
        assert_eq!(result.length, 80);
    }
}
