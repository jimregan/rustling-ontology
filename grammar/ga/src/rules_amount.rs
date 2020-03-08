use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;

pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
             number_check!(),
             b.reg(r"(?:%|faoin? g?ch?[eéè]ad|sa g?ch?[eéè]ad)")?,
             |number, _| Ok(PercentageValue(number.value().value()))
    );
    Ok(())
}

