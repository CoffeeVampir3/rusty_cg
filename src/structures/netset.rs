use bevy::{utils::HashSet};

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
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

// Recursive expansion of Reflect! macro
// ======================================

#[allow(unused_mut)]
impl<T> bevy::reflect::GetTypeRegistration for NetSet<T>
where
    T: 'static + Send + Sync + Eq + Copy + core::hash::Hash,
    HashSet<T>: bevy::reflect::Reflect,
{
    fn get_type_registration() -> bevy::reflect::TypeRegistration {
        let mut registration = bevy::reflect::TypeRegistration::of::<NetSet<T>>();
        registration.insert::<bevy::reflect::ReflectFromPtr>(
            bevy::reflect::FromType::<NetSet<T>>::from_type(),
        );
        let ignored_indices = ::core::iter::IntoIterator::into_iter([]);
        registration.insert::<bevy::reflect::serde::SerializationData>(
            bevy::reflect::serde::SerializationData::new(ignored_indices),
        );
        registration
    }
}
impl<T> bevy::reflect::Typed for NetSet<T>
where
    T: 'static + Send + Sync + Eq + Copy + core::hash::Hash,
    HashSet<T>: bevy::reflect::Reflect,
{
    fn type_info() -> &'static bevy::reflect::TypeInfo {
        static CELL: bevy::reflect::utility::GenericTypeInfoCell =
            bevy::reflect::utility::GenericTypeInfoCell::new();
        CELL.get_or_insert::<Self, _>(|| {
            let fields = [bevy::reflect::NamedField::new::<HashSet<T>>("set")];
            let info = bevy::reflect::StructInfo::new::<Self>("NetSet", &fields);
            bevy::reflect::TypeInfo::Struct(info)
        })
    }
}
impl<T> bevy::reflect::Struct for NetSet<T>
where
    T: 'static + Send + Sync + Eq + Copy + core::hash::Hash,
    HashSet<T>: bevy::reflect::Reflect,
{
    fn field(&self, name: &str) -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
        match name {
            "set" => ::core::option::Option::Some(&self.set),
            _ => ::core::option::Option::None,
        }
    }
    fn field_mut(&mut self, name: &str) -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
        match name {
            "set" => ::core::option::Option::Some(&mut self.set),
            _ => ::core::option::Option::None,
        }
    }
    fn field_at(&self, index: usize) -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
        match index {
            0usize => ::core::option::Option::Some(&self.set),
            _ => ::core::option::Option::None,
        }
    }
    fn field_at_mut(
        &mut self,
        index: usize,
    ) -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
        match index {
            0usize => ::core::option::Option::Some(&mut self.set),
            _ => ::core::option::Option::None,
        }
    }
    fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
        match index {
            0usize => ::core::option::Option::Some("set"),
            _ => ::core::option::Option::None,
        }
    }
    fn field_len(&self) -> usize {
        1usize
    }
    fn iter_fields(&self) -> bevy::reflect::FieldIter {
        bevy::reflect::FieldIter::new(self)
    }
    fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
        let mut dynamic: bevy::reflect::DynamicStruct = ::core::default::Default::default();
        dynamic.set_name(::std::string::ToString::to_string(
            bevy::reflect::Reflect::type_name(self),
        ));
        dynamic.insert_boxed("set", bevy::reflect::Reflect::clone_value(&self.set));
        dynamic
    }
}
impl<T> bevy::reflect::Reflect for NetSet<T>
where
    T: 'static + Send + Sync + Eq + Copy + core::hash::Hash,
    HashSet<T>: bevy::reflect::Reflect,
{
    #[inline]
    fn type_name(&self) -> &str {
        ::core::any::type_name::<Self>()
    }
    #[inline]
    fn get_type_info(&self) -> &'static bevy::reflect::TypeInfo {
        <Self as bevy::reflect::Typed>::type_info()
    }
    #[inline]
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn ::core::any::Any> {
        self
    }
    #[inline]
    fn as_any(&self) -> &dyn ::core::any::Any {
        self
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
        self
    }
    #[inline]
    fn into_reflect(
        self: ::std::boxed::Box<Self>,
    ) -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
        self
    }
    #[inline]
    fn as_reflect(&self) -> &dyn bevy::reflect::Reflect {
        self
    }
    #[inline]
    fn as_reflect_mut(&mut self) -> &mut dyn bevy::reflect::Reflect {
        self
    }
    #[inline]
    fn clone_value(&self) -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
        ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
    }
    #[inline]
    fn set(
        &mut self,
        value: ::std::boxed::Box<dyn bevy::reflect::Reflect>,
    ) -> ::core::result::Result<(), ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
        *self = <dyn bevy::reflect::Reflect>::take(value)?;
        ::core::result::Result::Ok(())
    }
    #[inline]
    fn apply(&mut self, value: &dyn bevy::reflect::Reflect) {
        if let bevy::reflect::ReflectRef::Struct(struct_value) =
            bevy::reflect::Reflect::reflect_ref(value)
        {
            for (i, value) in
                ::core::iter::Iterator::enumerate(bevy::reflect::Struct::iter_fields(struct_value))
            {
                let name = bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                bevy::reflect::Struct::field_mut(self, name).map(|v| v.apply(value));
            }
        } else {
            panic!("Attempted to apply non-struct type to struct type.");
        }
    }
    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
        bevy::reflect::ReflectRef::Struct(self)
    }
    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
        bevy::reflect::ReflectMut::Struct(self)
    }
    fn reflect_owned(self: ::std::boxed::Box<Self>) -> bevy::reflect::ReflectOwned {
        bevy::reflect::ReflectOwned::Struct(self)
    }
    fn reflect_partial_eq(
        &self,
        value: &dyn bevy::reflect::Reflect,
    ) -> ::core::option::Option<bool> {
        bevy::reflect::struct_partial_eq(self, value)
    }
}