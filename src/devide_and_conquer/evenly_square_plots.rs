/// Representation of a land area containing height and length
/// Examples:
/// 1680 meters X 640 meters
/// ```
/// use algorithms::devide_and_conquer::evenly_square_plots::*;
/// let land = Land::new(1680, 640);
/// ```
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
