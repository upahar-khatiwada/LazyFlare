use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::utils::constants::{ARECORDS, CNAMERECORDS};

pub fn create_response(buf: &[u8]) -> String {
    let (headers, all_records_of_current_request): (String, Vec<String>) = create_headers(buf);
    // println!("Headers: {headers}");
    let (question_bytes, answer_bytes) = create_body(&buf[12..], &all_records_of_current_request);

    format!("{}{}{}", headers, question_bytes, answer_bytes)
}

fn create_body(buf: &[u8], current_request_records: &Vec<String>) -> (String, String) {
    let (domain_name, domain_type): (String, String) = get_domain_name_and_record_type(&buf);
    let question_bytes: String = create_dns_question(&domain_name, &domain_type);

    // println!("Body bytes: {question_bytes}");
    // println!("{:?}", current_request_records);

    let answer_bytes: String = create_answers(&domain_type, &current_request_records);

    (question_bytes, answer_bytes)
}

fn create_answers(domain_type: &String, records: &Vec<String>) -> String {
    let mut answer_bytes = String::new();

    for record in records {
        let line: &str = record.split(';').next().unwrap().trim();
        let parts: Vec<&str> = line.split_whitespace().collect();

        let ttl: u32 = parts[1].parse().unwrap();
        let rdata = parts[4];
        
        answer_bytes.push_str("C00C");

        if domain_type == "0001" {
            answer_bytes.push_str("0001");
        } else {
            answer_bytes.push_str("0005");
        }

        answer_bytes.push_str("0001"); // IN CLASS

        answer_bytes.push_str(&format!("{:08X}", ttl));

        if domain_type == "0001" {
            let mut rdata_bytes = String::new();
            let octets: Vec<u8> = rdata
                .split('.')
                .map(|x: &str| x.parse::<u8>().unwrap())
                .collect();
            for b in octets {
                rdata_bytes.push_str(&format!("{:02X}", b));
            }

            let rdlength = rdata_bytes.len() / 2;
            answer_bytes.push_str(&format!("{:04X}", rdlength));
            answer_bytes.push_str(&rdata_bytes);
        } else {
            // THIS PART IS FOR CNAME RECORDS
            let mut rdata_bytes = String::new();
            for part in rdata.trim_end_matches('.').split('.') {
                rdata_bytes.push_str(&format!("{:02X}", part.len()));
                for b in part.as_bytes() {
                    rdata_bytes.push_str(&format!("{:02X}", b));
                }
            }
            rdata_bytes.push_str("00");

            let rdlength = rdata_bytes.len() / 2;
            answer_bytes.push_str(&format!("{:04X}", rdlength));
            answer_bytes.push_str(&rdata_bytes);
        }
    }

    return answer_bytes;
}

fn create_dns_question(domain_name: &String, domain_type: &String) -> String {
    let mut question_bytes: String = String::new();

    for part in domain_name.split('.') {
        let len: usize = part.len();
        question_bytes.push_str(&format!("{:02X}", len));

        for b in part.as_bytes() {
            question_bytes.push_str(&format!("{:02X}", b));
        }
    }

    question_bytes.push_str("00");

    if domain_type == "0001" {
        question_bytes.push_str(&format!("{:04X}", 1));
    } else {
        question_bytes.push_str(&format!("{:04X}", 5));
    }

    question_bytes.push_str("0001"); // DNS CLASS IN

    question_bytes
}

fn get_domain_name_and_record_type(buf: &[u8]) -> (String, String) {
    let mut x: usize = 0;
    let mut domain_string: String = String::new();
    let mut domain_type: String = String::new();

    while buf[x] != 0 {
        let fixed_length: usize = buf[x] as usize;
        x += 1;

        for i in 0..fixed_length {
            domain_string.push(buf[x + i] as char);
        }

        x += fixed_length;

        if buf[x] != 0 {
            domain_string.push('.');
        }
    }

    for i in x + 1..x + 3 {
        domain_type.push_str(&format!("{:02X}", buf[i]));
    }

    (domain_string, domain_type)
}

fn get_zone_file_for_domain(domain_name: &str) -> String {
    let mut domain_file = format!("records/{}.txt", domain_name);

    if !Path::new(&domain_file).exists() {
        if let Some(pos) = domain_name.find('.') {
            let parent_domain = &domain_name[pos + 1..];
            domain_file = format!("records/{}.txt", parent_domain);
        }
    }

    domain_file
}

fn get_answer_counts(buf: &[u8]) -> (usize, Vec<String>) {
    let (domain_name, domain_type): (String, String) = get_domain_name_and_record_type(&buf);

    // println!("Domain Name: {domain_name}");

    let mut domain_type_comparable: String = String::new();

    // A RECORD
    if domain_type == "0001" {
        domain_type_comparable.push_str(ARECORDS);
    }
    // CNAME RECORD
    else if domain_type == "0005" {
        domain_type_comparable.push_str(CNAMERECORDS);
    } else {
        panic!("This DNS server only supports Type A and Type CNAME Records!");
    }

    let mut all_records_of_domain_to_be_queried: Vec<String> = vec![];

    let zone_file = get_zone_file_for_domain(&domain_name);
    let query_domain = format!("{}.", domain_name);
    if let Ok(lines) = read_lines(zone_file) {
        for line in lines
            .skip(26)
            .filter_map(Result::ok)
            .skip_while(|line| line != &domain_type_comparable)
            .skip(1)
            .take_while(|line| !line.starts_with(";;"))
            .filter(|line| line.starts_with(&query_domain))
        {
            all_records_of_domain_to_be_queried.push(line);
        }
    }

    if all_records_of_domain_to_be_queried.len() == 0 {
        panic!(
            "No such domain found in this dns server!\nMaybe try adding the zone file inside the records directory."
        );
    }

    // println!("{:?}", all_records_of_domain_to_be_queried);
    // println!(
    //     "Length of records: {}",
    //     all_records_of_domain_to_be_queried.len() - 1
    // );

    // all_records_of_domain_to_be_queried.pop();
    (
        all_records_of_domain_to_be_queried.len(),
        all_records_of_domain_to_be_queried,
    ) // accounting for an empty string
}

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn create_headers(buf: &[u8]) -> (String, Vec<String>) {
    let id: String = buf[..2].iter().map(|i: &u8| format!("{:02X}", i)).collect();

    let flags: String = create_flags(&buf[2..4]);

    let qdcount: String = format!("{:04X}", 1); // question count is always 1

    let (ancount, all_records_of_current_request): (usize, Vec<String>) =
        get_answer_counts(&buf[12..]);
    let ancount: String = format!("{:04X}", ancount);

    // assuming 0 for these
    let nscount: String = format!("{:04X}", 0);
    let arcount: String = format!("{:04X}", 0);

    (
        format!(
            "{}{}{}{}{}{}",
            id, flags, qdcount, ancount, nscount, arcount
        ),
        all_records_of_current_request,
    )
}

fn create_flags(flags_buf: &[u8]) -> String {
    let b1: u8 = flags_buf[0];

    let qr: u8 = 1 << 7; // we are sending a response so its 1 always
    let opcode: u8 = b1 & 0x78;
    let aa: u8 = 1 << 2; // assuming our server is authorative
    let tc: u8 = 0 << 1; // we are only handling the packets less than 512 bytes for UDP protocol
    // let RD: u8 = b1 & 1;
    let rd: u8 = 0;
    let ra: i32 = 0 << 7; // not handling recursion
    let z: i32 = 0 << 4; // reserved bits
    let rcode: i32 = 0; // not handling error case

    let flags_byte1: u8 = qr | opcode | aa | tc | rd;
    let flags_byte2: i32 = ra | z | rcode;

    format!("{:02X}{:02X}", flags_byte1, flags_byte2)
}
