// Salih Kaplan
// Hakan Yakar
// Mustafa Angı
// Mustafa Koşmaz

// Problem 1: Implement a function that adds two numbers and returns the result.
// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
fn add(a: i32, b: i32) -> i32 {
    // Your code here
    return a + b;
}

// Problem 1.2: Implement a function that returns the larger of two numbers.
// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
fn max_of(a: i32, b: i32) -> i32 {
    // Your code here
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

// Problem 1.3: Implement a function that checks if a number is even.
// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
fn is_even(n: i32) -> bool {
    // Your code here
    return n % 2 == 0;
}

// Problem 2: Implement a function that calculates the factorial of a number.
// The factorial of n (n!) is the product of all positive integers less than or equal to n.
// For example, factorial(5) should return 120.
// https://doc.rust-lang.org/book/ch03-05-control-flow.html
fn factorial(n: u32) -> u32 {
    // Your code here
    if (n == 0) {
        return 1;
    } else {
        return factorial(n-1)*n;
    }
}

// Problem 3: Implement a function that reverses a given string.
// For example, reverse_string("rust") should return "tsur".
// https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str
// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
fn reverse_string(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars().rev() {
        result.push(c);
    }
    return result;
}

// Problem 4: Implement a function that finds the maximum element in a vector of integers.
// The function should return None if the vector is empty.
// https://doc.rust-lang.org/book/ch08-01-vectors.html
fn max_in_vector(v: &Vec<i32>) -> Option<i32> {
    // Your code here
    return v.iter().cloned().max();
}

// Problem 5: Implement a function that checks if a string is a palindrome.
// A palindrome is a word that reads the same backward as forward.
// You can use other functions if you want ;)
fn is_palindrome(s: &str) -> bool {
    // Your code here
    let mut result = String::new();
    for c in s.chars().rev() {
        result.push(c);
    }
    return result == s;
}

// Problem 6: Implement a function that calculates the sum of all the multiples of 3 or 5 below a given number n.
// For example, sum_of_multiples(10) should return 23, because 3 + 5 + 6 + 9 = 23.
fn sum_of_multiples(n: u32) -> u32 {
    // Your code here
    let mut result = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            result += i;
        }
    }
    return result;
}

// Problem 7: Implement a function that checks if a number is prime.
// The function should return true if the number is prime, and false otherwise.
// https://crypto.stackexchange.com/questions/72351/why-can-every-prime-number-be-written-as-6k%C2%B11
fn is_prime(n: u32) -> bool {
    // Your code here
    if n <= 1 {
        return false; // 0 and 1 are not prime numbers
    }
    if n <= 3 {
        return true; // 2 and 3 are prime numbers
    }
    for i in 2..n-1 {
        if (n % i == 0) {
            return false;
        }
    }
    return true;
}

fn main() {
    // You can test your functions here by calling them and printing the results
    println!("I am working");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(10, 5), 15);
        assert_eq!(add(-3, 3), 0);
        assert_eq!(add(-2, -2), -4);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(6), 720);
    }

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(-2), true);
        assert_eq!(is_even(-3), false);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("rust"), "tsur");
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string(""), "");
    }

    #[test]
    fn test_max_in_vector() {
        assert_eq!(max_in_vector(&vec![1, 3, 2, 5, 4]), Some(5));
        assert_eq!(max_in_vector(&vec![-1, -3, -2, -5, -4]), Some(-1));
        assert_eq!(max_in_vector(&vec![]), None);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(18), false);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("racecar"), true);
        assert_eq!(is_palindrome("hello"), false);
        assert_eq!(is_palindrome(""), true);
        assert_eq!(is_palindrome("a"), true);
    }

    #[test]
    fn test_sum_of_multiples() {
        assert_eq!(sum_of_multiples(10), 23);
        assert_eq!(sum_of_multiples(0), 0);
        assert_eq!(sum_of_multiples(1), 0);
        assert_eq!(sum_of_multiples(16), 60);
        assert_eq!(sum_of_multiples(1000), 233168); // The solution to Project Euler Problem 1
    }

    #[test]
    fn test_max() {
        assert_eq!(max_of(1, 2), 2);
        assert_eq!(max_of(5, 3), 5);
        assert_eq!(max_of(-1, -2), -1);
        assert_eq!(max_of(-5, 10), 10);
    }
}
