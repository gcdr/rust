// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Structs representing the analysis data from a crate.
//!
//! The `Dump` trait can be used together with `DumpVisitor` in order to
//! retrieve the data from a crate.

use rustc::hir::def_id::DefId;
use rustc::ty;
use syntax::ast::{CrateNum, NodeId};
use syntax::codemap::{Span, CodeMap};

#[derive(Debug, Clone, RustcEncodable)]
pub struct SpanData {
    file_name: String,
    byte_start: u32,
    byte_end: u32,
    /// 1-based.
    line_start: usize,
    line_end: usize,
    /// 1-based, character offset.
    column_start: usize,
    column_end: usize,
}

impl SpanData {
    pub fn from_span(span: Span, cm: &CodeMap) -> SpanData {
        let start = cm.lookup_char_pos(span.lo);
        let end = cm.lookup_char_pos(span.hi);

        SpanData {
            file_name: start.file.name.clone(),
            byte_start: span.lo.0,
            byte_end: span.hi.0,
            line_start: start.line,
            line_end: end.line,
            column_start: start.col.0 + 1,
            column_end: end.col.0 + 1,
        }
    }
}

pub struct CrateData {
    pub name: String,
    pub number: u32,
    pub span: Span,
}

/// Data for any entity in the Rust language. The actual data contained varies
/// with the kind of entity being queried. See the nested structs for details.
#[derive(Debug, RustcEncodable)]
pub enum Data {
    /// Data for Enums.
    EnumData(EnumData),
    /// Data for extern crates.
    ExternCrateData(ExternCrateData),
    /// Data about a function call.
    FunctionCallData(FunctionCallData),
    /// Data for all kinds of functions and methods.
    FunctionData(FunctionData),
    /// Data about a function ref.
    FunctionRefData(FunctionRefData),
    /// Data for impls.
    ImplData(ImplData2),
    /// Data for trait inheritance.
    InheritanceData(InheritanceData),
    /// Data about a macro declaration.
    MacroData(MacroData),
    /// Data about a macro use.
    MacroUseData(MacroUseData),
    /// Data about a method call.
    MethodCallData(MethodCallData),
    /// Data for method declarations (methods with a body are treated as functions).
    MethodData(MethodData),
    /// Data for modules.
    ModData(ModData),
    /// Data for a reference to a module.
    ModRefData(ModRefData),
    /// Data for a struct declaration.
    StructData(StructData),
    /// Data for a struct variant.
    StructVariantDat(StructVariantData),
    /// Data for a trait declaration.
    TraitData(TraitData),
    /// Data for a tuple variant.
    TupleVariantData(TupleVariantData),
    /// Data for a typedef.
    TypeDefData(TypedefData),
    /// Data for a reference to a type or trait.
    TypeRefData(TypeRefData),
    /// Data for a use statement.
    UseData(UseData),
    /// Data for a global use statement.
    UseGlobData(UseGlobData),
    /// Data for local and global variables (consts and statics), and fields.
    VariableData(VariableData),
    /// Data for the use of some variable (e.g., the use of a local variable, which
    /// will refere to that variables declaration).
    VariableRefData(VariableRefData),
}

/// Data for the prelude of a crate.
#[derive(Debug, RustcEncodable)]
pub struct CratePreludeData {
    pub crate_name: String,
    pub crate_root: String,
    pub external_crates: Vec<ExternalCrateData>,
    pub span: Span,
}

/// Data for external crates in the prelude of a crate.
#[derive(Debug, RustcEncodable)]
pub struct ExternalCrateData {
    pub name: String,
    pub num: CrateNum,
    pub file_name: String,
}

/// Data for enum declarations.
#[derive(Clone, Debug, RustcEncodable)]
pub struct EnumData {
    pub id: NodeId,
    pub value: String,
    pub qualname: String,
    pub span: Span,
    pub scope: NodeId,
}

/// Data for extern crates.
#[derive(Debug, RustcEncodable)]
pub struct ExternCrateData {
    pub id: NodeId,
    pub name: String,
    pub crate_num: CrateNum,
    pub location: String,
    pub span: Span,
    pub scope: NodeId,
}

/// Data about a function call.
#[derive(Debug, RustcEncodable)]
pub struct FunctionCallData {
    pub span: Span,
    pub scope: NodeId,
    pub ref_id: DefId,
}

/// Data for all kinds of functions and methods.
#[derive(Clone, Debug, RustcEncodable)]
pub struct FunctionData {
    pub id: NodeId,
    pub name: String,
    pub qualname: String,
    pub declaration: Option<DefId>,
    pub span: Span,
    pub scope: NodeId,
}

/// Data about a function call.
#[derive(Debug, RustcEncodable)]
pub struct FunctionRefData {
    pub span: Span,
    pub scope: NodeId,
    pub ref_id: DefId,
}

#[derive(Debug, RustcEncodable)]
pub struct ImplData {
    pub id: NodeId,
    pub span: Span,
    pub scope: NodeId,
    pub trait_ref: Option<DefId>,
    pub self_ref: Option<DefId>,
}

#[derive(Debug, RustcEncodable)]
// FIXME: this struct should not exist. However, removing it requires heavy
// refactoring of dump_visitor.rs. See PR 31838 for more info.
pub struct ImplData2 {
    pub id: NodeId,
    pub span: Span,
    pub scope: NodeId,
    // FIXME: I'm not really sure inline data is the best way to do this. Seems
    // OK in this case, but generalising leads to returning chunks of AST, which
    // feels wrong.
    pub trait_ref: Option<TypeRefData>,
    pub self_ref: Option<TypeRefData>,
}

#[derive(Debug, RustcEncodable)]
pub struct InheritanceData {
    pub span: Span,
    pub base_id: DefId,
    pub deriv_id: NodeId
}

/// Data about a macro declaration.
#[derive(Debug, RustcEncodable)]
pub struct MacroData {
    pub span: Span,
    pub name: String,
    pub qualname: String,
}

/// Data about a macro use.
#[derive(Debug, RustcEncodable)]
pub struct MacroUseData {
    pub span: Span,
    pub name: String,
    pub qualname: String,
    // Because macro expansion happens before ref-ids are determined,
    // we use the callee span to reference the associated macro definition.
    pub callee_span: Span,
    pub scope: NodeId,
    pub imported: bool,
}

/// Data about a method call.
#[derive(Debug, RustcEncodable)]
pub struct MethodCallData {
    pub span: Span,
    pub scope: NodeId,
    pub ref_id: Option<DefId>,
    pub decl_id: Option<DefId>,
}

/// Data for method declarations (methods with a body are treated as functions).
#[derive(Clone, Debug, RustcEncodable)]
pub struct MethodData {
    pub id: NodeId,
    pub qualname: String,
    pub span: Span,
    pub scope: NodeId,
}

/// Data for modules.
#[derive(Debug, RustcEncodable)]
pub struct ModData {
    pub id: NodeId,
    pub name: String,
    pub qualname: String,
    pub span: Span,
    pub scope: NodeId,
    pub filename: String,
}

/// Data for a reference to a module.
#[derive(Debug, RustcEncodable)]
pub struct ModRefData {
    pub span: Span,
    pub scope: NodeId,
    pub ref_id: Option<DefId>,
    pub qualname: String
}

#[derive(Debug, RustcEncodable)]
pub struct StructData {
    pub span: Span,
    pub id: NodeId,
    pub ctor_id: NodeId,
    pub qualname: String,
    pub scope: NodeId,
    pub value: String
}

#[derive(Debug, RustcEncodable)]
pub struct StructVariantData {
    pub span: Span,
    pub id: NodeId,
    pub qualname: String,
    pub type_value: String,
    pub value: String,
    pub scope: NodeId
}

#[derive(Debug, RustcEncodable)]
pub struct TraitData {
    pub span: Span,
    pub id: NodeId,
    pub qualname: String,
    pub scope: NodeId,
    pub value: String
}

#[derive(Debug, RustcEncodable)]
pub struct TupleVariantData {
    pub span: Span,
    pub id: NodeId,
    pub name: String,
    pub qualname: String,
    pub type_value: String,
    pub value: String,
    pub scope: NodeId
}

/// Data for a typedef.
#[derive(Debug, RustcEncodable)]
pub struct TypedefData {
    pub id: NodeId,
    pub span: Span,
    pub qualname: String,
    pub value: String,
}

/// Data for a reference to a type or trait.
#[derive(Clone, Debug, RustcEncodable)]
pub struct TypeRefData {
    pub span: Span,
    pub scope: NodeId,
    pub ref_id: Option<DefId>,
    pub qualname: String,
}

#[derive(Debug, RustcEncodable)]
pub struct UseData {
    pub id: NodeId,
    pub span: Span,
    pub name: String,
    pub mod_id: Option<DefId>,
    pub scope: NodeId
}

#[derive(Debug, RustcEncodable)]
pub struct UseGlobData {
    pub id: NodeId,
    pub span: Span,
    pub names: Vec<String>,
    pub scope: NodeId
}

/// Data for local and global variables (consts and statics).
#[derive(Debug, RustcEncodable)]
pub struct VariableData {
    pub id: NodeId,
    pub name: String,
    pub qualname: String,
    pub span: Span,
    pub scope: NodeId,
    pub value: String,
    pub type_value: String,
}

/// Data for the use of some item (e.g., the use of a local variable, which
/// will refer to that variables declaration (by ref_id)).
#[derive(Debug, RustcEncodable)]
pub struct VariableRefData {
    pub name: String,
    pub span: Span,
    pub scope: NodeId,
    pub ref_id: DefId,
}

// Emitted ids are used to cross-reference items across crates. DefIds and
// NodeIds do not usually correspond in any way. The strategy is to use the
// index from the DefId as a crate-local id. However, within a crate, DefId
// indices and NodeIds can overlap. So, we must adjust the NodeIds. If an
// item can be identified by a DefId as well as a NodeId, then we use the
// DefId index as the id. If it can't, then we have to use the NodeId, but
// need to adjust it so it will not clash with any possible DefId index.
pub fn normalize_node_id<'a>(tcx: &ty::TyCtxt<'a>, id: NodeId) -> usize {
    match tcx.map.opt_local_def_id(id) {
        Some(id) => id.index.as_usize(),
        None => id as usize + tcx.map.num_local_def_ids()
    }
}

// Macro to implement a normalize() function (see below for usage)
macro_rules! impl_normalize {
    ($($t:ty => $($field:ident),*);*) => {
        $(
            impl $t {
                pub fn normalize<'a>(mut self, tcx: &ty::TyCtxt<'a>) -> $t {
                    $(
                        self.$field = normalize_node_id(tcx, self.$field) as u32;
                    )*
                    self
                }
            }
        )*
    }
}

impl_normalize! {
    EnumData => id, scope;
    ExternCrateData => id, scope;
    FunctionCallData => scope;
    FunctionData => id, scope;
    FunctionRefData => scope;
    ImplData => id, scope;
    InheritanceData => deriv_id;
    MacroUseData => scope;
    MethodCallData => scope;
    MethodData => id, scope;
    ModData => id, scope;
    ModRefData => scope;
    StructData => ctor_id, id, scope;
    StructVariantData => id, scope;
    TupleVariantData => id, scope;
    TraitData => id, scope;
    TypedefData => id;
    TypeRefData => scope;
    UseData => id, scope;
    UseGlobData => id, scope;
    VariableData => id;
    VariableRefData => scope
}
