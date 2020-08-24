// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Types for wasm based tracing. Loosly inspired by `tracing-core` but
/// optimised for the specific use case.

use core::fmt::Debug;
use sp_std::{
	vec::Vec,
};
use sp_std::Writer;
use codec::{Encode, Decode};

/// The Tracing Level – the user can filter by this
#[derive(Clone, Encode, Decode, Debug)]
pub enum WasmLevel {
	/// This is a fatal errors
	ERROR,
	/// This is a warning you should be aware of
	WARN,
	/// Nice to now info
	INFO,
	/// Further information for debugging purposes
	DEBUG,
	/// The lowest level, keeping track of minute detail
	TRACE
}

/// A paramter value provided to the span/event
#[derive(Encode, Decode, Clone, Debug)]
pub enum WasmValue {
	U8(u8),
	I8(i8),
	U32(u32),
	I32(i32),
	I64(i64),
	U64(u64),
	Bool(bool),
	Str(Vec<u8>),
	/// Debug or Display call, this is most-likely a print-able UTF8 String
	Formatted(Vec<u8>),
	/// SCALE CODEC encoded object – the name should allow the received to know
	/// how to decode this.
	Encoded(Vec<u8>),
}

impl From<u8> for WasmValue {
	fn from(u: u8) -> WasmValue {
		WasmValue::U8(u)
	}
}

impl From<&i8> for WasmValue {
	fn from(inp: &i8) -> WasmValue {
		WasmValue::I8(inp.clone())
	}
}

impl From<&str> for WasmValue {
	fn from(inp: &str) -> WasmValue {
		WasmValue::Str(inp.as_bytes().to_vec())
	}
}

impl From<&&str> for WasmValue {
	fn from(inp: &&str) -> WasmValue {
		WasmValue::Str((*inp).as_bytes().to_vec())
	}
}

impl From<bool> for WasmValue {
	fn from(inp: bool) -> WasmValue {
		WasmValue::Bool(inp)
	}
}

impl From<&core::fmt::Arguments<'_>> for WasmValue {
	fn from(inp: &core::fmt::Arguments<'_>) -> WasmValue {
		let mut buf = Writer::default();
		core::fmt::write(&mut buf, *inp).expect("Writing of arguments doesn't fail");
		WasmValue::Formatted(buf.into_inner())
	}
}

impl From<i8> for WasmValue {
	fn from(u: i8) -> WasmValue {
		WasmValue::I8(u)
	}
}

impl From<i32> for WasmValue {
	fn from(u: i32) -> WasmValue {
		WasmValue::I32(u)
	}
}

impl From<&i32> for WasmValue {
	fn from(u: &i32) -> WasmValue {
		WasmValue::I32(*u)
	}
}

impl From<u32> for WasmValue {
	fn from(u: u32) -> WasmValue {
		WasmValue::U32(u)
	}
}

impl From<&u32> for WasmValue {
	fn from(u: &u32) -> WasmValue {
		WasmValue::U32(*u)
	}
}

impl From<u64> for WasmValue {
	fn from(u: u64) -> WasmValue {
		WasmValue::U64(u)
	}
}

impl From<i64> for WasmValue {
	fn from(u: i64) -> WasmValue {
		WasmValue::I64(u)
	}
}

/// The name of a field provided as the argument name when contstructing an
/// `event!` or `span!`.
/// Generally generated automaticaly via `stringify` from an `'static &str`.
/// Likely print-able.
#[derive(Encode, Decode, Clone, Debug)]
pub struct WasmFieldName(Vec<u8>);


impl From<Vec<u8>> for WasmFieldName {
	fn from(v: Vec<u8>) -> Self {
		WasmFieldName(v)
	}
}

impl From<&str> for WasmFieldName {
	fn from(v: &str) -> Self {
		WasmFieldName(v.as_bytes().to_vec())
	}
}

/// A list of `WasmFieldName`s in the order provided
#[derive(Encode, Decode, Clone, Debug)]
pub struct WasmFields(Vec<WasmFieldName>);

impl WasmFields {
	pub fn iter(&self) -> core::slice::Iter<'_, WasmFieldName> {
		self.0.iter()
	}
}

impl From<Vec<WasmFieldName>> for WasmFields {
	fn from(v: Vec<WasmFieldName>) -> WasmFields {
		WasmFields(v.into())
	}
}

impl From<Vec<&str>> for WasmFields {
	fn from(v: Vec<&str>) -> WasmFields {
		WasmFields(v.into_iter().map(|v| v.into()).collect())
	}
}

impl WasmFields {
	/// Create an empty entry
	pub fn empty() -> Self {
		WasmFields(Vec::with_capacity(0))
	}
}


/// A list of `WasmFieldName`s with the given `WasmValue` (if provided)
/// in the order specified.
#[derive(Encode, Decode, Clone, Debug)]
pub struct WasmValuesSet(Vec<(WasmFieldName, Option<WasmValue>)>);

impl From<Vec<(WasmFieldName, Option<WasmValue>)>> for WasmValuesSet {
	fn from(v: Vec<(WasmFieldName, Option<WasmValue>)>) -> Self {
		WasmValuesSet(v)
	}
}
impl From<Vec<(&&WasmFieldName, Option<WasmValue>)>> for WasmValuesSet {
	fn from(v: Vec<(&&WasmFieldName, Option<WasmValue>)>) -> Self {
		WasmValuesSet(v.into_iter().map(|(k, v)| ((**k).clone(), v)).collect())
	}
}

impl From<Vec<(&&str, Option<WasmValue>)>> for WasmValuesSet {
	fn from(v: Vec<(&&str, Option<WasmValue>)>) -> Self {
		WasmValuesSet(v.into_iter().map(|(k, v)| ((*k).into(), v)).collect())
	}
}

impl WasmValuesSet {
	/// Create an empty entry
	pub fn empty() -> Self {
		WasmValuesSet(Vec::with_capacity(0))
	}
}

/// Metadata provides generic information about the specifc location of the
/// `span!` or `event!` call on the wasm-side.
#[derive(Encode, Decode, Clone, Debug)]
pub struct WasmMetadata {
	/// The name given to `event!`/`span!`, `&'static str` converted to bytes
	pub name: Vec<u8>,
	/// The given target to `event!`/`span!` – or module-name, `&'static str` converted to bytes
	pub target: Vec<u8>,
	/// The level of this entry
	pub level: WasmLevel,
	/// The file this was emitted from – useful for debugging;  `&'static str` converted to bytes
	pub file: Vec<u8>,
	/// The specific line number in the file – useful for debugging
	pub line: u32,
	/// The module path;  `&'static str` converted to bytes
	pub module_path: Vec<u8>,
	/// Whether this is a call  to `span!` or `event!`
	pub is_span: bool,
	/// The list of fields specified in the call
	pub fields: WasmFields,
}

/// Span or Event Attributes
#[derive(Encode, Decode, Clone, Debug)]
pub struct WasmEntryAttributes {
	/// the parent, if directly specified – otherwise assume most inner span
	pub parent_id: Option<u64>,
	/// the metadata of the location
	pub metadata: WasmMetadata,
	/// the Values provided
	pub fields: WasmValuesSet,
}

#[cfg(feature = "std")]
mod std_features {

	use tracing_core::callsite;
	use tracing;

	/// Static entry use for wasm-originated metadata.
	pub struct WasmCallsite;
	impl callsite::Callsite for WasmCallsite {
		fn set_interest(&self, _: tracing_core::Interest) { unimplemented!() }
		fn metadata(&self) -> &tracing_core::Metadata { unimplemented!() }
	}
	static CALLSITE: WasmCallsite =  WasmCallsite;
	/// The identifier we are using to inject the wasm events in the generic `tracing` system
	pub static WASM_TRACE_IDENTIFIER: &'static str = "wasm_tracing";
	/// The fieldname for the wasm-originated name
	pub static WASM_NAME_KEY: &'static str = "name";
	/// The fieldname for the wasm-originated target
	pub static WASM_TARGET_KEY: &'static str = "target";
	/// The the list of all static field names we construct from the given metadata
	pub static GENERIC_FIELDS: &'static [&'static str] = &[WASM_TARGET_KEY, WASM_NAME_KEY, "file", "line", "module_path", "params"];

	// Implementation Note:
	// the original `tracing` crate generates these static metadata entries at every `span!` and
	// `event!` location to allow for highly optimised filtering. For us to allow level-based emitting
	// of wasm events we need these static metadata entries to inject into that system. We then provide
	// generic `From`-implementations picking the right metadata to refer to.

	static SPAN_ERROR_METADATA : tracing_core::Metadata<'static> = tracing::Metadata::new(
		WASM_TRACE_IDENTIFIER, WASM_TRACE_IDENTIFIER, tracing::Level::ERROR, None, None, None,
		tracing_core::field::FieldSet::new(GENERIC_FIELDS, tracing_core::identify_callsite!(&CALLSITE)),
		tracing_core::metadata::Kind::SPAN
	);

	static SPAN_WARN_METADATA : tracing_core::Metadata<'static> = tracing::Metadata::new(
		WASM_TRACE_IDENTIFIER, WASM_TRACE_IDENTIFIER, tracing::Level::WARN, None, None, None,
		tracing_core::field::FieldSet::new(GENERIC_FIELDS, tracing_core::identify_callsite!(&CALLSITE)),
		tracing_core::metadata::Kind::SPAN
	);
	static SPAN_INFO_METADATA : tracing_core::Metadata<'static> = tracing::Metadata::new(
		WASM_TRACE_IDENTIFIER, WASM_TRACE_IDENTIFIER, tracing::Level::INFO, None, None, None,
		tracing_core::field::FieldSet::new(GENERIC_FIELDS, tracing_core::identify_callsite!(&CALLSITE)),
		tracing_core::metadata::Kind::SPAN
	);

	static SPAN_DEBUG_METADATA : tracing_core::Metadata<'static> = tracing::Metadata::new(
		WASM_TRACE_IDENTIFIER, WASM_TRACE_IDENTIFIER, tracing::Level::DEBUG, None, None, None,
		tracing_core::field::FieldSet::new(GENERIC_FIELDS, tracing_core::identify_callsite!(&CALLSITE)),
		tracing_core::metadata::Kind::SPAN
	);

	static SPAN_TRACE_METADATA : tracing_core::Metadata<'static> = tracing::Metadata::new(
		WASM_TRACE_IDENTIFIER, WASM_TRACE_IDENTIFIER, tracing::Level::TRACE, None, None, None,
		tracing_core::field::FieldSet::new(GENERIC_FIELDS, tracing_core::identify_callsite!(&CALLSITE)),
		tracing_core::metadata::Kind::SPAN
	);

	static EVENT_ERROR_METADATA : tracing_core::Metadata<'static> = tracing::Metadata::new(
		WASM_TRACE_IDENTIFIER, WASM_TRACE_IDENTIFIER, tracing::Level::ERROR, None, None, None,
		tracing_core::field::FieldSet::new(GENERIC_FIELDS, tracing_core::identify_callsite!(&CALLSITE)),
		tracing_core::metadata::Kind::EVENT
	);

	static EVENT_WARN_METADATA : tracing_core::Metadata<'static> = tracing::Metadata::new(
		WASM_TRACE_IDENTIFIER, WASM_TRACE_IDENTIFIER, tracing::Level::WARN, None, None, None,
		tracing_core::field::FieldSet::new(GENERIC_FIELDS, tracing_core::identify_callsite!(&CALLSITE)),
		tracing_core::metadata::Kind::EVENT
	);

	static EVENT_INFO_METADATA : tracing_core::Metadata<'static> = tracing::Metadata::new(
		WASM_TRACE_IDENTIFIER, WASM_TRACE_IDENTIFIER, tracing::Level::INFO, None, None, None,
		tracing_core::field::FieldSet::new(GENERIC_FIELDS, tracing_core::identify_callsite!(&CALLSITE)),
		tracing_core::metadata::Kind::EVENT
	);

	static EVENT_DEBUG_METADATA : tracing_core::Metadata<'static> = tracing::Metadata::new(
		WASM_TRACE_IDENTIFIER, WASM_TRACE_IDENTIFIER, tracing::Level::DEBUG, None, None, None,
		tracing_core::field::FieldSet::new(GENERIC_FIELDS, tracing_core::identify_callsite!(&CALLSITE)),
		tracing_core::metadata::Kind::EVENT
	);

	static EVENT_TRACE_METADATA : tracing_core::Metadata<'static> = tracing::Metadata::new(
		WASM_TRACE_IDENTIFIER, WASM_TRACE_IDENTIFIER, tracing::Level::TRACE, None, None, None,
		tracing_core::field::FieldSet::new(GENERIC_FIELDS, tracing_core::identify_callsite!(&CALLSITE)),
		tracing_core::metadata::Kind::EVENT
	);

	impl From<&crate::WasmMetadata> for &'static tracing_core::Metadata<'static> {
		fn from(wm: &crate::WasmMetadata) -> &'static tracing_core::Metadata<'static> {
			match (&wm.level, wm.is_span) {
				(&crate::WasmLevel::ERROR, true) => &SPAN_ERROR_METADATA,
				(&crate::WasmLevel::WARN, true) => &SPAN_WARN_METADATA,
				(&crate::WasmLevel::INFO, true) => &SPAN_INFO_METADATA,
				(&crate::WasmLevel::DEBUG, true) => &SPAN_DEBUG_METADATA,
				(&crate::WasmLevel::TRACE, true) => &SPAN_TRACE_METADATA,
				(&crate::WasmLevel::ERROR, false) => &EVENT_ERROR_METADATA,
				(&crate::WasmLevel::WARN, false) => &EVENT_WARN_METADATA,
				(&crate::WasmLevel::INFO, false) => &EVENT_INFO_METADATA,
				(&crate::WasmLevel::DEBUG, false) => &EVENT_DEBUG_METADATA,
				(&crate::WasmLevel::TRACE, false) => &EVENT_TRACE_METADATA,
			}
		}
	}

	impl From<crate::WasmEntryAttributes> for tracing::Span {
		fn from(a: crate::WasmEntryAttributes) -> tracing::Span {
			let name = std::str::from_utf8(&a.metadata.name).unwrap_or_default();
			let target = std::str::from_utf8(&a.metadata.target).unwrap_or_default();
			let file = std::str::from_utf8(&a.metadata.file).unwrap_or_default();
			let line = a.metadata.line;
			let module_path = std::str::from_utf8(&a.metadata.module_path).unwrap_or_default();
			let params = a.fields;
			let metadata : &tracing_core::metadata::Metadata<'static> = (&a.metadata).into();

			tracing::span::Span::child_of(
				a.parent_id.map(|i|tracing_core::span::Id::from_u64(i)),
				&metadata,
				&tracing::valueset!{ metadata.fields(), target, name, file, line, module_path, ?params }
			)
		}
	}

	impl crate::WasmEntryAttributes {
		/// convert the given Attributes to an event and emit it using `tracing_core`.
		pub fn emit(self: crate::WasmEntryAttributes) {
			let name = std::str::from_utf8(&self.metadata.name).unwrap_or_default();
			let target = std::str::from_utf8(&self.metadata.target).unwrap_or_default();
			let file = std::str::from_utf8(&self.metadata.file).unwrap_or_default();
			let line = self.metadata.line;
			let module_path = std::str::from_utf8(&self.metadata.module_path).unwrap_or_default();
			let params = self.fields;
			let metadata : &tracing_core::metadata::Metadata<'static> = (&self.metadata).into();

			tracing_core::Event::child_of(
				self.parent_id.map(|i|tracing_core::span::Id::from_u64(i)),
				&metadata,
				&tracing::valueset!{ metadata.fields(), target, name, file, line, module_path, ?params }
			)
		}
	}
}

#[cfg(feature = "std")]
pub use std_features::*;