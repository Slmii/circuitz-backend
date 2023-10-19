use candid::{ IDLProg, IDLValue, IDLArgs, parser::types::{ IDLType, PrimType, TypeField }, types::value::IDLField };
use serde_json::Value as JsonValue;

/// Options for how to represent `Vec<u8>`
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum BytesFormat {
	/// Data is represented as an array of numbers: `[1,34,0]`
	#[default]
	Numbers,
	/// Data is represented as hex: `"A4B7"`
	Hex,
}

#[derive(Default)]
pub struct Idl2JsonOptions {
	/// How to represent `Vec<u8>`
	pub bytes_as: Option<BytesFormat>,
	/// How to represent `Vec<u8>` of at least some given length.
	pub long_bytes_as: Option<(usize, BytesFormat)>,
	/// Type definitions.
	///
	/// Note:
	/// - An `IDLProg`  corresponds to a parsed `.did` file.
	/// - Typically either no `IDLProg` is available or one `IDLProg`
	///   is provided, corresponding to the `.did` file of a canister
	///   and that one `.did` file has all required definitions.
	/// - In rare cases, multiple IDLProgs are needed.  If so,
	///   `idl2json` will use the first match it finds.  It is the
	///   caller's responsibility to ensure that there are no conflicting definitions.
	pub prog: Vec<IDLProg>,
	/// Compact JSON, without formatting whitespace.
	pub compact: bool,
}

/// Converts a candid IDLValue to a serde JsonValue, without type information.
///
/// Note: The textual format in parentheses `(  )` represents IDLArgs containing
/// zero or more IDLValues.  Unless you definitely wish to convert a single value
/// you may wish to consider `idl_args2json` instead.
pub fn idl2json(idl: &IDLValue, options: &Idl2JsonOptions) -> JsonValue {
	match idl {
		IDLValue::Bool(bool) => JsonValue::Bool(*bool),
		IDLValue::Null => JsonValue::Null,
		IDLValue::Text(s) => JsonValue::String(s.clone()),
		IDLValue::Number(s) => JsonValue::String(s.clone()), // Unspecified number type
		IDLValue::Float64(f) =>
			serde_json::Number
				::from_f64(*f)
				.map(JsonValue::Number)
				.unwrap_or_else(|| JsonValue::String("NaN".to_string())),
		IDLValue::Opt(value) => JsonValue::Array(vec![idl2json(value, options)]),
		IDLValue::Vec(value) =>
			convert_bytes(value, options).unwrap_or_else(|_| convert_non_bytes_array(value, options)),
		IDLValue::Record(value) =>
			JsonValue::Object(
				value
					.iter()
					.map(|field| (format!("{}", field.id), idl2json(&field.val, options)))
					.collect()
			),
		IDLValue::Variant(field) =>
			JsonValue::Object(
				vec![(format!("{}", field.0.id), idl2json(&field.0.val, options))]
					.into_iter()
					.collect()
			),
		IDLValue::Principal(p) => JsonValue::String(format!("{}", p)),
		IDLValue::Service(p) => JsonValue::String(format!("{}", p)),
		IDLValue::Func(p, c) =>
			JsonValue::Object(
				vec![
					("principal".to_string(), JsonValue::String(format!("{}", p))),
					("code".to_string(), JsonValue::String(c.to_string()))
				]
					.into_iter()
					.collect()
			),
		IDLValue::None => JsonValue::Array(vec![]),
		IDLValue::Int(i) => JsonValue::String(format!("{}", i)),
		IDLValue::Nat(i) => JsonValue::String(format!("{}", i)),
		IDLValue::Nat8(i) => JsonValue::Number(serde_json::Number::from(*i)),
		IDLValue::Nat16(i) => JsonValue::Number(serde_json::Number::from(*i)),
		IDLValue::Nat32(i) => JsonValue::Number(serde_json::Number::from(*i)),
		IDLValue::Nat64(i) => JsonValue::String(format!("{}", i)),
		IDLValue::Int8(i) => JsonValue::Number(serde_json::Number::from(*i)),
		IDLValue::Int16(i) => JsonValue::Number(serde_json::Number::from(*i)),
		IDLValue::Int32(i) => JsonValue::Number(serde_json::Number::from(*i)),
		IDLValue::Int64(i) => JsonValue::String(format!("{}", i)),
		IDLValue::Float32(f) =>
			serde_json::Number
				::from_f64(*f as f64)
				.map(JsonValue::Number)
				.unwrap_or_else(|| JsonValue::String("NaN".to_string())),
		IDLValue::Reserved => JsonValue::String(idl.to_string()),
	}
}

/// Conver
pub(crate) fn convert_non_bytes_array(value: &[IDLValue], options: &Idl2JsonOptions) -> JsonValue {
	JsonValue::Array(
		value
			.iter()
			.map(|item| idl2json(item, options))
			.collect()
	)
}

/// Converts a candid IDLArgs to a serde JsonValue, without type information.
///
/// Note: The textual format `( )` containing zero or more values represents an IDLArgs.
pub fn idl_args2json(args: &IDLArgs, options: &Idl2JsonOptions) -> JsonValue {
	convert_non_bytes_array(&args.args, options)
}

/// Converts supposedly binary data.  Returns an error if the data is not binary.
pub fn convert_bytes(bytes: &[IDLValue], options: &Idl2JsonOptions) -> Result<JsonValue, ()> {
	if let Some((len, bytes_format)) = options.long_bytes_as {
		if bytes.len() >= len {
			return format_bytes(bytes, &bytes_format);
		}
	}
	format_bytes(bytes, &options.bytes_as.unwrap_or_default())
}
/// Converts formats supposedly binary data.  Returns an error if the data is not binary.
fn format_bytes(bytes: &[IDLValue], bytes_format: &BytesFormat) -> Result<JsonValue, ()> {
	match bytes_format {
		BytesFormat::Numbers =>
			Ok(
				JsonValue::Array(
					bytes
						.iter()
						.map(|item| {
							if let IDLValue::Nat8(value) = item {
								Ok(JsonValue::Number(serde_json::Number::from(*value)))
							} else {
								Err(())
							}
						})
						.collect::<Result<Vec<JsonValue>, ()>>()?
				)
			),
		BytesFormat::Hex => {
			let mut ans = String::with_capacity(bytes.len() * 2);
			for byte in bytes {
				if let IDLValue::Nat8(value) = byte {
					ans.push_str(nybble2hex(value >> 4));
					ans.push_str(nybble2hex(value & 0xf));
				} else {
					return Err(());
				}
			}
			Ok(JsonValue::String(ans))
		}
		#[cfg(feature = "crypto")]
		BytesFormat::Sha256 => {
			let mut hasher = Sha256::new();
			for byte in bytes {
				if let IDLValue::Nat8(value) = byte {
					hasher.update([*value]);
				} else {
					return Err(());
				}
			}
			let digest = hasher.finalize();
			Ok(JsonValue::String(format!("Bytes with sha256: {digest:x}")))
		}
	}
}

fn nybble2hex(nybble: u8) -> &'static str {
	match nybble {
		0 => "0",
		1 => "1",
		2 => "2",
		3 => "3",
		4 => "4",
		5 => "5",
		6 => "6",
		7 => "7",
		8 => "8",
		9 => "9",
		10 => "a",
		11 => "b",
		12 => "c",
		13 => "d",
		14 => "e",
		15 => "f",
		_ => "?",
	}
}

/// Converts a candid IDLValue to a serde JsonValue, with keys as names where possible.
///
/// - Key names MAY be incorrect.  They are provided on a best-effort basis.
/// - If types are incompatible with the data, the data wins.
/// - Data is never omitted.
/// - Fields are never added, even if the schema suggests that some fields are missing.
///
/// The data is preserved at all cost, the schema is applied only to make the data easier to understand and use.
///
/// Note: The textual format in parentheses `(  )` represents IDLArgs containing
/// zero or more IDLValues.  Unless you definitely wish to convert a single value
/// you may wish to consider `idl_args2json_with_weak_names` instead.
pub fn idl2json_with_weak_names(idl: &IDLValue, idl_type: &IDLType, options: &Idl2JsonOptions) -> JsonValue {
	match (idl, idl_type) {
		(idl, IDLType::VarT(type_name)) => {
			if let Some(resolved_type) = get_type_from_any(&options.prog, type_name) {
				idl2json_with_weak_names(idl, &resolved_type, options)
			} else {
				// TODO: Return a set of warnings.  Under the "best effort" mantra, we proceed as
				// best we can but it would be nice to provide some feedback.
				idl2json(idl, options)
			}
		}
		(IDLValue::Bool(bool), _) => JsonValue::Bool(*bool),
		(IDLValue::Null, _) => JsonValue::Null,
		(IDLValue::Text(s), _) => JsonValue::String(s.clone()),
		(IDLValue::Number(s), _) => JsonValue::String(s.clone()), // Unspecified number type
		(IDLValue::Float64(f), _) =>
			serde_json::Number
				::from_f64(*f)
				.map(JsonValue::Number)
				.unwrap_or_else(|| JsonValue::String("NaN".to_string())),
		(IDLValue::Opt(value), IDLType::OptT(opt_type)) => {
			JsonValue::Array(vec![idl2json_with_weak_names(value, opt_type, options)])
		}
		(IDLValue::Opt(_value), _) => idl2json(idl, options), // Fallback for mismatched types
		(IDLValue::Vec(value), IDLType::VecT(item_type)) =>
			match &**item_type {
				IDLType::PrimT(prim_t) if *prim_t == PrimType::Nat8 =>
					convert_bytes(value, options).unwrap_or_else(|_| convert_non_bytes_array(value, options)),
				_ =>
					JsonValue::Array(
						value
							.iter()
							.map(|item| idl2json_with_weak_names(item, item_type, options))
							.collect()
					),
			}
		(IDLValue::Vec(_value), _) => idl2json(idl, options), // Fallback for mismatched types
		(IDLValue::Record(value), IDLType::RecordT(record_types)) =>
			JsonValue::Object(
				value
					.iter()
					.map(|field| convert_idl_field(field, record_types, options))
					.collect()
			),
		(IDLValue::Record(_value), _) => idl2json(idl, options), // Fallback for mismatched types
		(IDLValue::Variant(field), IDLType::VariantT(record_types)) =>
			JsonValue::Object(vec![convert_idl_field(&field.0, record_types, options)].into_iter().collect()),
		(IDLValue::Variant(_field), _) => idl2json(idl, options), // Fallback for mismatched types
		(IDLValue::Principal(p), _) => JsonValue::String(p.to_string()),
		(IDLValue::Service(p), _) => JsonValue::String(p.to_string()),
		(IDLValue::Func(p, c), _) =>
			JsonValue::Object(
				vec![
					("principal".to_string(), JsonValue::String(p.to_string())),
					("code".to_string(), JsonValue::String(c.to_string()))
				]
					.into_iter()
					.collect()
			),
		(IDLValue::None, _) => JsonValue::Array(vec![]),
		(IDLValue::Int(i), _) => JsonValue::String(i.to_string()),
		(IDLValue::Nat(i), _) => JsonValue::String(i.to_string()),
		(IDLValue::Nat8(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
		(IDLValue::Nat16(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
		(IDLValue::Nat32(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
		(IDLValue::Nat64(i), _) => JsonValue::String(i.to_string()),
		(IDLValue::Int8(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
		(IDLValue::Int16(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
		(IDLValue::Int32(i), _) => JsonValue::Number(serde_json::Number::from(*i)),
		(IDLValue::Int64(i), _) => JsonValue::String(i.to_string()),
		(IDLValue::Float32(f), _) => {
			// As far as I can see, JsonValue does not have an explicit NaN type so we provide NaN as a string.
			serde_json::Number
				::from_f64(*f as f64)
				.map(JsonValue::Number)
				.unwrap_or_else(|| JsonValue::String("NaN".to_string()))
		}
		(IDLValue::Reserved, _) => JsonValue::String(idl.to_string()),
	}
}

/// Find a type in any of a list of IDLProgs.
///
/// Note: A canister .did file represents an IDLProg.  That canister .did file may depend on definitions made elsewhere.
pub fn get_type_from_any(progs: &[IDLProg], name: &str) -> Option<IDLType> {
	progs.iter().find_map(|prog| idl_prog::get_type(prog, name))
}

/// Returns a typed IDLField as a (key, value) pair.
///
/// - The key is obtained from the type, if possible, else is the raw key as given.
/// - The value is a typed conversion, if the type is as specified, else it is converted without the benefit of type information.
fn convert_idl_field(field: &IDLField, record_types: &[TypeField], options: &Idl2JsonOptions) -> (String, JsonValue) {
	let field_id = field.id.get_id();
	let field_type = record_types.iter().find(|field_type| field_type.label.get_id() == field_id);
	field_type
		.map(|field_type| {
			(field_type.label.to_string(), idl2json_with_weak_names(&field.val, &field_type.typ, options))
		})
		.unwrap_or_else(|| (field.id.to_string(), idl2json(&field.val, options)))
}

/// Polyfills for the candid IDLProg struct.
pub mod idl_prog {
	use candid::{ parser::types::{ Dec, IDLType, IDLTypes }, IDLProg };

	/// Gets a type defined in a program declarations section.
	#[deprecated(since = "0.8.6", note = "Please use `get_type()` instead.")]
	pub fn get(prog: &IDLProg, key: &str) -> Option<IDLType> {
		get_type(prog, key)
	}

	/// Gets a type defined in a program declarations section.
	pub fn get_type(prog: &IDLProg, key: &str) -> Option<IDLType> {
		prog.decs.iter().find_map(|x| {
			if let Dec::TypD(y) = x {
				if y.id == key {
					return Some(y.typ.clone());
				}
			}
			None
		})
	}

	/// Gets the arguments for creating a service.
	///
	/// This will return None if the prog contains no service aka actor of type ClassT.
	pub fn get_init_arg_type(prog: &IDLProg) -> Option<IDLTypes> {
		if let Some(IDLType::ClassT(args, _)) = &prog.actor { Some(IDLTypes { args: args.clone() }) } else { None }
	}
}
