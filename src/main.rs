use example::fs_example::{
    create_file, delete_file,
    read_file::{m1, m2, m3},
    write_file::w1,
};

mod example;

fn main() -> std::io::Result<()> {
    w1()
}
