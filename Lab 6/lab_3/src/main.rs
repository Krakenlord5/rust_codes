trait SimpleAnalyzable {
    fn mean(&self) -> f64,
    fn median(&self) -> f64,
}

struct SimpleDataSet {
    data: Vec<f64>,
}

impl SimpleDataSet {
    fn new(data: Vec<f64>) -> Self {
        SimpleDataSet {
            data: data,
        }
    }

    fn filter<F>(&self, predicate: F) -> Self
    where F: Fn(&f64) -> bool
    {
        SimpleDataSet {
            data: self.data.iter().cloned().filter(predicate).collect(),
        }
    }
}

impl SimpleAnalyzable for SimpleDataSet {}

fn main() {
    println!("Hello, world!");
}
