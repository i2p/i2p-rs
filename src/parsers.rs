use std::collections::HashMap;
use nom::{IResult, space, alphanumeric};

fn is_space(chr: char) -> bool {
    chr == ' ' || chr == '\t'
}

fn is_next_line(chr: char) -> bool {
    chr == '\n'
}

fn is_space_or_next_line(chr: char) -> bool {
    is_space(chr) || is_next_line(chr)
}

named!(key_value <&str, (&str, &str)>,
    chain!(
             space?                              ~
        key: alphanumeric                        ~
             space?                              ~
             tag_s!("=")                         ~
             space?                              ~
        val: take_till_s!(is_space_or_next_line) ,
        || { (key, val) }
    )
);

named!(keys_and_values<&str, Vec<(&str, &str)> >, many0!(key_value));

fn options(input: &str) -> IResult<&str, HashMap<&str, &str> > {
    let mut h: HashMap<&str, &str> = HashMap::new();

    match keys_and_values(input) {
        IResult::Done(i, tuple_vec) => {
            for &(k,v) in &tuple_vec {
                h.insert(k, v);
            }
            IResult::Done(i, h)
        },
        IResult::Incomplete(a) => IResult::Incomplete(a),
        IResult::Error(a)      => IResult::Error(a)
    }
}

named!(pub sam_hello <&str, HashMap<&str, &str> >,
    chain!(
                       tag_s!("HELLO") ~
                       space?          ~
                       tag_s!("REPLY") ~
        parse_options: options         ,
        || { parse_options }
    )
);
