use crate::word::Word;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::thread;

pub(crate) trait IntoHashMap<K, V> {
    fn to_hashmap(self) -> HashMap<K, V>;
}

impl IntoHashMap<char, u16> for &str {
    fn to_hashmap(self) -> HashMap<char, u16> {
        let mut letters: HashMap<char, u16> = HashMap::new();
        self.chars().for_each(|c| {
            let lettre_counter = letters.entry(c).or_insert(0);
            *lettre_counter += 1;
        });
        letters
    }
}

pub(crate) trait HashMapUtils<K, V> {
    fn contains(&self, other: &HashMap<K, V>) -> bool;
    fn merge(&mut self, other: &HashMap<K, V>);
}

impl HashMapUtils<char, u16> for HashMap<char, u16> {
    fn contains(&self, other: &HashMap<char, u16>) -> bool {
        for (key, val) in other.iter() {
            if !self.contains_key(key) {
                return false;
            }
            if self.get(key).unwrap() < val {
                return false;
            }
        }
        true
    }

    fn merge(&mut self, other: &HashMap<char, u16>) {
        for (key, val) in other.iter() {
            let entry = self.entry(*key).or_insert(0);
            *entry += val
        }
    }
}

pub(crate) trait ToTupleIndex {
    fn to_tuple_index(&self) -> Vec<(usize, usize)>;
}

pub(crate) trait FromTupleIndex {
    fn from_tuple_index(&self, tuple: Vec<(usize, usize)>) -> Self;
}

impl ToTupleIndex for Vec<&Word> {
    #[inline]
    fn to_tuple_index(&self) -> Vec<(usize, usize)> {
        self.iter()
            .enumerate()
            .map(|(i, w)| (i, w.weight()))
            .collect()
    }
}

impl FromTupleIndex for Vec<&Word> {
    #[inline]
    fn from_tuple_index(&self, tuple: Vec<(usize, usize)>) -> Self {
        tuple.iter().map(|x| *self.get(x.0).unwrap()).collect()
    }
}

pub(crate) fn find_sum(mut data: Vec<(usize, usize)>, goal: usize) -> Vec<Vec<(usize, usize)>> {


#[inline]
    data.sort_unstable_by(|a, b| a.extract().cmp(&b.extract()));
    let data1 = data.into_iter().enumerate().rev();
    {
        let mut data = data1;
        let goal = goal;
        let rest = 0;
        let floor: Vec<(usize, usize)> = vec![];
        let mut buffer = vec![];
        let floor_sum = floor.iter().map(|x| x.extract()).sum::<usize>();
        let mut thread_vec = Vec::new();
        while let Some((index, number)) = data.next() {
            println!("{}", index);
            match (number.extract() + floor_sum).cmp(&goal) {
                Ordering::Equal => {
                    let mut v = vec![number];
                    v.extend_from_slice(&floor);
                    buffer.push(v)
                }
                Ordering::Less => {
                    if (index + 1) * number.extract() < rest {
                        break;
                    }
                    let mut v = vec![number];
                    let cloned_data = data.clone();
                    v.extend_from_slice(&floor);
                    let th = thread::spawn(move || {
                        find_sum_rec(
                            cloned_data,
                            goal,
                            goal - floor_sum - number.extract(),
                            v,
                        )
                    });
                    thread_vec.push(th);
                }
            
                Ordering::Greater => {}
            }
        }
        for t in thread_vec {
            buffer.append(&mut t.join().unwrap())
        }
        buffer
    }
}

fn find_sum_rec<I>(
    mut data: I,
    goal: usize,
    rest: usize,
    floor: Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>>
where
    I: Iterator<Item = (usize, (usize, usize))> + Clone + Send + Sync,
{
    let mut buffer = vec![];
    let floor_sum = floor.iter().map(|x| x.1).sum::<usize>();

    while let Some((index, number)) = data.next() {
        match (number.1 + floor_sum).cmp(&goal) {
            Ordering::Equal => {
                let mut v = vec![number];
                v.extend_from_slice(&floor);
                buffer.push(v)
            }
            Ordering::Less => {
                if (index + 1) * number.1 < rest {
                    break;
                }
                let mut v = vec![number];
                v.extend_from_slice(&floor);
                find_sum_rec(
                    data.clone(),
                    goal,
                    goal - floor_sum - number.extract(),
                    v,
                ).into_iter().for_each(|x| buffer.push(x))
            }

            Ordering::Greater => {}
        }
    }
    buffer
}

#[cfg(test)]
#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr),*) => {
        {
            let mut hm = HashMap::new();
            $(
                hm.insert($k, $v);
            )*
            hm
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn empty_str() {
        assert_eq!("".to_hashmap(), HashMap::new())
    }

    #[test]
    fn not_empty_str() {
        let letters = hashmap!['a' => 2, 'b' => 2, 'e' => 1, 'r' => 1];
        assert_eq!("babare".to_hashmap(), letters)
    }

    #[test]
    fn str_with_all_letter() {
        let _start = Instant::now();
        "abcdefghijklmnopqrstuvwxuyz".to_hashmap();
        // println!("str_with_all_letter {:?}", _start.elapsed());
    }
    #[test]
    fn str_with_all_letter_multiple_time() {
        let _start = Instant::now();
        "abcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyzabcdefghijklmnopqrstuvwxuyz".to_hashmap();
        // println!("str_with_all_letter_multiple_time {:?}", _start.elapsed());
    }

    #[test]
    fn test_hashmap_contains_hashmap() {
        let hm1 = hashmap!['a' => 2, 'b' => 1];
        let hm2 = hashmap!['a' => 3, 'b' => 1, 'c' => 1];
        assert!(hm2.contains(&hm1));
        assert!(!hm1.contains(&hm2));
        assert!(hm1.contains(&hm1));
    }

    #[test]
    fn merge_existing_key() {
        let mut hm1 = hashmap!['a' => 2, 'b' => 1];
        let hm2 = hashmap!['a' => 3, 'b' => 1];
        hm1.merge(&hm2);
        assert_eq!(hm1, hashmap!['a' => 5, 'b' => 2])
    }

    #[test]
    fn merge_with_new_key() {
        let mut hm1 = hashmap!['a' => 2, 'b' => 1];
        let hm2 = hashmap!['a' => 3, 'b' => 1, 'c' => 2];
        hm1.merge(&hm2);
        assert_eq!(hm1, hashmap!['a' => 5, 'b' => 2, 'c' => 2])
    }

    #[macro_export]
    macro_rules! tuple {
        ($($w: expr),*) => {
            {
                let mut vec = vec![];
                $(
                    vec.push((10 * $w, $w));
                )*
                vec
            }
        };
    }

    #[test]
    fn much_data() {
        let start = Instant::now();
        let data: Vec<(usize, usize)> = tuple![
            1, 1, 10, 10, 15, 20, 19, 24, 24, 23, 15, 19, 19, 23, 25, 19, 23, 27, 20, 24, 28, 15,
            28, 29, 29, 16, 20, 34, 26, 20, 24, 38, 30, 28, 28, 32, 24, 38, 24, 28, 28, 31, 35, 30,
            36, 39, 44, 39, 44, 28, 38, 24, 28, 28, 28, 32, 46, 29, 39, 38, 40, 43, 46, 47, 25, 29,
            29, 34, 29, 29, 29, 33, 33, 47, 34, 34, 39, 43, 39, 39, 45, 40, 39, 43, 29, 41, 44, 40,
            31, 35, 35, 49, 47, 48, 42, 47, 51, 38, 43, 30, 39, 43, 38, 43, 33, 46, 47, 47, 31, 40,
            44, 45, 32, 36, 36, 50, 36, 40, 40, 44, 54, 36, 40, 44, 48, 58, 41, 45, 45, 40, 49, 48,
            58, 49, 58, 58, 44, 48, 48, 62, 29, 33, 47, 42, 36, 40, 40, 39, 43, 43, 33, 43, 47, 46,
            51, 42, 42, 58, 47, 46, 51, 44, 48, 48, 43, 48, 48, 35, 53, 37, 41, 41, 47, 47, 43, 57,
            59, 43, 61, 38, 47, 51, 65, 52, 34, 44, 48, 48, 34, 48, 48, 48, 52, 39, 53, 53, 47, 44,
            48, 48, 39, 44, 58, 58, 41, 59, 59, 44, 44, 43, 48, 47, 48, 52, 52, 61, 60, 49, 49, 50,
            59, 50, 53, 54, 54, 45, 49, 49, 67, 38, 51, 59, 53, 47, 61, 43, 57, 48, 49, 41, 58, 58,
            57, 52, 47, 57, 45, 50, 59, 49, 63, 41, 51, 41, 54, 55, 55, 46, 50, 50, 51, 55, 55, 45,
            55, 45, 59, 59, 45, 49, 49, 63, 63, 56, 60, 60, 55, 45, 49, 49, 63, 59, 63, 54, 54, 55,
            59, 59, 63, 67, 63, 50, 60, 63, 64, 64, 64, 59, 58, 67, 68, 64, 68, 68, 63, 53, 67, 67,
            63, 67, 67, 48, 52, 43, 47, 47, 61, 45, 55, 45, 59, 59, 58, 48, 58, 48, 62, 62, 52, 51,
            65, 61, 61, 47, 51, 51, 56, 59, 65, 63, 53, 67, 67, 56, 70, 48, 62, 54, 48, 46, 56, 46,
            51, 42, 42, 58, 47, 46, 51, 44, 48, 48, 43, 48, 48, 35, 53, 37, 41, 41, 47, 47, 43, 57,
            44, 45, 32, 36, 36, 50, 36, 40, 40, 44, 54, 36, 40, 44, 48, 58, 41, 45, 45, 40, 49, 48,
            58, 49, 58, 58, 44, 48, 48, 62, 29, 33, 47, 42, 36, 40, 40, 39, 43, 43, 33, 43, 47, 46,
            51, 42, 42, 58, 47, 46, 51, 44, 48, 48, 43, 48, 48, 35, 53, 37, 41, 41, 47, 47, 43, 57,
            59, 43, 61, 38, 47, 51, 65, 52, 34, 44, 48, 48, 34, 48, 48, 48, 52, 39, 53, 53, 47, 44,
            48, 48, 39, 44, 58, 58, 41, 59, 59, 44, 44, 43, 48, 47, 48, 52, 52, 61, 60, 49, 49, 50,
            59, 50, 53, 54, 54, 45, 49, 49, 67, 38, 51, 59, 53, 47, 61, 43, 57, 48, 49, 41, 58, 58,
            57, 52, 47, 57, 45, 50, 59, 49, 63, 41, 51, 41, 54, 55, 55, 46, 50, 50, 51, 55, 55, 45,
            55, 45, 59, 59, 45, 49, 49, 63, 63, 56, 60, 60, 55, 45, 49, 49, 63, 59, 63, 54, 54, 55,
            59, 59, 63, 67, 63, 50, 60, 63, 64, 64, 64, 59, 58, 67, 68, 64, 68, 68, 63, 53, 67, 67,
            63, 67, 67, 48, 52, 43, 47, 47, 61, 45, 55, 45, 59, 59, 58, 48, 58, 48, 62, 62, 52, 51,
            65, 61, 61, 47, 51, 51, 56, 59, 65, 63, 53, 67, 67, 56, 70, 48, 62, 54, 48, 46, 56, 46,
            51, 42, 42, 58, 47, 46, 51, 44, 48, 48, 43, 48, 48, 35, 53, 37, 41, 41, 47, 47, 43, 57,
            59, 43, 61, 38, 47, 51, 65, 52, 34, 44, 48, 48, 34, 48, 48, 48, 52, 39, 53, 53, 47, 44,
            48, 48, 39, 44, 58, 58, 41, 59, 59, 44, 44, 43, 48, 47, 48, 52, 52, 61, 60, 49, 49, 50,
            59, 50, 53, 54, 54, 45, 49, 49, 67, 38, 51, 59, 53, 47, 61, 43, 57, 48, 49, 41, 58, 58,
            57, 52, 47, 57, 45, 50, 59, 49, 63, 41, 51, 41, 54, 55, 55, 46, 50, 50, 51, 55, 55, 45,
            55, 45, 59, 59, 45, 49, 49, 63, 63, 56, 60, 60, 55, 45, 49, 49, 63, 59, 63, 54, 54, 55,
            59, 59, 63, 67, 63, 50, 60, 63, 64, 64, 64, 59, 58, 67, 68, 64, 68, 68, 63, 53, 67, 67,
            63, 67, 67, 48, 52, 43, 47, 47, 61, 45, 55, 45, 59, 59, 58, 48, 58, 48, 62, 62, 52, 51,
            65, 61, 61, 47, 51, 51, 56, 59, 65, 63, 53, 67, 67, 56, 70, 48, 62, 54, 48, 46, 56, 46,
            59, 60, 60, 59, 66, 52, 66, 62, 57, 62, 50, 52, 62, 52, 56, 56, 65, 63, 66, 52, 70, 67,
            53, 52, 53, 58, 52, 63, 52, 66, 63, 53, 66, 67, 67, 58, 63, 51, 51, 60, 49, 63, 63, 62,
            66, 67, 71, 66, 80, 64, 68, 68, 59, 59, 63, 63, 63, 52, 57, 68, 51, 55, 55, 64, 54, 64,
            67, 68, 68, 69, 73, 73, 72, 57, 70, 78, 64, 58, 66, 62, 66, 70, 70, 62, 67, 67, 60, 61,
            66, 67, 66, 64, 50, 68, 69, 60, 60, 55, 63, 55, 65, 55, 68, 69, 69, 60, 70, 60, 73, 74,
            74, 64, 64, 59, 64, 54, 68, 68, 68, 82, 65, 65, 78, 64, 67, 68, 68, 63, 73, 73, 64, 74,
            77, 78, 78, 73, 82, 69, 64, 77, 73, 86, 72, 78, 82, 82, 72, 82, 72, 86, 86, 86, 62, 52,
            66, 66, 64, 64, 59, 63, 63, 77, 67, 67, 67, 70, 66, 66, 70, 70, 75, 54, 68, 77, 66, 70,
            70, 72, 75, 67, 67, 65, 65, 60, 64, 78, 71, 76, 60, 64, 64, 69, 71, 70, 71, 75, 75, 66,
            70, 70, 70, 84, 82, 66, 70, 72, 86, 68, 71, 72, 72, 71, 71, 72, 67, 56, 69, 70, 70, 68,
            66, 85, 74, 83, 73, 87, 87, 68, 68, 81, 81, 72, 71, 66, 76, 74, 60, 70, 73, 74, 74, 83,
            74, 73, 68, 91, 86, 91, 73, 83, 81, 85, 85, 89, 89, 80, 64, 69, 78, 75, 64, 74, 68, 82,
            74, 74, 69, 79, 79, 74, 79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78,
            59, 60, 60, 59, 66, 52, 66, 62, 57, 62, 50, 52, 62, 52, 56, 56, 65, 63, 66, 52, 70, 67,
            53, 52, 53, 58, 52, 63, 52, 66, 63, 53, 66, 67, 67, 58, 63, 51, 51, 60, 49, 63, 63, 62,
            66, 67, 71, 66, 80, 64, 68, 68, 59, 59, 63, 63, 63, 52, 57, 68, 51, 55, 55, 64, 54, 64,
            67, 68, 68, 69, 73, 73, 72, 57, 70, 78, 64, 58, 66, 62, 66, 70, 70, 62, 67, 67, 60, 61,
            66, 67, 66, 64, 50, 68, 69, 60, 60, 55, 63, 55, 65, 55, 68, 69, 69, 60, 70, 60, 73, 74,
            74, 64, 64, 59, 64, 54, 68, 68, 68, 82, 65, 65, 78, 64, 67, 68, 68, 63, 73, 73, 64, 74,
            77, 78, 78, 73, 82, 69, 64, 77, 73, 86, 72, 78, 82, 82, 72, 82, 72, 86, 86, 86, 62, 52,
            66, 66, 64, 64, 59, 63, 63, 77, 67, 67, 67, 70, 66, 66, 70, 70, 75, 54, 68, 77, 66, 70,
            70, 72, 75, 67, 67, 65, 65, 60, 64, 78, 71, 76, 60, 64, 64, 69, 71, 70, 71, 75, 75, 66,
            70, 70, 70, 84, 82, 66, 70, 72, 86, 68, 71, 72, 72, 71, 71, 72, 67, 56, 69, 70, 70, 68,
            66, 85, 74, 83, 73, 87, 87, 68, 68, 81, 81, 72, 71, 66, 76, 74, 60, 70, 73, 74, 74, 83,
            74, 73, 68, 91, 86, 91, 73, 83, 81, 85, 85, 89, 89, 80, 64, 69, 78, 75, 64, 74, 68, 82,
            74, 74, 69, 79, 79, 74, 79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78,
            92, 73, 83, 83, 87, 82, 91, 87, 87, 91, 71, 79, 78, 68, 82, 82, 73, 79, 83, 83, 86, 78,
            67, 68, 68, 69, 73, 73, 72, 57, 70, 78, 64, 58, 66, 62, 66, 70, 70, 62, 67, 67, 60, 61,
            66, 67, 66, 64, 50, 68, 69, 60, 60, 55, 63, 55, 65, 55, 68, 69, 69, 60, 70, 60, 73, 74,
            74, 64, 64, 59, 64, 54, 68, 68, 68, 82, 65, 65, 78, 64, 67, 68, 68, 63, 73, 73, 64, 74,
            77, 78, 78, 73, 82, 69, 64, 77, 73, 86, 72, 78, 82, 82, 72, 82, 72, 86, 86, 86, 62, 52,
            66, 66, 64, 64, 59, 63, 63, 77, 67, 67, 67, 70, 66, 66, 70, 70, 75, 54, 68, 77, 66, 70,
            70, 72, 75, 67, 67, 65, 65, 60, 64, 78, 71, 76, 60, 64, 64, 69, 71, 70, 71, 75, 75, 66,
            70, 70, 70, 84, 82, 66, 70, 72, 86, 68, 71, 72, 72, 71, 71, 72, 67, 56, 69, 70, 70, 68,
            66, 85, 74, 83, 73, 87, 87, 68, 68, 81, 81, 72, 71, 66, 76, 74, 60, 70, 73, 74, 74, 83,
            74, 73, 68, 91, 86, 91, 73, 83, 81, 85, 85, 89, 89, 80, 64, 69, 78, 75, 64, 74, 68, 82,
            74, 74, 69, 79, 79, 74, 79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78,
            92, 73, 83, 83, 87, 82, 91, 87, 87, 91, 71, 79, 78, 68, 82, 82, 73, 79, 83, 83, 86, 78,
            85, 89, 89, 76, 87, 69, 79, 83, 69, 79, 83, 83, 89, 85, 89, 89, 89, 85, 89, 91, 86, 85,
            67, 68, 68, 69, 73, 73, 72, 57, 70, 78, 64, 58, 66, 62, 66, 70, 70, 62, 67, 67, 60, 61,
            66, 67, 66, 64, 50, 68, 69, 60, 60, 55, 63, 55, 65, 55, 68, 69, 69, 60, 70, 60, 73, 74,
            74, 64, 64, 59, 64, 54, 68, 68, 68, 82, 65, 65, 78, 64, 67, 68, 68, 63, 73, 73, 64, 74,
            77, 78, 78, 73, 82, 69, 64, 77, 73, 86, 72, 78, 82, 82, 72, 82, 72, 86, 86, 86, 62, 52,
            66, 66, 64, 64, 59, 63, 63, 77, 67, 67, 67, 70, 66, 66, 70, 70, 75, 54, 68, 77, 66, 70,
            70, 72, 75, 67, 67, 65, 65, 60, 64, 78, 71, 76, 60, 64, 64, 69, 71, 70, 71, 75, 75, 66,
            70, 70, 70, 84, 82, 66, 70, 72, 86, 68, 71, 72, 72, 71, 71, 72, 67, 56, 69, 70, 70, 68,
            66, 85, 74, 83, 73, 87, 87, 68, 68, 81, 81, 72, 71, 66, 76, 74, 60, 70, 73, 74, 74, 83,
            74, 73, 68, 91, 86, 91, 73, 83, 81, 85, 85, 89, 89, 80, 64, 69, 78, 75, 64, 74, 68, 82,
            74, 74, 69, 79, 79, 74, 79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78,
            92, 73, 83, 83, 87, 82, 91, 87, 87, 91, 71, 79, 78, 68, 82, 82, 73, 79, 83, 83, 86, 78,
            67, 68, 68, 69, 73, 73, 72, 57, 70, 78, 64, 58, 66, 62, 66, 70, 70, 62, 67, 67, 60, 61,
            66, 67, 66, 64, 50, 68, 69, 60, 60, 55, 63, 55, 65, 55, 68, 69, 69, 60, 70, 60, 73, 74,
            74, 64, 64, 59, 64, 54, 68, 68, 68, 82, 65, 65, 78, 64, 67, 68, 68, 63, 73, 73, 64, 74,
            77, 78, 78, 73, 82, 69, 64, 77, 73, 86, 72, 78, 82, 82, 72, 82, 72, 86, 86, 86, 62, 52,
            66, 66, 64, 64, 59, 63, 63, 77, 67, 67, 67, 70, 66, 66, 70, 70, 75, 54, 68, 77, 66, 70,
            70, 72, 75, 67, 67, 65, 65, 60, 64, 78, 71, 76, 60, 64, 64, 69, 71, 70, 71, 75, 75, 66,
            70, 70, 70, 84, 82, 66, 70, 72, 86, 68, 71, 72, 72, 71, 71, 72, 67, 56, 69, 70, 70, 68,
            66, 85, 74, 83, 73, 87, 87, 68, 68, 81, 81, 72, 71, 66, 76, 74, 60, 70, 73, 74, 74, 83,
            74, 73, 68, 91, 86, 91, 73, 83, 81, 85, 85, 89, 89, 80, 64, 69, 78, 75, 64, 74, 68, 82,
            74, 74, 69, 79, 79, 74, 79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78,
            92, 73, 83, 83, 87, 82, 91, 87, 87, 91, 71, 79, 78, 68, 82, 82, 73, 79, 83, 83, 86, 78,
            85, 89, 89, 76, 87, 69, 79, 83, 69, 79, 83, 83, 89, 85, 89, 89, 89, 85, 89, 91, 86, 85,
            75, 85, 92, 82, 82, 85, 79, 74, 88, 87, 92, 92, 77, 83, 92, 83, 87, 89, 78, 88, 83, 93,
            88, 92, 88, 88, 93, 88, 88, 87, 87, 97, 97, 92, 92, 86, 87, 90, 90, 92, 88, 87, 97, 88,
            74, 74, 69, 79, 79, 74, 79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78,
            92, 73, 83, 83, 87, 82, 91, 87, 87, 91, 71, 79, 78, 68, 82, 82, 73, 79, 83, 83, 86, 78,
            85, 89, 89, 76, 87, 69, 79, 83, 69, 79, 83, 83, 89, 85, 89, 89, 89, 85, 89, 91, 86, 85,
            75, 85, 92, 82, 82, 85, 79, 74, 88, 87, 92, 92, 77, 83, 92, 83, 87, 89, 78, 88, 83, 93,
            88, 92, 88, 88, 93, 88, 88, 87, 87, 97, 97, 92, 92, 86, 87, 90, 90, 92, 88, 87, 97, 88,
            66, 85, 74, 83, 73, 87, 87, 68, 68, 81, 81, 72, 71, 66, 76, 74, 60, 70, 73, 74, 74, 83,
            74, 74, 69, 79, 79, 74, 79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78,
            92, 73, 83, 83, 87, 82, 91, 87, 87, 91, 71, 79, 78, 68, 82, 82, 73, 79, 83, 83, 86, 78,
            85, 89, 89, 76, 87, 69, 79, 83, 69, 79, 83, 83, 89, 85, 89, 89, 89, 85, 89, 91, 86, 85,
            75, 85, 92, 82, 82, 85, 79, 74, 88, 87, 92, 92, 77, 83, 92, 83, 87, 89, 78, 88, 83, 93,
            88, 92, 88, 88, 93, 88, 88, 87, 87, 97, 97, 92, 92, 86, 87, 90, 90, 92, 88, 87, 97, 88,
            74, 74, 69, 79, 79, 74, 79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78,
            92, 73, 83, 83, 87, 82, 91, 87, 87, 91, 71, 79, 78, 68, 82, 82, 73, 79, 83, 83, 86, 78,
            85, 89, 89, 76, 87, 69, 79, 83, 69, 79, 83, 83, 89, 85, 89, 89, 89, 85, 89, 91, 86, 85,
            75, 85, 92, 82, 82, 85, 79, 74, 88, 87, 92, 92, 77, 83, 92, 83, 87, 89, 78, 88, 83, 93,
            88, 92, 88, 88, 93, 88, 88, 87, 87, 97, 97, 92, 92, 86, 87, 90, 90, 92, 88, 87, 97, 88,
            66, 85, 74, 83, 73, 87, 87, 68, 68, 81, 81, 72, 71, 66, 76, 74, 60, 70, 73, 74, 74, 83,
            74, 73, 68, 91, 86, 91, 73, 83, 81, 85, 85, 89, 89, 80, 64, 69, 78, 75, 64, 74, 68, 82,
            74, 74, 69, 79, 79, 74, 79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78,
            92, 73, 83, 83, 87, 82, 91, 87, 87, 91, 71, 79, 78, 68, 82, 82, 73, 79, 83, 83, 86, 78,
            85, 89, 89, 76, 87, 69, 79, 83, 69, 79, 83, 83, 89, 85, 89, 89, 89, 85, 89, 91, 86, 85,
            75, 85, 92, 82, 82, 85, 79, 74, 88, 87, 92, 92, 77, 83, 92, 83, 87, 89, 78, 88, 83, 93,
            88, 92, 88, 88, 93, 88, 88, 87, 87, 97, 97, 92, 92, 86, 87, 90, 90, 92, 88, 87, 97, 88,
            74, 74, 69, 79, 79, 74, 79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78,
            92, 73, 83, 83, 87, 82, 91, 87, 87, 91, 71, 79, 78, 68, 82, 82, 73, 79, 83, 83, 86, 78,
            85, 89, 89, 76, 87, 69, 79, 83, 69, 79, 83, 83, 89, 85, 89, 89, 89, 85, 89, 91, 86, 85,
            75, 85, 92, 82, 82, 85, 79, 74, 88, 87, 92, 92, 77, 83, 92, 83, 87, 89, 78, 88, 83, 93,
            88, 92, 88, 88, 93, 88, 88, 87, 87, 97, 97, 92, 92, 86, 87, 90, 90, 92, 88, 87, 97, 88,
            88, 104, 91, 91, 94, 83, 93, 97, 102, 106, 106, 101, 102, 106, 102, 116, 85, 89, 89,
            76, 87, 69, 79, 83, 69, 79, 83, 83, 89, 85, 89, 89, 89, 85, 89, 91, 86, 85, 75, 85, 92,
            82, 82, 85, 79, 74, 88, 87, 92, 92, 77, 83, 92, 83, 87, 89, 78, 88, 83, 93, 88, 92, 88,
            88, 93, 88, 88, 87, 87, 97, 97, 92, 92, 86, 87, 90, 90, 92, 88, 87, 97, 88, 74, 74, 69,
            79, 79, 74, 79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78, 92, 73, 83,
            83, 87, 82, 91, 87, 87, 91, 71, 79, 78, 68, 82, 82, 73, 79, 83, 83, 86, 78, 85, 89, 89,
            76, 87, 69, 79, 83, 69, 79, 83, 83, 89, 85, 89, 89, 89, 85, 89, 91, 86, 85, 75, 85, 92,
            82, 82, 85, 79, 74, 88, 87, 92, 92, 77, 83, 92, 83, 87, 89, 78, 88, 83, 93, 88, 92, 88,
            88, 93, 88, 88, 87, 87, 97, 97, 92, 92, 86, 87, 90, 90, 92, 88, 87, 97, 88, 88, 104,
            91, 91, 94, 83, 93, 97, 102, 106, 106, 101, 102, 106, 102, 116, 74, 74, 69, 79, 79, 74,
            79, 73, 79, 83, 83, 78, 73, 87, 79, 68, 82, 82, 83, 78, 91, 78, 92, 73, 83, 83, 87, 82,
            91, 87, 87, 91, 71, 79, 78, 68, 82, 82, 73, 79, 83, 83, 86, 78, 85, 89, 89, 76, 87, 69,
            79, 83, 69, 79, 83, 83, 89, 85, 89, 89, 89, 85, 89, 91, 86, 85, 75, 85, 92, 82, 82, 85,
            79, 74, 88, 87, 92, 92, 77, 83, 92, 83, 87, 89, 78, 88, 83, 93, 88, 92, 88, 88, 93, 88,
            88, 87, 87, 97, 97, 92, 92, 86, 87, 90, 90, 92, 88, 87, 97, 88, 88, 104, 91, 91, 94,
            83, 93, 97, 102, 106, 106, 101, 102, 106, 102, 116, 120
        ];
        println!("1 - {:?}, len: {}", start.elapsed(), data.len());
        let start = Instant::now();
        let x = find_sum(data, 121);
        assert_eq!(x.len(), 17525403);
        println!("2 - {:?}", start.elapsed());
    }
}
