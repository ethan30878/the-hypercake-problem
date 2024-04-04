// Name: Ethan Glenn
// Date Completed: April 4th, 2024
// I pledge that I followed the academic integrity policy in assignment.

fn main() 
{
    use std::io;

    let mut hd = String::new();
    println!("Welcome to the Hypercake Experience: Please specify the degree of hyperdimensionality (k) that your cake possesses: ");

    io::stdin().read_line(&mut hd).expect("Failed to read line.");
    let k: i32 = hd.trim().parse().expect("Invalid input.");

    let mut sl = String::new();
    println!("Great! How many hyperplanar slices (n) would you like to use? ");

    io::stdin().read_line(&mut sl).expect("Failed to read line.");
    let n: i32 = sl.trim().parse().expect("Invalid input.");

    println!("Congratulations! Your Hypercake can be sliced into a maximum of {0} pieces!", hypercake(n, k));  
}

fn factorial(n: i32) -> i32
{
    if n <= 1 { return 1; }
    return n * factorial(n - 1);
}

fn combinations(n: i32, r: i32) -> i32 
{
    if !(r >= 0 && r <= n) { return 0; }
    return factorial(n) / (factorial(r) * factorial(n - r));
}

fn hypercake(n: i32, k: i32) -> i32
{
    if k < 1 { return combinations(n, k); }
    return combinations(n, k) + hypercake(n, k - 1);
}

#[cfg(test)]
mod tests 
{
    use super::*;

    const EXAMPLE_CASES : [[i32; 4]; 11] = 
    [ 
        [1, 1, 1, 1],
        [2, 2, 2, 2],
        [4, 4, 4, 4],
        [7, 8, 8, 8],
        [11, 15, 16, 16],
        [16, 26, 31, 32],
        [22, 42, 57, 63],
        [29, 64, 99, 120],
        [37, 93, 163, 219],
        [46, 130, 256, 382],
        [56, 176, 386, 638]
    ];

    #[test]
    fn test_hypercake()
    {
        for i in 0..11
        {
            for j in 0..4 
            {
                assert_eq!(hypercake(i, j + 2), EXAMPLE_CASES[i as usize][j as usize]);
            }
        }
    }
}