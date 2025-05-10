use phper::classes::{ClassEntity, Interface, Visibility};
use phper::functions::{Argument, ReturnType};
use phper::objects::StateObj;
use phper::types::ReturnTypeHint;
use phper::{modules::Module, php_get_module, values::ZVal};
use roaring::RoaringBitmap;

/// Wrapper for the Rust RoaringBitmap library
pub struct RoaringBitmapWrapper {
    bitmap: RoaringBitmap,
}

impl RoaringBitmapWrapper {
    /// Creates a new empty RoaringBitmap
    pub fn new() -> Self {
        RoaringBitmapWrapper {
            bitmap: RoaringBitmap::new(),
        }
    }

    pub fn from_vec(values: Vec<u32>) -> Self {
        RoaringBitmapWrapper {
            bitmap: RoaringBitmap::from_iter(values),
        }
    }

    pub fn insert(&mut self, value: u32) -> bool {
        self.bitmap.insert(value)
    }

    // 检查是否包含某个值
    pub fn contains(&self, value: u32) -> bool {
        self.bitmap.contains(value)
    }

    pub fn remove(&mut self, value: u32) -> bool {
        self.bitmap.remove(value)
    }

    pub fn count(&self) -> u64 {
        self.bitmap.len()
    }

    pub fn clear(&mut self) {
        self.bitmap.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.bitmap.is_empty()
    }

    pub fn min(&self) -> Option<u32> {
        self.bitmap.min()
    }

    pub fn max(&self) -> Option<u32> {
        self.bitmap.max()
    }

    pub fn to_vec(&self) -> Vec<u32> {
        self.bitmap.iter().collect()
    }

    pub fn union(&mut self, other: &Self) {
        self.bitmap |= &other.bitmap;
    }

    pub fn intersect(&mut self, other: &Self) {
        self.bitmap &= &other.bitmap;
    }

    pub fn difference(&mut self, other: &Self) {
        self.bitmap -= &other.bitmap;
    }

    pub fn symmetric_difference(&mut self, other: &Self) {
        self.bitmap ^= &other.bitmap;
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.bitmap.is_subset(&other.bitmap)
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        self.bitmap.is_superset(&other.bitmap)
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.bitmap.is_disjoint(&other.bitmap)
    }
}

impl AsRef<RoaringBitmap> for RoaringBitmapWrapper {
    fn as_ref(&self) -> &RoaringBitmap {
        &self.bitmap
    }
}

impl AsMut<RoaringBitmap> for RoaringBitmapWrapper {
    fn as_mut(&mut self) -> &mut RoaringBitmap {
        &mut self.bitmap
    }
}

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        "roaring_bitmap",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    let mut roaring_bitmap =
        ClassEntity::<RoaringBitmapWrapper>::new_with_state_constructor("RoaringBitmap", || {
            RoaringBitmapWrapper::new()
        });

    // Implement PHP interfaces
    roaring_bitmap.implements(Interface::from_name("Stringable"));
    roaring_bitmap.implements(Interface::from_name("Countable"));

    roaring_bitmap
        .add_method(
            "insert",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, arguments: &mut [ZVal]| {
                let bitmap_instance = this.as_mut_state();
                let value = arguments[0].as_long().unwrap() as u32;
                let result = bitmap_instance.insert(value);
                Ok::<_, phper::Error>(result)
            },
        )
        .argument(Argument::new("value"))
        .return_type(ReturnType::new(ReturnTypeHint::Bool));

    roaring_bitmap
        .add_method(
            "contains",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, arguments: &mut [ZVal]| {
                let bitmap_instance = this.as_state();
                let value = arguments[0].as_long().unwrap() as u32;
                let result = bitmap_instance.contains(value);
                Ok::<_, phper::Error>(result)
            },
        )
        .argument(Argument::new("value"))
        .return_type(ReturnType::new(ReturnTypeHint::Bool));

    roaring_bitmap
        .add_method(
            "remove",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, arguments: &mut [ZVal]| {
                let bitmap_instance = this.as_mut_state();
                let value = arguments[0].as_long().unwrap() as u32;
                let result = bitmap_instance.remove(value);
                Ok::<_, phper::Error>(result)
            },
        )
        .argument(Argument::new("value"))
        .return_type(ReturnType::new(ReturnTypeHint::Bool));

    // Countable interface implementation
    roaring_bitmap
        .add_method(
            "count",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, _: &mut [ZVal]| {
                let bitmap_instance = this.as_state();
                let count = bitmap_instance.count() as i64;
                Ok::<_, phper::Error>(count)
            },
        )
        .return_type(ReturnType::new(ReturnTypeHint::Int));

    roaring_bitmap
        .add_method(
            "__toString",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, _: &mut [ZVal]| {
                let bitmap_instance = this.as_state();
                let count = bitmap_instance.count();
                let min = bitmap_instance
                    .min()
                    .map_or_else(|| String::from("none"), |v| v.to_string());
                let max = bitmap_instance
                    .max()
                    .map_or_else(|| String::from("none"), |v| v.to_string());

                Ok::<_, phper::Error>(format!(
                    "RoaringBitmap(count: {}, min: {}, max: {})",
                    count, min, max
                ))
            },
        )
        .return_type(ReturnType::new(ReturnTypeHint::String));

    // Additional utility methods
    roaring_bitmap.add_method(
        "clear",
        Visibility::Public,
        |this: &mut StateObj<RoaringBitmapWrapper>, _: &mut [ZVal]| {
            let bitmap_instance = this.as_mut_state();
            bitmap_instance.clear();
            Ok::<_, phper::Error>(())
        },
    );

    roaring_bitmap
        .add_method(
            "isEmpty",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, _: &mut [ZVal]| {
                let bitmap_instance = this.as_state();
                let result = bitmap_instance.is_empty();
                Ok::<_, phper::Error>(result)
            },
        )
        .return_type(ReturnType::new(ReturnTypeHint::Bool));

    roaring_bitmap
        .add_method(
            "min",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, _: &mut [ZVal]| {
                let bitmap_instance = this.as_state();
                match bitmap_instance.min() {
                    Some(value) => Ok::<_, phper::Error>(ZVal::from(value as i64)),
                    None => Ok::<_, phper::Error>(ZVal::from(())),
                }
            },
        )
        .return_type(ReturnType::new(ReturnTypeHint::Int).allow_null());

    roaring_bitmap
        .add_method(
            "max",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, _: &mut [ZVal]| {
                let bitmap_instance = this.as_state();
                match bitmap_instance.max() {
                    Some(value) => Ok::<_, phper::Error>(ZVal::from(value as i64)),
                    None => Ok::<_, phper::Error>(ZVal::from(())),
                }
            },
        )
        .return_type(ReturnType::new(ReturnTypeHint::Int).allow_null());

    // Set operations
    roaring_bitmap
        .add_method(
            "union",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, arguments: &mut [ZVal]| {
                let bitmap_instance = this.as_mut_state();
                let other = arguments[0].expect_z_obj()?;
                let other_bitmap =
                    unsafe { other.as_state_obj::<RoaringBitmapWrapper>().as_state() };

                bitmap_instance.union(other_bitmap);
                Ok::<_, phper::Error>(())
            },
        )
        .argument(Argument::new("other"));

    roaring_bitmap
        .add_method(
            "intersect",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, arguments: &mut [ZVal]| {
                let bitmap_instance = this.as_mut_state();
                let other = arguments[0].expect_z_obj()?;
                let other_bitmap =
                    unsafe { other.as_state_obj::<RoaringBitmapWrapper>().as_state() };
                bitmap_instance.intersect(other_bitmap);
                Ok::<_, phper::Error>(())
            },
        )
        .argument(Argument::new("other"));

    roaring_bitmap
        .add_method(
            "difference",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, arguments: &mut [ZVal]| {
                let bitmap_instance = this.as_mut_state();
                let other = arguments[0].expect_z_obj()?;
                let other_bitmap =
                    unsafe { other.as_state_obj::<RoaringBitmapWrapper>().as_state() };
                bitmap_instance.difference(other_bitmap);
                Ok::<_, phper::Error>(())
            },
        )
        .argument(Argument::new("other"));

    roaring_bitmap
        .add_method(
            "symmetricDifference",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, arguments: &mut [ZVal]| {
                let bitmap_instance = this.as_mut_state();
                let other = arguments[0].expect_z_obj()?;
                let other_bitmap =
                    unsafe { other.as_state_obj::<RoaringBitmapWrapper>().as_state() };
                bitmap_instance.symmetric_difference(other_bitmap);
                Ok::<_, phper::Error>(())
            },
        )
        .argument(Argument::new("other"));

    // Set comparison methods
    roaring_bitmap
        .add_method(
            "isSubset",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, arguments: &mut [ZVal]| {
                let bitmap_instance = this.as_state();
                let other = arguments[0].expect_z_obj()?;
                let other_bitmap =
                    unsafe { other.as_state_obj::<RoaringBitmapWrapper>().as_state() };
                let result = bitmap_instance.is_subset(other_bitmap);
                Ok::<_, phper::Error>(result)
            },
        )
        .argument(Argument::new("other"))
        .return_type(ReturnType::new(ReturnTypeHint::Bool));

    roaring_bitmap
        .add_method(
            "isSuperset",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, arguments: &mut [ZVal]| {
                let bitmap_instance = this.as_state();
                let other = arguments[0].expect_z_obj()?;
                let other_bitmap =
                    unsafe { other.as_state_obj::<RoaringBitmapWrapper>().as_state() };
                let result = bitmap_instance.is_superset(other_bitmap);
                Ok::<_, phper::Error>(result)
            },
        )
        .argument(Argument::new("other"))
        .return_type(ReturnType::new(ReturnTypeHint::Bool));

    roaring_bitmap
        .add_method(
            "isDisjoint",
            Visibility::Public,
            |this: &mut StateObj<RoaringBitmapWrapper>, arguments: &mut [ZVal]| {
                let bitmap_instance = this.as_state();
                let other = arguments[0].expect_z_obj()?;
                let other_bitmap =
                    unsafe { other.as_state_obj::<RoaringBitmapWrapper>().as_state() };
                let result = bitmap_instance.is_disjoint(other_bitmap);
                Ok::<_, phper::Error>(result)
            },
        )
        .argument(Argument::new("other"))
        .return_type(ReturnType::new(ReturnTypeHint::Bool));

    // Add the class to the module
    module.add_class(roaring_bitmap);

    module
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
        let mut rb = RoaringBitmapWrapper::new();
        assert!(!rb.contains(42));
        rb.insert(42);
        assert!(rb.contains(42));
    }

    #[test]
    fn test_remove() {
        let mut rb = RoaringBitmapWrapper::new();
        rb.insert(1);
        assert!(rb.contains(1));
        rb.remove(1);
        assert!(!rb.contains(1));
    }

    #[test]
    fn test_count_and_is_empty() {
        let mut rb = RoaringBitmapWrapper::new();
        assert_eq!(rb.count(), 0);
        assert!(rb.is_empty());
        rb.insert(1);
        assert_eq!(rb.count(), 1);
        assert!(!rb.is_empty());
    }

    #[test]
    fn test_min_max() {
        let mut rb = RoaringBitmapWrapper::new();
        assert_eq!(rb.min(), None);
        assert_eq!(rb.max(), None);
        rb.insert(10);
        rb.insert(20);
        rb.insert(5);
        assert_eq!(rb.min(), Some(5));
        assert_eq!(rb.max(), Some(20));
    }

    #[test]
    fn test_union() {
        let mut rb1 = RoaringBitmapWrapper::from_vec(vec![1, 2, 3]);
        let rb2 = RoaringBitmapWrapper::from_vec(vec![3, 4, 5]);
        rb1.union(&rb2);
        let mut result = rb1.to_vec();
        result.sort();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_intersect() {
        let mut rb1 = RoaringBitmapWrapper::from_vec(vec![1, 2, 3]);
        let rb2 = RoaringBitmapWrapper::from_vec(vec![2, 3, 4]);
        rb1.intersect(&rb2);
        let mut result = rb1.to_vec();
        result.sort();
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_difference() {
        let mut rb1 = RoaringBitmapWrapper::from_vec(vec![1, 2, 3]);
        let rb2 = RoaringBitmapWrapper::from_vec(vec![2, 3, 4]);
        rb1.difference(&rb2);
        let mut result = rb1.to_vec();
        result.sort();
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_symmetric_difference() {
        let mut rb1 = RoaringBitmapWrapper::from_vec(vec![1, 2, 3]);
        let rb2 = RoaringBitmapWrapper::from_vec(vec![2, 3, 4]);
        rb1.symmetric_difference(&rb2);
        let mut result = rb1.to_vec();
        result.sort();
        assert_eq!(result, vec![1, 4]);
    }

    #[test]
    fn test_subset_superset_disjoint() {
        let rb1 = RoaringBitmapWrapper::from_vec(vec![1, 2]);
        let rb2 = RoaringBitmapWrapper::from_vec(vec![1, 2, 3]);
        let rb3 = RoaringBitmapWrapper::from_vec(vec![4, 5]);
        assert!(rb1.is_subset(&rb2));
        assert!(rb2.is_superset(&rb1));
        assert!(rb1.is_disjoint(&rb3));
    }
}
