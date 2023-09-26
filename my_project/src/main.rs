struct FilterCondition<T> {
    condition: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.condition == item
    }
}

fn custom_filter<T>(collection: Vec<T>, filter_condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq,
{
    collection.into_iter().filter(|x| filter_condition.is_match(x)).collect()
}

fn main() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 2, 2];

    let filter_condition = FilterCondition { condition: 2 };

    let filtered_vector = custom_filter(vector, &filter_condition);

    println!("New vector: {:?}", filtered_vector);
}
