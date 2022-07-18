/// # Evenly square plots
///
/// ## Problem:
/// - Suppose you are a farmer and you want to devide your farm evenly into square plots.
/// - You want the plots to be as big as possible.
/// 
/// Example cases:
/// 1. Given a land of 50x25 meters, a valid square slot to devide this land would be 25m since we can devide this land in 2 slots of 25x25.
/// 2. Given a land of 1680x640 a valid square slot to devide this land would be 80x80, because 21 slots of 80m^2 are equivalent to 1680.
/// 
/// To learn more, read [here](https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/the-euclidean-algorithm#:~:text=The%20Algorithm,%3D%20B%E2%8B%85Q%20%2B%20R)) about the Euclid's Algorithm.
///
/// ---
/// To solve this problem using D&C, there are two steps:
///
/// 1. Figure out the base case. This should be the simplest possible case.
/// 2. Devide or decrease your problem until it becomes the base case.
///
/// ---
/// Example step by step:
/// 
/// There is a land of 1680X640 meters and we need to devide it in evenly square plots.
///
/// 1. Pick the smallest side of the land, which in this case 640m related to the height of the land.
/// 2. Subtract the largest size by the smallest side until the largest becomes the smaller.
/// 3. If after the subtraction the remaining sides are equal, we found the base case and this is our answer. Otherwise, we move to step 4.
/// 4. Get the remaining value and repeat the process from step 1 but with the remaining land size.
///     With the 1680x640 example, the remaining value after the first subtraction will be 400x640.
///     So we call step 1 again with a land of 400x640
/// 5. We repeat the steps with the remaining land until we get our base case.
///         In this example, by repeating the steps we would get to the base case of 80x160.
///         As 80 is half of 160, we found our base case and we know that 80x80 is the biggest land size that would devide this whole land evenly.
pub mod evenly_square_plots;