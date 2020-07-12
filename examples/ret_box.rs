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

fn t_writer() {
    // &mut Box<dyn Write>
    let out = &mut get_output();
    m(out);
}

fn m<T>(writer: &mut T)
where
    T: std::io::Write,
{
    writer.write("hello world\n".as_bytes());
}

// fn get_serializer<'a, T: std::io::Write>(writer: &'a mut T) ->
// Result<Box<dyn ESerializer + 'a>> {     // &mut Serializer<&mut T,
// CompactFormatter>     // let orig = ;
//     // let mut orig = Box::new(serde_json::Serializer::new(writer));
//     // let sr = &mut ;
//     let erased = ESerializer::erase(&mut
// serde_json::Serializer::new(writer));     Ok(Box::new(erased))
// }

fn get_serializer<'a, T>(ser: &'a mut T) -> Result<Box<dyn ESerializer>>
where
    'a: 'static,
    &'a mut T: serde::Serializer,
    // &'static mut T::Ok: 'static,
{
    // let r = *ser;
    let erased = ESerializer::erase(ser);
    Ok(Box::new(erased))
}

fn get_data() -> Box<dyn ESerialize> {
    let mine = Test {
        name: "hello".to_string(),
    };

    Box::new(mine)
}

fn main() -> Result<()> {
    t_writer();

    let n = ret_box();

    let o = get_output();
    let mut j: Box<
        serde_json::Serializer<
                Box<dyn std::io::Write>,
                serde_json::ser::CompactFormatter,
            > + 'static,
    > = Box::new(serde_json::Serializer::new(o));
    let ser = j.as_mut();
    let mut orig = get_serializer(ser)?;
    let sr = orig.as_mut();

    // println
    let _ = j;
    // let sr = orig.as_mut();

    let data = get_data();

    data.erased_serialize(sr);

    println!("Got: {}", n);
    Ok(())
}
