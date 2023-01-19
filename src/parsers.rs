use nom::{
	alt, do_parse, named, separated_list, tag, take_till,
	character::complete::{alphanumeric1 as alphanumeric, space1 as space},
};
fn is_space(chr: char) -> bool {
	chr == ' ' || chr == '\t'
}

fn is_next_line(chr: char) -> bool {
	chr == '\n'
}

fn is_space_or_next_line(chr: char) -> bool {
	is_space(chr) || is_next_line(chr)
}

fn is_double_quote(chr: char) -> bool {
	chr == '\"'
}

named!(quoted_value <&str, &str>,
	do_parse!(
			 tag!("\"")                  >>
		val: take_till!(is_double_quote) >>
			 tag!("\"")                  >>
		(val)
	)
);

named!(value <&str, &str>, take_till!(is_space_or_next_line));

named!(key_value <&str, (&str, &str)>,
	do_parse!(
		key: alphanumeric               >>
			 tag!("=")                >>
		val: alt!(quoted_value | value) >>
		(key, val)
	)
);

named!(keys_and_values<&str, Vec<(&str, &str)> >, separated_list!(space, key_value));

named!(pub sam_hello <&str, Vec<(&str, &str)> >,
	do_parse!(
			  tag!("HELLO REPLY ") >>
		opts: keys_and_values        >>
			  tag!("\n")           >>
		(opts)
	)
);

named!(pub sam_session_status <&str, Vec<(&str, &str)> >,
	do_parse!(
			  tag!("SESSION STATUS ") >>
		opts: keys_and_values           >>
			  tag!("\n")              >>
		(opts)
	)
);

named!(pub sam_stream_status <&str, Vec<(&str, &str)> >,
	do_parse!(
			  tag!("STREAM STATUS ") >>
		opts: keys_and_values          >>
			  tag!("\n")             >>
		(opts)
	)
);

named!(pub sam_naming_reply <&str, Vec<(&str, &str)> >,
	do_parse!(
			  tag!("NAMING REPLY ") >>
		opts: keys_and_values         >>
			  tag!("\n")            >>
		(opts)
	)
);

named!(pub sam_dest_reply <&str, Vec<(&str, &str)> >,
	do_parse!(
			  tag!("DEST REPLY ") >>
		opts: keys_and_values       >>
			  tag!("\n")          >>
		(opts)
	)
);

#[cfg(test)]
mod tests {
	use nom::error::ErrorKind;

	#[test]
	fn hello() {
		use crate::parsers::sam_hello;

		assert_eq!(
			sam_hello("HELLO REPLY RESULT=OK VERSION=3.1\n"),
			Ok(("", vec![("RESULT", "OK"), ("VERSION", "3.1")]))
		);
		assert_eq!(
			sam_hello("HELLO REPLY RESULT=NOVERSION\n"),
			Ok(("", vec![("RESULT", "NOVERSION")]))
		);
		assert_eq!(
			sam_hello("HELLO REPLY RESULT=I2P_ERROR MESSAGE=\"Something failed\"\n"),
			Ok((
				"",
				vec![("RESULT", "I2P_ERROR"), ("MESSAGE", "Something failed")]
			))
		);
	}

	#[test]
	fn session_status() {
		use crate::parsers::sam_session_status;

		assert_eq!(
			sam_session_status("SESSION STATUS RESULT=OK DESTINATION=privkey\n"),
			Ok(("", vec![("RESULT", "OK"), ("DESTINATION", "privkey")]))
		);
		assert_eq!(
			sam_session_status("SESSION STATUS RESULT=DUPLICATED_ID\n"),
			Ok(("", vec![("RESULT", "DUPLICATED_ID")]))
		);
	}

	#[test]
	fn stream_status() {
		use crate::parsers::sam_stream_status;

		assert_eq!(
			sam_stream_status("STREAM STATUS RESULT=OK\n"),
			Ok(("", vec![("RESULT", "OK")]))
		);
		assert_eq!(
			sam_stream_status(
				"STREAM STATUS RESULT=CANT_REACH_PEER MESSAGE=\"Can't reach peer\"\n"
			),
			Ok((
				"",
				vec![
					("RESULT", "CANT_REACH_PEER"),
					("MESSAGE", "Can't reach peer")
				]
			))
		);
	}

	#[test]
	fn naming_reply() {
		use crate::parsers::sam_naming_reply;

		assert_eq!(
			sam_naming_reply("NAMING REPLY RESULT=OK NAME=name VALUE=dest\n"),
			Ok((
				"",
				vec![("RESULT", "OK"), ("NAME", "name"), ("VALUE", "dest")]
			))
		);
		assert_eq!(
			sam_naming_reply("NAMING REPLY RESULT=KEY_NOT_FOUND\n"),
			Ok(("", vec![("RESULT", "KEY_NOT_FOUND")]))
		);	
		if let Err(err) = sam_naming_reply("NAMINGREPLY RESULT=KEY_NOT_FOUND\n") {
			match err {
				nom::Err::Error((_, e)) => {
					assert_eq!(e, ErrorKind::Tag);
				}	
				nom::Err::Failure((_, e)) => {
					assert_eq!(e, ErrorKind::Tag);
				}
				nom::Err::Incomplete(e) => {
					panic!("unepxected error");
				}
			}
		} else {
			panic!("expected error");
		}
		if let Err(err) = sam_naming_reply("NAMING  REPLY RESULT=KEY_NOT_FOUND\n") {
			match err {
				nom::Err::Error((_, e)) => {
					assert_eq!(e, ErrorKind::Tag);
				}	
				nom::Err::Failure((_, e)) => {
					assert_eq!(e, ErrorKind::Tag);
				}
				nom::Err::Incomplete(e) => {
					panic!("unepxected error");
				}
			}
		} else {
			panic!("expected error");
		}
	}

	#[test]
	fn dest_reply() {
		use crate::parsers::sam_dest_reply;

		assert_eq!(
			sam_dest_reply("DEST REPLY PUB=foo PRIV=foobar\n"),
			Ok(("", vec![("PUB", "foo"), ("PRIV", "foobar")]))
		);
	}
}
