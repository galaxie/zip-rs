use std::io;
use std::io::prelude::*;
use std::ascii::AsciiExt;
use types::ZipFileData;
use result::ZipResult;
use spec;
use util;
use util::WriteIntExt;

pub fn write_local_file_header<T: Write>(writer: &mut T, file: &ZipFileData) -> ZipResult<()>
{
    try!(writer.write_le_u32(spec::LOCAL_FILE_HEADER_SIGNATURE));
    try!(writer.write_le_u16(20));
    let flag = if !file.file_name.is_ascii() { 1u16 << 11 } else { 0 };
    try!(writer.write_le_u16(flag));
    try!(writer.write_le_u16(file.compression_method as u16));
    try!(writer.write_le_u16(util::tm_to_msdos_time(file.last_modified_time)));
    try!(writer.write_le_u16(util::tm_to_msdos_date(file.last_modified_time)));
    try!(writer.write_le_u32(file.crc32));
    try!(writer.write_le_u32(file.compressed_size as u32));
    try!(writer.write_le_u32(file.uncompressed_size as u32));
    try!(writer.write_le_u16(file.file_name.as_bytes().len() as u16));
    let extra_field = try!(build_extra_field(file));
    try!(writer.write_le_u16(extra_field.len() as u16));
    try!(writer.write_all(file.file_name.as_bytes()));
    try!(writer.write_all(extra_field.as_slice()));

    Ok(())
}

pub fn update_local_file_header<T: Write+io::Seek>(writer: &mut T, file: &ZipFileData) -> ZipResult<()>
{
    static CRC32_OFFSET : u64 = 14;
    try!(writer.seek(io::SeekFrom::Start(file.header_start + CRC32_OFFSET)));
    try!(writer.write_le_u32(file.crc32));
    try!(writer.write_le_u32(file.compressed_size as u32));
    try!(writer.write_le_u32(file.uncompressed_size as u32));
    Ok(())
}

pub fn write_central_directory_header<T: Write>(writer: &mut T, file: &ZipFileData) -> ZipResult<()>
{
    try!(writer.write_le_u32(spec::CENTRAL_DIRECTORY_HEADER_SIGNATURE));
    try!(writer.write_le_u16(0x14FF));
    try!(writer.write_le_u16(20));
    let flag = if !file.file_name.is_ascii() { 1u16 << 11 } else { 0 };
    try!(writer.write_le_u16(flag));
    try!(writer.write_le_u16(file.compression_method as u16));
    try!(writer.write_le_u16(util::tm_to_msdos_time(file.last_modified_time)));
    try!(writer.write_le_u16(util::tm_to_msdos_date(file.last_modified_time)));
    try!(writer.write_le_u32(file.crc32));
    try!(writer.write_le_u32(file.compressed_size as u32));
    try!(writer.write_le_u32(file.uncompressed_size as u32));
    try!(writer.write_le_u16(file.file_name.as_bytes().len() as u16));
    let extra_field = try!(build_extra_field(file));
    try!(writer.write_le_u16(extra_field.len() as u16));
    try!(writer.write_le_u16(0));
    try!(writer.write_le_u16(0));
    try!(writer.write_le_u16(0));
    try!(writer.write_le_u32(0));
    try!(writer.write_le_u32(file.header_start as u32));
    try!(writer.write_all(file.file_name.as_bytes()));
    try!(writer.write_all(extra_field.as_slice()));

    Ok(())
}

fn build_extra_field(_file: &ZipFileData) -> ZipResult<Vec<u8>>
{
    let writer = Vec::new();
    // Future work
    Ok(writer)
}
