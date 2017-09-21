/*
Copyright ⓒ 2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
This module contains documentation on how to use the `macro_attr!` macro.

# Supported Forms

The `macro_attr!` macro's job is primarily to parse an item definition, stripping out non-standard attribute forms.  This allows the standard forms to be passed through to the compiler, with the non-standard forms being interpreted and expanded by `macro_attr!` itself.

In particular, this macro looks for the following kinds of syntax:

- Derivations where the name of the derivation is suffixed with a `!`.  *e.g.* `#[derive(Thing!)]`, `#[derive(Whatsis!(some, args))]`.
- Top-level attributes whose names are suffixed with a `!`.  *e.g.* `#[do_a_thing!]`, `#[something_else!(more, args)]`.

The use of `#[derive(Name~!)]` is explained in the section on the `use-proc-macros` feature.

Unlike "real" attributes, optional parenthesised arguments after the `!` are allowed, and can be arbitrary token trees, meaning they can contain any properly delimited token sequence.  These are supported to allow custom attribute macros to easily take arguments, as well as supporting multiple derivations on a single type (each with its own arguments).

# Derivation Macros

A derivation macro is responsible for parsing an item given to it, and emitting any additional items as needed.  They *cannot* change the item itself, and do not receive any other attributes attached to the item.

Given the following input:

```ignore
#[derive(Copy, Name!(args...), Clone, Another!, Debug)]
struct Foo;
```

`macro_attr!` will expand to the equivalent of:

```ignore
#[derive(Copy, Clone, Debug)]
struct Foo;

Name!((args...) struct Foo;);
Another!(() struct Foo;);
```

Note that macro derives may be mixed with regular derives, or put in their own `#[derive(...)]` attribute.  Also note that macro derive invocations are *not* passed the other attributes on the item; input will consist of the arguments provided to the derivation (*i.e.* `(args...)` in this example), the item's visibility (if any), and the item definition itself.

A macro derivation invoked *without* arguments will be treated as though it was invoked with empty parentheses.  *i.e.* `#[derive(Name!)]` is equivalent to `#[derive(Name!())]`.

A derivation macro may expand to any number of new items derived from the provided input.  There is no way for a derivation macro to alter the item itself (for that, use a macro attribute).

# Attribute Macros

When `macro_attr!` encounters an attribute suffixed with a `!` (*e.g.* `#[name!(args...)]`), it invokes the macro `name!` with everything lexically *after* that attribute.  An attribute macro is free to add to, remove from, or alter the provided input as it sees fit, before instructing `macro_attr!` to resume parsing.  As a result, attribute macros *must* produce an item as output.

This power also makes writing attribute macros more difficult, however.  A properly-written attribute macro should be prepared to deal with an item adorned with arbitrary attributes, including non-standard `macro_attr!` attributes.

As an example example, given the following input:

```ignore
#[make_unitary!]
#[repr(C)]
#[rename_to!(Quux)]
#[doc="Test."]
struct Bar { field: i32 }
```

`macro_attr!` will expand to:

```ignore
make_unitary! {
    (), then $resume,
    #[repr(C)]
    #[rename_to!(Quux)]
    #[doc="Test."]
    struct Bar { field: i32 };
}
```

Note that `$resume` is **not** literal.  When implementing an attribute macro, you should accept this part as `$resume:tt`, and not attempt to inspect or deconstruct the contents.  Its contents are considered an implementation detail of `macro_attr!` and may change at any time.

Assuming `make_unitary!` removes the body of the `struct` it is attached to, `macro_attr!` *requires* that it expand to:

```ignore
macro_attr_callback! {
    $resume,
    #[repr(C)]
    #[rename_to!(Quxx)]
    #[doc="Test."]
    struct Bar;
}
```

`macro_attr!` will then resume parsing, and expand to:

```ignore
rename_to! {
    (Quxx), then $resume,
    #[doc="Test."]
    struct Bar;
}
```

Assuming `rename_to!` does the obvious thing and changes the name of the item it is attached to, it should expand to:

```ignore
macro_attr_callback! {
    $resume,
    #[doc="Test."]
    struct Quxx;
}
```

Once more, `macro_attr!` will resume, and produce the final expansion of:

```ignore
#[repr(C)]
#[doc="Test."]
struct Quxx;
```

Note that normal attributes are automatically carried through and re-attached to the item.

# Recursion Usage

`macro_attr!` tries to keep recursion usage to a minimum, but can still consume a considerable number of levels.  Large inputs can quickly consume the available macro recursion limit.  This limit can be easily raised, but it makes for a less-than-ideal user experience if you are authoring macros to be used by others.  Conveniently, modern versions of `rustc` will prompt to user to raise the recursion limit (with the appropriate syntax) automatically as needed.  Even so, you are advised to limit macro recursion where reasonable to do so.  This will also *generally* result in more performant macros.

`macro_attr!` itself will recurse linearly with the number of attributes in the input, plus the number of individual derivations (of any kind), plus the number of macro derivations.

Macro derivations, once parsed, are expanded in parallel.  This means that the *most* deeply recursive macro derivation determines the number of recursion levels needed to successfully expand *all* macro derivations.

Macro attributes, once parsed, are expanded in *serial*.  This is due to the way Rust macros work.  As such, macro attributes should try to recurse as little as possible.

# The `use-proc-macros` Feature

To aid in writing code that can use procedural macros *and* work on versions of Rust prior to 1.15, `macro_attr!` supports the `Name~!` invocation form.  Specifically:

- `#[derive(Name~!)]`
- `#[name~!]`
- `#[name~!(...)]`

If the crate is being compiled by Rust 1.15 or higher *and* the `use-proc-macros` feature is enabled, each of the above will be rewritten to:

- `#[derive(Name)]`
- `#[name]`
- `#[name(...)]`

If the crate is being compiled by Rust 1.14 or prior, *or* the `use-proc-macros` feature is not enabled, each of the above will be rewritten to:

- `#[derive(Name!())]`
- `#[name!()]`
- `#[name!(...)]`

The reason a feature is required is that Cargo provides no mechanism for choosing dependencies based on the version of Rust being used.  Procedural macro crates *cannot* be compiled on versions of Rust 1.15.  This breaks compilation, even if the macros are not actively being used.  It is also currently impossible to build a "hybrid" crate that exposes regular macros prior to 1.15, and procedural macros after.

As a result, the *only* way to safely use procedural macros whilst still maintaining compatibility with earlier versions of Rust is via a feature.

If you wish to use this in your own crates, it is recommended that you expose a `use-proc-macros` feature for your crate, and forward it to `macro-attrs` like so:

```toml
[features]
use-proc-macros = ["macro-attr/use-proc-macros"]
```

Whether you should enable this feature by default or not is a matter of opinion.  It is easier to enable a single feature than it is to *disable* a single feature, so my personal preference is to require opt-in.

Keep in mind that Cargo features *must* be *additive*: you should *never* implement "negative" features that disable functionality.

# Addendum: The `std` Feature

Given that this crate does not use anything from the `std` crate, why does it have a feature for enabling/disabling it?

The simple answer is: because you *can't* disable it in old (but still supported) versions of Rust.  Attempting to do so causes a compilation error.  Thus, there is a feature that must be explicitly disabled to stop linking to `std`, on the basis that any project which needs to avoid linking to `std` won't work on such old versions of Rust anyway.
*/