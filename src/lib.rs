use phper::classes::Visibility;
use phper::objects::StateObj;
use phper::strings::ZString;
use phper::{echo, functions::Argument, modules::Module, php_get_module, values::ZVal};

use phper::{classes::{ClassEntity, ClassEntry, array_access_class}};
use roaring::RoaringBitmap;

pub struct RoaringBitmapWrapper {
    bitmap: RoaringBitmap,
}

impl RoaringBitmapWrapper {
    pub fn new() -> Self {
        RoaringBitmapWrapper {
            bitmap: RoaringBitmap::new(),
        }
    }

    // 插入值
    pub fn insert(&mut self, value: u32) {
        self.bitmap.insert(value);
    }

    // 检查是否包含某个值
    pub fn contains(&self, value: u32) -> bool {
        self.bitmap.contains(value)
    }

    // 删除值
    pub fn remove(&mut self, value: u32) {
        self.bitmap.remove(value);
    }

    // pub fn __toString(&self) -> String {
    //     format!("RoaringBitmap with {} elements", self.bitmap.len())
    // }
    // pub fn __to_string(&self) -> Vec<u8> {
    //     vec![1u8]
    //     // format!("RoaringBitmap with {} elements", self.bitmap.len())
    // }
    // pub fn __tostring(&self) -> String {
    //     format!("RoaringBitmap with {} elements", self.bitmap.len())
    // }

    pub fn count(&self) -> i32 {
        self.bitmap.len() as i32
    }
}

// 为 RoaringBitmapWrapper 实现 AsMut 特性
impl AsMut<RoaringBitmap> for RoaringBitmapWrapper {
    fn as_mut(&mut self) -> &mut RoaringBitmap {
        &mut self.bitmap
    }
}

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // let roaringBitmap = ClassEntity::new("RoaringBitmap");
    let mut roaring_bitmap = ClassEntity::<RoaringBitmapWrapper>::new_with_state_constructor(
        "RoaringBitmap",
        || RoaringBitmapWrapper::new(),
    );
    // roaringBitmap.new_with_state_constructor(
    //     "RoaringBitmap",
    //     || RoaringBitmapWrapper::new()
    // );
    // roaring_bitmap.implements(iterator_class);
    roaring_bitmap.implements(|| ClassEntry::from_globals("Stringable").unwrap());
    roaring_bitmap.implements(|| ClassEntry::from_globals("Countable").unwrap());

    
    roaring_bitmap.add_method(
        "insert",
        Visibility::Public,
        |this: &mut StateObj<RoaringBitmapWrapper>, arguments: &mut [ZVal]| {
            let bitmap_instance = this.as_mut_state();
            let value = arguments[0].as_long().unwrap() as u32;
            bitmap_instance.insert(value);
            Ok::<_, phper::Error>(format!("Current: fdaf"))

            // Ok(())
        },
    ).argument(Argument::by_val("value"));
    roaring_bitmap.add_method(
        "__toString",
        Visibility::Public,
        |this: &mut StateObj<RoaringBitmapWrapper>, _: &mut [ZVal]| {
            Ok::<_, phper::Error>(format!("字符串"))
        },
    );

    roaring_bitmap.add_method(
        "ss",
        Visibility::Public,
        |this: &mut StateObj<RoaringBitmapWrapper>, _: &mut [ZVal]| {
            // let result_string = format!("RoaringBitmap with {} elements", 1);
            // 返回 Ok 包含一个 ZVal 类型的字符串
            Ok::<_, phper::Error>(format!("Current: fdaf"))
        },
    );
    roaring_bitmap.add_method(
        "count",
        Visibility::Public,
        |this: &mut StateObj<RoaringBitmapWrapper>, _: &mut [ZVal]| {
            // let result_string = format!("RoaringBitmap with {} elements", 1);
            // 返回 Ok 包含一个 ZVal 类型的字符串
            Ok::<_, phper::Error>(1i64)
        },
    );

    module.add_class(roaring_bitmap);

    module
}