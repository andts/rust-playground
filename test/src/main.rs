#[derive(Debug)]
struct Column {
    data: Vec<(i32, i32)> //vector of tuples, mapping row key to value
}

#[derive(Debug)]
struct IntFrame {
    name: String,
    fields: Vec<String>, //array of column names
    data: Vec<Column> //array of columns, same size as fields
}

impl IntFrame {
    fn add_column(&mut self, name: &str, col: Column) {
        self.fields.push(String::from(name));
        self.data.push(col);
    }
}

fn main() {
    println!("Hello {}!", "world");

    let mut f1 = IntFrame {
        name: String::from("test"),
        fields: vec!(),
        data: vec!(),
    };
    println!("IntFrame: {:?}", f1);

    f1.add_column("field3", Column {
        data: vec![(1, 10), (2, 20), (3, 30)]
    });
    println!("IntFrame: {:?}", f1);
    println!("IntFrame: {:?}", f1);

//    let another_field : String = String::from("field4");
//
//    f1.fields.push(another_field);
//    println!("IntFrame: {:?}", f1);
}
