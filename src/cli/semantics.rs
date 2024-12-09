use crate::semantics::id::IdSemantics;
use crate::semantics::unsigned_const::UnsignedConstSemantics;

pub fn semantics_html(id_semantics: IdSemantics, unsigned_const_semantics: UnsignedConstSemantics) -> String {
    let semantics = format!(
        "ids: <span class=\"accent\">{:?}</span>\nunsigned_constants: <span class=\"accent\">{:?}</span>",
        id_semantics.vec, unsigned_const_semantics.vec
    );

    semantics
}