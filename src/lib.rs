#[no_std]

/// Takes a closure definition where the first parameter will be a [`Fn`] to the closure itself.
/// Returns a recursive closure with the same signature, except the first parameter will be
/// eliminated.
/// 
/// The passed closure needs to have at least one parameter. This
/// first parameter can be used to call the closure itself, achieving recursion.
/// It must not be annotated with a type.
/// 
/// Additional parameters will be parameters of the resulting closure.
/// All additional parameters must be annotated with types.
/// 
/// The closure definition needs to have a result-type annotation.
/// 
/// `move` can be used and has the [usual semantic](https://doc.rust-lang.org/1.18.0/book/first-edition/closures.html#move-closures).
/// 
/// # Example
/// 
/// ```
/// use fix_fn::fix_fn;
///  
/// let fib = fix_fn!(|fib, i: u32| -> u32 {
///     if i <= 1 {
///         i
///     } else {
///         // fib will call the closure recursively
///         fib(i - 1) + fib(i - 2)
///     }
/// });
///
/// // resulting lambda only has the `i: u32` parameter
/// assert_eq!(fib(7), 13);
/// ```
#[macro_export]
macro_rules! fix_fn {
    (
        $($mov:ident)? |$self_arg:ident $(, $arg_name:ident : $arg_type:ty)* $(,)? |
            -> $ret_type:ty
        $body:block
    ) => {{
        trait HideFn {
            fn call(&self, $($arg_name : $arg_type ,)*) -> $ret_type;
        }

        struct HideFnImpl<F: Fn(&dyn HideFn, $($arg_type ,)*) -> $ret_type>(F);

        impl<F: Fn(&dyn HideFn, $($arg_type ,)*) -> $ret_type> HideFn for HideFnImpl<F> {
            #[inline]
            fn call(&self, $($arg_name : $arg_type ,)*) -> $ret_type {
                self.0(self, $($arg_name ,)*)
            }
        }

        let inner = HideFnImpl(
            #[inline]
            $($mov)?
            |$self_arg, $($arg_name : $arg_type ,)*| -> $ret_type {
                let $self_arg = |$($arg_name : $arg_type ),*| $self_arg.call($($arg_name ,)*);
                {
                    $body
                }
            }
        );


        #[inline]
        move |$($arg_name : $arg_type),*| -> $ret_type {
            inner.call($($arg_name),*)
        }
    }};
    (
        $($mov:ident)? |$($arg_name:ident $(: $arg_type:ty)?),* $(,)?|
        $body:expr
    ) => {
        compile_error!("Closure passed to fix_fn needs return type!");
    };
    (
        $($mov:ident)? |$self_arg:ident : $self_type:ty $(, $arg_name:ident $(: $arg_type:ty)?)* $(,)? |
            -> $ret_type:ty
        $body:block
    ) => {
        compile_error!(concat!("First parameter ", stringify!($self_arg), " may not have type annotation!"));
    };
    (
        $($mov:ident)? |$self_arg:ident $(, $arg_name:ident $(: $arg_type:ty)?)* $(,)? |
            -> $ret_type:ty
        $body:block
    ) => {
        compile_error!("All parameters except first need to have an explicit type annotation!");
    };
}
