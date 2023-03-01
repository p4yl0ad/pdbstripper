// let re = Regex::new(r"C:\\.*\.pdb").unwrap();
// let text = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00C:\\aaa.pdb\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
//
// let mat = re.find(text).unwrap();
// println!("START: {}, END: {}", mat.start(), mat.end());
//
// println!("{}", (mat.end() - mat.start()));