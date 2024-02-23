struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        item % 2 == self.value % 2
    }
}

fn custom_filter(collection: &[i32], condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_collection = Vec::new();
    for item in collection {
        if condition.is_match(item) {
            filtered_collection.push(item.clone());
        }
    }
    filtered_collection
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let condition = FilterCondition { value: 2 };

    let filtered_numbers: Vec<i32> = custom_filter(&numbers, &condition);

    println!("Original numbers: {:?}", numbers);
    println!("Filtered numbers: {:?}", filtered_numbers);
}
