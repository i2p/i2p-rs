use nom::{space, alphanumeric};

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

named!(pub sam_hello <&str, Vec<(&str, &str)> >,
    chain!(
                       tag_s!("HELLO")  ~
                       space?           ~
                       tag_s!("REPLY")  ~
        parse_options: keys_and_values  ,
        || { parse_options }
    )
);

named!(pub sam_naming_lookup <&str, Vec<(&str, &str)> >,
       chain!(
           tag_s!("NAMING") ~
           space?           ~
           tag_s!("REPLY")  ~
           parse_options: keys_and_values,
           || { parse_options }
       )
);

named!(pub sam_stream_session <&str, Vec<(&str, &str)> >,
       chain!(
           tag_s!("HELLO")      ~
               space?           ~
               tag_s!("REPLY")  ~
               parse_options: keys_and_values  ,
           || { parse_options }
       )
);
