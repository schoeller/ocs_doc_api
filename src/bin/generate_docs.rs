use clap::Parser;
use ocs_doc_api::{
    embedded_doc_api_ops_md, embedded_object_model_docs_md,
};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "generate_docs")]
struct Args {
    #[arg(long, short, value_name = "DIR")]
    out_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    fs::create_dir_all(&args.out_dir)?;
    fs::write(
        args.out_dir.join("object_model_docs.md"),
        embedded_object_model_docs_md(),
    )?;
    fs::write(
        args.out_dir.join("doc_api_ops.md"),
        embedded_doc_api_ops_md(),
    )?;
    println!("Wrote docs to {}", args.out_dir.display());
    Ok(())
}
