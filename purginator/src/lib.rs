use parcel_css::stylesheet::StyleSheet;
use purger::traits::Purger;
pub mod purger;
pub mod stylesheet;

pub fn purge(mut stylesheet: StyleSheet, purgers: &[&dyn Purger]) -> StyleSheet {
    for purger_impl in purgers.iter() {
        purger_impl.purge(&mut stylesheet);
    }

    stylesheet
}
