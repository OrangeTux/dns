// Serialize and deserialize domain names.
//
// Domain names can be encoded in different ways:
// 1) as a series of labels ending with a 0 byte.
// 2) as a pointer
// 3) as a series of labels ending with a pointer
//
// Inside the question section, domain names are always encoded with #1.
// However, inside resource records, domain names can be encoded using all 3 methods.
//
// For more info, see sections:
// * 3.1. Name space definitions
// * 4.1.4. Message compression
use crate::DecodeError;
use core::fmt;
use std::iter::Peekable;
use std::slice::Iter;

pub struct Name(Vec<u8>);

pub(crate) fn into_string(name: Name) -> Result<String, DecodeError> {
    if is_pointer(name.0[0]) || is_pointer(name.0[name.0.len() - 1]) {
        return Err(DecodeError::IllegalValue(String::from(
            "can't stringify domain name since it contains pointers",
        )));
    }

    let mut utf8_bytes = vec![];
    for byte in name.0 {
        if byte == 0 {
            continue;
        }

        if is_label(byte) {
            if !utf8_bytes.is_empty() {
                utf8_bytes.push(b'.');
            }
            continue;
        }

        utf8_bytes.push(byte);
    }

    Ok(std::str::from_utf8(&utf8_bytes)
        .map_err(|_| {
            DecodeError::IllegalValue(
                "failed to parse value as qname: value not valid UTF-8".into(),
            )
        })?
        .to_string())
}

// Take the number of bytes that make up the domain name.
// TODO: Not sure if the term 'take' is the correct term here. `Iterator::take()` exists, but that returns
// a `Take` instance, not a `Vec`. Also, `Iterator::take()` is a bound method and doesn't receive
// an argument (other than `self`).
pub(crate) fn take_name(value: &mut Peekable<Iter<'_, u8>>) -> Result<Name, DecodeError> {
    let high_byte = **value.peek().ok_or(DecodeError::NotEnoughBytes)?;

    if is_pointer(high_byte) {
        println!("First byte of name indicate it's a pointer.");
        // Pointers are 2 bytes wide.
        let high_byte = *value.next().ok_or(DecodeError::NotEnoughBytes)?;
        let low_byte = *value.next().ok_or(DecodeError::NotEnoughBytes)?;
        return Ok(Name(vec![high_byte, low_byte]));
    }

    if !is_label(high_byte) {
        println!(
            "First byte of name is invalid: it should indicate it's a pointer or a label.{:b}",
            high_byte
        );
        // Pointers are 2 bytes wide.
        return Err(DecodeError::IllegalValue(
            "failed to parse domain name: it's not a pointer or label".to_string(),
        ));
    }
    println!("First byte of name is a label: {:b}", high_byte);

    let mut labels = take_labels(value)?;
    let byte = **value.peek().ok_or(DecodeError::NotEnoughBytes)?;

    if byte == 0 {
        let byte = *value.next().ok_or(DecodeError::NotEnoughBytes)?;
        labels.push(byte);
        return Ok(Name(labels));
    }

    if is_pointer(byte) {
        let high_byte = *value.next().ok_or(DecodeError::NotEnoughBytes)?;
        let low_byte = *value.next().ok_or(DecodeError::NotEnoughBytes)?;
        labels.push(low_byte);
        return Ok(Name(labels));
    }

    Err(DecodeError::IllegalValue(
        "label doesn't end with a null byte or pointer".to_string(),
    ))
}

fn take_labels(value: &mut Peekable<Iter<'_, u8>>) -> Result<Vec<u8>, DecodeError> {
    let mut labels = vec![];
    loop {
        let high_byte = **value.peek().ok_or(DecodeError::NotEnoughBytes)?;

        if !is_label(high_byte) {
            return Ok(labels);
        }

        if high_byte == 0 {
            return Ok(labels);
        }

        if !labels.is_empty() {
            labels.push(b'.');
        }

        labels.append(&mut take_label(&mut *value)?);
    }
}

fn take_label(value: &mut Peekable<Iter<'_, u8>>) -> Result<Vec<u8>, DecodeError> {
    let length_of_label: usize = (*value.next().ok_or(DecodeError::NotEnoughBytes)?).into();
    println!("Label is has lenght {length_of_label}");
    let mut label: Vec<u8> = Vec::with_capacity(length_of_label);
    for _ in 0..length_of_label {
        let char = *value.next().ok_or(DecodeError::NotEnoughBytes)?;
        label.push(char);
    }

    Ok(label)
}

// A pointer can be detected by inspecting the first 2 bits. They must be high.
fn is_pointer(value: u8) -> bool {
    value & 0b1100_0000 == 0b1100_0000
}

// A pointer can be detected by inspecting the first 2 bits. They must be low.
fn is_label(name: u8) -> bool {
    name & 0b1100_0000 == 0b0000_0000
}
