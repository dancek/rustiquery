extern crate test;

pub struct List<T> {
    items: Vec<T>,
}

impl<T> List<T> where T: Clone {
    pub fn select<F, U>(&self, f: F) -> List<U> where F: Fn(&T) -> U {
        let mut tmp = Vec::new();
        for item in self.items.iter() {
            tmp.push(f(item));
        }
        List { items: tmp }
    }

    pub fn where_<F>(&self, f: F) -> List<T> where F: Fn(&T) -> bool {
        let mut tmp = Vec::new();
        for item in self.items.iter() {
            if f(item) {
                tmp.push((*item).clone());
            }
        }
        List { items: tmp }
    }
}

#[cfg(test)]
mod tests {
    use super::List;
    use test::Bencher;

    #[test]
    fn where_even_number() {
        let xs = List { items: vec![1,2,3] };
        let filtered = xs.where_(|&x| x % 2 == 0);
        assert_eq!(filtered.items.len(), 1);
    }

    #[test]
    fn select_half_negative() {
        let xs = List { items: vec![1u8,2,3] };
        let ys = xs.select(|&x| (x as f32) / -2.0);
        assert_eq!(ys.items[0], -0.5);
    }

    #[bench]
    fn bench_select_id(b: &mut Bencher) {
        let xs = List { items: vec![1u32,2,3] };
        b.iter(|| xs.select(|&x| x));
    }

    #[bench]
    fn bench_where_even(b: &mut Bencher) {
        let xs = List { items: vec![1u32,2,3] };
        b.iter(|| xs.where_(|&x| x % 2 == 0));
    }
}
