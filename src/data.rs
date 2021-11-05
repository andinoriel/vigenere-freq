use std::collections::HashMap;
use std::fmt;

struct Data {
    lengths: Vec<usize>,
    count: usize,
    frequency: f32,
}

pub struct DataSet {
    data: Vec<(String, Data)>,
}

#[allow(dead_code)]
impl DataSet {
    pub fn new() -> Self {
        DataSet { data: vec![] }
    }

    /// sort data in desceding order
    pub fn sort(&mut self) {
        self.data.sort_by(|a, b| b.1.count.cmp(&a.1.count));
    }

    /// trim data and leave only those that
    /// occurred more than `val` + 1 times
    pub fn trim(&mut self, val: usize) {
        self.data.retain(|x| x.1.count > val);
    }

    /// leave only `count` sequences
    /// in the dataset
    pub fn crop(&mut self, count: usize) {
        if count != usize::MAX {
            self.data.drain(count..self.data.len());
        }
    }

    /// find all sequences of length `len`,
    /// calculate it's frequence and distances
    pub fn analyze(&mut self, text: &str, len: usize) {
        let mut frequencies: HashMap<String, Data> = HashMap::new();

        for i in 0..text.chars().count() - len + 1 {
            let syllable = &text[i..i + len];

            if !frequencies.contains_key(syllable) {
                let mut result: Vec<usize> = Vec::new();
                let matches: Vec<(usize, &str)> = text.match_indices(syllable).collect();

                for i in 1..matches.len() {
                    result.push(matches[i].0 - matches[i - 1].0);
                }
                frequencies.insert(
                    syllable.to_owned(),
                    Data {
                        lengths: result,
                        count: matches.len(),
                        frequency: (matches.len() * len) as f32 / (text.chars().count() as f32),
                    },
                );
            }
        }

        self.data = frequencies.into_iter().collect();
    }

    /// get data about divisibles the divisors of the calculated distances
    /// for sequences in the `range` of divisors
    pub fn get_divide_data(&self, range: std::ops::Range<usize>) -> Vec<(usize, usize, usize)> {
        let mut result: Vec<(usize, usize, usize)> = Vec::new();
        let mut total = 0;

        for i in &self.data {
            total += i.1.count;
        }

        for k in range {
            let mut count = 0;

            for i in &self.data {
                for j in &i.1.lengths {
                    if j % k == 0 {
                        count += 1;
                    }
                }
            }
            result.push((k, count, total));
        }
        result
    }
}

#[allow(dead_code)]
impl fmt::Display for DataSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.data {
            writeln!(
                f,
                "Finding \"{}\" - total: {}; freq: {:.2}%\n{:?}\n",
                i.0,
                i.1.count,
                i.1.frequency * 100 as f32,
                i.1.lengths,
            )?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::DataSet;

    #[test]
    fn test_sort() {
        let encrypted = "abcab";

        let mut ds = DataSet::new();
        ds.analyze(&encrypted, 2);

        ds.sort();

        assert_eq!(ds.data[0].1.count, 2);
        assert_eq!(ds.data[1].1.count, 1);
        assert_eq!(ds.data[2].1.count, 1);
    }

    #[test]
    fn test_trim() {
        let encrypted = "abcab";

        let mut ds = DataSet::new();
        ds.analyze(&encrypted, 2);

        // trimming sequences and leaving only
        // those that occurred in the encrypted text
        // more then 1 times
        ds.trim(1);

        assert!(ds.data[0].1.lengths.len() > 0);
    }

    #[test]
    fn test_crop() {
        let encrypted = "abcab";

        let mut ds = DataSet::new();
        ds.analyze(&encrypted, 2);

        // leaving only 1 finding sequence
        // in the dataset
        ds.crop(1);

        assert_eq!(ds.data.len(), 1);
    }

    #[test]
    fn test_analyze() {
        let encrypted = "abcab";

        let mut ds = DataSet::new();
        ds.analyze(&encrypted, 2);

        // finding "ab" and test
        for i in &ds.data {
            if i.0 == "ab" {
                assert_eq!(i.1.lengths, [3]);
                assert_eq!(i.1.frequency, 0.8);
            }
        }
    }
    #[test]
    fn test_get_divide_data() {
        let encrypted = "abcab";

        let mut ds = DataSet::new();
        ds.analyze(&encrypted, 2);

        // make sure that the distance between "ab" in
        // the encrypted text is divisible by 3 and
        // it happens exactly 1 times
        // (2, x, y)
        // (3, x, y) -> [1] x % 3 == 0 -> 1 times
        // (4, x, y)
        // (5, x, y)
        // (6, x, y)
        assert_eq!(ds.get_divide_data(2..7)[1].1, 1);
    }
}
