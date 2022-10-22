/*******************************************************************************
 * Copyright (c) ArSysOp 2018-2022
 *
 * RGM Sources are publicly available only for
 * informational, review, analysis and consulting purposes.
 *
 * Definitions, terms and conditions for using RGM Sources are stated by ArSysOp Source License version 1.0.
 * See http://arsysop.ru/licenses/rgm/ArSysOpSourceLicense-1.0.txt
 *
 * RGM Sources are provided on "as is" basis.
 * ArSysOp is not responsible for any damages, losses, legal prosecution
 * or other consequences of any sort that using RGM Sources can cause to you
 * (as an individual or Legal Entity), even if aware of such consequences.
 *
*******************************************************************************/
use std::fs::File;

use clap::Parser;
use html_parser::Dom;
use loft_it::{
    parse::parse,
    test::{create_base, create_test, TestConfiguration},
};

#[derive(Parser)]
struct Args {
    configuration: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let file = File::open(args.configuration).expect("No such configuration file.");
    let configuration: TestConfiguration =
        serde_yaml::from_reader(file).expect("Incorrect configuration file");
    let result = reqwest::get(&configuration.url).await;
    let body = match result {
        Ok(response) => response.text().await,
        Err(e) => Result::Err(e),
    };
    println!("{}", &configuration.url);
    let result = Dom::parse(body.unwrap().as_str()).and_then(|dom| Ok(parse(dom).unwrap()));
    match result {
        Ok(result) => {
            create_base(&configuration);
            result
                .into_iter()
                .for_each(|section| create_test(section, &configuration));
        }
        Err(error) => println!("An error occurred: {}", error.to_string()),
    }
}
