// use hir::Semantics;
// use ide_db::{base_db::FileId, RootDatabase};
// use text_edit::TextEdit;

// // Feature: Organize Imports
// //
// // Remove all unused imports in current file.
// //
// // |===
// // | Editor  | Action Name
// //
// // | VS Code | **rust-analyzer: Organize Imports**
// // |===
// //
// pub(crate) fn organize_imports(db: &RootDatabase, file_id: FileId) -> Option<TextEdit> {
//     let sema = Semantics::new(db);
//     let file = sema.parse(file_id);

//     // let item = if range.range.is_empty() {
//     //     SyntaxElement::Token(pick_best_token(
//     //         file.syntax().token_at_offset(range.range.start()),
//     //         |kind| match kind {
//     //             SyntaxKind::IDENT | SyntaxKind::LIFETIME_IDENT => 2,
//     //             kind if kind.is_trivia() => 0,
//     //             _ => 1,
//     //         },
//     //     )?)
//     // } else {
//     //     file.syntax().covering_element(range.range)
//     // };
//     None
// }
