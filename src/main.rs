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

    let mut flag_set = false;
    if args.is_present(FLAG_HEADERS) {
        flag_set = true;
        print_headers(&records);
    }

    if !flag_set {
        print_info(&records, &content);
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
        .takes_value(true)
        .multiple_occurrences(true)
        .about("print record types listed (seperated by ,)")
    )
    .get_matches()
}


#[derive(PartialEq, Eq)]
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
}

struct Record {
    record_type: RecordType,
    //inclusive interval
    start: usize,
    end: usize,
    //end-start, kept here for convenience
    len: usize, 
}

fn parse_records(content: &Vec<u8>) -> Vec<Record> {
    let mut result = vec![];
    let mut ix = 0;
    while ix < content.len() {
        let record_type = content[ix];
        let record_len = ((content[ix+2] as usize) << 8) | content[ix+1] as usize;

        if record_len == 0 {
            break;
        }

        result.push(Record{record_type: RecordType::from_byte(record_type), start: ix + 3, end: ix + 3 + record_len, len: record_len});
        ix += record_len + 3;
    };
    result
}

fn print_headers(records : &Vec<Record>) {
    println!("Idx    Type Size");
    for (ix, record) in records.iter().enumerate() {
        println!("{:>3} {:>7} {:04x}", ix, record.record_type.to_string(), record.len);
    }
}

fn print_info(records : &Vec<Record>, bytes: &Vec<u8>) {
    for record in records {
        if record.record_type == RecordType::THEADR {
            print_record_theadr(record, bytes);
            println!();
        } else if record.record_type == RecordType::COMENT {
            print_record_coment(record, bytes);
            println!();
        }
    }
}

//Record prints
fn print_record_theadr(record: &Record, bytes: &Vec<u8>) {
    println!("THEADR:");
    let str_size = bytes[record.start] as usize;
    let str_bytes = &bytes[record.start+1..record.start+1+str_size];
    let str = String::from_utf8_lossy(str_bytes);
    println!("{:>7} {}", "name", str);
}

fn print_record_coment(record: &Record, bytes: &Vec<u8>) {
    println!("COMENT:");
    let cmt_type = bytes[record.start];
    let cmt_class = bytes[record.start+1];
    let cmt_str = &bytes[record.start+2..(record.start+(record.len-3))];

    println!("{:>12} {}", "NP", cmt_type & 0x80);
    println!("{:>12} {}", "NL", cmt_type & 0x40);
    println!("{:>12} {:x}", "class", cmt_class);
    println!("{:>12} {}", "commentary", String::from_utf8_lossy(cmt_str));
}

