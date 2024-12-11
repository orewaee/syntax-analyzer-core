pub fn semantics_html(
    ids: Vec<String>,
    unsigned_consts: Vec<String>,
    signed_consts: Vec<String>,
    id_semantics: (String, Vec<String>),
    range: (i32, i32, i32),
    iterations: i32,
) -> String {
    let counter = if id_semantics.1.len() == 0 { format!("<span class=\"accent\">{}</span>", id_semantics.0) } else { "<span class=\"dimmed\">missing</span>".to_string() };
    let array = if id_semantics.1.len() != 0 { format!("<span class=\"accent\">{}</span>", id_semantics.0) } else { "<span class=\"dimmed\">missing</span>".to_string() };

    let semantics = format!(
        "chain elements:<br>\
        &nbsp;&nbsp;ids: <span class=\"accent\">{:?}</span><br>\
        &nbsp;&nbsp;unsigned_constants: <span class=\"accent\">{:?}</span><br>\
        &nbsp;&nbsp;signed_constants: <span class=\"accent\">{:?}</span><br><br>\
        id semantics:<br>\
        &nbsp;&nbsp;counter: {}<br>\
        &nbsp;&nbsp;array: {}<br>\
        &nbsp;&nbsp;indexes: <span class=\"accent\">{:?}</span><br><br>\
        range semantics:<br>\
        &nbsp;&nbsp;from: <span class=\"accent\">{}</span><br>\
        &nbsp;&nbsp;to: <span class=\"accent\">{}</span><br>\
        &nbsp;&nbsp;step: <span class=\"accent\">{}</span><br>\
        &nbsp;&nbsp;iterations: <span class=\"accent\">{}</span>",
        ids, unsigned_consts, signed_consts,

        counter,
        array,

        id_semantics.1,

        range.0, range.1, range.2, iterations
    );

    semantics
}