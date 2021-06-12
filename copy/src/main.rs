#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use std::fs;
use std::io;
use std::path::Path;

/// Copy the existing directory `src` to the target path `dst`.
fn copy_dir_to(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.is_dir() {
        fs::create_dir(dst)?;
    }

    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
    }

    Ok(())
}

#[cfg(unix)]
use std::os::unix::fs::symlink;

/// Stub implementation of `symlink` for platforms that don't provide it.
#[cfg(not(unix))]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, _dst: Q) -> std::io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Other,
                       format!("can't copy symbolic link: {}",
                               src.as_ref().display())))
}

/// Copy whatever is at `src` to the target path `dst`.
fn copy_to(src: &Path, src_type: &fs::FileType, dst: &Path) -> io::Result<()> {
    if src_type.is_file() {
        fs::copy(src, dst)?;
    } else if src_type.is_dir() {
        copy_dir_to(src, dst)?;
    } else if src_type.is_symlink() {
        let target = src.read_link()?;
        symlink(target, dst)?;
    } else {
        return Err(io::Error::new(io::ErrorKind::Other,
                                  format!("don't know how to copy: {}",
                                          src.display())));
    }
    Ok(())
}

fn copy_into<P, Q>(source: P, destination: Q) -> io::Result<()>
    where P: AsRef<Path>,
          Q: AsRef<Path>
{
    let src = source.as_ref();
    let dst = destination.as_ref();

    match src.file_name() {
        None => {
            return Err(io::Error::new(io::ErrorKind::Other,
                                      format!("can't copy nameless directory: {}",
                                              src.display())));
        }
        Some(src_name) => {
            let md = src.metadata()?;
            copy_to(src, &md.file_type(), &dst.join(src_name))?;
        }
    }
    Ok(())
}

fn dwim_copy<P, Q>(source: P, destination: Q) -> io::Result<()>
    where P: AsRef<Path>,
          Q: AsRef<Path>
{
    let src = source.as_ref();
    let dst = destination.as_ref();

    if dst.is_dir() {
        copy_into(src, dst)
    } else {
        let md = src.metadata()?;
        copy_to(src, &md.file_type(), dst)
    }
}

fn copy_main() -> io::Result<()> {
    let args = std::env::args_os().collect::<Vec<_>>();
    if args.len() < 3 {
        println!("usage: copy FILE... DESTINATION");
    } else if args.len() == 3 {
        dwim_copy(&args[1], &args[2])?;
    } else {
        let dst = Path::new(&args[args.len() - 1]);
        if !dst.is_dir() {
            return Err(io::Error::new(io::ErrorKind::Other,
                                      format!("target '{}' is not a directory",
                                              dst.display())));
        }
        for i in 1 .. args.len() - 1 {
            copy_into(&args[i], dst)?;
        }
    }
    Ok(())
}

fn main() {
    use std::io::Write;

    if let Err(err) = copy_main() {
        writeln!(io::stderr(), "error: {}", err).unwrap();
    }
}
