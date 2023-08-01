use serde::{Deserialize, Serialize};

use crate::label;

use self::list::TypedList;
pub use self::object::{Object, ZObject};

pub mod list;
pub mod object;

pub type ZUnit = Reference<label::Z24>;

/// a Z9/reference
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Reference<Id = String> {
    #[serde(rename = "Z9K1")]
    pub id: Id,
}

impl<Id> ZObject for Reference<Id> {
    type ZType = label::Z9;
}

/// A Z60/natural language.
#[derive(Serialize, Deserialize, Debug)]
pub struct NaturalLanguage {
    #[serde(rename = "Z60K1", with = "object")]
    pub code: ZString,
    #[serde(rename = "Z60K2", with = "object")]
    pub code_aliases: TypedList<ZString>,
}

/// A Z11/monolingual text.
#[derive(Serialize, Deserialize, Debug)]
pub struct MonolingualText {
    #[serde(rename = "Z11K1", with = "object")]
    pub language: NaturalLanguage,
    #[serde(rename = "Z12K1", with = "object")]
    pub text: ZString,
}

/// A Z12/monolingual text.
#[derive(Serialize, Deserialize, Debug)]
pub struct MultilingualText {
    #[serde(rename = "Z12K1", with = "object")]
    pub texts: TypedList<MonolingualText>,
}

/// A Z17/argument declaration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Argument<Type: ZObject> {
    #[serde(rename = "Z17K1", with = "object")]
    #[serde(bound(serialize = "Type: Serialize", deserialize = "Type: Deserialize<'de>"))]
    pub ty: Type,
    #[serde(rename = "Z17K2", with = "object")]
    pub key: ZString,
    #[serde(rename = "Z17K3", with = "object")]
    pub label: MultilingualText,
}

/// A Z8/function.
#[derive(Serialize, Deserialize, Debug)]
pub struct Function<
    Type: ZObject,
    ReturnType: ZObject,
    TestCase: ZObject,
    Implementation: ZObject,
    Identity: ZObject,
> {
    #[serde(rename = "Z8K1", with = "object")]
    #[serde(bound(serialize = "Type: Serialize", deserialize = "Type: Deserialize<'de>"))]
    pub arguments: TypedList<Argument<Type>>,

    #[serde(rename = "Z8K2", with = "object")]
    #[serde(bound(
        serialize = "ReturnType: Serialize",
        deserialize = "ReturnType: Deserialize<'de>"
    ))]
    pub return_type: ReturnType,

    #[serde(rename = "Z8K3", with = "object")]
    #[serde(bound(
        serialize = "TestCase: Serialize",
        deserialize = "TestCase: Deserialize<'de>"
    ))]
    pub test_cases: TypedList<TestCase>,

    #[serde(rename = "Z8K4", with = "object")]
    #[serde(bound(
        serialize = "Implementation: Serialize",
        deserialize = "Implementation: Deserialize<'de>"
    ))]
    pub implementations: TypedList<Implementation>,

    #[serde(rename = "Z8K5", with = "object")]
    #[serde(bound(
        serialize = "Identity: Serialize",
        deserialize = "Identity: Deserialize<'de>"
    ))]
    pub identity: Identity,
}

/// An Z14/implementation
#[derive(Serialize, Deserialize, Debug)]
pub struct Implementation<Fn: ZObject> {
    /// The function that this implementation is for
    #[serde(rename = "Z14K1", with = "object")]
    #[serde(bound(serialize = "Fn: Serialize", deserialize = "Fn: Deserialize<'de>"))]
    pub function: Fn,
    #[serde(rename = "Z14K3", with = "object")]
    pub code: Code,
}

/// A Z6/string
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ZString {
    #[serde(rename = "Z6K1")]
    pub value: String,
}

impl From<&'_ str> for ZString {
    #[inline]
    fn from(value: &'_ str) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl ZObject for ZString {
    type ZType = label::Z6;
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TypeListArgs<Type> {
    #[serde(rename = "Z881K1")]
    pub ty: Type,
}

/// a Z7/function call
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct FunctionCall<Func: ZObject, Args> {
    #[serde(rename = "Z7K1", with = "object")]
    #[serde(bound(serialize = "Func: Serialize", deserialize = "Func: Deserialize<'de>"))]
    pub function: Func,
    #[serde(flatten)]
    pub args: Args,
}

/// Represents a Z22/Pair.
#[derive(Deserialize, Serialize, Debug)]
pub struct Pair<A: ZObject, B: ZObject> {
    #[serde(rename = "Z22K1", with = "object")]
    #[serde(bound(serialize = "A: Serialize", deserialize = "A: Deserialize<'de>"))]
    pub left: A,
    #[serde(rename = "Z22K2", with = "object")]
    #[serde(bound(serialize = "B: Serialize", deserialize = "B: Deserialize<'de>"))]
    pub right: B,
}

/// Represents a Z61/Programming language.
#[derive(Deserialize, Serialize, Debug)]
pub struct ProgrammingLanguage {
    #[serde(rename = "Z61K1", with = "object")]
    pub code: ZString,
}

/// Rperesents a Z16/Code.
#[derive(Serialize, Deserialize, Debug)]
pub struct Code {
    #[serde(rename = "Z16K1", with = "object")]
    pub language: ProgrammingLanguage,
    #[serde(rename = "Z16K2", with = "object")]
    pub code: ZString,
}

/// Implements `ZObject` for types. Should not be used for `ZString` and `Reference`,
/// as those types have special ZTypes (not wrapped with `Object<Reference<>>`)
macro_rules! impl_zobject {
    ($( $(@[$($tt:tt)*])? $ty:ty = $label:ident ),*$(,)?) => {
        $(impl $(<$($tt)*>)? ZObject for $ty {
            type ZType = Object<Reference<label::$label>>;
        })*
    };
}

impl_zobject! {
    @[Func: ZObject, Args] FunctionCall<Func, Args> = Z7,
    MonolingualText = Z11,
    MultilingualText = Z12,
    @[Fn: ZObject] Implementation<Fn> = Z14,
    Code = Z16,
    @[Type: ZObject] Argument<Type> = Z17,
    @[A: ZObject, B: ZObject] Pair<A, B> = Z22,
    NaturalLanguage = Z60,
    ProgrammingLanguage = Z61,
}
