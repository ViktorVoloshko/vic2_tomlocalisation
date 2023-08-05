use std::{io::{Read, Write}, fs::File, fs::read_dir};
use toml::Table;


fn write_countries(localisation: &mut File, countries: &Vec<Table>) {
    write_countries_names(localisation, countries);
    write_countries_adjectives(localisation, countries);
    write_countries_ideologies(localisation, countries);
    write_countries_parties(localisation, countries);
}

fn write_countries_names(localisation: &mut File, countries: &Vec<Table>) {
    let buf = String::from("# COUNTRIES NAMES\n");
    localisation.write_all(buf.as_bytes()).unwrap();

    for country in countries {
        let buf = format!("{};{};X\n",
                country["tag"].as_str().unwrap(), country["name"].as_str().unwrap());
        localisation.write_all(buf.as_bytes()).unwrap();
    }
}

fn write_countries_adjectives(localisation: &mut File, countries: &Vec<Table>) {
    let buf = String::from("# COUNTRIES ADJECTIVES\n");
    localisation.write_all(buf.as_bytes()).unwrap();

    for country in countries {
        let buf = format!("{}_ADJ;{};X\n",
                country["tag"].as_str().unwrap(), country["adjective"].as_str().unwrap());
        localisation.write_all(buf.as_bytes()).unwrap();
    }
}

fn write_countries_ideologies(localisation: &mut File, countries: &Vec<Table>) {
    let buf = String::from("# COUNTRIES IDEOLOGIES\n");
    localisation.write_all(buf.as_bytes()).unwrap();

    for country in countries {
        if country["ideologies"].is_table() {
            for ideology in country["ideologies"].as_table().unwrap() {
                let buf = format!("{}_{};{};X\n",
                        country["tag"].as_str().unwrap(), ideology.0, ideology.1.as_str().unwrap());
                localisation.write_all(buf.as_bytes()).unwrap();
            }
        }
    }
}

fn write_countries_parties(localisation: &mut File, countries: &Vec<Table>) {
    let buf = String::from("# COUNTRIES PARTIES\n");
    localisation.write_all(buf.as_bytes()).unwrap();

    for country in countries {
        if country["parties"].is_table() {
            for party in country["parties"].as_table().unwrap() {
                let buf = format!("{}_{};{};X\n",
                        country["tag"].as_str().unwrap(), party.0, party.1.as_str().unwrap());
                localisation.write_all(buf.as_bytes()).unwrap();
            }
        }
    }
}

fn main() {
    let paths = read_dir("toml/countries").unwrap();
    let mut countries: Vec<Table> = Vec::new();
    for file in paths {
        let mut buf = String::new();
        File::open(file.unwrap().path()).unwrap().read_to_string(&mut buf).unwrap();
        countries.push(buf.parse::<Table>().unwrap());
    }
    
    let mut localisation = File::create("vic2/localisation/localisation.csv").unwrap();

    write_countries(&mut localisation, &countries);
}