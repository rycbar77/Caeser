#![allow(dead_code)]
#![allow(unused)]
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use plotters::prelude::*;

fn encrypt(s: &str,rot: u8) -> String
{
    let mut s_ret=String::new();
    for c in s.chars(){
        if  ('a' as u8) <= c as u8 && (c as u8) <= 'z' as u8
        {
            let mut tmp =(c as u8) -('a' as u8);
            tmp=(26+tmp+rot)%26+'a' as u8;
            let ch=tmp as char;
            s_ret.push(ch);
        }
        else if ('A' as u8) <= (c as u8) && (c as u8) <= ('Z' as u8)
        {
            let mut tmp =(c as u8)-('A' as u8);
            tmp=(26+tmp+rot)%26+'A' as u8;
            let ch=tmp as char;
            s_ret.push(ch);
        }
        else {
            s_ret.push(c);
            continue;
        }
    }
    s_ret
}

fn decrypt(s: &str,rot: u8) -> String
{
    let mut s_ret=String::new();
    for c in s.chars(){
        if  ('a' as u8) <= (c as u8) && (c as u8) <= ('z' as u8)
        {
            let mut tmp =(c as u8) -('a' as u8);
            tmp=(26+tmp-rot)%26+'a' as u8;
            // plus 26 to avoid 'attempt to subtract with overflow' error since u8 is unsigned
            let ch=tmp as char;
            s_ret.push(ch);
        }
        else if ('A' as u8) <= (c as u8) && (c as u8) <= ('Z' as u8)
        {
            let mut tmp =(c as u8)-('A' as u8);
            tmp=(26+tmp-rot)%26+'A' as u8;
            // plus 26 to avoid 'attempt to subtract with overflow' error since u8 is unsigned
            let ch=tmp as char;
            s_ret.push(ch);
        }
        else {
            s_ret.push(c);
            continue;
        }
    }
    s_ret
}

fn collect_lower(s: &String)->HashMap<char,i32>{
    let mut hp: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        if  ('a' as u8) <= (c as u8) && (c as u8) <= ('z' as u8)
        {
            let counter=hp.entry(c).or_insert(0);
            *counter += 1;
        }
    }
    hp
}

fn collect_upper(s: &String)->HashMap<char,i32>{
    let mut hp: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        if  ('A' as u8) <= (c as u8) && (c as u8) <= ('Z' as u8)
        {
            let counter=hp.entry(c).or_insert(0);
            *counter += 1;
        }
    }
    hp
}

fn collect(s: &String)->HashMap<char,i32>{
    let mut hp: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        if  (('a' as u8) <= (c as u8) && (c as u8) <= ('z' as u8)) || (('A' as u8) <= (c as u8) && (c as u8) <= ('Z' as u8))
        {
            let mut tmp: char;
            if (c as u8)>=('a' as u8)
            {
                    tmp=c; 
            }
            else
            {
                tmp=((c as u8)-('A' as u8)+('a' as u8)) as char;
            }
            let counter=hp.entry(tmp).or_insert(0);
            *counter += 1;
        }
    }
    hp
}

fn main()->Result<(), Box<dyn std::error::Error>> {
    
    // Open file and read to string
    let filename=String::from("text.txt");
    // println!("In file {}", filename);
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // PlainText word frequency
    let plain_text1=collect(&contents);
    let plain_text2=collect_lower(&contents);
    let plain_text3=collect_upper(&contents);

    // Encrypt and Decrypt
    let encrypted=encrypt(&contents[..],3);
    // println!("With text:\n{}", contents);
    // println!("\n\n\n\n");
    // println!("encrypted:\n{}", encrypted);
    // println!("\n\n\n\n");
    let decrypted=decrypt(&encrypted[..],3);
    // println!("decrypted:\n{}", decrypted);
    let path="encrypted.txt";
    fs::write(path, &encrypted);
    // CipherText word frequency
    let cipher_text1=collect(&encrypted);
    let cipher_text2=collect_lower(&encrypted);
    let cipher_text3=collect_upper(&encrypted);

    let root =
        BitMapBackend::new("Word_Frequency.png", (1920, 1920)).into_drawing_area();

    root.fill(&WHITE)?;
    let (upper, lower) = root.split_vertically(960);
    let (upperleft,upperright) =upper.split_horizontally(960);
    let (lowerleft,lowerright) =lower.split_horizontally(960);

    // PlainText Word Frequency
    let mut chart = ChartBuilder::on(&upperleft)
        .x_label_area_size(75)
        .y_label_area_size(80)
        .margin(40)
        .caption("PlainText Word Frequency", ("sans-serif", 50.0).into_font())
        .build_ranged('a' as u32..'z' as u32+1, 0i32..1600i32)?;
    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("Count")
        .x_desc("Alphabet")
        .label_style(("sans-serif", 30).into_font())
        .axis_desc_style(("sans-serif", 40).into_font())
        .draw()?;

    
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(plain_text1.iter().map(|c| (*c.0 as u32,*c.1))),
    )?;
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(plain_text2.iter().map(|c| (*c.0 as u32,*c.1))),
    )?;

    // PlainText Word Frequency(Lowercase)
    let mut chart = ChartBuilder::on(&upperright)
        .x_label_area_size(75)
        .y_label_area_size(80)
        .margin(40)
        .caption("PlainText Word Frequency(Lowercase)", ("sans-serif", 50.0).into_font())
        .build_ranged('a' as u32..'z' as u32+1, 0i32..1600i32)?;
    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("Count")
        .x_desc("Alphabet")
        .label_style(("sans-serif", 30).into_font())
        .axis_desc_style(("sans-serif", 40).into_font())
        .draw()?;


    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(plain_text2.iter().map(|c| (*c.0 as u32,*c.1))),
    )?;
    


    // CipherText Word Frequency
    let mut chart = ChartBuilder::on(&lowerleft)
        .x_label_area_size(75)
        .y_label_area_size(80)
        .margin(40)
        .caption("CipherText Word Frequency", ("sans-serif", 50.0).into_font())
        .build_ranged('a' as u32..'z' as u32+1, 0i32..1600i32)?;
    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("Count")
        .x_desc("Alphabet")
        .label_style(("sans-serif", 30).into_font())
        .axis_desc_style(("sans-serif", 40).into_font())
        .draw()?;

    
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.mix(0.5).filled())
            .data(cipher_text1.iter().map(|c| (*c.0 as u32,*c.1))),
    )?;
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.mix(0.5).filled())
            .data(cipher_text2.iter().map(|c| (*c.0 as u32,*c.1))),
    )?;


    // CipherText Word Frequency(Lowercase)
    let mut chart = ChartBuilder::on(&lowerright)
        .x_label_area_size(75)
        .y_label_area_size(80)
        .margin(40)
        .caption("CipherText Word Frequency(Lowercase)", ("sans-serif", 50.0).into_font())
        .build_ranged('a' as u32..'z' as u32+1, 0i32..1600i32)?;
    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("Count")
        .x_desc("Alphabet")
        .label_style(("sans-serif", 30).into_font())
        .axis_desc_style(("sans-serif", 40).into_font())
        .draw()?;


    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.mix(0.5).filled())
            .data(cipher_text2.iter().map(|c| (*c.0 as u32,*c.1))),
    )?;
    Ok(())
}


