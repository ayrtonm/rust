#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

trait MyTrait {}

impl MyTrait for () {}

type Bar = impl MyTrait;

impl MyTrait for Bar {}
//~^ ERROR: conflicting implementations of trait `MyTrait` for type `()`

fn bazr() -> Bar { }

fn main() {}
