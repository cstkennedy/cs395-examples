use std::fmt::Debug;

pub fn py_repr_from_debug<T: Debug>(obj: T) -> String {
    let repr_str = format!("{:?}", obj);
    let repr_str = repr_str.replace(" { ", "(");
    let repr_str = repr_str.replace(" }", ")");
    let repr_str = repr_str.replace(": ", "=");

    repr_str
}
