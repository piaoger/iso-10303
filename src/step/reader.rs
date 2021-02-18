use super::parser::exchange_file;
use super::structure::*;
use std::path::Path;

/// ```text
/// ENTITY file_description;
/// description : LIST [1:?] OF STRING (256);
/// implementation_level : STRING (256);
/// END_ENTITY;
/// ```
#[derive(Default, Debug)]
pub struct StepFileDescription {
    description: Vec<String>,
    implementation_level: String,
}

impl StepFileDescription {

    pub fn description(&self) -> &Vec<String> {
        &self.description
    }

    pub fn implementation_level(&self) -> &String {
        &self.implementation_level
    }

    pub fn form_parameters(parameters: Vec<Parameter>) -> Self {
        let mut entity = StepFileDescription::default();
        for (index, parameter) in parameters.into_iter().enumerate() {
            match index {
                0usize => entity.description = parameter.into(),
                1usize => entity.implementation_level = parameter.into(),
                _ => {}
            }
        }
        entity
    }
}

/// ```text
/// ENTITY file_name;
/// name : STRING (256);
/// time_stamp : time_stamp_text;
/// author : LIST [ 1 : ? ] OF STRING (256);
/// organization : LIST [ 1 : ? ] OF STRING (256);
/// preprocessor_version : STRING (256);
/// originating_system : STRING (256);
/// authorization : STRING (256);
/// END_ENTITY;
/// TYPE time_stamp_text = STRING(256);
/// END_TYPE;
/// ```
#[derive(Default, Debug)]
pub struct StepFileName {
    name: String,
    time_stamp: String,
    author: Vec<String>,
    organization: Vec<String>,
    preprocessor_version : String,
    originating_system: String,
    authorisation: String,
}

impl StepFileName {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn time_stamp(&self) -> &String {
        &self.time_stamp
    }

    pub fn author(&self) -> &Vec<String> {
        &self.author
    }

    pub fn organization(&self) -> &Vec<String> {
        &self.organization
    }

    pub fn preprocessor_version(&self) -> &String {
        &self.preprocessor_version
    }

    pub fn originating_system(&self) -> &String {
        &self.originating_system
    }

    pub fn authorisation(&self) -> &String {
        &self.authorisation
    }

    pub fn form_parameters(parameters: Vec<Parameter>) -> Self {
        let mut entity = StepFileName::default();
        for (index, parameter) in parameters.into_iter().enumerate() {
            match index {
                0usize => entity.name = parameter.into(),
                1usize => entity.time_stamp = parameter.into(),
                2usize => entity.author = parameter.into(),
                3usize => entity.organization = parameter.into(),
                4usize => entity.preprocessor_version = parameter.into(),
                5usize => entity.originating_system = parameter.into(),
                6usize => entity.authorisation = parameter.into(),
                _ => {}
            }
        }
        entity
    }
}

/// ``` text
/// ENTITY file_schema;
/// schema_identifiers : LIST [1:?] OF UNIQUE schema_name;
/// END_ENTITY;
/// TYPE schema_name = STRING(1024);
/// END_TYPE;
/// #[derive(Default, Debug)]
/// ```
#[derive(Default, Debug)]
pub struct StepFileSchema {
    schema_identifiers: Vec<String>,
}

impl StepFileSchema {

    pub fn schema_identifiers(&self) -> &Vec<String> {
        &self.schema_identifiers
    }

    pub fn form_parameters(parameters: Vec<Parameter>) -> Self {
        let mut entity = StepFileSchema::default();
        for (index, parameter) in parameters.into_iter().enumerate() {
            match index {
                0usize => entity.schema_identifiers = parameter.into(),
                _ => {}
            }
        }
        entity
    }
}


/// ```
/// // HEADER
/// file_description (mandatory)
/// file_name (mandatory)
/// file_schema (mandatory)
/// schema_population (optional)
/// file_population (optional)
/// section_language (optional)
/// section_context (optional)
/// ```
/// Sample Header
/// ```text
/// ISO-10303-21;
/// HEADER;
///
/// FILE_DESCRIPTION(
/// /* description */ ('GeoReference'),
/// /* implementation level */ '2;1');
///
/// FILE_NAME(
/// /* name */ 'GeoReference.ifc',
/// /* time_stamp */ '2014-08-22T15:03:00',
/// /* author */ ('Geiger'),
/// /* organization */ ('KIT'),
/// /* preprocessor_version */ 'Handmade',
/// /* originating_system */ '',
/// /* authorisation */ 'none');
///
/// FILE_SCHEMA (('IFC4'));
/// ENDSEC;
/// ```
#[derive(Default, Debug)]
pub struct StepFileHeader {
    file_description: StepFileDescription,
    file_name: StepFileName,
    file_schema: StepFileSchema,
}

impl StepFileHeader {
    pub fn file_description(&self) -> &StepFileDescription {
        &self.file_description
    }

    pub fn file_name(&self) -> &StepFileName {
        &self.file_name
    }

    pub fn file_schema(&self) -> &StepFileSchema {
        &self.file_schema
    }

    pub fn form_parameters(header: Vec<TypedParameter>) -> Self {
        let mut file_header = StepFileHeader::default();

        for (_,   typed_parameter) in header.into_iter().enumerate() {
            match typed_parameter.type_name.as_str() {
                "FILE_SCHEMA" => {
                   file_header.file_schema = StepFileSchema::form_parameters(typed_parameter.parameters);

                },
                "FILE_NAME" => {
                   file_header.file_name =  StepFileName::form_parameters(typed_parameter.parameters);

                },
                "FILE_DESCRIPTION" => {
                   file_header.file_description =  StepFileDescription::form_parameters(typed_parameter.parameters);
                },
                _ => {
                    panic!("parameter type is not recognized: {}", typed_parameter.type_name);
                }
            }
        }

        file_header
    }
}


pub trait StepReader {
    fn read_simple_entity(&mut self, id: i64, typed_parameter: TypedParameter);

    fn read<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let bytes = std::fs::read(path)?;
        match exchange_file().parse(&bytes) {
            Ok(file) => {

                let header = StepFileHeader::form_parameters(file.header);
                println!("header entities: \n{:?}", header);
                println!("entities: {}", file.data.len());

                for instance in file.data {
                    if instance.value.len() == 1 {
                        for typed_parameter in instance.value {
                            // println!("read #{}", instance.id);
                            self.read_simple_entity(instance.id, typed_parameter);
                        }
                    }
                }
            }
            Err(err) => println!("{:?}", err),
        }

        Ok(())
    }
}
