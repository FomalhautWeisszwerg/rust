#![deny(mismatched_lifetime_syntaxes)]

struct Foo;

impl Foo {
    pub fn get_mut(&'static self, x: &mut u8) -> &mut u8 {
        //~^ ERROR eliding a lifetime that's named elsewhere is confusing
        unsafe { &mut *(x as *mut _) }
    }
}

fn main() {}
