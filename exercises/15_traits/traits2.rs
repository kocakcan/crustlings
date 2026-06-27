trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

fn main() {
    // You can optionally experiment here.
    let kocaks: Vec<String> = vec![
        String::from("Can"),
        String::from("Seyfi"),
        String::from("Leyli"),
        String::from("Dilan"),
        String::from("Medet"),
    ];
    // println!(
    //     "{:?}",
    //     kocaks
    //         .iter()
    //         .map(|k| format!("{k}Bar"))
    //         .collect::<Vec<String>>()
    // );
    println!("{:?}", kocaks.append_bar());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
