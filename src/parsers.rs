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

named!(pub sam_naming_reply <&str, Vec<(&str, &str)> >,
       chain!(
           tag_s!("NAMING") ~
           space?           ~
           tag_s!("REPLY")  ~
           parse_options: keys_and_values,
           || { parse_options }
       )
);

named!(pub sam_session_status <&str, Vec<(&str, &str)> >,
       chain!(
           tag_s!("HELLO")      ~
               space?           ~
               tag_s!("REPLY")  ~
               parse_options: keys_and_values  ,
           || { parse_options }
       )
);

#[cfg(test)]
mod tests {
    use nom::IResult::Done;

    #[test]
    fn hello() {
        use parsers::sam_hello;

        assert_eq!(
            sam_hello("HELLO REPLY RESULT=OK VERSION=3.1\n"),
            Done("\n", vec![("RESULT", "OK"), ("VERSION", "3.1")]));
        assert_eq!(
            sam_hello("HELLO REPLY RESULT=NOVERSION\n"),
            Done("\n", vec![("RESULT", "NOVERSION")]));
        assert_eq!(
            sam_hello("HELLO REPLY RESULT=I2P_ERROR MESSAGE=\"Something failed\"\n"),
            Done("\n", vec![("RESULT", "I2P_ERROR"), ("MESSAGE", "Something failed")]));
    }

    #[test]
    fn session_status() {
        use parsers::sam_session_status;

        assert_eq!(
            sam_session_status("SESSION STATUS RESULT=OK DESTINATION=privkey\n"),
            Done("\n", vec![("RESULT", "OK"), ("DESTINATION", "privkey")]));
        assert_eq!(
            sam_session_status("SESSION STATUS RESULT=DUPLICATED_ID\n"),
            Done("\n", vec![("RESULT", "DUPLICATED_ID")]));
    }

    #[test]
    fn naming_reply() {
        use parsers::sam_naming_reply;

        assert_eq!(
            sam_naming_reply("NAMING REPLY RESULT=OK NAME=name VALUE=dest\n"),
            Done("\n", vec![("RESULT", "OK"), ("NAME", "name"), ("VALUE", "dest")]));
        assert_eq!(
            sam_naming_reply("NAMING REPLY RESULT=KEY_NOT_FOUND\n"),
            Done("\n", vec![("RESULT", "KEY_NOT_FOUND")]));
    }
}
