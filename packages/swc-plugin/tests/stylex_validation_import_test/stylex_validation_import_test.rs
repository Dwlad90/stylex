use stylex_swc_plugin::ModuleTransformVisitor;
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
};

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_classname(tr.comments.clone(), Option::None),
    ignores_valid_imports,
    r#"
        import stylex from '@stylexjs/stylex';
        import {foo, bar} from 'other';

        export default stylex.create({
            foo: {
                color: 'red'
            },
        });
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_classname(tr.comments.clone(), Option::None),
    ignores_valid_requires,
    r#"
        const stylex = require('@stylexjs/stylex');
        const {foo, bar} = require('other');

        export default stylex.create({
            foo: {
                color: 'red'
            },
        });
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    named_declaration_export,
    r#"
        import stylex from '@stylexjs/stylex';
        export const styles = stylex.create({
            foo: {
                color: 'red'
            },
            bar: {
                color: 'blue'
            },
        });
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    does_nothing_when_stylex_not_imported,
    r#"
        export const styles = stylex.create({
            foo: {
                color: 'red'
            },
        });
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    named_property_export,
    r#"
        import stylex from '@stylexjs/stylex';
        const styles = stylex.create({
            foo: {
                color: 'red'
            },
        });
        export {styles}
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    default_export,
    r#"
        import stylex from '@stylexjs/stylex';
        export default stylex.create({
            foo: {
                color: 'red'
            },
        });
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    default_export_with_parenthesis,
    r#"
        import stylex from '@stylexjs/stylex';
        export default (stylex.create({
            foo: {
                color: 'red'
            },
        }));
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    module_export,
    r#"
        import stylex from '@stylexjs/stylex';
        const styles = stylex.create({
            foo: {
                color: 'red'
            },
        });
        module.export = styles;
    "#
);