/*! # Enums and Pattern Matching
 *
 * In this chapter, we’ll look at enumerations, also referred to as enums.
 * Enums allow you to define a type by enumerating its possible variants.
 * First we’ll define and use an enum to show how an enum can encode meaning along
 * with data. Next, we’ll explore a particularly useful enum, called `Option`,
 * which expresses that a value can be either something or nothing. Then we’ll
 * look at how pattern matching in the `match` expression makes it easy to run
 * different code for different values of an enum. Finally, we’ll cover how the
 * `if let` construct is another convenient and concise idiom available to handle
 * enums in your code.
 */
use std::any::type_name;
extern crate rand;
use rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Utility function to print type of a variable
fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}

/// # Enums and Pattern Matching Examples
///
/// Examples from Rust Book Ch. 6: Enums and Pattern Matching
fn main() {
    // Defining an Enum
    defining_an_enum();

    // Enum with associated `String` values
    enum_string_values();

    // Enum with different associated types
    enum_different_types();

    // Enum with methods
    enum_methods();

    // The `Option` Enum and Its Advantages Over Null Values
    option_type();

    // The `match` Control Flow Construct
    match_control_flow();

    // Patterns That Bind to Values
    match_control_flow_patterns();
}

/// # Defining an Enum
///
/// Where structs give you a way of grouping together related fields and data,
/// like a `Rectangle` with its `width` and `height`, enums give you a way of
/// saying a value is one of a possible set of values. For example, we may want
/// to say that `Rectangle` is one of a set of possible shapes that also
/// includes `Circle` and `Triangle`. To do this, Rust allows us to encode
/// these possibilities as an enum.
///
/// Let’s look at a situation we might want to express in code and see why
/// enums are useful and more appropriate than structs in this case. Say we
/// need to work with IP addresses. Currently, two major standards are used for
/// IP addresses: version four and version six. Because these are the only
/// possibilities for an IP address that our program will come across, we can
/// enumerate all possible variants, which is where enumeration gets its name.
///
/// Any IP address can be either a version four or a version six address, but
/// not both at the same time. That property of IP addresses makes the enum
/// data structure appropriate because an enum value can only be one of its
/// variants. Both version four and version six addresses are still
/// fundamentally IP addresses, so they should be treated as the same type when
/// the code is handling situations that apply to any kind of IP address.
///
/// We can express this concept in code by defining an IpAddrKind enumeration
/// and listing the possible kinds an IP address can be, `V4` and `V6`. These
/// are the variants of the enum.
///
fn defining_an_enum() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route_enum_kind(IpAddrKind::V4);
    route_enum_kind(IpAddrKind::V6);
    route_enum_kind(four);
    route_enum_kind(six);
}
/// # `IpAddrKind`
///
/// A custom `enum` data type to represent the possible kinds of IP addresses
///
/// We can use `IpAddrKind` elsewhere in our code.
enum IpAddrKind {
    V4,
    V6,
}

/// # Enum Values - function example
///
/// Note that the variants of the enum are namespaced under its identifier, and
/// we use a double colon to separate the two. This is useful because now both
/// values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type:
/// `IpAddrKind`. We can then, for instance, define a function that takes any
/// `IpAddrKind`
fn route_enum_kind(_ip_kind: IpAddrKind) {}

/// # Enum with associated `String` values
///
/// Representing the IP with just an enum is more concise: rather than an enum
/// inside a struct, we can put data directly into each enum variant. This new
/// definition of the `IpAddr` enum says that both `V4` and `V6` variants will
/// have associated `String` values
/// We attach data to each variant of the enum directly, so there is no need
/// for an extra struct.
fn enum_string_values() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
    println!("`home` is: {:#?}", home);
    println!("`loopback` is: {:#?}", loopback);
}
/// # `IpAddr` enum representing a specific address with a kind
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
/// # Enum with different associated types
///
/// There’s another advantage to using an enum rather than a struct: each
/// variant can have different types and amounts of associated data. Version
/// four IP addresses will always have four numeric components that will have
/// values between `0` and `255`. If we wanted to store `V4` addresses as four
/// `u8` values but still express `V6` addresses as one `String` value, we
/// wouldn’t be able to with a struct. Enums handle this case with ease
fn enum_different_types() {
    let home = IpAddrTypes::V4(127, 0, 0, 1);

    let loopback = IpAddrTypes::V6(String::from("::1"));
    println!("`home` is: {:#?}", home);
    println!("`loopback` is: {:#?}", loopback);
}
/// # Implementation of IpAddr with different associated types
///
/// An example alternative implementation of `IpAddr` using different
/// associated types for `V4` and `V6` variants.
#[derive(Debug)]
enum IpAddrTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}

/// # Rust standard library IpAddr example
///
/// The following example demonstrates how the Rust standard library implements
/// IpAddr: An enum containing two associated stuct variants
struct Ipv4Addr {
    // --snip--
}
/// # Rust standard library IpAddr example
///
/// The following example demonstrates how the Rust standard library implements
/// IpAddr: An enum containing two associated stuct variants
struct Ipv6Addr {
    // --snip--
}

/// # Implementation of IpAddr with different structs for each variant
///
/// This is how the rust standard library implements `IpAddr`:
/// it has the exact enum and variants that we’ve defined and used, but it
/// embeds the address data inside the variants in the form of two different
/// structs, which are defined differently for each variant
///
/// This code illustrates that you can put any kind of data inside an enum
/// variant: strings, numeric types, or structs, for example. You can even
/// include another enum! Also, standard library types are often not much more
/// complicated than what you might come up with.
///
/// Note that even though the standard library contains a definition for
/// `IpAddr`, we can still create and use our own definition without conflict
/// because we haven’t brought the standard library’s definition into our
/// scope. We’ll talk more about bringing types into scope in [Chapter 7][1].
///
/// [1]: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
enum IpAddrStdLibExample {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

/// # Enum with wide variety of types embedded in its variants.
///
/// `Message` enum whose variants each store different amounts and types of
/// values
/// This enum has four variants with different types:
///
/// - `Quit` has no data associated with it at all.
/// - `Move` has named fields, like a struct does.
/// - `Write` includes a single `String`.
/// - `ChangeColor` includes three `i32` values.
///
/// If we used the different structs, each of which has its own type, we
/// couldn’t as easily define a function to take any of these kinds of messages
/// as we could with the `Message` enum defined below, which is a single type.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// The following structs could hold the same data that the preceding enum variants hold:
/// Equivalent to `Message::Quit` enum variant
///
/// Struct that could hold the same data that the `Message::Quit` enum variant holds
struct QuitMessage; // unit struct
/// Equivalent to `Message::Move` enum variant
///
/// Struct that could hold the same data that the `Message:Move` enum variant holds
struct MoveMessage {
    x: i32,
    y: i32,
}
/// Equivalent to `Message::Write` enum variant
///
/// Struct that could hold the same data that the `Message::Write` enum variant holds
struct WriteMessage(String); // tuple struct
/// Equivalent to `Message::ChangeColor` enum variant
///
/// Struct that could hold the same data that the `Message::ChangeColor` enum variant holds
struct ChangeColorMessage(i32, i32, i32); // tuple struct

/// # Enum with methods
///
/// There is one more similarity between enums and structs: just as we’re able
/// to define methods on structs using `impl`, we’re also able to define
/// methods on enums. Here’s a method named `call` that we could define on our
/// `Message` enum
fn enum_methods() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

/// # Enum with methods example
///
/// An `impl` for `call` method on `Message` enum
impl Message {
    fn call(&self) {
        // method body would be defined here
        match self {
            Self::Write(string) => {
                println!("Message: {string}");
            }
            Message::Quit => unimplemented!(),
            Message::Move { x, y } => unimplemented!(),
            Message::ChangeColor(_, _, _) => unimplemented!(),
        }
    }
}

/// # The `Option` Enum and Its Advantages Over Null Values
///
/// Rust does not have nulls, but it does have an enum that can encode the
/// concept of a value being present or absent. This enum is `Option<T>`, and
/// it is defined by the standard library as follows:
///
///     enum Option<T> {
///         None,
///         Some(T),
///     }
/// The `Option<T>` enum is so useful that it’s even included in the prelude;
/// you don’t need to bring it into scope explicitly. Its variants are also
/// included in the prelude: you can use `Some` and `None` directly without the
/// `Option::` prefix. The `Option<T>` enum is still just a regular enum, and
/// `Some(T)` and `None` are still variants of type `Option<T>`.
///
/// The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a
/// generic type parameter, and we’ll cover generics in more detail in
/// [Chapter 10][1].  For now, all you need to know is that `<T>` means that
/// the `Some` variant of the `Option` enum can hold one piece of data of any
/// type, and that each concrete type that gets used in place of `T` makes the
/// overall `Option<T>` type a different type. Below in the function
/// `option_type` are some examples of using `Option` values to hold number
/// types and string types.
///
/// Note: for the `absent_number` variable in the `option_type` function below,
/// Rust requires us to annotate the overall `Option` type:
/// the compiler can’t infer the type that the corresponding `Some` enum
/// variant will hold by looking only at a `None` value. Here, we tell Rust
/// that we mean for `absent_number` to be of `type Option<i32>`.
///
/// [1]: https://doc.rust-lang.org/book/ch10-00-generics.html
fn option_type() {
    // Rust compiler can infer the Option<T> types for these variables
    let some_number = Some(5);
    let some_char = Some('e');

    // Rust compiler can't infer the `Some` variant value for a variable
    // defined initially with `None`.  It must be explicitly specified such as
    // in the following example:
    let absent_number: Option<i32> = None;

    println!("`some_number` is: {:?}", some_number);
    print!("Type of variable `some_number` is: ");
    print_type_of(&some_number);
    println!("`some_char` is: {:?}", some_char);
    print!("Type of variable `some_char` is: ");
    print_type_of(&some_char);
    println!("`absent_number` is: {:?}", absent_number);
    print!("Type of variable `absent_number` is: ");
    print_type_of(&absent_number);

    // When we have a `Some` value, we know that a value is present and the
    // value is held within the `Some`. When we have a `None` value, in some
    // sense it means the same thing as `null`: we don’t have a valid value.
    // So why is having `Option<T>` any better than having `null`?
    // In short, because `Option<T>` and `T` (where `T` can be any type) are
    // different types, the compiler won’t let us use an `Option<T>` value as
    // if it were definitely a valid value. For example, this code won’t
    // compile, because it’s trying to add an `i8` to an `Option<i8>`:
    let _x: i8 = 5;
    let _y: Option<i8> = Option::Some(5);

    // let sum = _x + _y; // Compile Error: cannot add `Option<i8>` to `i8`
}

/// # The `match` Control Flow Construct
///
/// Rust has an extremely powerful control flow construct called match that
/// allows you to compare a value against a series of patterns and then execute
/// code based on which pattern matches. Patterns can be made up of literal
/// values, variable names, wildcards, and many other things; [Chapter 18][1]
/// covers all the different kinds of patterns and what they do. The power of
/// match comes from the expressiveness of the patterns and the fact that the
/// compiler confirms that all possible cases are handled.
///
/// Think of a `match` expression as being like a coin-sorting machine: coins
/// slide down a track with variously sized holes along it, and each coin falls
/// through the first hole it encounters that it fits into. In the same way,
/// values go through each pattern in a `match`, and at the first pattern the
/// value “fits,” the value falls into the associated code block to be used
/// during execution.
///
/// Speaking of coins, let’s use them as an example using `match`! We can write
/// a function that takes an unknown US coin and, in a similar way as the
/// counting machine, determines which coin it is and returns its value in
/// cents, as shown in `value_in_cents()`.
///
/// [1]: https://doc.rust-lang.org/book/ch18-00-patterns.html
fn match_control_flow() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    let mut rng = thread_rng();
    let vec_coins = Coin::iter().collect::<Vec<_>>();
    let random_coin = vec_coins.choose(&mut rng).unwrap();

    println!("`penny` value_in_cents = {:?}", value_in_cents(&penny));
    println!("`nickel` value_in_cents = {:?}", value_in_cents(&nickel));
    println!("`dime` value_in_cents = {:?}", value_in_cents(&dime));
    println!("`quarter` value_in_cents = {:?}", value_in_cents(&quarter));
    println!(
        "`random_coin` value_in_cents = {:?}",
        value_in_cents(random_coin)
    );
}

/// # value_in_cents(): Example of the `match` Control Flow Construct
///
/// A function that takes an unknown US coin and, in a similar way as a
/// counting machine, determines which coin it is and returns its value in
/// cents.
fn value_in_cents(coin: &Coin) -> u8 {
    // An enum and a match expression that has the variants of the enum as its
    // patterns
    match coin {
        Coin::Penny => {
            // If you want to run multiple lines of code in a match arm, you
            // must use curly brackets, and the comma following the arm is then
            // optional.
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/// # Enum to represent `Coin`s
///
/// An enum that has all variants of `Coin` types
#[derive(Debug, EnumIter)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

/// # A `noop` macro to perform no operation
///
/// Source: [Reddit: best no op macro? assert!(true)? What do you think of this
/// error handling pattern?][1]
///
/// [1]:
///     https://www.reddit.com/r/rust/comments/8powj8/comment/e0czkeu/?utm_source=reddit&utm_medium=web2x&context=3
macro_rules! noop {
    () => {};
}

/// # Patterns That Bind to Values
///
/// Another useful feature of match arms is that they can bind to the parts of
/// the values that match the pattern. This is how we can extract values out of
/// enum variants.
///
/// As an example, let’s change one of our enum variants to hold data inside it.
/// From 1999 through 2008, the United States minted quarters with different
/// designs for each of the 50 states on one side. No other coins got state
/// designs, so only quarters have this extra value. We can add this information
/// to our `enum` by changing the `Quarter` variant to include a `UsState` value
/// stored inside it, which we’ve done below.
fn match_control_flow_patterns() {
    let penny = Coin2::Penny;
    let nickel = Coin2::Nickel;
    let dime = Coin2::Dime;
    let quarter = Coin2::Quarter(UsState::default());
    let mut rng = thread_rng();
    let vec_coins = Coin2::iter().collect::<Vec<_>>();
    let mut random_coin = *vec_coins.choose(&mut rng).unwrap();
    let vec_states = UsState::iter().collect::<Vec<_>>();
    let random_state: &UsState;

    match random_coin {
        Coin2::Penny => {
            noop!();
        }
        Coin2::Nickel => {
            noop!();
        }
        Coin2::Dime => {
            noop!();
        }
        Coin2::Quarter(_) => {
            random_state = vec_states.choose(&mut rng).unwrap();
            random_coin = Coin2::Quarter(*random_state);
        }
    }

    println!(
        "`penny` value_in_cents_state_quarters = {:?}",
        value_in_cents_state_quarters(&penny)
    );
    println!(
        "`nickel` value_in_cents_state_quarters = {:?}",
        value_in_cents_state_quarters(&nickel)
    );
    println!(
        "`dime` value_in_cents_state_quarters = {:?}",
        value_in_cents_state_quarters(&dime)
    );
    println!(
        "`quarter` value_in_cents_state_quarters = {:?}",
        value_in_cents_state_quarters(&quarter)
    );
    println!(
        "`random_coin` value_in_cents_state_quarters = {:?}",
        value_in_cents_state_quarters(&random_coin)
    );
}

/// #  Enum to represent `UsState`s for all 50 US State Quarters
///
/// `Quarter` variant to include a `UsState` value stored inside it
#[derive(Debug, EnumIter, Clone, Copy)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

/// # `Default` trait implementation for `UsState`
///
/// You have a better chance of finding `Virginia` quarters than any others.
/// Mint records show nearly 1.6 billion were put into circulation.
///
/// Therefore, the `Coin2::Quarter(UsState::Virginia)` variant is the most
/// probable to find and is set to be the `Default` variant
impl Default for UsState {
    fn default() -> Self {
        UsState::Virginia
    }
}

/// # Enum to represent `Coin`s and US State Quarters
///
/// A `Coin` enum in which the `Quarter` variant also holds a `UsState` value
#[derive(Debug, EnumIter, Clone, Copy)]
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/// # Example of the `match` Control Flow Construct with Patterns That Bind to Values
///
/// A function that takes an unknown US coin and, in a similar way as a counting
/// machine, determines which coin it is and returns its value in cents.
///
/// If a `Quarter` variant is matched, the `UsState` component of the tuple
/// struct is printed
fn value_in_cents_state_quarters(coin: &Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
