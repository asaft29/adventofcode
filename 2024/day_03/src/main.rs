mod part_1;
mod part_2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part_1::execute()?;
    part_2::execute()?;
    Ok(())
}
