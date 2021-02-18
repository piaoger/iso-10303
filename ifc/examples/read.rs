use iso_10303::step::StepReader;
use iso_10303_ifc::ifc4;
use iso_10303_ifc::ifc2x3;
use std::path::Path;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "xbim-pathfinder")]
struct CmdArgs {
    /// input ifc file
    #[structopt(long = "input", short = "i")]
    input : String,
}

fn read_ifc2x3(input_file:&String) {

    let instant = std::time::Instant::now();
    let mut parsing_time = 0.0;
    let mut reader = ifc2x3::Ifc2x3Reader::new();
    match reader.read(input_file) {
        Ok(_) => {
            parsing_time = instant.elapsed().as_secs_f64();
            for context in reader.get_entities::<ifc4::IfcDirection>() {
                println!("IFCDIRECTION: {:?}", context);
            }
            let mut total = 0;
            for (type_id, entity_ids) in reader.type_ids {
                println!("{:?} - {} ({})", type_id, reader.type_names[&type_id], entity_ids.len());
                total += entity_ids.len();
            }
            println!("simple entities: {}", total);
        }
        Err(err) => println!("{:?}", err),
    }
    println!("elapsed time: {} seconds", parsing_time);
}

fn read_ifc4(input_file:&String) {

    let instant = std::time::Instant::now();
    let mut parsing_time = 0.0;
    let mut reader = ifc4::Ifc4Reader::new();
    match reader.read(input_file) {
        Ok(_) => {
            parsing_time = instant.elapsed().as_secs_f64();
            for entity in reader.get_entities::<ifc4::IfcBuildingStorey>() {
                println!("IFCDIRECTION: {:?}", entity.global_id());
            }
            let mut total = 0;
            for (type_id, entity_ids) in reader.type_ids {
                println!("{:?} - {} ({})", type_id, reader.type_names[&type_id], entity_ids.len());
                total += entity_ids.len();
            }
            println!("simple entities: {}", total);
        }
        Err(err) => println!("{:?}", err),
    }
    println!("elapsed time: {} seconds", parsing_time);


}

fn main() {
    let mut opt = CmdArgs::from_args();
    let CmdArgs { input  } = opt;

    let bytes = std::fs::read(&input).unwrap();
    let schema_name = match iso_10303::step::parser::exchange_file().parse(&bytes) {
            Ok(file) => {
                let header = iso_10303::step::StepFileHeader::form_parameters(file.header);
                header.file_schema().schema_identifiers()[0].to_uppercase()
            }
            Err(err) => "unknown".to_string(),
    };

    println!("schema: {:?}", schema_name);

    if schema_name == "IFC4" {
        read_ifc4(&input);
    } else if schema_name == "IFC2X3" {
        read_ifc4(&input);
    } else {
        println!("unsupported schema: {}", schema_name );
    }


}
