# Faking a variadic concat function in rust

A little toy project trying to avoid macros for string concatenation and printing. It's probably not very useful but serves for learning purposes. Could probably be improved by using `std::any`.

Rust's function signatures have to be known at compile-time. The number of arguments is fixed. To create a variadic function I'm trying to work around this, by using an array slice of an argument type and generics. Using this is not very ergonomic. It might be improved if the enum variants could be converted implicitly. Then one might write:

```
concat(&["Foo ", 123])
```

This is not really improving readability compared to

```
concat!("Foo ", 123])
```

but it allows also `String`s to be passed.
