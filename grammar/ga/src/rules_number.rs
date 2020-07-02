use std::f64;

use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_3("intersect (with and)",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             b.reg(r#"agus|is|[’']s"#)?,
             number_check!(),
             |a, _, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_1_terminal("integer (0..10)",
                      b.reg(r#""(?:a )?(n[áaà]id|(?:h|t-?)aon|dh[áaà]|tr[íiì]|ceithre|c[úuù]ig|seacht|s[éeè]|(?:h|t-?)ocht|naoi|deich)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "náid" => 0,
                              "naid" => 0,
                              "nàid" => 0,
                              "aon" => 1,
                              "haon" => 1,
                              "t-aon" => 1,
                              "taon" => 1,
                              "dhá" => 2,
                              "dha" => 2,
                              "dhà" => 2,
                              "trí" => 3,
                              "tri" => 3,
                              "trì" => 3,
                              "ceithre" => 4,
                              "cúig" => 5,
                              "cuig" => 5,
                              "cùig" => 5,
                              "sé" => 6,
                              "se" => 6,
                              "sè" => 6,
                              "seacht" => 7,
                              "ocht" => 8,
                              "hocht" => 8,
                              "tocht" => 8,
                              "t-ocht" => 8,
                              "naoi" => 9,
                              "deich" => 10,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new_with_grain(value, 1)
                      });
    b.rule_1_terminal("single",
                      b.reg(r#"amh[áaà]in"#)?,
                      |_| IntegerValue::new_with_grain(1, 1)
    );
    b.rule_1_terminal("a pair",
                      b.reg(r#"m?bh?eirt"#)?,
                      |_| IntegerValue::new_with_grain(2, 1)
    );
    b.rule_1_terminal("couple",
                      b.reg(r#"c[úuù]pla"#)?,
                      |_| IntegerValue::new_with_grain(2, 1)
    );
    b.rule_1_terminal("teens",
                      b.reg(r#"n?dh?[éeè]ag"#)?,
                      |_| IntegerValue::new_with_grain(10, 1)
    );
    b.rule_1("few", b.reg(r#"roinnt|[áaà]irithe"#)?, |_| {
        Ok(IntegerValue {
            value: 3,
            grain: Some(1),
            precision: Approximate,
            ..IntegerValue::default()
        })
    });
    b.rule_1_terminal("integer (20..90)",
                      b.reg(r#"(fiche|tr[íiì]ocha|daichead|caoga|seasca|seacht[óoò]|ocht[óoò]|n[óoò]cha)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "fiche" => 20,
                              "tríocha" => 30,
                              "triocha" => 30,
                              "trìocha" => 30,
                              "daichead" => 40,
                              "caoga" => 50,
                              "seasca" => 60,
                              "seachtó" => 70,
                              "seachto" => 70,
                              "seachtò" => 70,
                              "ochtó" => 80,
                              "ochto" => 80,
                              "ochtò" => 80,
                              "nócha" => 90,
                              "nocha" => 90,
                              "nòcha" => 90,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new_with_grain(value, 1)
                      });
    b.rule_2("integer 21..99",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check_by_range!(1, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("integer 21..99",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"-"#)?,
             integer_check_by_range!(1, 9),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1_terminal("integer (numeric)",
                      b.reg(r#"(\d{1,18})"#)?,
                      |text_match| IntegerValue::new(text_match.group(0).parse()?));
    b.rule_1_terminal("integer with thousands separator ,",
                      b.reg(r#"(\d{1,3}(,\d\d\d){1,5})"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(",", "");
                          let value: i64 = reformatted_string.parse()?;
                          IntegerValue::new(value)
                      });

    b.rule_1_terminal("100, 1_000, 1_000_000, 1_000_000_000",
                      b.reg(r#"(g?ch?[éeè]ad|mh?[íiì]lt?e|mh?ill?i[úuù]i?n|m?bh?ill?i[úuù]i?n)?"#)?,
                      |text_match| {
                          let (value, grain) = match text_match.group(1).as_ref() {
                              "céad" => (100, 2),
                              "cead" => (100, 2),
                              "cèad" => (100, 2),
                              "gcéad" => (100, 2),
                              "gcead" => (100, 2),
                              "gcèad" => (100, 2),
                              "chéad" => (100, 2),
                              "chead" => (100, 2),
                              "chèad" => (100, 2),
                              // next three will never happen
                              "gchéad" => (100, 2),
                              "gchead" => (100, 2),
                              "gchèad" => (100, 2),
                              "míle" => (1_000, 3),
                              "mile" => (1_000, 3),
                              "mìle" => (1_000, 3),
                              "mhíle" => (1_000, 3),
                              "mhile" => (1_000, 3),
                              "mhìle" => (1_000, 3),
                              "mílte" => (1_000, 3),
                              "milte" => (1_000, 3),
                              "mìlte" => (1_000, 3),
                              "mhílte" => (1_000, 3),
                              "mhilte" => (1_000, 3),
                              "mhìlte" => (1_000, 3),
                              "milliún" => (1_000_000, 6),
                              "milliun" => (1_000_000, 6),
                              "milliùn" => (1_000_000, 6),
                              "miliún" => (1_000_000, 6),
                              "miliun" => (1_000_000, 6),
                              "miliùn" => (1_000_000, 6),
                              "mhilliún" => (1_000_000, 6),
                              "mhilliun" => (1_000_000, 6),
                              "mhilliùn" => (1_000_000, 6),
                              "mhiliún" => (1_000_000, 6),
                              "mhiliun" => (1_000_000, 6),
                              "mhiliùn" => (1_000_000, 6),
                              "milliúin" => (1_000_000, 6),
                              "milliuin" => (1_000_000, 6),
                              "milliùin" => (1_000_000, 6),
                              "miliúin" => (1_000_000, 6),
                              "miliuin" => (1_000_000, 6),
                              "miliùin" => (1_000_000, 6),
                              "mhilliúin" => (1_000_000, 6),
                              "mhilliuin" => (1_000_000, 6),
                              "mhilliùin" => (1_000_000, 6),
                              "mhiliúin" => (1_000_000, 6),
                              "mhiliuin" => (1_000_000, 6),
                              "mhiliùin" => (1_000_000, 6),
                              "billiún" => (1_000_000_000, 9),
                              "billiun" => (1_000_000_000, 9),
                              "billiùn" => (1_000_000_000, 9),
                              "biliún" => (1_000_000_000, 9),
                              "biliun" => (1_000_000_000, 9),
                              "biliùn" => (1_000_000_000, 9),
                              "billiúin" => (1_000_000_000, 9),
                              "billiuin" => (1_000_000_000, 9),
                              "billiùin" => (1_000_000_000, 9),
                              "biliúin" => (1_000_000_000, 9),
                              "biliuin" => (1_000_000_000, 9),
                              "biliùin" => (1_000_000_000, 9),
                              "bhilliún" => (1_000_000_000, 9),
                              "bhilliun" => (1_000_000_000, 9),
                              "bhilliùn" => (1_000_000_000, 9),
                              "bhiliún" => (1_000_000_000, 9),
                              "bhiliun" => (1_000_000_000, 9),
                              "bhiliùn" => (1_000_000_000, 9),
                              "bhilliúin" => (1_000_000_000, 9),
                              "bhilliuin" => (1_000_000_000, 9),
                              "bhilliùin" => (1_000_000_000, 9),
                              "bhiliúin" => (1_000_000_000, 9),
                              "bhiliuin" => (1_000_000_000, 9),
                              "bhiliùin" => (1_000_000_000, 9),
                              "mbilliún" => (1_000_000_000, 9),
                              "mbilliun" => (1_000_000_000, 9),
                              "mbilliùn" => (1_000_000_000, 9),
                              "mbiliún" => (1_000_000_000, 9),
                              "mbiliun" => (1_000_000_000, 9),
                              "mbiliùn" => (1_000_000_000, 9),
                              "mbilliúin" => (1_000_000_000, 9),
                              "mbilliuin" => (1_000_000_000, 9),
                              "mbilliùin" => (1_000_000_000, 9),
                              "mbiliúin" => (1_000_000_000, 9),
                              "mbiliuin" => (1_000_000_000, 9),
                              "mbiliùin" => (1_000_000_000, 9),
                              // these will never happen
                              "mbhilliún" => (1_000_000_000, 9),
                              "mbhilliun" => (1_000_000_000, 9),
                              "mbhilliùn" => (1_000_000_000, 9),
                              "mbhiliún" => (1_000_000_000, 9),
                              "mbhiliun" => (1_000_000_000, 9),
                              "mbhiliùn" => (1_000_000_000, 9),
                              "mbhilliúin" => (1_000_000_000, 9),
                              "mbhilliuin" => (1_000_000_000, 9),
                              "mbhilliùin" => (1_000_000_000, 9),
                              "mbhiliúin" => (1_000_000_000, 9),
                              "mbhiliuin" => (1_000_000_000, 9),
                              "mbhiliùin" => (1_000_000_000, 9),
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new_with_grain(value, grain)
                      }
    );

    b.rule_2("200..900, 2_000..9_000, 2_000_000..9_000_000_000",
             integer_check_by_range!(1, 999),
             b.reg(r#"(g?ch?[éeè]ad|mh?[íiì]lt?e|mh?ill?i[úuù]i?n|m?bh?ill?i[úuù]i?n)?"#)?,
             |integer, text_match| {
                 let (value, grain) = match text_match.group(1).as_ref() {
                     "céad" => (100, 2),
                     "cead" => (100, 2),
                     "cèad" => (100, 2),
                     "gcéad" => (100, 2),
                     "gcead" => (100, 2),
                     "gcèad" => (100, 2),
                     "chéad" => (100, 2),
                     "chead" => (100, 2),
                     "chèad" => (100, 2),
                     // next three will never happen
                     "gchéad" => (100, 2),
                     "gchead" => (100, 2),
                     "gchèad" => (100, 2),
                     "míle" => (1_000, 3),
                     "mile" => (1_000, 3),
                     "mìle" => (1_000, 3),
                     "mhíle" => (1_000, 3),
                     "mhile" => (1_000, 3),
                     "mhìle" => (1_000, 3),
                     "mílte" => (1_000, 3),
                     "milte" => (1_000, 3),
                     "mìlte" => (1_000, 3),
                     "mhílte" => (1_000, 3),
                     "mhilte" => (1_000, 3),
                     "mhìlte" => (1_000, 3),
                     "milliún" => (1_000_000, 6),
                     "milliun" => (1_000_000, 6),
                     "milliùn" => (1_000_000, 6),
                     "miliún" => (1_000_000, 6),
                     "miliun" => (1_000_000, 6),
                     "miliùn" => (1_000_000, 6),
                     "mhilliún" => (1_000_000, 6),
                     "mhilliun" => (1_000_000, 6),
                     "mhilliùn" => (1_000_000, 6),
                     "mhiliún" => (1_000_000, 6),
                     "mhiliun" => (1_000_000, 6),
                     "mhiliùn" => (1_000_000, 6),
                     "milliúin" => (1_000_000, 6),
                     "milliuin" => (1_000_000, 6),
                     "milliùin" => (1_000_000, 6),
                     "miliúin" => (1_000_000, 6),
                     "miliuin" => (1_000_000, 6),
                     "miliùin" => (1_000_000, 6),
                     "mhilliúin" => (1_000_000, 6),
                     "mhilliuin" => (1_000_000, 6),
                     "mhilliùin" => (1_000_000, 6),
                     "mhiliúin" => (1_000_000, 6),
                     "mhiliuin" => (1_000_000, 6),
                     "mhiliùin" => (1_000_000, 6),
                     "billiún" => (1_000_000_000, 9),
                     "billiun" => (1_000_000_000, 9),
                     "billiùn" => (1_000_000_000, 9),
                     "biliún" => (1_000_000_000, 9),
                     "biliun" => (1_000_000_000, 9),
                     "biliùn" => (1_000_000_000, 9),
                     "billiúin" => (1_000_000_000, 9),
                     "billiuin" => (1_000_000_000, 9),
                     "billiùin" => (1_000_000_000, 9),
                     "biliúin" => (1_000_000_000, 9),
                     "biliuin" => (1_000_000_000, 9),
                     "biliùin" => (1_000_000_000, 9),
                     "bhilliún" => (1_000_000_000, 9),
                     "bhilliun" => (1_000_000_000, 9),
                     "bhilliùn" => (1_000_000_000, 9),
                     "bhiliún" => (1_000_000_000, 9),
                     "bhiliun" => (1_000_000_000, 9),
                     "bhiliùn" => (1_000_000_000, 9),
                     "bhilliúin" => (1_000_000_000, 9),
                     "bhilliuin" => (1_000_000_000, 9),
                     "bhilliùin" => (1_000_000_000, 9),
                     "bhiliúin" => (1_000_000_000, 9),
                     "bhiliuin" => (1_000_000_000, 9),
                     "bhiliùin" => (1_000_000_000, 9),
                     "mbilliún" => (1_000_000_000, 9),
                     "mbilliun" => (1_000_000_000, 9),
                     "mbilliùn" => (1_000_000_000, 9),
                     "mbiliún" => (1_000_000_000, 9),
                     "mbiliun" => (1_000_000_000, 9),
                     "mbiliùn" => (1_000_000_000, 9),
                     "mbilliúin" => (1_000_000_000, 9),
                     "mbilliuin" => (1_000_000_000, 9),
                     "mbilliùin" => (1_000_000_000, 9),
                     "mbiliúin" => (1_000_000_000, 9),
                     "mbiliuin" => (1_000_000_000, 9),
                     "mbiliùin" => (1_000_000_000, 9),
                     // these will never happen
                     "mbhilliún" => (1_000_000_000, 9),
                     "mbhilliun" => (1_000_000_000, 9),
                     "mbhilliùn" => (1_000_000_000, 9),
                     "mbhiliún" => (1_000_000_000, 9),
                     "mbhiliun" => (1_000_000_000, 9),
                     "mbhiliùn" => (1_000_000_000, 9),
                     "mbhilliúin" => (1_000_000_000, 9),
                     "mbhilliuin" => (1_000_000_000, 9),
                     "mbhilliùin" => (1_000_000_000, 9),
                     "mbhiliúin" => (1_000_000_000, 9),
                     "mbhiliuin" => (1_000_000_000, 9),
                     "mbhiliùin" => (1_000_000_000, 9),
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 // 'sé' can be the number six, or the pronoun he/it, so
                 // check that the second number is lenited (= has 'h'
                 // inserted as the second letter), otherwise it wasn't
                 // the number
                 let multiple = if multiple == 6 && text_match[1] != 'h' {
                     1
                 } else {
                     integer.value().value
                 };
                 IntegerValue::new_with_grain(multiple * value, grain)
             }
    );
    b.rule_1_terminal("dozen",
                      b.reg(r#"dosaen"#)?,
                      |_| Ok(IntegerValue {
                          value: 12,
                          grain: Some(1),
                          group: true,
                          ..IntegerValue::default()
                      })
    );
    b.rule_2("number dozen",
             integer_check_by_range!(1, 99),
             integer_check!(|integer: &IntegerValue| integer.group),
             |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     group: true,
                     ..IntegerValue::default()
                 })
             });

    b.rule_1("decimal number",
             b.reg(r#"(\d*\.\d+)"#)?,
             |text_match| {
                 let value: f64 = text_match.group(0).parse()?;
                 Ok(FloatValue {
                     value: value,
                     ..FloatValue::default()
                 })
             });
    b.rule_2("<integer> and a half",
             integer_check!(),
             b.reg(r#"go leith"#)?,
             |integer, _| FloatValue::new(integer.value().value as f64 + 0.5)
    );
    b.rule_2("<integer> and a quarter",
             integer_check!(),
             b.reg(r#"agus ceathr[úuù]"#)?,
             |integer, _| FloatValue::new(integer.value().value as f64 + 0.25)
    );
    b.rule_3("number dot number",
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             b.reg(r#"ponc"#)?,
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             |a, _, b| {
                 let value: f64 = format!("{}.{}", a.value().value, b.value().value).parse()?;
                 Ok(FloatValue {
                     value,
                     ..FloatValue::default()
                 })
             });
    b.rule_4("number dot zero... number",
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             b.reg(r#"ponc"#)?,
             b.reg(r#"(?:a )?n[áaà]id"#)?,
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             |a, _, zeros, b| {
                 let zeros_string =  std::iter::repeat("0").take(zeros.group(0).split_whitespace().count()).collect::<String>();
                 let value: f64 = format!("{}.{}{}", a.value().value, zeros_string, b.value().value).parse()?;
                 Ok(FloatValue {
                     value,
                     ..FloatValue::default()
                 })

             });
    b.rule_1_terminal("decimal with thousands separator",
                      b.reg(r#"(\d+(,\d\d\d)+\.\d+)"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(",", "");
                          let value: f64 = reformatted_string.parse()?;
                          Ok(FloatValue {
                              value: value,
                              ..FloatValue::default()
                          })
                      });
    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|m[íiì]neas"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             value: integer.value * -1,
                             prefixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         FloatValue {
                             value: float.value * -1.0,
                             prefixed: true,
                             ..float
                         }
                             .into()
                     }
                 })
             });
    b.rule_2("numbers prefix with +, positive",
             b.reg(r#"\+"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             prefixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         FloatValue {
                             prefixed: true,
                             ..float
                         }
                             .into()
                     }
                 })
             }
    );
    b.rule_2("numbers suffixes (K, M, G)",
             number_check!(|number: &NumberValue| !number.suffixed()),
             b.reg_neg_lh(r#"([kmg])"#, r#"^[^\W\$€]"#)?,
             |a, text_match| -> RuleResult<NumberValue> {
                 let multiplier = match text_match.group(0).as_ref() {
                     "k" => 1000,
                     "m" => 1000000,
                     "g" => 1000000000,
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             value: integer.value * multiplier,
                             suffixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         let product = float.value * (multiplier as f64);
                         if product.floor() == product {
                             IntegerValue {
                                 value: product as i64,
                                 suffixed: true,
                                 ..IntegerValue::default()
                             }
                                 .into()
                         } else {
                             FloatValue {
                                 value: product,
                                 suffixed: true,
                                 ..float
                             }
                                 .into()
                         }
                     }
                 })
             });
    b.rule_1_terminal("ordinals (first..10th)",
                      b.reg(r#"(ch[éeè]ad|h?aon[úuù]|t-?aon[úuù]|dara|tr[íiì][úuù]|ceathr[úuù]|c[úuù]igi[úuù]|s[éeè][úuù]|seacht[úuù]|ocht[úuù]|t-?ocht[úuù]|nao[úuù]|deichi[úuù])"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "t-aonú" => 1,
                              "t-aonu" => 1,
                              "t-aonù" => 1,
                              "taonú" => 1,
                              "taonu" => 1,
                              "taonù" => 1,
                              "aonú" => 1,
                              "aonu" => 1,
                              "aonù" => 1,
                              "haonú" => 1,
                              "haonu" => 1,
                              "haonù" => 1,
                              "chéad" => 1,
                              "chead" => 1,
                              "chèad" => 1,
                              "dara" => 2,
                              "tríú" => 3,
                              "tríu" => 3,
                              "tríù" => 3,
                              "triú" => 3,
                              "triu" => 3,
                              "triù" => 3,
                              "trìú" => 3,
                              "trìu" => 3,
                              "trìù" => 3,
                              "ceathrú" => 4,
                              "ceathru" => 4,
                              "cúigiú" => 5,
                              "cúigiu" => 5,
                              "cúigiù" => 5,
                              "cuigiú" => 5,
                              "cuigiu" => 5,
                              "cuigiù" => 5,
                              "cùigiú" => 5,
                              "cùigiu" => 5,
                              "cùigiù" => 5,
                              "séú" => 6,
                              "séu" => 6,
                              "séù" => 6,
                              "seú" => 6,
                              "seu" => 6,
                              "seù" => 6,
                              "sèú" => 6,
                              "sèu" => 6,
                              "sèù" => 6,
                              "seachtú" => 7,
                              "seachtu" => 7,
                              "seachtù" => 7,
                              "t-ochtú" => 8,
                              "tochtú" => 8,
                              "ochtú" => 8,
                              "hochtú" => 8,
                              "t-ochtu" => 8,
                              "tochtu" => 8,
                              "ochtu" => 8,
                              "hochtu" => 8,
                              "t-ochtù" => 8,
                              "tochtù" => 8,
                              "ochtù" => 8,
                              "hochtù" => 8,
                              "naoú" => 9,
                              "naou" => 9,
                              "naoù" => 9,
                              "deichiú" => 10,
                              "deichiu" => 10,
                              "deichiù" => 10,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          Ok(OrdinalValue::new(value))
                      });
    b.rule_1_terminal("ordinals (20th...90th)",
                      b.reg(r#"(fichi[úuù]|tr[íiì]ochad[úuù]|daichead[úuù]|caogad[úuù]|seascad[úuù]|seacht[óoò]d[úuù]|h?ocht[óoò]d[úuù]|t-?ocht[óoò]d[úuù]|n[óoò]chad[úuù])"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "fichiú" => 20,
                              "fichiu" => 20,
                              "fichiù" => 20,
                              "tríochadú" => 30,
                              "tríochadu" => 30,
                              "tríochadù" => 30,
                              "triochadú" => 30,
                              "triochadu" => 30,
                              "triochadù" => 30,
                              "trìochadú" => 30,
                              "trìochadu" => 30,
                              "trìochadù" => 30,
                              "daicheadú" => 40,
                              "daicheadu" => 40,
                              "daicheadù" => 40,
                              "caogadú" => 50,
                              "caogadu" => 50,
                              "caogadù" => 50,
                              "seascadú" => 60,
                              "seascadu" => 60,
                              "seascadù" => 60,
                              "seachtódú" => 70,
                              "seachtódu" => 70,
                              "seachtódù" => 70,
                              "seachtodú" => 70,
                              "seachtodu" => 70,
                              "seachtodù" => 70,
                              "seachtòdú" => 70,
                              "seachtòdu" => 70,
                              "seachtòdù" => 70,
                              "ochtódú" => 80,
                              "ochtódu" => 80,
                              "ochtódù" => 80,
                              "ochtodú" => 80,
                              "ochtodu" => 80,
                              "ochtodù" => 80,
                              "ochtòdú" => 80,
                              "ochtòdu" => 80,
                              "ochtòdù" => 80,
                              "hochtódú" => 80,
                              "hochtódu" => 80,
                              "hochtódù" => 80,
                              "hochtodú" => 80,
                              "hochtodu" => 80,
                              "hochtodù" => 80,
                              "hochtòdú" => 80,
                              "hochtòdu" => 80,
                              "hochtòdù" => 80,
                              "t-ochtódú" => 80,
                              "t-ochtódu" => 80,
                              "t-ochtódù" => 80,
                              "t-ochtodú" => 80,
                              "t-ochtodu" => 80,
                              "t-ochtodù" => 80,
                              "t-ochtòdú" => 80,
                              "t-ochtòdu" => 80,
                              "t-ochtòdù" => 80,
                              "tochtódú" => 80,
                              "tochtódu" => 80,
                              "tochtódù" => 80,
                              "tochtodú" => 80,
                              "tochtodu" => 80,
                              "tochtodù" => 80,
                              "tochtòdú" => 80,
                              "tochtòdu" => 80,
                              "tochtòdù" => 80,
                              "nóchadú" => 90,
                              "nóchadu" => 90,
                              "nóchadù" => 90,
                              "nochadú" => 90,
                              "nochadu" => 90,
                              "nochadù" => 90,
                              "nòchadú" => 90,
                              "nòchadu" => 90,
                              "nòchadù" => 90,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          Ok(OrdinalValue::new(value))
                      });
    b.rule_2("21th..99th",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 9),
             |integer, ordinal| {
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             });

    b.rule_3("21th..99th",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"-"#)?,
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 9),
             |integer, _, ordinal| {
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             });

    b.rule_1_terminal("ordinal (100, 1_000, 1_000_000)",
                      b.reg(r#"(g?ch?[éeè]ad|mh?[íiì]li|mh?ill?i[úuù]n|m?bh?ill?i[úuù]n)[úuù]"#)?,
                      |text_match| {
                          let (value, grain) = match text_match.group(1).as_ref() {
                              "céad" => (100, 2),
                              "cead" => (100, 2),
                              "cèad" => (100, 2),
                              "chéad" => (100, 2),
                              "chead" => (100, 2),
                              "chèad" => (100, 2),
                              "gcéad" => (100, 2),
                              "gcead" => (100, 2),
                              "gcèad" => (100, 2),
                              "gchéad" => (100, 2),
                              "gchead" => (100, 2),
                              "gchèad" => (100, 2),
                              "míli" => (1_000, 3),
                              "mili" => (1_000, 3),
                              "mìli" => (1_000, 3),
                              "mhíli" => (1_000, 3),
                              "mhili" => (1_000, 3),
                              "mhìli" => (1_000, 3),
                              "milliún" => (1_000_000, 6),
                              "milliun" => (1_000_000, 6),
                              "milliùn" => (1_000_000, 6),
                              "miliún" => (1_000_000, 6),
                              "miliun" => (1_000_000, 6),
                              "miliùn" => (1_000_000, 6),
                              "mhilliún" => (1_000_000, 6),
                              "mhilliun" => (1_000_000, 6),
                              "mhilliùn" => (1_000_000, 6),
                              "mhiliún" => (1_000_000, 6),
                              "mhiliun" => (1_000_000, 6),
                              "mhiliùn" => (1_000_000, 6),
                              "billiún" => (1_000_000_000, 9),
                              "billiun" => (1_000_000_000, 9),
                              "billiùn" => (1_000_000_000, 9),
                              "biliún" => (1_000_000_000, 9),
                              "biliun" => (1_000_000_000, 9),
                              "biliùn" => (1_000_000_000, 9),
                              "bhilliún" => (1_000_000_000, 9),
                              "bhilliun" => (1_000_000_000, 9),
                              "bhilliùn" => (1_000_000_000, 9),
                              "bhiliún" => (1_000_000_000, 9),
                              "bhiliun" => (1_000_000_000, 9),
                              "bhiliùn" => (1_000_000_000, 9),
                              "mbilliún" => (1_000_000_000, 9),
                              "mbilliun" => (1_000_000_000, 9),
                              "mbilliùn" => (1_000_000_000, 9),
                              "mbiliún" => (1_000_000_000, 9),
                              "mbiliun" => (1_000_000_000, 9),
                              "mbiliùn" => (1_000_000_000, 9),
                              "mbhilliún" => (1_000_000_000, 9),
                              "mbhilliun" => (1_000_000_000, 9),
                              "mbhilliùn" => (1_000_000_000, 9),
                              "mbhiliún" => (1_000_000_000, 9),
                              "mbhiliun" => (1_000_000_000, 9),
                              "mbhiliùn" => (1_000_000_000, 9),
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          Ok(OrdinalValue::new_with_grain(value, grain))
                      }
    );

    b.rule_2("ordinal (200..900, 2_000..9_000, 2_000_000..9_000_000_000)",
             integer_check_by_range!(1, 999),
             b.reg(r#"(ch?[éeè]ad|mh?[íiì]li|mh?ill?i[úuù]n|m?bh?ill?i[úuù]n)[úuù]"#)?,
             |integer, text_match| {
                 let (value, grain) = match text_match.group(1).as_ref() {
                     "céad" => (100, 2),
                     "cead" => (100, 2),
                     "cèad" => (100, 2),
                     "chéad" => (100, 2),
                     "chead" => (100, 2),
                     "chèad" => (100, 2),
                     "míli" => (1_000, 3),
                     "mili" => (1_000, 3),
                     "mìli" => (1_000, 3),
                     "mhíli" => (1_000, 3),
                     "mhili" => (1_000, 3),
                     "mhìli" => (1_000, 3),
                     "milliún" => (1_000_000, 6),
                     "milliun" => (1_000_000, 6),
                     "milliùn" => (1_000_000, 6),
                     "miliún" => (1_000_000, 6),
                     "miliun" => (1_000_000, 6),
                     "miliùn" => (1_000_000, 6),
                     "mhilliún" => (1_000_000, 6),
                     "mhilliun" => (1_000_000, 6),
                     "mhilliùn" => (1_000_000, 6),
                     "mhiliún" => (1_000_000, 6),
                     "mhiliun" => (1_000_000, 6),
                     "mhiliùn" => (1_000_000, 6),
                     "billiún" => (1_000_000_000, 9),
                     "billiun" => (1_000_000_000, 9),
                     "billiùn" => (1_000_000_000, 9),
                     "biliún" => (1_000_000_000, 9),
                     "biliun" => (1_000_000_000, 9),
                     "biliùn" => (1_000_000_000, 9),
                     "bhilliún" => (1_000_000_000, 9),
                     "bhilliun" => (1_000_000_000, 9),
                     "bhilliùn" => (1_000_000_000, 9),
                     "bhiliún" => (1_000_000_000, 9),
                     "bhiliun" => (1_000_000_000, 9),
                     "bhiliùn" => (1_000_000_000, 9),
                     "mbilliún" => (1_000_000_000, 9),
                     "mbilliun" => (1_000_000_000, 9),
                     "mbilliùn" => (1_000_000_000, 9),
                     "mbiliún" => (1_000_000_000, 9),
                     "mbiliun" => (1_000_000_000, 9),
                     "mbiliùn" => (1_000_000_000, 9),
                     "mbhilliún" => (1_000_000_000, 9),
                     "mbhilliun" => (1_000_000_000, 9),
                     "mbhilliùn" => (1_000_000_000, 9),
                     "mbhiliún" => (1_000_000_000, 9),
                     "mbhiliun" => (1_000_000_000, 9),
                     "mbhiliùn" => (1_000_000_000, 9),
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 let multiple = if multiple == 6 && text_match[1] != 'h' {
                     1
                 } else {
                     integer.value().value
                 };
                 Ok(OrdinalValue::new_with_grain(multiple * value, grain))
             }
    );

    b.rule_2("ordinal (1_1_000..9_999_999_000)",
             integer_check_by_range!(1000, 99_999_999_000),
             ordinal_check!(|ordinal: &OrdinalValue| {
            let grain = ordinal.grain.unwrap_or(0);
            grain == 2 || grain % 3 == 0
        }),
             |integer, ordinal| {
                 let grain = ordinal.value().grain.unwrap_or(0);
                 let next_grain = (grain / 3) * 3 + 3;
                 if integer.value().value % 10i64.pow(next_grain as u32) != 0 { return Err(RuleError::Invalid.into()); }
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             }
    );

    b.rule_2("ordinal (101...9_999_999)",
             integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
             ordinal_check_by_range!(1, 99),
             |integer, ordinal| {
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             }
    );
    b.rule_3("ordinal (101...9_999_999)",
             integer_check!(|integer: &IntegerValue| integer.value >= 100 || integer.value % 100 == 0),
             b.reg(r#"agus|is|[’']s"#)?,
             ordinal_check_by_range!(1, 99),
             |integer, _, ordinal| {
                 Ok(OrdinalValue::new(integer.value().value + ordinal.value().value))
             }
    );
    b.rule_1_terminal("ordinal (digits)",
                      b.reg(r#"0*(\d+) ?(st|nd|rd|th|adh|a|d|ú|u|ù)"#)?,
                      |text_match| {
                          let value: i64 = text_match.group(1).parse()?;
                          Ok(OrdinalValue::new(value))
                      });
    b.rule_2("the <ordinal>",
             b.reg(r#"an"#)?,
             ordinal_check!(),
             |_, ordinal| Ok((*ordinal.value()).prefixed()));
    Ok(())
}
