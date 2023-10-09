use candid::Principal;

pub fn whitelist() -> Vec<Principal> {
	[
		Principal::from_text("tzq3i-wbpyf-lg3xw-whs7v-znvez-btcmr-dcmho-moqam-qmjhf-xnzgt-dae".to_string()).unwrap(),
	].to_vec()
}
