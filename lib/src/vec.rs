use std::marker::PhantomData;

struct Vec<T>{
    fix : PhantomData<T>
}

impl<T> Vec<T> {

    pub fn new() {

    }
}