use std::sync::Arc;

use farmfe_core::{
  context::CompilationContext,
  module::{
    meta_data::script::{statement::SwcId, ScriptModuleMetaData},
    ModuleId,
  },
  swc_common::Mark,
  HashSet,
};
use farmfe_toolkit::{
  script::{idents_collector::UnresolvedIdentCollector, swc_try_with::try_with},
  swc_ecma_visit::VisitWith,
};

pub fn analyze_unresolved_idents(
  module_id: &ModuleId,
  meta: &ScriptModuleMetaData,
  context: &Arc<CompilationContext>,
) -> HashSet<SwcId> {
  // collect statements information, top level idents, unresolved_idents from the ast
  let unresolved_mark = Mark::from_u32(meta.unresolved_mark);

  // fill unresolved_idents
  let mut unresolved_ident_collector = UnresolvedIdentCollector::new(unresolved_mark);
  let cm = context.meta.get_module_source_map(module_id);
  try_with(cm, &context.meta.script.globals, || {
    meta.ast.visit_with(&mut unresolved_ident_collector);
  })
  .unwrap();

  unresolved_ident_collector.unresolved_idents
}

pub fn analyze_top_level_idents(meta: &ScriptModuleMetaData) -> HashSet<SwcId> {
  meta
    .statements
    .iter()
    .filter(|s| s.import_info.is_none())
    .flat_map(|s| s.defined_idents.clone())
    .collect()
}
