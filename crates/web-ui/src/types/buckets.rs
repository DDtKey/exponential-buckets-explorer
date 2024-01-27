use std::ops::Deref;

#[derive(Debug, Clone)]
pub(crate) struct Buckets(Vec<Bucket>);

#[derive(Debug, Clone)]
pub(crate) struct Bucket {
    number: u32,
    le: f64,
}

impl Buckets {
    pub(crate) fn calculate(initial_value: f64, factor: f64, num_of_buckets: u32) -> Self {
        // Include last bucket, which is open ended
        // TODO: use bounded integer type to restrict passing too large values.
        //      Overflow isn't possible due to validations, but this api is isolated
        let num_of_buckets = num_of_buckets
            .checked_add(1)
            .expect("number of buckets - overflow");
        let mut buckets = Vec::with_capacity(num_of_buckets as usize);
        let mut next_value = initial_value;

        // starting from second bucket, since first one is already added
        for bucket_num in 1..num_of_buckets {
            buckets.push(Bucket::new(bucket_num, next_value));
            next_value *= factor;
        }
        // last bucket is open ended
        buckets.push(Bucket::new(num_of_buckets, f64::INFINITY));

        Self(buckets)
    }
}

impl Bucket {
    fn new(number: u32, le: f64) -> Self {
        Self { number, le }
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn le(&self) -> f64 {
        self.le
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
