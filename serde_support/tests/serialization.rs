mod teststructs;
use cbor_enhanced_serde::ser::to_bytes;
use teststructs::*;

fn print_as_hex(input: &[u8]) {
    let string = format!("{:02x?}", input);
    let string = string.replace(", ", "").replace("[", "").replace("]", "");
    println!("{}", string);
}
#[test]
fn test_list() {
    let result = to_bytes(&vec![1i8, 2i8, 3i8, 4i8, 5i8]).unwrap();
    print_as_hex(&result);
    assert_eq!(result, vec![]);
}
#[test]
fn test_list_in_list() {
    let list_in_list = ListInList::default();
    let result = to_bytes(&list_in_list).unwrap();
    print_as_hex(&result);
    assert_eq!(result, vec![]);
}
