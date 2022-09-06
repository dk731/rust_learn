fn main() {
    let num_list1 = vec![1, 3, 1, 5, 6, 3, 3, 8, 81, 1, 3, 6];

    println!("Largest in num_list1: {}", largest(&num_list1));

    let str1 = String::from("small string");
    let str2 = "very very long string";

    println!("Longest string is: {}", logest(&str1, str2));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn logest<'a, 'b>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
