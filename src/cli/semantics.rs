use crate::semantics::id::IdSemantics;
use crate::semantics::signed_const::SignedConstSemantics;
use crate::semantics::unsigned_const::UnsignedConstSemantics;

pub fn semantics_html(
    id_semantics: IdSemantics,
    unsigned_const_semantics: UnsignedConstSemantics,
    signed_const_semantics: SignedConstSemantics
) -> String {
    let semantics = format!(
        "ids: <span class=\"accent\">{:?}</span><br>unsigned_constants: <span class=\"accent\">{:?}</span><br>signed_constants: <span class=\"accent\">{:?}</span>",
        id_semantics.vec, unsigned_const_semantics.vec, signed_const_semantics.vec
    );

    semantics
}