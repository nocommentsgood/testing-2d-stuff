use godot::{obj::bounds, prelude::*};

pub struct MyGd<T>
where
    T: GodotClass + godot::obj::Bounds<Memory = bounds::MemManual>,
{
    inner: Gd<T>,
}
