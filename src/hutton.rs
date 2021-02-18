extern crate hutton_rust;
#[macro_use]
extern crate sciter;
use sciter::{ Value, types::{ BOOL, VALUE } };
#[no_mangle]
pub extern "system" fn SciterLibraryInit(api: &'static sciter::ISciterAPI, exported: &mut VALUE) -> BOOL {
	sciter::set_host_api(api);
	let ext_api = vmap! { "encrypt" => encrypt };
	ext_api.pack_to(exported);
	true as BOOL
}
pub fn encrypt(args: &[Value]) -> Value {
	let input = args[0].to_string().replace("\"", "");
	let password = args[1].to_string().replace("\"", "");
	let key = args[2].to_string().replace("\"", "");
	let decrypt = args[3].to_bool().unwrap();
	let ciphertext = hutton_rust::encrypt(&input, &password, &key, decrypt);
	Value::from(ciphertext)
}