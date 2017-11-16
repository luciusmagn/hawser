extern crate gcc;

fn main() {
	gcc::Build::new()
		.flag("-fPIC")
		.warnings(false)
		.file("buffer/buffer.c")
		.compile("buffer")
}
