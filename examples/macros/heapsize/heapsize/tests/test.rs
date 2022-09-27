use heapsize::HeapSize;

#[derive(HeapSize)]
struct Demo<'a, T: ?Sized> {
    a: Box<T>,
    b: u8,
    c: &'a str,
    d: String,
}

#[test]
fn test_heapsize_derive() {
    let demo = Demo {
        a: b"byestring".to_vec().into_boxed_slice(),
        b: 255,
        c: "&'static str",
        d: "String".to_owned(),
    };

    println!(
        "heap size = {} + {} + {} + {} = {}",
        demo.a.heap_size_of_children(),
        demo.b.heap_size_of_children(),
        demo.c.heap_size_of_children(),
        demo.d.heap_size_of_children(),
        demo.heap_size_of_children()
    );
}