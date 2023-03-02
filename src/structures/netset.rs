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

    pub fn all(&self, tags: &[T]) -> bool {
        for tag in tags {
            if !self.contains(tag) {
                return false
            }
        }
        true
    }

    pub fn any(&self, tags: &[T]) -> bool {
        for tag in tags {
            if self.contains(tag) {
                return true
            }
        }
        false
    }

    pub fn none(&self, tags: &[T]) -> bool {
        let mut res = false;
        for tag in tags {
            res |= self.contains(tag);
        }
        !res
    }

    pub fn all_g(&self, other_set: &NetSet<T>) -> bool {
        self.set.is_superset(&other_set.set)
    }

    pub fn any_g(&self, other_set: &NetSet<T>) -> bool {
        for tag in &other_set.set {
            if self.set.contains(&tag) {
                return true
            }
        }
        false
    }

    pub fn none_g(&self, other_set: &NetSet<T>) -> bool {
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
    
        assert_eq!(test_tags.all_g(&test_tags), true, "Failed all self test.");
        assert_eq!(test_tags.all(&[TestTwo, TestOne]), true, "Failed all TestTwo TestOne");
        assert_eq!(test_tags.all(&[TestTwo]), true, "Failed all TestTwo");
    
        assert_eq!(test_tags.all_g(&test_tags2), true, "Failed all test tags 1 V test tags 2");
        assert_eq!(test_tags2.all_g(&test_tags), false, "Failed all test tags 2 V test tags 1");
        assert_eq!(test_tags2.all(&[TestTwo, TestOne]), false, "Failed all TestTwo TestOne.");
    
        assert_eq!(test_tags2.all(&[TestThree]), false, "Failed all third.");
        assert_eq!(test_tags2.all_g(&test_tags3), false, "Failed all third g.");

        assert_eq!(test_tags.all_g(&test_tags3), false, "Failed all third g.");
        assert_eq!(test_tags3.all_g(&test_tags), false, "Failed all third g.");
    }
    
    #[test]
    fn gameplay_tag_group_test_any() {
        use TestEnum::*;
        let test_tags = NetSet::<TestEnum>::make_test_set(&[TestTwo, TestOne]);
        let test_tags2 = NetSet::<TestEnum>::make_test_set(&[TestOne]);
        let test_tags3 = NetSet::<TestEnum>::make_test_set(&[TestThree]);
    
        assert_eq!(test_tags.any_g(&test_tags), true, "Failed any self test.");
        assert_eq!(test_tags.any(&[TestTwo, TestOne]), true, "Failed any TestTwo TestOne");
        assert_eq!(test_tags.any(&[TestTwo]), true, "Failed any TestTwo");
    
        assert_eq!(test_tags.any_g(&test_tags2), true, "Failed any test tags 1 V test tags 2");
        assert_eq!(test_tags2.any_g(&test_tags), true, "Failed any test tags 2 V test tags 1");
        assert_eq!(test_tags2.any(&[TestTwo, TestOne]), true, "Failed any TestTwo TestOne.");
    
        assert_eq!(test_tags2.any(&[TestThree]), false, "Failed any third.");
        assert_eq!(test_tags2.any_g(&test_tags3), false, "Failed any third g.");
    }
    
    #[test]
    fn gameplay_tag_group_test_none() {
        use TestEnum::*;
        let test_tags = NetSet::<TestEnum>::make_test_set(&[TestTwo, TestOne]);
        let test_tags2 = NetSet::<TestEnum>::make_test_set(&[TestOne]);
        let test_tags3 = NetSet::<TestEnum>::make_test_set(&[TestThree]);
    
        assert_eq!(test_tags.none_g(&test_tags), false, "Failed none self test.");
        assert_eq!(test_tags.none(&[TestTwo, TestOne]), false, "Failed none TestTwo TestOne");
        assert_eq!(test_tags.none(&[TestTwo]), false, "Failed none TestTwo");
    
        assert_eq!(test_tags.none_g(&test_tags2), false, "Failed none test tags 1 V test tags 2");
        assert_eq!(test_tags2.none_g(&test_tags), false, "Failed none test tags 2 V test tags 1");
        assert_eq!(test_tags2.none(&[TestTwo, TestOne]), false, "Failed none TestTwo TestOne.");
    
        assert_eq!(test_tags2.none(&[TestThree]), true, "Failed none third.");
        assert_eq!(test_tags2.none_g(&test_tags3), true, "Failed none third g.");
    }
}