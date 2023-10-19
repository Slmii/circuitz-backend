use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum TupleVariant<T> {
	Zero,
	One(T),
	Two(T, T),
	Three(T, T, T),
	Four(T, T, T, T),
	Five(T, T, T, T, T),
	Six(T, T, T, T, T, T),
	Seven(T, T, T, T, T, T, T),
	Eight(T, T, T, T, T, T, T, T),
	Nine(T, T, T, T, T, T, T, T, T),
	Ten(T, T, T, T, T, T, T, T, T, T),
}
