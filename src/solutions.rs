
//! A collection of functions for solving Project Euler questions.
//! Based upon the parameters for the Hacker Rank implementations

//! Currently missing: #4, #7

//======================================================================
// Problem 1
//======================================================================

/// Sums up all the multiples of two integers [a,b] up to a certain integer n.
///
/// Note that these multiples are NOT double-counted.
///
/// # Examples
/// ```
/// extern crate project_euler;
/// use project_euler::solutions::sum_multiples;
///
/// fn main() {
///     let a = 2_i64; let b = 3_i64;
///     let n = 10_i64;
///     // This should count 2+3+4+6+8+9+10 = 32
///     assert_eq!(32, sum_multiples(n, [a,b]));
/// }
/// ```
pub fn sum_multiples(end: i64, m: [i64; 2] ) -> i64{
    let n1 = (end-1) / m[0];
    let n2 = (end-1) / m[1];
    let n3 = (end-1) / (m[0]*m[1]);

    let sum1 = m[0] * (n1*(n1+1)) / 2;
    let sum2 = m[1] * (n2*(n2+1)) / 2;
    let sum3 = (m[0]*m[1]) * (n3*(n3+1)) / 2;

    sum1+sum2-sum3
}


//======================================================================
// Problem 2
//======================================================================

/// Holds and implements even Fibonacci numbers
pub struct FibThree {
    fib_pair: [i64;2],
}

impl FibThree {
    pub fn new() -> FibThree {
        FibThree {fib_pair: [0,1]}
    }
}

impl Iterator for FibThree {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        let gn: i64  = self.fib_pair[0];
        let gn1: i64 = self.fib_pair[1];
        self.fib_pair = [3*gn + 2*gn1, 2*gn + gn1];
        Some(self.fib_pair[0])
    }
}

/// Sums the even Fibonacci numbers up to an integer, n.
///
/// # Examples
/// ```
/// use project_euler::solutions::count_even;
///
/// fn main() {
///     let n = 10_i32;
///     //The Fibonacci sequence is 1,1,2,3,5,8,13,...
///     // Thus we should get 2 + 8 = 10
///     assert_eq!(10, count_even(n));
/// }
/// ```
pub fn count_even(n: i32) -> i32 {
    let mut fib_seq = FibThree::new();
    let mut cnt = 0_i32;
    loop {
        fib_seq.next();
        let check = fib_seq.fib_pair[0] as i32;
        if check < n {
            cnt += check;
        } else {
            break;
        }
    }
    cnt
}
//======================================================================
// Problem 3
//======================================================================

/// Solves for the larget prime factor of an integer, n.
///
/// # Examples
/// ```
/// use project_euler::solutions::largest_prime_factor;
///
/// fn main() {
///     let n = 420_i64;
///     // The prime factors of 420 are 2,2,3,5,7.
///     // The largest should then be 7.
///     assert_eq!(7, largest_prime_factor(n));
/// }
/// ```
pub fn largest_prime_factor(x: i64) -> i64 {
    let mut target: i64 = x;
    let mut ind: i64 = 2;
    while ind.pow(2) <= target {
        if target % ind == 0 {
            target = target / ind;
        } else {
            ind += 1;
        }
    }
    target
}

//======================================================================
// Problem 4
//======================================================================

//======================================================================
// Problem 5
//======================================================================

/// Computes the smallest multiple of all the integers from 1 to n, n \in Integers.
///
/// # Examples
/// ```
/// use project_euler::solutions::compute_multiple;
///
/// fn main() {
///     let n = 4_i32;
///     // The smallest smallest multiple of 1,2,3,4 is 12;
///     assert_eq!(12, compute_multiple(n));
/// }
/// ```
pub fn compute_multiple (range_n: i32) -> i32 {
    let range_n: usize = range_n as usize;
    let mut list: Vec<usize> = (1..(range_n+1)).collect();
    let mut multiple: usize = 1;
    let mut index: usize = 0;

    while index < range_n {
        let temp = list[index];
        multiple *= temp;
        for i in index..range_n {
            if list[i] % temp == 0 {
                list[i] /= temp;
            } else {
                continue;
            }
        }
        index += 1
    }
    multiple as i32

}

//======================================================================
// Problem 6
//======================================================================
/// Finds the absolute difference between the sum of squares, 1^2 + 2^2 + ... + n^2, and the square of the sum, (1 + 2 + ... + n)^2.
///
/// # Examples
/// ```
/// use project_euler::solutions::squared_sum_diff;
///
/// fn main() {
///     let n = 3_i32;
///     // The sum of squares is 1^2 + 2^2 + 3^2 = 14.
///     // The square of the sum is (1 + 2 + 3)^2 = 36.
///     // Their absolute difference is then |14 - 36| = 22.
///     assert_eq!(22, squared_sum_diff(n));
/// }
/// ```
pub fn squared_sum_diff (range_n: i32) -> i32 {
    let square_of_sum: i32 = ((range_n * (range_n+1)) / 2).pow(2);
    let sum_of_squares: i32 = (range_n*(range_n+1)*(2*range_n+1))/6;
    (square_of_sum - sum_of_squares).abs()
}

//======================================================================
// Problem 7
//======================================================================

//======================================================================
// Problem 8
//======================================================================

/// Find the largest product of k sequential numbers in a sequence of n digits.
///
/// # Examples
/// ```
/// use project_euler::solutions::largest_product;
///
/// fn main() {
///     let seq = String::from("123456");
///     let p: Vec<i64> = vec![6_i64, 2_i64];
///     //The sequence has n = 6 digits and we choose k = 2.
///     // The largest product of 2 sequential digits is 5*6 = 30.
///     assert_eq!(30, largest_product(seq,p));
/// }
/// ```
pub fn largest_product(seq: String, p: Vec<i64>) -> i64 {
    let n = p[0]; let k = p[1];
    let mut digits: Vec<_> = Vec::new();
    let mut prod: i64 = 1;
    let mut seq = seq;

    for _ in 0..k {
        let c = seq.pop().unwrap();
        let digit: i64 = c.to_digit(10)
                          .unwrap() as i64;
        digits.push(digit);
        prod *= digit;
    }
    let mut max_prod = prod;

    for _ in 0..(n-k) {
        let c = seq.pop().unwrap();
        let digit: i64 = c.to_digit(10)
                          .unwrap() as i64;
        digits.push(digit);
        prod *= digit;

        let divide = digits.remove(0);
        if divide == 0 {
            prod = digits.iter()
            .fold(1, |prod,dig| prod * dig);
        } else {
            prod = prod / divide;
        }

        if prod > max_prod {
            max_prod = prod;
        }

    }

    max_prod
}


//======================================================================
// Problem 9
//======================================================================

/// Given N, finds the maximum product, a*b*c, of pythagorean triples {(a,b,c)| a^2 + b^2 = c^2 } such that a+b+c = N.
///
/// # Examples
/// ```
/// use project_euler::solutions::pythagorean_triplets;
///
/// fn main() {
///     let n = 12_i32;
///     // The only pythagorean triple that sums to 12 is (3,4,5); 3+4+5 = 12.
///     // Thus the largest product is 3*4*5 = 60.
///     assert_eq!(60, pythagorean_triplets(n));
/// }
/// ```
pub fn pythagorean_triplets(n: i32) -> i64{
    let mut triples: Vec<i64> = Vec::new();
    for a in 1..(n/2 + 1) {
        let num = (n - 2*a)*n;
        let den = (n - a)*2;
        if num > 0 && num % den == 0 {
            let b = num/den;
            let c = n-(a+b);
            triples.push((a*b*c) as i64);
        }
    }
    let solution = match triples.iter().max() {
        Some(num) => *num,
        None      => -1_i64,
    };
    solution
}

//======================================================================
// Problem 10
//======================================================================

/// Store and generate a vector of primes.
pub struct PrimeCache {
    pub primes: Vec<i64>,
    pub bookmark: i64,
}

impl PrimeCache {
    pub fn new() -> PrimeCache {
        PrimeCache { primes: vec![2,3,5,7], bookmark: 8}
    }

    /// Add new primes to the structure up to an integer n.
    ///
    /// # Examples
    /// ```
    /// use project_euler::solutions::PrimeCache;
    ///
    /// let mut cache = PrimeCache::new();
    /// assert_eq!(cache.primes, vec![2,3,5,7]);
    /// cache.fill(25);
    /// assert_eq!(cache.primes, vec![2,3,5,7,11,13,17,19,23]);
    /// ```
    pub fn fill(&mut self, n:i64) {
        let mut check = self.bookmark;
        while check <= n {
            let mut is_prime = true;
            let mut index: usize = 0;
            while self.primes[index].pow(2) <= check {
                if check % self.primes[index] == 0 {
                    is_prime = false;
                    break;
                }
                index += 1;
            }

            if is_prime {self.primes.push(check);}
            check += 1
        }
        self.bookmark = check;
    }
}

/// Sums up primes up to an integer n.
///
/// # Examples
/// ```
/// use project_euler::solutions::{sum_primes, PrimeCache};
///
/// fn main() {
///     let n = 12_i64;
///     let mut cache = PrimeCache::new();
///     // The sum of primes less than 12 is, 2 + 3 + 5 + 7 + 11 = 28.
///     assert_eq!(28, sum_primes(n,&mut cache));
/// }
/// ```
pub fn sum_primes(n: i64, cache:&mut PrimeCache) -> i64 {
    if n > cache.bookmark {
        cache.fill(n);
    }
    let mut total: i64 = 0_i64;
    let mut primes = cache.primes.iter();
    loop {
        let prime = match primes.next() {
            Some(num) => *num,
            None      => {break},
        };
        if prime > n { break ;}
        total += prime;
    }
    total
}

//======================================================================
// Problem 11
//======================================================================

/// Move across a grid and computes products of k sequential numbers in each direction.
///
/// # Examples
/// ```
/// use project_euler::solutions::grid_prod;
///
/// fn main() {
///     let grid: Vec<Vec<i32>> = vec![ vec![1,2,3], vec![1,2,3], vec![1,2,3]];
///     let k = 3_usize;
///     // The largest product of 3 sequential numbers in [1,2,3\\ 1,2,3\\ 1,2,3] is 3*3*3 = 27.
///     assert_eq!(27, grid_prod(grid, k));
/// }
/// ```
pub fn grid_prod(grid: Vec<Vec<i32>>, k: usize) -> i64 {

    fn mult_down(grid: &Vec<Vec<i32>>, coord: (usize,usize), k: usize) -> i64 {
        let n = grid.len();
        if coord.0 > n-k {
            return 0_i64;
        } else {
            let (row,col) = coord;
            let mut total = 1;
            for i in 0..k {
                total *= grid[row + i][col];
            }
            total as i64
        }
    }
    fn mult_right(grid: &Vec<Vec<i32>>, coord: (usize,usize), k: usize) -> i64 {
        let n = grid.len();
        if coord.1 > n-k {
            return 0_i64;
        } else {
            let (row,col) = coord;
            let mut total = 1;
            for j in 0..k {
                total *= grid[row][col + j];
            }
            total as i64
        }
    }
    fn mult_down_right(grid: &Vec<Vec<i32>>, coord: (usize,usize), k: usize) -> i64{
        let n = grid.len();
        if coord.0 > n-k || coord.1 > n-k {
            return 0_i64;
        } else {
            let (row,col) = coord;
            let mut total = 1;
            for d in 0..k {
                total *= grid[row + d][col + d];
            }
            total as i64
        }
    }

    fn mult_up_right(grid: &Vec<Vec<i32>>, coord: (usize,usize), k: usize) -> i64{
        let n = grid.len();
        if coord.0 < k-1 || coord.1 > n-k {
            return 0_i64;
        } else {
            let (row,col) = coord;
            let mut total = 1;
            for d in 0..k {
                total *= grid[row - d][col + d];
            }
            total as i64
        }
    }

    let n = grid.len();
    let mut max: i64 = 0_i64;
    for row in 0..n {
        for col in 0..n {
            let right = mult_right(&grid, (row,col), k);
            let down  = mult_down(&grid, (row,col), k);
            let down_right = mult_down_right(&grid, (row,col),k);
            let up_right = mult_up_right(&grid, (row,col),k);
            if right > max {max = right};
            if down > max {max = down};
            if down_right > max {max = down_right};
            if up_right > max {max = up_right};
        }
    }
    max
}

//======================================================================
// Problem 12
//======================================================================
use std::collections::HashMap;

/// Stores the number of factors a number has in a HashMap.
pub struct DivisorCache {
    divisors: HashMap<i64, i64>,
}

impl DivisorCache {
    pub fn new() -> DivisorCache {
        let mut cache = HashMap::new();
        cache.insert(1,1);
        cache.insert(2,2);
        DivisorCache { divisors: cache }
    }

    /// Fetches the number of factors an integer has. It is calculated and stored if not currently stored.
    ///
    /// # Examples
    /// ```
    /// use project_euler::solutions::DivisorCache;
    ///
    /// let mut cache = DivisorCache::new();
    /// assert_eq!(cache.get(7), 2); //The factors of 7 are 1,7.
    /// assert_eq!(cache.get(28), 6); //The factors of 28 are 1,2,4,7,14,28.
    /// ```
    pub fn get(&mut self, key: i64) -> i64 {
        if !self.divisors.contains_key(&key) {
            let mut local_divisors = vec![1_i64, key];
            let mut test = 2_i64;
            let check = key;

            while test.pow(2) < check {
                if check % test == 0 {
                    local_divisors.push(test);
                    local_divisors.push(check/test);
                }
                test += 1;
            }
            if test.pow(2) == check { local_divisors.push(test) };
            self.divisors.insert(key,local_divisors.len() as i64);
        }
        *self.divisors.get(&key).unwrap()
    }
}

/// Finds the smallest triangular number, T_n, such that the number of factors of T_n is greater than a specified integer N.
///
/// # Examples
/// ```
/// use project_euler::solutions::{DivisorCache, triangle_divisors};
///
/// fn main() {
///     let mut cache = DivisorCache::new();
///     let n = 2_i64;
///     // The first few T_n are 1,3,6,10,15,21,...
///     // 6 is the first one with more than 2 factors.
///     assert_eq!(6, triangle_divisors(n, &mut cache));
/// }
/// ```
pub fn triangle_divisors(n: i64, cache: &mut DivisorCache) -> i64 {
    let mut divisor_count = 1_i64;
    let mut index         = 1_i64;
    let mut triangle_number = 1_i64;
    while divisor_count <= n {
        triangle_number = (index * (index+1)) / 2;
        if cache.divisors.contains_key(&triangle_number) {
            divisor_count = cache.get(triangle_number);
        } else {
            if index % 2 == 0 {
                divisor_count = cache.get(index/2) * cache.get(index+1);
            } else {
                divisor_count = cache.get(index) * cache.get((index + 1)/2);
            }
        }
        index += 1;
    }
    triangle_number
}

//======================================================================
// Problem 12
//======================================================================
