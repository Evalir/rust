//@ revisions: e2021 e2024
//
//@[e2021] edition: 2021
//@[e2024] edition: 2024
//
//@[e2021] run-pass
//@[e2021] run-rustfix
//@[e2024] check-fail

fn main() {
    m();
    q();
    let _ = meow();
    let _ = fallback_return();
    let _ = fully_apit();
}

fn m() {
    //[e2021]~^ WARN this function depends on never type fallback being `()`
    //[e2021]~| WARN this was previously accepted by the compiler but is being phased out; it will become a hard error in Rust 2024 and in a future release in all editions!
    let x = match true {
        true => Default::default(),
        //[e2024]~^ error: the trait bound `!: Default` is not satisfied
        false => panic!("..."),
    };

    dbg!(x);
}

fn q() -> Option<()> {
    //[e2021]~^ WARN this function depends on never type fallback being `()`
    //[e2021]~| WARN this was previously accepted by the compiler but is being phased out; it will become a hard error in Rust 2024 and in a future release in all editions!
    fn deserialize<T: Default>() -> Option<T> {
        Some(T::default())
    }

    deserialize()?;
    //[e2024]~^ error: the trait bound `!: Default` is not satisfied

    None
}

// Make sure we turbofish the right argument
fn help<'a: 'a, T: Into<()>, U>(_: U) -> Result<T, ()> {
    Err(())
}
fn meow() -> Result<(), ()> {
    //[e2021]~^ WARN this function depends on never type fallback being `()`
    //[e2021]~| WARN this was previously accepted by the compiler but is being phased out; it will become a hard error in Rust 2024 and in a future release in all editions!
    help(1)?;
    //[e2024]~^ error: the trait bound `(): From<!>` is not satisfied
    Ok(())
}

pub fn takes_apit<T>(_y: impl Fn() -> T) -> Result<T, ()> {
    Err(())
}

pub fn fallback_return() -> Result<(), ()> {
    //[e2021]~^ WARN this function depends on never type fallback being `()`
    //[e2021]~| WARN this was previously accepted by the compiler but is being phased out; it will become a hard error in Rust 2024 and in a future release in all editions!
    takes_apit(|| Default::default())?;
    //[e2024]~^ error: the trait bound `!: Default` is not satisfied
    Ok(())
}

fn mk<T>() -> Result<T, ()> {
    Err(())
}

fn takes_apit2(_x: impl Default) {}

fn fully_apit() -> Result<(), ()> {
    //[e2021]~^ WARN this function depends on never type fallback being `()`
    //[e2021]~| WARN this was previously accepted by the compiler but is being phased out; it will become a hard error in Rust 2024 and in a future release in all editions!
    takes_apit2(mk()?);
    //[e2024]~^ error: the trait bound `!: Default` is not satisfied
    Ok(())
}
