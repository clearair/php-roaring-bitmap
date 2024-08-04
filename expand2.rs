#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use ext_php_rs::{info_table_end, info_table_row, info_table_start, prelude::*};
use ext_php_rs::{exception::PhpResult, types::Zval, zend::ce};
use roaring::RoaringBitmap;
use ext_php_rs::zend::ModuleEntry;
pub struct RoaringBitmapWrapper {
    bitmap: RoaringBitmap,
}
#[automatically_derived]
impl ::core::default::Default for RoaringBitmapWrapper {
    #[inline]
    fn default() -> RoaringBitmapWrapper {
        RoaringBitmapWrapper {
            bitmap: ::core::default::Default::default(),
        }
    }
}
impl<'a> ::ext_php_rs::convert::FromZendObject<'a> for &'a RoaringBitmapWrapper {
    #[inline]
    fn from_zend_object(
        obj: &'a ::ext_php_rs::types::ZendObject,
    ) -> ::ext_php_rs::error::Result<Self> {
        let obj = ::ext_php_rs::types::ZendClassObject::<
            RoaringBitmapWrapper,
        >::from_zend_obj(obj)
            .ok_or(::ext_php_rs::error::Error::InvalidScope)?;
        Ok(&**obj)
    }
}
impl<'a> ::ext_php_rs::convert::FromZendObjectMut<'a> for &'a mut RoaringBitmapWrapper {
    #[inline]
    fn from_zend_object_mut(
        obj: &'a mut ::ext_php_rs::types::ZendObject,
    ) -> ::ext_php_rs::error::Result<Self> {
        let obj = ::ext_php_rs::types::ZendClassObject::<
            RoaringBitmapWrapper,
        >::from_zend_obj_mut(obj)
            .ok_or(::ext_php_rs::error::Error::InvalidScope)?;
        Ok(&mut **obj)
    }
}
impl<'a> ::ext_php_rs::convert::FromZval<'a> for &'a RoaringBitmapWrapper {
    const TYPE: ::ext_php_rs::flags::DataType = ::ext_php_rs::flags::DataType::Object(
        Some(<RoaringBitmapWrapper as ::ext_php_rs::class::RegisteredClass>::CLASS_NAME),
    );
    #[inline]
    fn from_zval(zval: &'a ::ext_php_rs::types::Zval) -> ::std::option::Option<Self> {
        <Self as ::ext_php_rs::convert::FromZendObject>::from_zend_object(zval.object()?)
            .ok()
    }
}
impl<'a> ::ext_php_rs::convert::FromZvalMut<'a> for &'a mut RoaringBitmapWrapper {
    const TYPE: ::ext_php_rs::flags::DataType = ::ext_php_rs::flags::DataType::Object(
        Some(<RoaringBitmapWrapper as ::ext_php_rs::class::RegisteredClass>::CLASS_NAME),
    );
    #[inline]
    fn from_zval_mut(
        zval: &'a mut ::ext_php_rs::types::Zval,
    ) -> ::std::option::Option<Self> {
        <Self as ::ext_php_rs::convert::FromZendObjectMut>::from_zend_object_mut(
                zval.object_mut()?,
            )
            .ok()
    }
}
impl ::ext_php_rs::convert::IntoZendObject for RoaringBitmapWrapper {
    #[inline]
    fn into_zend_object(
        self,
    ) -> ::ext_php_rs::error::Result<
        ::ext_php_rs::boxed::ZBox<::ext_php_rs::types::ZendObject>,
    > {
        Ok(::ext_php_rs::types::ZendClassObject::new(self).into())
    }
}
impl ::ext_php_rs::convert::IntoZval for RoaringBitmapWrapper {
    const TYPE: ::ext_php_rs::flags::DataType = ::ext_php_rs::flags::DataType::Object(
        Some(<RoaringBitmapWrapper as ::ext_php_rs::class::RegisteredClass>::CLASS_NAME),
    );
    #[inline]
    fn set_zval(
        self,
        zv: &mut ::ext_php_rs::types::Zval,
        persistent: bool,
    ) -> ::ext_php_rs::error::Result<()> {
        use ::ext_php_rs::convert::IntoZendObject;
        self.into_zend_object()?.set_zval(zv, persistent)
    }
}
impl RoaringBitmapWrapper {
    pub fn __construct() -> Self {
        RoaringBitmapWrapper {
            bitmap: RoaringBitmap::new(),
        }
    }
    #[doc(hidden)]
    pub fn _internal_php___construct(
        ex: &mut ::ext_php_rs::zend::ExecuteData,
    ) -> ::ext_php_rs::class::ConstructorResult<Self> {
        use ::ext_php_rs::convert::IntoZval;
        use ::ext_php_rs::class::ConstructorResult;
        let parser = ex.parser();
        let parser = parser.parse();
        if parser.is_err() {
            return ConstructorResult::ArgError;
        }
        Self::__construct().into()
    }
    pub fn insert(&mut self, value: u32) {
        self.bitmap.insert(value);
    }
    #[doc(hidden)]
    pub extern "C" fn _internal_php_insert(
        ex: &mut ::ext_php_rs::zend::ExecuteData,
        retval: &mut ::ext_php_rs::types::Zval,
    ) {
        use ::ext_php_rs::convert::IntoZval;
        let mut value = ::ext_php_rs::args::Arg::new(
            "value",
            <u32 as ::ext_php_rs::convert::FromZvalMut>::TYPE,
        );
        let (parser, this) = ex.parser_method::<Self>();
        let parser = parser.arg(&mut value).parse();
        if parser.is_err() {
            return;
        }
        let this = match this {
            Some(this) => this,
            None => {
                ::ext_php_rs::exception::PhpException::default(
                        "Failed to retrieve reference to `$this`".into(),
                    )
                    .throw()
                    .unwrap();
                return;
            }
        };
        let result = this
            .insert(
                match value.val() {
                    Some(val) => val,
                    None => {
                        ::ext_php_rs::exception::PhpException::default(
                                "Invalid value given for argument `value`.".into(),
                            )
                            .throw()
                            .expect(
                                "Failed to throw exception: Invalid value given for argument `value`.",
                            );
                        return;
                    }
                },
            );
        if let Err(e) = result.set_zval(retval, false) {
            let e: ::ext_php_rs::exception::PhpException = e.into();
            e.throw().expect("Failed to throw exception");
        }
    }
    pub fn contains(&self, value: u32) -> bool {
        self.bitmap.contains(value)
    }
    #[doc(hidden)]
    pub extern "C" fn _internal_php_contains(
        ex: &mut ::ext_php_rs::zend::ExecuteData,
        retval: &mut ::ext_php_rs::types::Zval,
    ) {
        use ::ext_php_rs::convert::IntoZval;
        let mut value = ::ext_php_rs::args::Arg::new(
            "value",
            <u32 as ::ext_php_rs::convert::FromZvalMut>::TYPE,
        );
        let (parser, this) = ex.parser_method::<Self>();
        let parser = parser.arg(&mut value).parse();
        if parser.is_err() {
            return;
        }
        let this = match this {
            Some(this) => this,
            None => {
                ::ext_php_rs::exception::PhpException::default(
                        "Failed to retrieve reference to `$this`".into(),
                    )
                    .throw()
                    .unwrap();
                return;
            }
        };
        let result = this
            .contains(
                match value.val() {
                    Some(val) => val,
                    None => {
                        ::ext_php_rs::exception::PhpException::default(
                                "Invalid value given for argument `value`.".into(),
                            )
                            .throw()
                            .expect(
                                "Failed to throw exception: Invalid value given for argument `value`.",
                            );
                        return;
                    }
                },
            );
        if let Err(e) = result.set_zval(retval, false) {
            let e: ::ext_php_rs::exception::PhpException = e.into();
            e.throw().expect("Failed to throw exception");
        }
    }
    pub fn remove(&mut self, value: u32) {
        self.bitmap.remove(value);
    }
    #[doc(hidden)]
    pub extern "C" fn _internal_php_remove(
        ex: &mut ::ext_php_rs::zend::ExecuteData,
        retval: &mut ::ext_php_rs::types::Zval,
    ) {
        use ::ext_php_rs::convert::IntoZval;
        let mut value = ::ext_php_rs::args::Arg::new(
            "value",
            <u32 as ::ext_php_rs::convert::FromZvalMut>::TYPE,
        );
        let (parser, this) = ex.parser_method::<Self>();
        let parser = parser.arg(&mut value).parse();
        if parser.is_err() {
            return;
        }
        let this = match this {
            Some(this) => this,
            None => {
                ::ext_php_rs::exception::PhpException::default(
                        "Failed to retrieve reference to `$this`".into(),
                    )
                    .throw()
                    .unwrap();
                return;
            }
        };
        let result = this
            .remove(
                match value.val() {
                    Some(val) => val,
                    None => {
                        ::ext_php_rs::exception::PhpException::default(
                                "Invalid value given for argument `value`.".into(),
                            )
                            .throw()
                            .expect(
                                "Failed to throw exception: Invalid value given for argument `value`.",
                            );
                        return;
                    }
                },
            );
        if let Err(e) = result.set_zval(retval, false) {
            let e: ::ext_php_rs::exception::PhpException = e.into();
            e.throw().expect("Failed to throw exception");
        }
    }
    pub fn __toString(&self) -> String {
        {
            let res = ::alloc::fmt::format(
                format_args!(
                    "RoaringBitmap with {0} elements: {1:?}",
                    self.bitmap.len(),
                    self.bitmap,
                ),
            );
            res
        }
    }
    #[doc(hidden)]
    pub extern "C" fn _internal_php___toString(
        ex: &mut ::ext_php_rs::zend::ExecuteData,
        retval: &mut ::ext_php_rs::types::Zval,
    ) {
        use ::ext_php_rs::convert::IntoZval;
        let (parser, this) = ex.parser_method::<Self>();
        let parser = parser.parse();
        if parser.is_err() {
            return;
        }
        let this = match this {
            Some(this) => this,
            None => {
                ::ext_php_rs::exception::PhpException::default(
                        "Failed to retrieve reference to `$this`".into(),
                    )
                    .throw()
                    .unwrap();
                return;
            }
        };
        let result = this.__toString();
        if let Err(e) = result.set_zval(retval, false) {
            let e: ::ext_php_rs::exception::PhpException = e.into();
            e.throw().expect("Failed to throw exception");
        }
    }
    pub fn count(&self) -> i32 {
        32
    }
    #[doc(hidden)]
    pub extern "C" fn _internal_php_count(
        ex: &mut ::ext_php_rs::zend::ExecuteData,
        retval: &mut ::ext_php_rs::types::Zval,
    ) {
        use ::ext_php_rs::convert::IntoZval;
        let (parser, this) = ex.parser_method::<Self>();
        let parser = parser.parse();
        if parser.is_err() {
            return;
        }
        let this = match this {
            Some(this) => this,
            None => {
                ::ext_php_rs::exception::PhpException::default(
                        "Failed to retrieve reference to `$this`".into(),
                    )
                    .throw()
                    .unwrap();
                return;
            }
        };
        let result = this.count();
        if let Err(e) = result.set_zval(retval, false) {
            let e: ::ext_php_rs::exception::PhpException = e.into();
            e.throw().expect("Failed to throw exception");
        }
    }
}
/// Used by the `phpinfo()` function and when you run `php -i`.
pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    unsafe { ::ext_php_rs::ffi::php_info_print_table_start() };
    unsafe {
        ::ext_php_rs::ffi::php_info_print_table_row(
            <[()]>::len(&[(), ()]) as i32,
            ::std::ffi::CString::new("RoaringBitmap").unwrap().as_ptr(),
            ::std::ffi::CString::new("enabled").unwrap().as_ptr(),
        );
    };
    unsafe { ::ext_php_rs::ffi::php_info_print_table_end() };
}
static _RoaringBitmapWrapper_META: ::ext_php_rs::class::ClassMetadata<
    RoaringBitmapWrapper,
> = ::ext_php_rs::class::ClassMetadata::new();
impl ::ext_php_rs::class::RegisteredClass for RoaringBitmapWrapper {
    const CLASS_NAME: &'static str = "RoaringBitmap";
    const CONSTRUCTOR: ::std::option::Option<
        ::ext_php_rs::class::ConstructorMeta<Self>,
    > = Some(::ext_php_rs::class::ConstructorMeta {
        constructor: Self::_internal_php___construct,
        build_fn: {
            use ::ext_php_rs::builders::FunctionBuilder;
            fn build_fn(func: FunctionBuilder) -> FunctionBuilder {
                func
            }
            build_fn
        },
    });
    fn get_metadata() -> &'static ::ext_php_rs::class::ClassMetadata<Self> {
        &_RoaringBitmapWrapper_META
    }
    fn get_properties<'a>() -> ::std::collections::HashMap<
        &'static str,
        ::ext_php_rs::props::Property<'a, Self>,
    > {
        use ::std::iter::FromIterator;
        ::std::collections::HashMap::from_iter([])
    }
}
#[doc(hidden)]
pub extern "C" fn php_module_startup(ty: i32, module_number: i32) -> i32 {
    use ::ext_php_rs::constant::IntoConst;
    use ::ext_php_rs::flags::PropertyFlags;
    fn internal(ty: i32, module_number: i32) {}
    ::ext_php_rs::internal::ext_php_rs_startup();
    {
        let builder = ::ext_php_rs::builders::ClassBuilder::new("RoaringBitmap")
            .method(
                ::ext_php_rs::builders::FunctionBuilder::new(
                        "insert",
                        RoaringBitmapWrapper::_internal_php_insert,
                    )
                    .arg(
                        ::ext_php_rs::args::Arg::new(
                            "value",
                            <u32 as ::ext_php_rs::convert::FromZvalMut>::TYPE,
                        ),
                    )
                    .build()
                    .unwrap(),
                ::ext_php_rs::flags::MethodFlags::Public,
            )
            .method(
                ::ext_php_rs::builders::FunctionBuilder::new(
                        "contains",
                        RoaringBitmapWrapper::_internal_php_contains,
                    )
                    .arg(
                        ::ext_php_rs::args::Arg::new(
                            "value",
                            <u32 as ::ext_php_rs::convert::FromZvalMut>::TYPE,
                        ),
                    )
                    .returns(
                        <bool as ::ext_php_rs::convert::IntoZval>::TYPE,
                        false,
                        false,
                    )
                    .build()
                    .unwrap(),
                ::ext_php_rs::flags::MethodFlags::Public,
            )
            .method(
                ::ext_php_rs::builders::FunctionBuilder::new(
                        "remove",
                        RoaringBitmapWrapper::_internal_php_remove,
                    )
                    .arg(
                        ::ext_php_rs::args::Arg::new(
                            "value",
                            <u32 as ::ext_php_rs::convert::FromZvalMut>::TYPE,
                        ),
                    )
                    .build()
                    .unwrap(),
                ::ext_php_rs::flags::MethodFlags::Public,
            )
            .method(
                ::ext_php_rs::builders::FunctionBuilder::new(
                        "__toString",
                        RoaringBitmapWrapper::_internal_php___toString,
                    )
                    .returns(
                        <String as ::ext_php_rs::convert::IntoZval>::TYPE,
                        false,
                        false,
                    )
                    .build()
                    .unwrap(),
                ::ext_php_rs::flags::MethodFlags::Public,
            )
            .method(
                ::ext_php_rs::builders::FunctionBuilder::new(
                        "count",
                        RoaringBitmapWrapper::_internal_php_count,
                    )
                    .returns(
                        <i32 as ::ext_php_rs::convert::IntoZval>::TYPE,
                        false,
                        false,
                    )
                    .build()
                    .unwrap(),
                ::ext_php_rs::flags::MethodFlags::Public,
            )
            .implements(ce::stringable())
            .object_override::<RoaringBitmapWrapper>();
        let class = builder.build().expect("Unable to build class `RoaringBitmap`");
        _RoaringBitmapWrapper_META.set_ce(class);
    }
    internal(ty, module_number);
    0
}
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn get_module() -> *mut ::ext_php_rs::zend::ModuleEntry {
    fn internal(module: ModuleBuilder) -> ModuleBuilder {
        module
    }
    let mut builder = ::ext_php_rs::builders::ModuleBuilder::new(
            "php-roaring-bitmap",
            "0.1.0",
        )
        .startup_function(php_module_startup);
    let builder = internal(builder);
    match builder.build() {
        Ok(module) => module.into_raw(),
        Err(e) => {
            ::std::rt::panic_fmt(format_args!("Failed to build PHP module: {0:?}", e));
        }
    }
}
#[cfg(debug_assertions)]
#[no_mangle]
pub extern "C" fn ext_php_rs_describe_module() -> ::ext_php_rs::describe::Description {
    use ::ext_php_rs::describe::*;
    Description::new(Module {
        name: "php-roaring-bitmap".into(),
        functions: ::alloc::vec::Vec::new().into(),
        classes: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    Class {
                        name: "RoaringBitmap".into(),
                        docs: DocBlock(::alloc::vec::Vec::new().into()),
                        extends: abi::Option::None,
                        implements: <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new(["ce :: stringable()".into()]),
                            )
                            .into(),
                        properties: ::alloc::vec::Vec::new().into(),
                        methods: <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    Method {
                                        name: "__construct".into(),
                                        docs: DocBlock(::alloc::vec::Vec::new().into()),
                                        ty: MethodType::Constructor,
                                        params: ::alloc::vec::Vec::new().into(),
                                        retval: abi::Option::Some(Retval {
                                            ty: <RoaringBitmapWrapper as ::ext_php_rs::convert::IntoZval>::TYPE,
                                            nullable: false,
                                        }),
                                        _static: true,
                                        visibility: Visibility::Public,
                                    },
                                    Method {
                                        name: "insert".into(),
                                        docs: DocBlock(::alloc::vec::Vec::new().into()),
                                        ty: MethodType::Member,
                                        params: <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([
                                                    Parameter {
                                                        name: "value".into(),
                                                        ty: abi::Option::Some(
                                                            <u32 as ::ext_php_rs::convert::FromZvalMut>::TYPE,
                                                        ),
                                                        nullable: false,
                                                        default: abi::Option::None,
                                                    },
                                                ]),
                                            )
                                            .into(),
                                        retval: abi::Option::None,
                                        _static: false,
                                        visibility: Visibility::Public,
                                    },
                                    Method {
                                        name: "contains".into(),
                                        docs: DocBlock(::alloc::vec::Vec::new().into()),
                                        ty: MethodType::Member,
                                        params: <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([
                                                    Parameter {
                                                        name: "value".into(),
                                                        ty: abi::Option::Some(
                                                            <u32 as ::ext_php_rs::convert::FromZvalMut>::TYPE,
                                                        ),
                                                        nullable: false,
                                                        default: abi::Option::None,
                                                    },
                                                ]),
                                            )
                                            .into(),
                                        retval: abi::Option::Some(Retval {
                                            ty: <bool as ::ext_php_rs::convert::IntoZval>::TYPE,
                                            nullable: false,
                                        }),
                                        _static: false,
                                        visibility: Visibility::Public,
                                    },
                                    Method {
                                        name: "remove".into(),
                                        docs: DocBlock(::alloc::vec::Vec::new().into()),
                                        ty: MethodType::Member,
                                        params: <[_]>::into_vec(
                                                #[rustc_box]
                                                ::alloc::boxed::Box::new([
                                                    Parameter {
                                                        name: "value".into(),
                                                        ty: abi::Option::Some(
                                                            <u32 as ::ext_php_rs::convert::FromZvalMut>::TYPE,
                                                        ),
                                                        nullable: false,
                                                        default: abi::Option::None,
                                                    },
                                                ]),
                                            )
                                            .into(),
                                        retval: abi::Option::None,
                                        _static: false,
                                        visibility: Visibility::Public,
                                    },
                                    Method {
                                        name: "__toString".into(),
                                        docs: DocBlock(::alloc::vec::Vec::new().into()),
                                        ty: MethodType::Member,
                                        params: ::alloc::vec::Vec::new().into(),
                                        retval: abi::Option::Some(Retval {
                                            ty: <String as ::ext_php_rs::convert::IntoZval>::TYPE,
                                            nullable: false,
                                        }),
                                        _static: false,
                                        visibility: Visibility::Public,
                                    },
                                    Method {
                                        name: "count".into(),
                                        docs: DocBlock(::alloc::vec::Vec::new().into()),
                                        ty: MethodType::Member,
                                        params: ::alloc::vec::Vec::new().into(),
                                        retval: abi::Option::Some(Retval {
                                            ty: <i32 as ::ext_php_rs::convert::IntoZval>::TYPE,
                                            nullable: false,
                                        }),
                                        _static: false,
                                        visibility: Visibility::Public,
                                    },
                                ]),
                            )
                            .into(),
                        constants: ::alloc::vec::Vec::new().into(),
                    },
                ]),
            )
            .into(),
        constants: ::alloc::vec::Vec::new().into(),
    })
}
