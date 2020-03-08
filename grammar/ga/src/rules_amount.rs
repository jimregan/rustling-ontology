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

pub fn rules_finance(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect (X cents)",
             amount_of_money_check!(),
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, b| helpers::compose_money(a.value(), b.value()));
    b.rule_3("intersect (and X cents)",
             amount_of_money_check!(),
             b.reg(r#"agus|is|[’']s"#)?,
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, _, b| helpers::compose_money(&a.value(), &b.value()));
    b.rule_2("intersect",
             amount_of_money_check!(),
             number_check!(),
             |a, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_3("intersect (and number)",
             amount_of_money_check!(),
             b.reg(r#"agus|is|[’']s"#)?,
             number_check!(),
             |a, _, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_1_terminal("$",
                      b.reg(r#"\$|n?dh?ollai?rs?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("$") })
    );
    b.rule_1_terminal("USD",
                      b.reg(r#"us[d\$]|n?dh?ollai?rs? ((na )?St[áaà]t( Aontaithe (Mheirice[áaà])?)?|Mheirice[áaà])"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("USD") })
    );
    b.rule_1_terminal("AUD",
                      b.reg(r#"au[d\$]|n?dh?ollai?rs? na hAstr[áaà]ile"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_1_terminal("CAD",
                      b.reg(r#"cad|n?dh?ollai?rs? Cheanada"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CAD") })
    );
    b.rule_1_terminal("HKD",
                      b.reg(r#"hkd|n?dh?ollai?rs? Hong Cong"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("HKD") })
    );
    b.rule_1_terminal("EUR",
                      b.reg(r#"€|(?:[e€]uro?s?)"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("EUR") })
    );
    b.rule_1_terminal("£",
                      b.reg(r#"£|b?ph?ui?nt|pounds?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("GBP",
                      b.reg(r#"gbp|b?ph?ui?nt (steirlin[ng]|na (Breataine|R[íiì]ochta Aontaithe))"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_1_terminal("CHF",
                      b.reg(r#"chf|(bh)?fh?ranc(anna)? na hEilv[éeè]ise"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CHF") })
    );
    b.rule_1_terminal("KR",
                      b.reg(r#"kroner?|g?ch?or[óoò]i?n(acha?)?|kr"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KR") })
    );
    b.rule_1_terminal("DKK",
                      b.reg(r#"dkk|(?:kroner?|g?ch?or[óoò]i?n(acha?)?) na Danmhairge"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("DKK") })
    );
    b.rule_1_terminal("NOK",
                      b.reg(r#"nok|(?:kroner?|g?ch?or[óoò]i?n(acha?)?) na hIorua"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("NOK") })
    );
    b.rule_1_terminal("SEK",
                      b.reg(r#"sek|swedish (?:krona|kronor|g?ch?or[óoò]i?n(acha?)?) na Sualainne"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("SEK") })
    );
    b.rule_1_terminal("RUB",
                      b.reg(r#"(ro?ubles?|rub|r[úuù]bal)( na R[úuù]ise)"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("RUB") })
    );
    b.rule_1_terminal("INR",
                      b.reg(r#"inr|rs\.?|rupees?|r[úuù]pa[íiì](the)( na hIndia)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_1_terminal("JPY",
                      b.reg(r#"jpy|yens?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("CNY",
                      b.reg(r#"cny|cnh|rmb|yuans?|renminbis?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CNY") })
    );
    b.rule_1_terminal("¥",
                      b.reg(r#"¥"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
    b.rule_1_terminal("KRW",
                      b.reg(r#"₩|krw|wons? na C[óoò]ir[éeè]( theas)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
    b.rule_1_terminal("฿",
                      b.reg(r#"฿|bitcoins?|boi?nn gh?iot[áaà]in"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("฿") })
    );
    b.rule_1_terminal("cent",
                      b.reg(r#"g?ch?entimes?|g?ch?ei?nt(s|e?anna)?|b?ph?enn(?:y|ies)|b?ph?ingin[íiìe]|c|¢"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("cent") })
    );
    b.rule_2("<unit> <amount>",
             money_unit!(),
             number_check!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: b.value().value(),
                     unit: a.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    b.rule_2("<amount> <unit>",
             number_check!(),
             money_unit!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: a.value().value(),
                     unit: b.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    b.rule_2("about <amount-of-money>",
             b.reg(r#"(?:thart ar|timpeall|tuaiream is|a bheag n[óoò] a mh[óoò]r|go garbh|ag bord[áaà]il ar)"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"glan|(go )?d[íiì]reach( glan)||go dt[íiì] an cei?nt"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             });
    Ok(())
}
