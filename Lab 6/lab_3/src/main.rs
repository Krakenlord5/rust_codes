trait SimpleAnalyzable {
    fn mean(&self) -> f64;
    fn median(&self) -> f64;
}

impl SimpleAnalyzable for Vec<f64> {
    fn mean(&self) -> f64 {
        let mut total: f64 = 0.0;
        for i in self {
            total += *i
        }
        return (total / self.len() as f64) as f64
    }

    fn median(&self) -> f64 {
        if self.len() % 2 == 1 {
            return self[(self.len() / 2) as usize];
        } else {
            return ((self[(self.len() / 2) as usize] + self[(self.len() / 2) as usize + 1]) / 2.0) as f64;
        }
    }
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

impl SimpleAnalyzable for SimpleDataSet {
    fn mean(&self) -> f64 {
        let mut total: f64 = 0.0;
        for i in &self.data {
            total += *i
        }
        return (total / self.data.len() as f64) as f64
    }

    fn median(&self) -> f64 {
        if self.data.len() % 2 == 1 {
            return self.data[(self.data.len() / 2) as usize];
        } else {
            return ((self.data[(self.data.len() / 2) as usize] + self.data[(self.data.len() / 2) as usize + 1]) / 2.0) as f64;
        }
    }
}

fn main() {
    let data = SimpleDataSet::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    println!("Mean: {}", data.mean());
    println!("Median: {}", data.median());
    let filtered_data = data.filter(|&x| x > 4.0);
    println!("Flitered Mean: {}", filtered_data.mean());
    println!("Filtered Median: {}", filtered_data.median());
}
