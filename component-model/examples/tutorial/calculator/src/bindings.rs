// Generated by `wit-bindgen` 0.24.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod docs {
    #[allow(dead_code)]
    pub mod calculator {
        #[allow(dead_code, clippy::all)]
        pub mod add {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn add(a: u32, b: u32) -> u32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "bytecode_alliance:calculator/add@0.1.0")]
                    extern "C" {
                        #[link_name = "add"]
                        fn wit_import(_: i32, _: i32) -> i32;
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i32(&a), _rt::as_i32(&b));
                    ret as u32
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod docs {
        #[allow(dead_code)]
        pub mod calculator {
            #[allow(dead_code, clippy::all)]
            pub mod calculate {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                pub enum Op {
                    Add,
                }
                impl ::core::fmt::Debug for Op {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            Op::Add => f.debug_tuple("Op::Add").finish(),
                        }
                    }
                }

                impl Op {
                    pub(crate) unsafe fn _lift(val: u8) -> Op {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }

                        match val {
                            0 => Op::Add,

                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }

                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_eval_expression_cabi<T: Guest>(
                    arg0: i32,
                    arg1: i32,
                    arg2: i32,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 =
                        T::eval_expression(Op::_lift(arg0 as u8), arg1 as u32, arg2 as u32);
                    _rt::as_i32(result0)
                }
                pub trait Guest {
                    fn eval_expression(op: Op, x: u32, y: u32) -> u32;
                }
                #[doc(hidden)]

                macro_rules! __export_docs_calculator_calculate_0_1_0_cabi{
        ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

          #[export_name = "bytecode_alliance:calculator/calculate@0.1.0#eval-expression"]
          unsafe extern "C" fn export_eval_expression(arg0: i32,arg1: i32,arg2: i32,) -> i32 {
            $($path_to_types)*::_export_eval_expression_cabi::<$ty>(arg0, arg1, arg2)
          }
        };);
      }
                #[doc(hidden)]
                pub(crate) use __export_docs_calculator_calculate_0_1_0_cabi;
            }
        }
    }
}
mod _rt {

    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }

    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }

    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }

    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_calculator_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::bytecode_alliance::calculator::calculate::__export_docs_calculator_calculate_0_1_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::bytecode_alliance::calculator::calculate);
  )
}
#[doc(inline)]
pub(crate) use __export_calculator_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.24.0:calculator:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 313] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xb8\x01\x01A\x02\x01\
A\x04\x01B\x02\x01@\x02\x01ay\x01by\0y\x04\0\x03add\x01\0\x03\x01\x19bytecode_alliance:calcul\
ator/add@0.1.0\x05\0\x01B\x04\x01m\x01\x03add\x04\0\x02op\x03\0\0\x01@\x03\x02op\
\x01\x01xy\x01yy\0y\x04\0\x0feval-expression\x01\x02\x04\x01\x1fbytecode_alliance:calculator/\
calculate@0.1.0\x05\x01\x04\x01\x20bytecode_alliance:calculator/calculator@0.1.0\x04\0\x0b\x10\
\x01\0\x0acalculator\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-comp\
onent\x070.202.0\x10wit-bindgen-rust\x060.24.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
