use bevy::{utils::HashSet, reflect::Reflect};

#[derive(Default, Clone, Reflect, serde::Serialize, serde::Deserialize)]
pub struct NetSet<T> where T: 'static + Send + Sync + Eq + Copy + core::hash::Hash {
    set: HashSet<T>
}

impl<T> NetSet<T> where T: 'static + Send + Sync + Eq + Copy + core::hash::Hash {
    pub fn new(items: &[T]) -> Self {
        let mut set = HashSet::<T>::default();
        for item in items {
            set.insert(*item);
        }
        Self { set } 
    }

    pub fn add(&mut self, val: T) {
        self.set.insert(val);
    }

    pub fn add_many(&mut self, tags: &[T]) {
        for tag in tags {
            self.set.insert(*tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &T) {
        self.set.remove(tag);
    }

    pub fn remove_tags(&mut self, tags: &[T]) {
        for tag in tags {
            self.set.remove(tag);
        }
    }

    pub fn contains(&self, tag: &T) -> bool {
        self.set.contains(tag)
    }

    pub fn contains_all_slice(&self, items: &[T]) -> bool {
        for item in items {
            if !self.contains(item) {
                return false
            }
        }
        true
    }

    pub fn contains_any_slice(&self, items: &[T]) -> bool {
        for item in items {
            if self.contains(item) {
                return true
            }
        }
        false
    }

    pub fn contains_none_slice(&self, items: &[T]) -> bool {
        let mut res = false;
        for item in items {
            res |= self.contains(item);
        }
        !res
    }

    pub fn contains_all(&self, other_set: &NetSet<T>) -> bool {
        other_set.set.is_subset(&self.set)
    }

    pub fn contains_any(&self, other_set: &NetSet<T>) -> bool {
        for item in &other_set.set {
            if self.set.contains(&item) {
                return true
            }
        }
        false
    }

    pub fn contains_none(&self, other_set: &NetSet<T>) -> bool {
        self.set.is_disjoint(&other_set.set)
    }
}

#[cfg(test)]
mod tag_tests {
    use super::*;

    #[derive(Default, Clone, Copy, PartialEq, Hash, Eq, serde::Serialize, serde::Deserialize)]
    enum TestEnum {
        #[default]
        TestOne,
        TestTwo,
        TestThree,
        TestFour,
    }

    impl<T> NetSet<T> where T: 'static + Send + Sync + Eq + Copy + core::hash::Hash {
        #[cfg(test)]
        fn make_test_set(test_items: &[T]) -> Self {
            Self::new(test_items) 
        }
    }

    #[test]
    fn gameplay_tag_group_test_all() {
        use TestEnum::*;
        let test_tags = NetSet::<TestEnum>::make_test_set(&[TestTwo, TestOne]);
        let test_tags2 = NetSet::<TestEnum>::make_test_set(&[TestOne]);
        let test_tags3 = NetSet::<TestEnum>::make_test_set(&[TestThree]);
        let test_tags4 = NetSet::<TestEnum>::make_test_set(&[TestTwo, TestOne, TestThree, TestFour]);
    
        assert_eq!(test_tags.contains_all(&test_tags), true, "Failed all self test.");
        assert_eq!(test_tags.contains_all_slice(&[TestTwo, TestOne]), true, "Failed all TestTwo TestOne");
        assert_eq!(test_tags.contains_all_slice(&[TestTwo]), true, "Failed all TestTwo");
    
        assert_eq!(test_tags.contains_all(&test_tags2), true, "Failed all test tags 1 V test tags 2");
        assert_eq!(test_tags2.contains_all(&test_tags), false, "Failed all test tags 2 V test tags 1");
        assert_eq!(test_tags2.contains_all_slice(&[TestTwo, TestOne]), false, "Failed all TestTwo TestOne.");
    
        assert_eq!(test_tags2.contains_all_slice(&[TestThree]), false, "Failed all third.");
        assert_eq!(test_tags2.contains_all(&test_tags3), false, "Failed all third g.");

        assert_eq!(test_tags.contains_all(&test_tags4), false, "Failed all fourth g1.");
    }
    
    #[test]
    fn gameplay_tag_group_test_any() {
        use TestEnum::*;
        let test_tags = NetSet::<TestEnum>::make_test_set(&[TestTwo, TestOne]);
        let test_tags2 = NetSet::<TestEnum>::make_test_set(&[TestOne]);
        let test_tags3 = NetSet::<TestEnum>::make_test_set(&[TestThree]);
        let test_tags4 = NetSet::<TestEnum>::make_test_set(&[TestTwo, TestOne, TestThree, TestFour]);
    
        assert_eq!(test_tags.contains_any(&test_tags), true, "Failed any self test.");
        assert_eq!(test_tags.contains_any_slice(&[TestTwo, TestOne]), true, "Failed any TestTwo TestOne");
        assert_eq!(test_tags.contains_any_slice(&[TestTwo]), true, "Failed any TestTwo");
    
        assert_eq!(test_tags.contains_any(&test_tags2), true, "Failed any test tags 1 V test tags 2");
        assert_eq!(test_tags2.contains_any(&test_tags), true, "Failed any test tags 2 V test tags 1");
        assert_eq!(test_tags2.contains_any_slice(&[TestTwo, TestOne]), true, "Failed any TestTwo TestOne.");
    
        assert_eq!(test_tags2.contains_any_slice(&[TestThree]), false, "Failed any third.");
        assert_eq!(test_tags2.contains_any(&test_tags3), false, "Failed any third g.");

        assert_eq!(test_tags4.contains_any(&test_tags3), true, "Failed any fourth g.");
    }
    
    #[test]
    fn gameplay_tag_group_test_none() {
        use TestEnum::*;
        let test_tags = NetSet::<TestEnum>::make_test_set(&[TestTwo, TestOne]);
        let test_tags2 = NetSet::<TestEnum>::make_test_set(&[TestOne]);
        let test_tags3 = NetSet::<TestEnum>::make_test_set(&[TestThree]);
        let test_tags4 = NetSet::<TestEnum>::make_test_set(&[TestTwo, TestOne, TestThree, TestFour]);
    
        assert_eq!(test_tags.contains_none(&test_tags), false, "Failed none self test.");
        assert_eq!(test_tags.contains_none_slice(&[TestTwo, TestOne]), false, "Failed none TestTwo TestOne");
        assert_eq!(test_tags.contains_none_slice(&[TestTwo]), false, "Failed none TestTwo");
    
        assert_eq!(test_tags.contains_none(&test_tags2), false, "Failed none test tags 1 V test tags 2");
        assert_eq!(test_tags2.contains_none(&test_tags), false, "Failed none test tags 2 V test tags 1");
        assert_eq!(test_tags2.contains_none_slice(&[TestTwo, TestOne]), false, "Failed none TestTwo TestOne.");
    
        assert_eq!(test_tags2.contains_none_slice(&[TestThree]), true, "Failed none third.");
        assert_eq!(test_tags2.contains_none(&test_tags3), true, "Failed none third g.");

        assert_eq!(test_tags4.contains_none(&test_tags3), false, "Failed any fouth g.");
    }
}