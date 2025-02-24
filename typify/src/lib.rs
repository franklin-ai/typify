// Copyright 2023 Oxide Computer Company

//! # Typify
//!
//! Typify lets you convert JSON Schema documents into Rust types. It can be
//! used via a macro [`import_types!`] or a `build.rs` file.
//!
//! A typical use looks like this:
//! ```
//! # use typify_macro::import_types;
//! # use serde::{Deserialize,Serialize};
//! import_types!("../example.json");
//! ```
//!
//! This expands to type definitions corresponding to the types from the file
//! `example.json`. The types are `pub` and have a number of useful associated
//! `impl`s including [Debug], [Clone],
//! [Serialize](https://docs.rs/serde/latest/serde/trait.Serialize.html), and
//! [Deserialize](https://docs.rs/serde/latest/serde/trait.Deserialize.html).
//!
//! Alternatively, you may use the expanded form:
//! ```
//! # use typify_macro::import_types;
//! # use serde::{Deserialize,Serialize};
//! import_types!(schema = "../example.json");
//! ```
//!
//! If you want to add additional derives for the generated types, you can
//! specify them with the `derives` property of the expanded form:
//! ```
//! # use typify_macro::import_types;
//! # use serde::{Deserialize,Serialize};
//! import_types!(
//!     schema = "../example.json",
//!     derives = [schemars::JsonSchema],
//! );
//! ```
//!
//! Generated structs can optionally include a builder-style interface:
//! ```
//! # mod x {
//! # use typify_macro::import_types;
//! # use serde::{Deserialize,Serialize};
//! import_types!(
//!     schema = "../example.json",
//!     struct_builder = true,
//! );
//! # }
//! ```
//!
//! With this set, consumers can construct a struct `Veggie` as follows:
//! ```
//! # mod x {
//! # use typify_macro::import_types;
//! # use serde::{Deserialize,Serialize};
//! # import_types!(
//! #    schema = "../example.json",
//! #    struct_builder = true,
//! # );
//! # fn _x() {
//! let veggie: Veggie = Veggie::builder()
//!     .veggie_name("radish")
//!     .veggie_like(true)
//!     .try_into()
//!     .unwrap();
//! # }
//! # }
//! ```
//!
//! # Altering Conversion
//!
//! ## Renames and additional derivations
//!
//! You can specify renames types or add additional derive macros for generated
//! types using the `patch` syntax:
//! ```
//! # use typify_macro::import_types;
//! # use serde::{Deserialize,Serialize};
//! import_types!(
//!     schema = "../example.json",
//!     patch = {
//!         Veggie = {
//!             rename = "Vegetable",
//!             derives = [ schemars::JsonSchema ],
//!         }
//!     }
//! );
//! ```
//!
//! ## Replacement types
//!
//! You can replace a generated type with an existing type by specifying an
//! association between the name of a type with the type to use in its place:
//! ```
//! # mod my_fancy_networking_crate {
//! # #[derive(serde::Deserialize)]
//! # pub struct Ipv6Cidr(String);
//! # }
//! # use typify_macro::import_types;
//! # use serde::{Deserialize,Serialize};
//! import_types!(
//!     schema = "../example.json",
//!     replace = {
//!         Ipv6Cidr = my_fancy_networking_crate::Ipv6Cidr,
//!     }
//! );
//! ```
//!
//! ## Conversion overrides
//!
//! You can override a conversion for a particular JSON schema construct by
//! specifying an association between the schema and the type.
//! ```
//! # mod my_fancy_uuid_crate {
//! # #[derive(serde::Deserialize)]
//! # pub struct MyUuid(String);
//! # }
//! # use typify_macro::import_types;
//! # use serde::{Deserialize,Serialize};
//! import_types!(
//!     schema = "../example.json",
//!     convert = {
//!         {
//!             type = "string",
//!             format = "uuid",
//!         } = my_fancy_uuid_crate::MyUuid,
//!     }
//! );
//! ```
//!
//! # Macro vs. `build.rs`
//!
//! While using the [`import_types!`] macro is quite a bit simpler, you can
//! also construct output in a `build.rs` script. Doing so requires a little
//! more work to process the JSON Schema document and write out the file to
//! your intended location. The biggest benefit is that the generated type
//! definitions are significantly easier to inspect. The macro-generated types
//! can be viewed with `cargo expand` and they (like `build.rs`-derived types)
//! have generated documentation, but if you find that you'd like to see the
//! actual code generated you may prefer a `build.rs`.
//!
//! ## Builder interface
//!
//! Typify exports a [TypeSpace] interface that is intended for programmatic
//! construction of types. This can be for something simple like a `build.rs`
//! script or something more complex like a generator whose input includes JSON
//! schema type definitions.
//!
//! # Mapping JSON Schema to Rust
//!
//! JSON Schema allows for extreme flexibility. As such, there are some schemas
//! that Typify isn't able to interpret (please file an issue!). In general,
//! though, Typify does a pretty job of mapping JSON Schema types to Rust. For
//! more information, see the project's
//! [README.md](https://github.com/oxidecomputer/typify).

pub use typify_impl::Error;
pub use typify_impl::Type;
pub use typify_impl::TypeDetails;
pub use typify_impl::TypeEnum;
pub use typify_impl::TypeEnumVariant;
pub use typify_impl::TypeId;
pub use typify_impl::TypeNewtype;
pub use typify_impl::TypeSpace;
pub use typify_impl::TypeSpaceImpl;
pub use typify_impl::TypeSpacePatch;
pub use typify_impl::TypeSpaceSettings;
pub use typify_impl::TypeStruct;
pub use typify_impl::TypeStructPropInfo;

pub use typify_macro::import_types;
