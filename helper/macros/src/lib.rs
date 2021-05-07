

/// Generate a nested uniform for .. in loop.
/// 
/// # Examples
/// ```
/// for_each!{ (a, b, ...) in [0..n] => { do_something() }}
/// ```
/// Expands to
/// ```
/// for a in 0..n {
///     for b in 0..n {
///         // ... {
///             do_something()
///         // }
///     }
/// }
/// ```
#[macro_export]
macro_rules! for_each {
    (($id:ident) in [$range:expr] => $operation:expr) => {
        for $id in $range {
            $operation
        }
    };
    (( $id:ident $( , $more_id:ident)* ) in [$range:expr] => $operation:expr) => {
        for_each!{($id) in [$range] => {
            for_each!{( $( $more_id ),* ) in [$range] => $operation}
        }}
    };
}


/// Generate a nested for .. in loop with variadic range.
/// 
/// # Examples
/// ```
/// for_eachs!{ (a, b, ...) in [0..n1, 0..n2, ...] => { do_something() }}
/// ```
/// Expands to
/// ```
/// for a in 0..n1 {
///     for b in 0..n2 {
///         // ... {
///             do_something()
///         // }
///     }
/// }
/// ```
#[macro_export]
macro_rules! for_eachs {
    (($id:ident) in [$range:expr] => $operation:expr) => {
        for $id in $range {
            $operation
        }
    };
    (( $id:ident $( , $more_id:ident)* ) in [ $range:expr $( , $more_range:expr)* ] => $operation:expr) => {
        for_eachs!{($id) in [$range] => {
            for_eachs!(( $( $more_id ),* ) in [ $( $more_range ),* ] => $operation);
        }}
    };
}