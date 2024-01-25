use std::ops::{Deref, Range};

#[derive(Debug, Clone)]
pub(crate) struct Buckets(Vec<Bucket>);

#[derive(Debug, Clone)]
pub(crate) struct Bucket {
    number: u32,
    range: Range<f64>,
}

impl Buckets {
    pub(crate) fn calculate(initial_value: f64, factor: f64, num_of_buckets: u32) -> Self {
        // Overflow isn't possible due to validations, but this api is isolated
        // TODO: use bounded integer type to restrict passing too large values
        let capacity = num_of_buckets
            .checked_add(1)
            .expect("number of buckets - overflow");

        let mut buckets = Vec::with_capacity(capacity as usize);
        buckets.push(Bucket::new(1, 0.0..initial_value));

        let mut current_value = initial_value;

        // starting from second bucket, since first one is already added
        if num_of_buckets > 2 {
            for bucket_num in 2..num_of_buckets {
                let next_value = current_value * factor;
                buckets.push(Bucket::new(bucket_num, current_value..next_value));
                current_value = next_value;
            }
        }
        // last bucket is open ended
        buckets.push(Bucket::new(num_of_buckets, current_value..f64::INFINITY));

        Self(buckets)
    }
}

impl Bucket {
    fn new(number: u32, range: Range<f64>) -> Self {
        Self { number, range }
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn start(&self) -> f64 {
        self.range.start
    }

    pub fn end(&self) -> f64 {
        self.range.end
    }
}

impl Deref for Buckets {
    type Target = Vec<Bucket>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Buckets {
    type Item = Bucket;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
