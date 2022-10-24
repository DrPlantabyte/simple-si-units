use std::error::Error;
// NOTE: requires `regex = "1.6"` under [build-dependencies] in Cargo.toml


fn main() {
	println!("Running build.rs...");
	match increment_build() {
		Ok(_) => println!("...Success!"),
		Err(e) => {
			println!("...Failure!");
			eprintln!("{:?}", e);
			panic!();
		}
	}
}


/// Increments the build number in a toml file by reading the provided file path, parsing and
/// updating the `version = "#.#.#"` line, and then writing the new version number back to the file.
/// If not already using `\n` line endings, the line endings will be changed to `\n` as a side-
/// effect of this process.
/// # Returns
/// Returns a `Result<String, Box<dyn Error>>` where the Ok value is the new version string and the
/// Err value is whichever error prevented the operation from occurring. The file will not be
/// corrupted even in the event of an error.
fn increment_build() -> Result<String, Box<dyn Error>> {
	use std::io::*;
	use std::fs::File;
	use std::path::Path;
	use std::fs;
	use regex::Regex;
	let proj_root = Path::new(env!("CARGO_MANIFEST_DIR"));
	let cargo_toml = proj_root.join("Cargo.toml");
	let pattern = Regex::new("^\\s*version\\s*=\\s*\".*\"\\s*$")?;
	let mut new_lines: Vec<String> = Vec::with_capacity(1024);
	let mut new_version = String::from("?");
	let mut do_once = true;
	{ // read and process
		let f = File::open(cargo_toml.as_path())?;
		let lines = BufReader::new(f).lines();
		for r_line in lines {
			let line_string = r_line?;
			let line_str = line_string.as_str();
			if do_once && pattern.is_match(line_str) {
				do_once = false;
				let tmp = line_str.split("=").collect::<Vec<&str>>()[1].replace("\"", "");
				let old_version = tmp.trim();
				print!("Incremented version {}", old_version);
				let mut ver_vec = old_version.split(".").collect::<Vec<&str>>();
				let b_num = ver_vec.pop().ok_or("empty version string")?.parse::<i64>()?;
				let inc_bnum_str = format!("{}", b_num + 1);
				ver_vec.push(inc_bnum_str.as_str());
				new_version = ver_vec.join(".");
				println!(" to version {}", new_version);
				new_lines.push(format!("version = \"{}\"", new_version));
			} else {
				new_lines.push(line_string);
			}
		}
	}
	{ // write
		fs::write(cargo_toml, new_lines.join("\n"))?;
	}
	Ok(new_version)
}
