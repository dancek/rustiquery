pub struct List<T> {
    items: Vec<T>,
}

impl<T> List<T> where T: Clone {
    pub fn where_<F>(&self, f: F) -> List<T> where F: Fn(&T) -> bool {
        let mut tmp = Vec::new();
        for item in self.items.iter() {
            if f(item) {
                tmp.push((*item).clone());
            }
        }
        return List { items: tmp }
    }
}

#[test]
fn where_bool_true() {
    let xs = List { items: vec![1u8,2,3] };
    let filtered = xs.where_(|x| *x % 2 == 0);
    assert_eq!(filtered.items.len(), 1);
}
