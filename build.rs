fn main() {
  let path = "./build";
  let lib = "whoisparser";

  println!("cargo:rustc-link-search=native={}", path);
  println!("cargo:rustc-link-lib=static={}", lib);
}
