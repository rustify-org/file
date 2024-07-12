pub mod fs_example {

    pub fn open_file() -> std::io::Result<()> {
        let f = std::fs::File::open("Cargo.toml")?;
        println!("{:?}", f);
        Ok(())
    }

    pub fn create_file() -> std::io::Result<()> {
        let mut f = std::fs::File::create("foo.txt")?;
        println!("{:?}", f);
        // f.write_all(b"hello world!")?;
        // f.write(b"hello world!")?;
        Ok(())
    }
    pub fn delete_file() -> std::io::Result<()> {
        let mut f = std::fs::remove_file("foo.txt")?;
        Ok(())
    }
    pub mod read_file {
        use std::io::{BufRead, BufReader, Read, Write};

        pub fn m1() -> std::io::Result<()> {
            // let mut f = std::fs::File::create("foo.txt")?;
            // f.write_all(b"12312\n")?;
            // f.write_all(b"12312\n")?;
            // f.write_all(b"12312\n")?;
            // f.write_all("返回\n".as_bytes())?;
            // f.write_all(b"12312\n")?;
            // f.write_all(b"12312\n")?;
            let mut f = std::fs::File::open("foo.txt")?;
            let mut data = Vec::new();
            f.read_to_end(&mut data)?;
            println!("{:?}", data);
            Ok(())
        }
        pub fn m2() -> std::io::Result<()> {
            let mut f = std::fs::File::open("foo.txt")?;
            let mut content = String::new();
            f.read_to_string(&mut content)?;
            println!("{:?}", content);
            Ok(())
        }
        pub fn m3() -> std::io::Result<()> {
            let f = std::fs::File::open("foo.txt")?;
            let reader = BufReader::new(f);
            for (index, line) in reader.lines().enumerate() {
                let line = line?;
                println!("{}: {}", index + 1, line);
            }
            Ok(())
        }
    }
    pub mod write_file {
        use std::{env, io::Write as _};

        pub fn w1() -> std::io::Result<()> {
            let temp_dir = env::temp_dir();
            println!("temp_dir: {:?}", temp_dir);
            let temp_file = temp_dir.join("temp_file");
            let mut file = std::fs::File::create(temp_file)?;
            writeln!(&mut file, "hello world!\n")?;
            Ok(())
        }
    }
}
