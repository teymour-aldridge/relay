/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use common::SourceLocationKey;
use fixture_tests::Fixture;
use graphql_ir::{build, Program};
use graphql_syntax::parse;
use graphql_text_printer::print_fragment;
use graphql_transforms::sort_selections;
use std::sync::Arc;
use test_schema::get_test_schema;

pub fn transform_fixture(fixture: &Fixture) -> Result<String, String> {
    let source_location = SourceLocationKey::standalone(fixture.file_name);
    let schema = get_test_schema();
    let ast = parse(fixture.content, source_location).unwrap();
    let ir = build(&schema, &ast.definitions).unwrap();
    let program = Program::from_definitions(Arc::clone(&schema), ir);
    let next_program = sort_selections(&program);

    assert_eq!(
        next_program.fragments().count(),
        program.fragments().count()
    );
    let mut printed = next_program
        .fragments()
        .map(|def| print_fragment(&schema, def))
        .collect::<Vec<_>>();
    printed.sort();
    Ok(printed.join("\n\n"))
}
