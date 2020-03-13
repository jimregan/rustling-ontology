use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain};


pub fn rules_celebration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    // Included as Holiday but otherwise nth cycles not supported
    b.rule_2("nth sunday of advent",
             ordinal_check!(),
             b.reg(r#"n?dh?omhnach den? aidbhint"#)?,
             |ordinal, _| {
                 Ok(helpers::day_of_week(Weekday::Sun)?
                     .the_nth_after(-(4 - ordinal.value().value) - 1, &helpers::month_day(12, 25)?)?
                     .form(Form::Celebration))
             }
    );

    b.rule_1_terminal("christmas",
                      b.reg(r#"(?:l[áaà] )nollai?g"#)?,
                      |_| Ok(helpers::month_day(12, 25)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("christmas eve",
                      b.reg(r#"h?o[íiì]che nollag"#)?,
                      |_| Ok(helpers::month_day(12, 24)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("new year's eve",
                      b.reg(r#"h?o[íiì]che (?:chinn bh?liana|na seanbhliana|na coda m[óoò]ire)"#)?,
                      |_| Ok(helpers::month_day(12, 31)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("new year's day",
                      b.reg(r#"l[áaà] (?:caille|na bliana (?:nua|[úuù]ire))"#)?,
                      |_| Ok(helpers::month_day(1, 1)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("valentine's day",
                      b.reg(r#"l[áaà] (?:fh[éeè]ile |san |[’']?le )?vailint[íiì]n"#)?,
                      |_| Ok(helpers::month_day(2, 14)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("Palm sunday",
                      b.reg(r#"n?dh?omhnach na (?:pailme|slat)"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, -7, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Thursday",
                      b.reg(r#"n?dh?[éeè]ardaoin (mand[áaà]la|chorp chr[íiì]ost|naofa|beannaithe)"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, -3, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Friday",
                      b.reg(r#"h?aoine (an )?ch[éeè]asta"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, -2, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("Holy Saturday",
                      b.reg(r#"t?sh?atharn naofa"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, -1, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("Easter",
                      b.reg(r#"n?dh?omhnach c[áaà]sca"#)?,
                      |_| Ok(helpers::easter()?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("Easter Monday",
                      b.reg(r#"luan c[áaà]sca"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, 1, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("Ascension",
                      b.reg(r#"n?dh?[éeè]ardaoin deascabh[áaà]la"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, 39, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("Pentecost",
                      b.reg(r#"(?:an )?g?ch?inc[íiì]s"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, 49, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("Father's Day",
                      b.reg(r#"l[áaà] na n-?aithreacha"#)?,
                      |_| {
                          let sundays_of_june = helpers::month(6)?.intersect(&helpers::day_of_week(Weekday::Sun)?)?;
                          let second_week_of_june = helpers::cycle_nth_after(Grain::Week, 2, &helpers::month_day(6, 1)?)?;
                          Ok(sundays_of_june.intersect(&second_week_of_june)? // third sunday of June
                              .form(Form::Celebration))
                      }
    );
    b.rule_1_terminal("Mother's Day",
                      b.reg(r#"l[áaà] na m[áaà]ithreacha"#)?,
                      |_| Ok(helpers::cycle_nth_after(Grain::Day, -21, &helpers::easter()?)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("halloween day",
                      b.reg(r#"h?o[íiì]che shamhna"#)?,
                      |_| Ok(helpers::month_day(10, 31)?
                              .form(Form::Celebration))
    );
    b.rule_1_terminal("St Patrick's Day",
                      b.reg(r#"l[áaà] (?:fh[éeè]ile |[’']?le )?ph?[áaà]draig"#)?,
                      |_| Ok(helpers::month_day(3, 17)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("St Bridget's Day",
                      b.reg(r#"l[áaà] (?:fh[éeè]ile |[’']?le )?bhr[íiì]de"#)?,
                      |_| Ok(helpers::month_day(2, 1)?
                          .form(Form::Celebration))
    );
    b.rule_1_terminal("Epiphany",
                      b.reg(r#"(l[áaà] |an )?nollaig (bheag|na mban)|eipeaf[áaà]ine|l[áaà] chinn an d[áaà] l[áaà] dh[éeè]ag|f[éeè]ile na dtr[íiì] r[íiì]the"#)?,
                      |_| Ok(helpers::month_day(1, 6)?
                          .form(Form::Celebration))
    );

    Ok(())
}