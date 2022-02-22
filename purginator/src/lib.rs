use parcel_css::stylesheet::StyleSheet;
use purger::traits::Purger;
pub mod purger;
pub mod stylesheet;

pub fn purge<'a>(mut stylesheet: StyleSheet<'a>, purgers: &[&'a dyn Purger<'a>]) -> StyleSheet<'a> {
    for purger_impl in purgers.iter() {
        purger_impl.purge(&mut stylesheet);
    }

    stylesheet
}
