
pub struct FixedBinnedVector {
    elements: Vec<usize>,
    min: f32,
    max: f32,
    bin_size: f32,
}

impl FixedBinnedVector {
    pub fn new(min_val: f32, max_val: f32, bin_count: usize) -> Self {
        FixedBinnedVector {
            elements: vec![0; bin_count],
            min: min_val,
            max: max_val,
            bin_size: (max_val - min_val) / (bin_count as f32),
        }
    }

    pub fn insert(&mut self, val: f32) {
        let mut bin = if val < self.min {
            0
        } else if val > self.max {
            self.elements.len() - 1
        } else {
            ((val-self.min) / self.bin_size) as usize
        };

        if bin >= self.elements.len() {
            bin = self.elements.len() - 1;
        }

        self.elements[bin] += 1;
    }

    pub fn normalize(self) -> Vec<f32> {
        let max_count = *self.elements.iter().max().unwrap() as f32;

        self.elements.into_iter().map(|c| (c as f32) / max_count).collect()
    }
}