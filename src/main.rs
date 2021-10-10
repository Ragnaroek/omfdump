use std::io::Read;
use std::fs::File;

use clap::{Arg, App, ArgMatches};

const FLAG_HEADERS : &str = "headers";
const OPTION_RECORDS : &str = "records";
const PARAM_OBJFILE : &str = "OBJFILE";

fn main() -> std::io::Result<()> {
    let args = args();

    let mut file = File::open(args.value_of(PARAM_OBJFILE).expect("file name"))?;
    let mut content = vec![];
    file.read_to_end(&mut content)?;

    let records = parse_records(&content);    

    let mut print_default = true;
    if args.is_present(FLAG_HEADERS) {
        print_default = false;
        print_headers(&records);
    }

    if args.is_present(OPTION_RECORDS) {
        print_default = false;
        let values : Vec<&str> = args.values_of(OPTION_RECORDS).unwrap().collect();
        let types = to_record_types(values).expect("illegal record type");
        let recs_to_print = filter_records(&records, &types);
        print_records(recs_to_print, &content);
    }

    if print_default {
        print_records(filter_records(&records, &vec![RecordType::THEADR, RecordType::COMENT]), &content);
    }

    Ok(())
}

fn args() -> ArgMatches {
 App::new("omfdump")
    .version("0.1")
    .arg(Arg::new(PARAM_OBJFILE)
        .about("obj file to dump")
        .required(true)
        .index(1))
    .arg(Arg::new(FLAG_HEADERS)
        .short('h')
        .long(FLAG_HEADERS)
        .about("print record header information"))
    .arg(Arg::new(OPTION_RECORDS)
        .short('r')
        .long(OPTION_RECORDS)
        .value_delimiter(',') 
        .about("print record types listed (seperated by ,), use * for all records")
    )
    .get_matches()
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum RecordType {
    UNKNWN,
    THEADR,
    LHEADR,
    COMENT,
    MODEND,
    EXTDEF,
    PUBDEF,
    LINNUM,
    LNAMES,
    SEGDEF,
    GRPDEF,
    FIXUPP,
    LEDATA,
    LIDATA,
    COMDEF,
    BAKPAT,
    LEXTDEF,
    LPUBDEF,
    LCOMDEF,
    CEXTDEF,
    COMDAT,
    LINSYM,
    ALIAS,
    NBKPAT,
    LLNAMES,
    VERNUM,
    VENDEXT,
    LIBHEAD,
    LIBEND,
}

static ALL_TYPES : [RecordType; 29] = [
    RecordType::UNKNWN,
RecordType::THEADR,
RecordType::LHEADR,
RecordType::COMENT,
RecordType::MODEND,
RecordType::EXTDEF,
RecordType::PUBDEF,
RecordType::LINNUM,
RecordType::LNAMES,
RecordType::SEGDEF,
RecordType::GRPDEF,
RecordType::FIXUPP,
RecordType::LEDATA,
RecordType::LIDATA,
RecordType::COMDEF,
RecordType::BAKPAT,
RecordType::LEXTDEF,
RecordType::LPUBDEF,
RecordType::LCOMDEF,
RecordType::CEXTDEF,
RecordType::COMDAT,
RecordType::LINSYM,
RecordType::ALIAS,
RecordType::NBKPAT,
RecordType::LLNAMES,
RecordType::VERNUM,
RecordType::VENDEXT,
RecordType::LIBHEAD,
RecordType::LIBEND
];

impl RecordType {
    fn from_byte(u: u8) -> RecordType {
        match u {
            0x80 => RecordType::THEADR,
            0x82 => RecordType::LHEADR,
            0x88 => RecordType::COMENT,
            0x8A | 0x8B => RecordType::MODEND,
            0x8C => RecordType::EXTDEF,
            0x90 | 0x91 => RecordType::PUBDEF,
            0x94 | 0x95 => RecordType::LINNUM,
            0x96 => RecordType::LNAMES,
            0x98 | 0x99 => RecordType::SEGDEF,
            0x9A => RecordType::GRPDEF,
            0x9C | 0x9D => RecordType::FIXUPP,
            0xA0 | 0xA1 => RecordType::LEDATA,
            0xA2 | 0xA3 => RecordType::LIDATA,
            0xB0 => RecordType::COMDEF,
            0xB2 | 0xB3 => RecordType::BAKPAT,
            0xB4 => RecordType::LEXTDEF,
            0xB6 | 0xB7 => RecordType::LPUBDEF,
            0xB8 => RecordType::LCOMDEF,
            0xBC => RecordType::CEXTDEF,
            0xC2 | 0xC3 => RecordType::COMDAT,
            0xC4 | 0xC5 => RecordType::LINSYM,
            0xC6 => RecordType::ALIAS,
            0xC8 | 0xC9 => RecordType::NBKPAT,
            0xCA => RecordType::LLNAMES,
            0xCC => RecordType::VERNUM,
            0xCE => RecordType::VENDEXT,
            0xF0 => RecordType::LIBHEAD,
            0xF1 => RecordType::LIBEND,
            _ => RecordType::UNKNWN, 
        }
    }

    fn to_string(&self) -> &str {
        match self {
            RecordType::THEADR => "THEADR",
            RecordType::LHEADR => "LHEADR",
            RecordType::COMENT => "COMENT",
            RecordType::MODEND => "MODEND",
            RecordType::EXTDEF => "EXTDEF",
            RecordType::PUBDEF => "PUBDEF",
            RecordType::LINNUM => "LINNUM",
            RecordType::LNAMES => "LNAMES",
            RecordType::SEGDEF => "SEGDEF",
            RecordType::GRPDEF => "GRPDEF",
            RecordType::FIXUPP => "FIXUPP",
            RecordType::LEDATA => "LEDATA",
            RecordType::LIDATA => "LIDATA",
            RecordType::COMDEF => "COMDEF",
            RecordType::BAKPAT => "BAKPAT",
            RecordType::LEXTDEF => "LEXTDEF",
            RecordType::LPUBDEF => "LPUBDEF",
            RecordType::LCOMDEF => "LCOMDEF",
            RecordType::CEXTDEF => "CEXTDEF",
            RecordType::COMDAT => "COMDAT",
            RecordType::LINSYM => "LINSYM",
            RecordType::ALIAS => "ALIAS",
            RecordType::NBKPAT => "NBKPAT",
            RecordType::LLNAMES => "LLNAMES",
            RecordType::VERNUM => "VERNUM",
            RecordType::VENDEXT => "VENDEXT",
            RecordType::LIBHEAD => "LIBHEAD",
            RecordType::LIBEND => "LIBEND",
            _ => "UNKNWN",
        }
    }

    fn from_string(str: &str) -> RecordType {
        match str {
        "THEADR" => RecordType::THEADR,
        "LHEADR" => RecordType::LHEADR,
        "COMENT" => RecordType::COMENT,
        "MODEND" => RecordType::MODEND,
        "EXTDEF" => RecordType::EXTDEF,
        "PUBDEF" => RecordType::PUBDEF,
        "LINNUM" => RecordType::LINNUM,
        "LNAMES" => RecordType::LNAMES,
        "SEGDEF" => RecordType::SEGDEF,
        "GRPDEF" => RecordType::GRPDEF,
        "FIXUPP" => RecordType::FIXUPP,
        "LEDATA" => RecordType::LEDATA,
        "LIDATA" => RecordType::LIDATA,
        "COMDEF" => RecordType::COMDEF,
        "BAKPAT" => RecordType::BAKPAT,
        "LEXTDEF" => RecordType::LEXTDEF,
        "LPUBDEF" => RecordType::LPUBDEF,
        "LCOMDEF" => RecordType::LCOMDEF,
        "CEXTDEF" => RecordType::CEXTDEF,
        "COMDAT" => RecordType::COMDAT,
        "LINSYM" => RecordType::LINSYM,
        "ALIAS" => RecordType::ALIAS,
        "NBKPAT" => RecordType::NBKPAT,
        "LLNAMES" => RecordType::LLNAMES,
        "VERNUM" => RecordType::VERNUM,
        "VENDEXT" => RecordType::VENDEXT,
        "LIBHEAD" => RecordType::LIBHEAD,
        "LIBEND" => RecordType::LIBEND,
        _ => RecordType::UNKNWN,
        }
    }
}

struct Record {
    record_type: RecordType,
    even: bool,
    //inclusive interval, start of the record data
    start: usize,
    //inclusive interval, end of the record data (excluding the checksum)
    end: usize,
    //end-start, kept here for convenience (does not include the checksum)
    len: usize, 
}

fn parse_records(content: &Vec<u8>) -> Vec<Record> {
    let mut result = vec![];
    let mut ix = 0;
    while ix < content.len() {
        let record_type = content[ix];
        let record_len = ((content[ix+2] as usize) << 8) | content[ix+1] as usize; //-1 for excluding the checksum

        if record_len == 0 {
            break;
        }

        result.push(Record{
            record_type: RecordType::from_byte(record_type), 
            even: record_type % 2 == 0,
            start: ix + 3, 
            end: ix + 3 + (record_len-1), 
            len: (record_len-1)});
        ix += record_len + 3;
    };
    result
}

fn to_record_types(names: Vec<&str>) -> Result<Vec<RecordType>, String> {
    let mut result = Vec::with_capacity(names.len());
    for str in names {

        if str == "*" {
            for rec_type in &ALL_TYPES {
                if !result.contains(rec_type) {
                    result.push(*rec_type);
                } 
            }
            break;
        }

        let rec_type = RecordType::from_string(str);
        if rec_type == RecordType::UNKNWN {
            return Err(format!("Unknown type: {}", str));
        }
        if !result.contains(&rec_type) {
            result.push(rec_type);
        }
    }
    return Ok(result)
}

fn filter_records<'a>(records: &'a Vec<Record>, types: &'a Vec<RecordType>) -> Vec<&'a Record> {
    records.iter().filter(|rec| types.contains(&rec.record_type)).collect()
}

fn print_headers(records : &Vec<Record>) {
    println!("Idx    Type Size");
    for (ix, record) in records.iter().enumerate() {
        println!("{:>3} {:>7} {:04x}", ix, record.record_type.to_string(), record.len);
    }
}

fn print_records(records : Vec<&Record>, bytes: &Vec<u8>) {

    for record in records {
        println!("{}:", record.record_type.to_string());
        match record.record_type {    
            RecordType::THEADR => print_record_theadr(record, bytes),
            RecordType::COMENT => print_record_coment(record, bytes),
            RecordType::LNAMES => print_record_lnames(record, bytes),
            RecordType::SEGDEF => print_record_segdef(record, bytes),
            RecordType::PUBDEF => print_record_pubdef(record, bytes),
            RecordType::LEDATA => print_record_ledata(record, bytes),
            RecordType::UNKNWN => (),
            _ => println!("not implemented yet"), 
        }
        println!();
    }
}

//Record prints
fn print_record_theadr(record: &Record, bytes: &Vec<u8>) {
    let str_size = bytes[record.start] as usize;
    let str_bytes = &bytes[record.start+1..record.start+1+str_size];
    let str = String::from_utf8_lossy(str_bytes);
    println!("{:>7} {}", "Name", str);
}

fn print_record_coment(record: &Record, bytes: &Vec<u8>) {
    let cmt_type = bytes[record.start];
    let cmt_class = bytes[record.start+1];
    let cmt_str = &bytes[record.start+2..(record.start+(record.len-3))];

    println!("{:>12} {}", "NP", cmt_type & 0x80);
    println!("{:>12} {}", "NL", cmt_type & 0x40);
    println!("{:>12} {:x}", "Class", cmt_class);
    println!("{:>12} {}", "Commentary", String::from_utf8_lossy(cmt_str));
}

fn print_record_lnames(record: &Record, bytes: &Vec<u8>) {
    let mut offset = record.start;
    let mut first = true;

    while offset < record.end {
        let name_len = bytes[offset] as usize;
        let name_bytes = &bytes[offset+1..(offset+1+name_len)];
        let name = String::from_utf8_lossy(name_bytes);
        
        println!("{:>8} {}", if first {"Names"} else {""}, name);
        first = false;

        offset += 1 + name_len;
    }
}

fn print_record_segdef(record: &Record, bytes: &Vec<u8>) {
    let factor = if record.even {1} else {2};
    let mut offset = record.start;
    let attributes = bytes[offset];
    let a = (attributes & 0xE0) >> 5;
    let c = (attributes & 0x1C) >> 2;
    let b = (attributes & 0x02) >> 1;
    let p = attributes & 0x01;

    println!("{:>19} alignment:   {} ({})", "Attributes", a, segdef_alignment(a));
    println!("{:>19} combination: {} ({})", "", c, segdef_combination(c));
    println!("{:>19} big:         {}", "", b);
    println!("{:>19} p:           {}", "", p);
    offset += 1;
    if a == 0 {
        let frame_number = bytes[offset] as u16 | (bytes[offset+1] as u16) << 8;
        offset += 2;
        let frame_offset = bytes[offset];
        offset += 1;
        println!("{:>19} {}", "Frame Number", frame_number);
        println!("{:>19} {}", "Offset", frame_offset);
    }

    let seg_len = le_value(offset, 2 * factor, bytes);
    offset += 2 * factor;
    let seg_name_ix = le_value(offset, 1*factor, bytes);
    offset += 1;
    let class_name_ix = le_value(offset, 1*factor, bytes);
    offset += 1;
    let overlay_name_ix = le_value(offset, 1*factor, bytes);

    println!("{:>19} {}", "Length", seg_len);
    println!("{:>19} {}", "Seg Name Index", seg_name_ix);
    println!("{:>19} {}", "Class Name Index", class_name_ix);
    println!("{:>19} {}", "Overlay Name Index", overlay_name_ix);
}

fn segdef_alignment(a: u8) -> &'static str {
    match a {
        0 => "Absolute segment",
        1 => "Relocatable, byte aligned",
        2 => "Relocatable, word aligned",
        3 => "Relocatable, paragraph aligned",
        4 => "Relocatable, page aligned",
        5 => "Reloctable, double word aligned",
        6 => "Not supported",
        _ => "Not defined",
    }
}

fn segdef_combination(c: u8) -> &'static str {
    match c {
        0 => "Private",
        1 => "Reserved",
        2 | 4 | 7  => "Public",
        3 => "Reserved",
        5 => "Stack",
        6 => "Common",
        _ => "Not defined",
    }
}

fn print_record_pubdef(record: &Record, bytes: &Vec<u8>) {
    let factor = if record.even {1} else {2};
    let mut offset = record.start;

    let bg_ix = le_value(offset, factor, bytes);
    offset += factor;
    let bs_ix = le_value(offset, factor, bytes);
    offset += factor;

    println!("{:>19} {}", "Base Group Index", bg_ix);
    println!("{:>19} {}", "Base Segment Index", bs_ix);

    if bs_ix == 0 {
        let base_frame = le_value(offset, 2, bytes);
        offset += 2;
        println!("{:>19} {}", "Base Frame", base_frame);
    }

    let mut first = false;
    while offset < record.end {
        let name_len = bytes[offset] as usize;
        offset += 1;
        let name_bytes = &bytes[offset..offset+name_len];
        let name = String::from_utf8_lossy(name_bytes);
        offset += name_len;
        let public_offset = le_value(offset, 2 * factor, bytes);
        offset += 2 * factor;
        
        let type_index = le_value(offset, factor, bytes);
        offset += factor;

        println!("{:>19} name:          {}", if first {"Public Names"} else {""}, name);
        println!("{:>19} public offset: {}", "", public_offset);
        println!("{:>19} type index:    {}", "", type_index);
        first = false;
    }
}

fn print_record_ledata(record: &Record, bytes: &Vec<u8>) {
    let factor = if record.even {1} else {2};
    let mut offset = record.start;

    let seg_ix = le_value(offset, factor, bytes);
    offset += factor;
    let data_offset = le_value(offset, 2 * factor, bytes);
    offset += 2 * factor;

    println!("{:>19} {}", "Segment Index", seg_ix);
    println!("{:>19} {}", "Data Offset", data_offset);
    println!("{:>19} {}", "Data", data_excerpt(offset, record.end - offset, bytes));
}

//helper

fn le_value(offset: usize, len: usize, bytes: &Vec<u8>) -> u32 {
    let mut r = 0;
    for i in 0..len {
        let v = bytes[offset+i] as u32;
        r |= v << (i * 8);
    }
    r
}

fn data_excerpt(offset: usize, len: usize, bytes: &Vec<u8>) -> String {
    let mut s : String = "[".to_string();
    let num_ex = usize::min(len, 5);
    for i in 0..num_ex {
        s = format!("{}{:#04x}", &s, bytes[offset+i]);
        if i != (num_ex-1) {
            s = format!("{},", &s);
        }
    }
    if len > num_ex {
        s = format!("{},...", &s);
    }
    format!("{}] {} bytes", &s, len)    
}
