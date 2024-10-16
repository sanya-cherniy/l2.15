fn main() {
    let s1 = "hello"; // Строковый литерал

    let s2 = String::from("hello"); // Динамический массив байтов

    let s3 = s2.as_str(); // Строковый срез

    let size_of_s1 = std::mem::size_of_val(s1); // Передаем str

    let size_of_s2 = std::mem::size_of_val(&s2); // Передаем String

    let size_of_s3 = std::mem::size_of_val(&s3); // Передаем &str

    println!("{:?}", size_of_s1);
    println!("{:?}", size_of_s2);
    println!("{:?}", size_of_s3);
}
