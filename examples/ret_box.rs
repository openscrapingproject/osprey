use anyhow::Result;
use serde::Serialize;

fn ret_box() -> Box<i32> {
    Box::new(5)
}

fn get_output() -> Box<dyn std::io::Write> {
    return Box::new(std::io::stdout());
}
#[derive(Serialize)]
struct Test {
    name: String,
}

use erased_serde::Serialize as ESerialize;
use erased_serde::Serializer as ESerializer;

fn get_serializer() -> Result<Box<dyn ESerializer>> {
    let mut orig = Box::new(serde_json::Serializer::new(get_output()));
    // let sr = &mut ;
    Ok(Box::new(ESerializer::erase(orig.as_mut())))
}

fn get_data() -> Box<dyn ESerialize> {
    let mine = Test {
        name: "hello".to_string(),
    };

    Box::new(mine)
}

fn main() -> Result<()> {
    let n = ret_box();

    let mut orig = get_serializer()?;
    let sr = orig.as_mut();

    let data = get_data();

    data.erased_serialize(sr)?;

    println!("Got: {}", n);
    Ok(())
}
