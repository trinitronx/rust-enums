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

/// # Enums and Pattern Matching Examples
///
/// Examples from Rust Book Ch. 6: Enums and Pattern Matching
fn main() {
    // Defining an Enum
    defining_an_enum();

    // Enum with associated `String` values
    enum_string_values();
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
