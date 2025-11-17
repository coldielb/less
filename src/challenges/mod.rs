use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub type_signature: String,
    pub difficulty: usize,
    pub par_score: usize,
    pub test_cases: Vec<TestCase>,
    pub is_tutorial: bool,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub expected: String,
    pub description: String,
}

pub fn get_all_challenges() -> Vec<Challenge> {
    let mut challenges = vec![];

    // Tutorial challenges (1-5)
    challenges.extend(get_tutorial_challenges());

    // Regular challenges (6-30)
    challenges.extend(get_regular_challenges());

    challenges
}

fn get_tutorial_challenges() -> Vec<Challenge> {
    vec![
        Challenge {
            id: 1,
            name: "Double a Number".to_string(),
            description: "Write a function that doubles a number. This teaches basic arithmetic and function application.".to_string(),
            type_signature: "Int -> Int".to_string(),
            difficulty: 1,
            par_score: 80,
            is_tutorial: true,
            hint: Some("Use the * operator. Try: \\x -> x * 2".to_string()),
            test_cases: vec![
                TestCase {
                    input: "5".to_string(),
                    expected: "10".to_string(),
                    description: "double 5".to_string(),
                },
                TestCase {
                    input: "0".to_string(),
                    expected: "0".to_string(),
                    description: "double 0".to_string(),
                },
                TestCase {
                    input: "-3".to_string(),
                    expected: "-6".to_string(),
                    description: "double -3".to_string(),
                },
            ],
        },
        Challenge {
            id: 2,
            name: "Filter Positives".to_string(),
            description: "Filter a list to keep only positive numbers. Learn filter and lambda syntax.".to_string(),
            type_signature: "[Int] -> [Int]".to_string(),
            difficulty: 1,
            par_score: 90,
            is_tutorial: true,
            hint: Some("Use filter with a lambda: filter (\\x -> x > 0)".to_string()),
            test_cases: vec![
                TestCase {
                    input: "[1, -2, 3, -4, 5]".to_string(),
                    expected: "[1, 3, 5]".to_string(),
                    description: "filter positives".to_string(),
                },
                TestCase {
                    input: "[-1, -2, -3]".to_string(),
                    expected: "[]".to_string(),
                    description: "all negative".to_string(),
                },
                TestCase {
                    input: "[1, 2, 3]".to_string(),
                    expected: "[1, 2, 3]".to_string(),
                    description: "all positive".to_string(),
                },
            ],
        },
        Challenge {
            id: 3,
            name: "First Three".to_string(),
            description: "Get the first three elements of a list using take.".to_string(),
            type_signature: "[Int] -> [Int]".to_string(),
            difficulty: 1,
            par_score: 70,
            is_tutorial: true,
            hint: Some("Use partial application: take 3".to_string()),
            test_cases: vec![
                TestCase {
                    input: "[1, 2, 3, 4, 5]".to_string(),
                    expected: "[1, 2, 3]".to_string(),
                    description: "first three of five".to_string(),
                },
                TestCase {
                    input: "[1, 2]".to_string(),
                    expected: "[1, 2]".to_string(),
                    description: "list shorter than 3".to_string(),
                },
                TestCase {
                    input: "[]".to_string(),
                    expected: "[]".to_string(),
                    description: "empty list".to_string(),
                },
            ],
        },
        Challenge {
            id: 4,
            name: "Pattern Match Head".to_string(),
            description: "Use pattern matching to get the first element or return 0 for empty list.".to_string(),
            type_signature: "[Int] -> Int".to_string(),
            difficulty: 2,
            par_score: 100,
            is_tutorial: true,
            hint: Some("Use match with patterns: match list with [] -> 0 | h::t -> h".to_string()),
            test_cases: vec![
                TestCase {
                    input: "[1, 2, 3]".to_string(),
                    expected: "1".to_string(),
                    description: "head of [1,2,3]".to_string(),
                },
                TestCase {
                    input: "[]".to_string(),
                    expected: "0".to_string(),
                    description: "empty list".to_string(),
                },
                TestCase {
                    input: "[42]".to_string(),
                    expected: "42".to_string(),
                    description: "single element".to_string(),
                },
            ],
        },
        Challenge {
            id: 5,
            name: "Compose Double and Square".to_string(),
            description: "Use function composition to double a number then square it.".to_string(),
            type_signature: "Int -> Int".to_string(),
            difficulty: 2,
            par_score: 110,
            is_tutorial: true,
            hint: Some("Use >> for forward pipe or create functions: \\x -> (x * 2) ^ 2".to_string()),
            test_cases: vec![
                TestCase {
                    input: "3".to_string(),
                    expected: "36".to_string(),
                    description: "3 * 2 = 6, 6^2 = 36".to_string(),
                },
                TestCase {
                    input: "5".to_string(),
                    expected: "100".to_string(),
                    description: "5 * 2 = 10, 10^2 = 100".to_string(),
                },
                TestCase {
                    input: "0".to_string(),
                    expected: "0".to_string(),
                    description: "0 * 2 = 0, 0^2 = 0".to_string(),
                },
            ],
        },
    ]
}

fn get_regular_challenges() -> Vec<Challenge> {
    vec![
        Challenge {
            id: 6,
            name: "Sum List".to_string(),
            description: "Sum all numbers in a list using fold.".to_string(),
            type_signature: "[Int] -> Int".to_string(),
            difficulty: 2,
            par_score: 80,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[1, 2, 3, 4, 5]".to_string(),
                    expected: "15".to_string(),
                    description: "sum of 1..5".to_string(),
                },
                TestCase {
                    input: "[]".to_string(),
                    expected: "0".to_string(),
                    description: "empty list".to_string(),
                },
                TestCase {
                    input: "[-1, 1, -2, 2]".to_string(),
                    expected: "0".to_string(),
                    description: "mixed signs".to_string(),
                },
            ],
        },
        Challenge {
            id: 7,
            name: "Reverse List".to_string(),
            description: "Reverse a list. Can use the builtin or implement with recursion.".to_string(),
            type_signature: "[Int] -> [Int]".to_string(),
            difficulty: 2,
            par_score: 70,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[1, 2, 3, 4, 5]".to_string(),
                    expected: "[5, 4, 3, 2, 1]".to_string(),
                    description: "reverse [1..5]".to_string(),
                },
                TestCase {
                    input: "[]".to_string(),
                    expected: "[]".to_string(),
                    description: "empty list".to_string(),
                },
                TestCase {
                    input: "[1]".to_string(),
                    expected: "[1]".to_string(),
                    description: "single element".to_string(),
                },
            ],
        },
        Challenge {
            id: 8,
            name: "Find Even Numbers".to_string(),
            description: "Filter a list to get only even numbers.".to_string(),
            type_signature: "[Int] -> [Int]".to_string(),
            difficulty: 2,
            par_score: 90,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[1, 2, 3, 4, 5, 6]".to_string(),
                    expected: "[2, 4, 6]".to_string(),
                    description: "evens from 1..6".to_string(),
                },
                TestCase {
                    input: "[1, 3, 5]".to_string(),
                    expected: "[]".to_string(),
                    description: "all odd".to_string(),
                },
                TestCase {
                    input: "[]".to_string(),
                    expected: "[]".to_string(),
                    description: "empty list".to_string(),
                },
            ],
        },
        Challenge {
            id: 9,
            name: "Nth Fibonacci".to_string(),
            description: "Calculate the nth Fibonacci number using recursion.".to_string(),
            type_signature: "Int -> Int".to_string(),
            difficulty: 3,
            par_score: 130,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "0".to_string(),
                    expected: "0".to_string(),
                    description: "fib(0)".to_string(),
                },
                TestCase {
                    input: "1".to_string(),
                    expected: "1".to_string(),
                    description: "fib(1)".to_string(),
                },
                TestCase {
                    input: "10".to_string(),
                    expected: "55".to_string(),
                    description: "fib(10)".to_string(),
                },
                TestCase {
                    input: "15".to_string(),
                    expected: "610".to_string(),
                    description: "fib(15)".to_string(),
                },
            ],
        },
        Challenge {
            id: 10,
            name: "Is Prime".to_string(),
            description: "Check if a number is prime using filters and ranges.".to_string(),
            type_signature: "Int -> Bool".to_string(),
            difficulty: 3,
            par_score: 140,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "2".to_string(),
                    expected: "true".to_string(),
                    description: "2 is prime".to_string(),
                },
                TestCase {
                    input: "17".to_string(),
                    expected: "true".to_string(),
                    description: "17 is prime".to_string(),
                },
                TestCase {
                    input: "20".to_string(),
                    expected: "false".to_string(),
                    description: "20 is not prime".to_string(),
                },
                TestCase {
                    input: "1".to_string(),
                    expected: "false".to_string(),
                    description: "1 is not prime".to_string(),
                },
            ],
        },
        Challenge {
            id: 11,
            name: "Flatten List".to_string(),
            description: "Flatten a nested list one level using concat.".to_string(),
            type_signature: "[[Int]] -> [Int]".to_string(),
            difficulty: 2,
            par_score: 70,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[[1, 2], [3, 4], [5]]".to_string(),
                    expected: "[1, 2, 3, 4, 5]".to_string(),
                    description: "flatten nested".to_string(),
                },
                TestCase {
                    input: "[]".to_string(),
                    expected: "[]".to_string(),
                    description: "empty list".to_string(),
                },
                TestCase {
                    input: "[[]]".to_string(),
                    expected: "[]".to_string(),
                    description: "nested empty".to_string(),
                },
            ],
        },
        Challenge {
            id: 12,
            name: "Quicksort".to_string(),
            description: "Implement quicksort using pattern matching and list comprehensions.".to_string(),
            type_signature: "[Int] -> [Int]".to_string(),
            difficulty: 4,
            par_score: 180,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[3, 1, 4, 1, 5, 9, 2, 6]".to_string(),
                    expected: "[1, 1, 2, 3, 4, 5, 6, 9]".to_string(),
                    description: "sort random".to_string(),
                },
                TestCase {
                    input: "[]".to_string(),
                    expected: "[]".to_string(),
                    description: "empty list".to_string(),
                },
                TestCase {
                    input: "[5, 4, 3, 2, 1]".to_string(),
                    expected: "[1, 2, 3, 4, 5]".to_string(),
                    description: "reverse sorted".to_string(),
                },
            ],
        },
        Challenge {
            id: 13,
            name: "Count Occurrences".to_string(),
            description: "Count how many times a value appears in a list.".to_string(),
            type_signature: "Int -> [Int] -> Int".to_string(),
            difficulty: 3,
            par_score: 110,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "3 [1, 2, 3, 3, 4, 3]".to_string(),
                    expected: "3".to_string(),
                    description: "3 appears 3 times".to_string(),
                },
                TestCase {
                    input: "5 [1, 2, 3, 4]".to_string(),
                    expected: "0".to_string(),
                    description: "not in list".to_string(),
                },
                TestCase {
                    input: "1 []".to_string(),
                    expected: "0".to_string(),
                    description: "empty list".to_string(),
                },
            ],
        },
        Challenge {
            id: 14,
            name: "Remove Duplicates".to_string(),
            description: "Remove duplicate elements from a list, keeping first occurrence.".to_string(),
            type_signature: "[Int] -> [Int]".to_string(),
            difficulty: 3,
            par_score: 150,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[1, 2, 3, 2, 4, 1, 5]".to_string(),
                    expected: "[1, 2, 3, 4, 5]".to_string(),
                    description: "remove dups".to_string(),
                },
                TestCase {
                    input: "[1, 1, 1]".to_string(),
                    expected: "[1]".to_string(),
                    description: "all same".to_string(),
                },
                TestCase {
                    input: "[]".to_string(),
                    expected: "[]".to_string(),
                    description: "empty list".to_string(),
                },
            ],
        },
        Challenge {
            id: 15,
            name: "Map Using Fold".to_string(),
            description: "Implement map functionality using only fold.".to_string(),
            type_signature: "(Int -> Int) -> [Int] -> [Int]".to_string(),
            difficulty: 4,
            par_score: 150,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "(\\x -> x * 2) [1, 2, 3]".to_string(),
                    expected: "[2, 4, 6]".to_string(),
                    description: "double each".to_string(),
                },
                TestCase {
                    input: "(\\x -> x + 1) []".to_string(),
                    expected: "[]".to_string(),
                    description: "empty list".to_string(),
                },
                TestCase {
                    input: "(\\x -> x ^ 2) [1, 2, 3, 4]".to_string(),
                    expected: "[1, 4, 9, 16]".to_string(),
                    description: "square each".to_string(),
                },
            ],
        },
        Challenge {
            id: 16,
            name: "Filter Using Fold".to_string(),
            description: "Implement filter functionality using only fold.".to_string(),
            type_signature: "(Int -> Bool) -> [Int] -> [Int]".to_string(),
            difficulty: 4,
            par_score: 150,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "(\\x -> x > 2) [1, 2, 3, 4]".to_string(),
                    expected: "[3, 4]".to_string(),
                    description: "greater than 2".to_string(),
                },
                TestCase {
                    input: "(\\x -> x > 10) [1, 2, 3]".to_string(),
                    expected: "[]".to_string(),
                    description: "none match".to_string(),
                },
                TestCase {
                    input: "(\\x -> true) [1, 2, 3]".to_string(),
                    expected: "[1, 2, 3]".to_string(),
                    description: "all match".to_string(),
                },
            ],
        },
        Challenge {
            id: 17,
            name: "Zip Lists".to_string(),
            description: "Combine two lists into pairs (already have zip builtin, but make it work!).".to_string(),
            type_signature: "[Int] -> [Int] -> [[Int]]".to_string(),
            difficulty: 2,
            par_score: 70,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[1, 2, 3] [4, 5, 6]".to_string(),
                    expected: "[[1, 4], [2, 5], [3, 6]]".to_string(),
                    description: "zip equal lists".to_string(),
                },
                TestCase {
                    input: "[1, 2] [3, 4, 5]".to_string(),
                    expected: "[[1, 3], [2, 4]]".to_string(),
                    description: "first shorter".to_string(),
                },
                TestCase {
                    input: "[] [1, 2]".to_string(),
                    expected: "[]".to_string(),
                    description: "empty first".to_string(),
                },
            ],
        },
        Challenge {
            id: 18,
            name: "Cartesian Product".to_string(),
            description: "Create all pairs from two lists.".to_string(),
            type_signature: "[Int] -> [Int] -> [[Int]]".to_string(),
            difficulty: 4,
            par_score: 170,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[1, 2] [3, 4]".to_string(),
                    expected: "[[1, 3], [1, 4], [2, 3], [2, 4]]".to_string(),
                    description: "2x2 product".to_string(),
                },
                TestCase {
                    input: "[] [1, 2]".to_string(),
                    expected: "[]".to_string(),
                    description: "empty first".to_string(),
                },
                TestCase {
                    input: "[1] [2]".to_string(),
                    expected: "[[1, 2]]".to_string(),
                    description: "single elements".to_string(),
                },
            ],
        },
        Challenge {
            id: 19,
            name: "Pascal's Triangle Row".to_string(),
            description: "Generate the nth row of Pascal's triangle.".to_string(),
            type_signature: "Int -> [Int]".to_string(),
            difficulty: 4,
            par_score: 180,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "0".to_string(),
                    expected: "[1]".to_string(),
                    description: "row 0".to_string(),
                },
                TestCase {
                    input: "4".to_string(),
                    expected: "[1, 4, 6, 4, 1]".to_string(),
                    description: "row 4".to_string(),
                },
                TestCase {
                    input: "2".to_string(),
                    expected: "[1, 2, 1]".to_string(),
                    description: "row 2".to_string(),
                },
            ],
        },
        Challenge {
            id: 20,
            name: "Merge Sorted Lists".to_string(),
            description: "Merge two sorted lists into one sorted list.".to_string(),
            type_signature: "[Int] -> [Int] -> [Int]".to_string(),
            difficulty: 4,
            par_score: 170,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[1, 3, 5] [2, 4, 6]".to_string(),
                    expected: "[1, 2, 3, 4, 5, 6]".to_string(),
                    description: "interleaved".to_string(),
                },
                TestCase {
                    input: "[] [1, 2, 3]".to_string(),
                    expected: "[1, 2, 3]".to_string(),
                    description: "empty first".to_string(),
                },
                TestCase {
                    input: "[1, 2, 3] []".to_string(),
                    expected: "[1, 2, 3]".to_string(),
                    description: "empty second".to_string(),
                },
            ],
        },
        Challenge {
            id: 21,
            name: "Group Consecutive Duplicates".to_string(),
            description: "Group consecutive equal elements into sublists.".to_string(),
            type_signature: "[Int] -> [[Int]]".to_string(),
            difficulty: 4,
            par_score: 190,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[1, 1, 2, 3, 3, 3, 2]".to_string(),
                    expected: "[[1, 1], [2], [3, 3, 3], [2]]".to_string(),
                    description: "group consecutive".to_string(),
                },
                TestCase {
                    input: "[1, 2, 3]".to_string(),
                    expected: "[[1], [2], [3]]".to_string(),
                    description: "all different".to_string(),
                },
                TestCase {
                    input: "[]".to_string(),
                    expected: "[]".to_string(),
                    description: "empty list".to_string(),
                },
            ],
        },
        Challenge {
            id: 22,
            name: "Run-Length Encoding".to_string(),
            description: "Encode consecutive duplicates as pairs [count, value].".to_string(),
            type_signature: "[Int] -> [[Int]]".to_string(),
            difficulty: 4,
            par_score: 190,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[1, 1, 1, 2, 3, 3]".to_string(),
                    expected: "[[3, 1], [1, 2], [2, 3]]".to_string(),
                    description: "encode runs".to_string(),
                },
                TestCase {
                    input: "[1, 2, 3]".to_string(),
                    expected: "[[1, 1], [1, 2], [1, 3]]".to_string(),
                    description: "no runs".to_string(),
                },
                TestCase {
                    input: "[]".to_string(),
                    expected: "[]".to_string(),
                    description: "empty list".to_string(),
                },
            ],
        },
        Challenge {
            id: 23,
            name: "Partial Sums".to_string(),
            description: "Create a list of partial sums (running total).".to_string(),
            type_signature: "[Int] -> [Int]".to_string(),
            difficulty: 3,
            par_score: 140,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[1, 2, 3, 4]".to_string(),
                    expected: "[1, 3, 6, 10]".to_string(),
                    description: "partial sums".to_string(),
                },
                TestCase {
                    input: "[5]".to_string(),
                    expected: "[5]".to_string(),
                    description: "single element".to_string(),
                },
                TestCase {
                    input: "[]".to_string(),
                    expected: "[]".to_string(),
                    description: "empty list".to_string(),
                },
            ],
        },
        Challenge {
            id: 24,
            name: "Maximum Element".to_string(),
            description: "Find the maximum element in a list using fold.".to_string(),
            type_signature: "[Int] -> Int".to_string(),
            difficulty: 3,
            par_score: 120,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "[3, 1, 4, 1, 5, 9, 2]".to_string(),
                    expected: "9".to_string(),
                    description: "max of list".to_string(),
                },
                TestCase {
                    input: "[-5, -2, -10]".to_string(),
                    expected: "-2".to_string(),
                    description: "all negative".to_string(),
                },
                TestCase {
                    input: "[42]".to_string(),
                    expected: "42".to_string(),
                    description: "single element".to_string(),
                },
            ],
        },
        Challenge {
            id: 25,
            name: "All Satisfy".to_string(),
            description: "Check if all elements satisfy a predicate.".to_string(),
            type_signature: "(Int -> Bool) -> [Int] -> Bool".to_string(),
            difficulty: 3,
            par_score: 130,
            is_tutorial: false,
            hint: None,
            test_cases: vec![
                TestCase {
                    input: "(\\x -> x > 0) [1, 2, 3]".to_string(),
                    expected: "true".to_string(),
                    description: "all positive".to_string(),
                },
                TestCase {
                    input: "(\\x -> x > 0) [1, -1, 3]".to_string(),
                    expected: "false".to_string(),
                    description: "has negative".to_string(),
                },
                TestCase {
                    input: "(\\x -> x > 0) []".to_string(),
                    expected: "true".to_string(),
                    description: "empty list".to_string(),
                },
            ],
        },
    ]
}
