fn iterate_with_while(elements: &Vec<String>) {
    // Iter - separate struct from the original colors vector,
    // it contains a pointer pointing to the first element of the vector
    // if the next() method is called explicitly, the variable must be mutable
    let mut elements_iter = elements.iter();

    // every time next() is called, it moves the pointer to the next element
    // until it points to the last element
    // next() returns Some() if the pointer points to an element within the vector
    // and None if next() is called after the last element has been reached

    while let Some(element) = elements_iter.next() {
        println!("{}", element);
    }

}


fn iterate_with_for(elements: &Vec<String>) {
    // iterators are lazy, no iteration starts until the next() 
    // function is called either directly or indirectly by another function
    let elements_iter = elements.iter();

    for color in elements_iter {
        println!("{}", color);
    }
}

fn iterate_with_for_each(elements: &Vec<String>) {
    elements.iter().for_each(|element| println!("{}", element));
}

// This function would also accept &Vec<String> as parameters
fn iterate_over_vector_slices(elements: &[String]) {
    elements.iter().for_each(|element| println!("{}", element));
}

fn shorten_strings(elements: &mut Vec<String>) {
    elements
    .iter_mut()
    .for_each(|el| el.truncate(1));
}

// collect will use the return type annotation
// to decide what type of data structure to return
// in this case collect will create a vector of strings
// since the return annotation is Vec<String>
// it would have also worked if e.g. std::collections::LinkedList<String>
// would have been provided.
// the preferred way however, is to define what collection type to return
// by using the following sintax:
// .collect::<collection_type<T>>()
// e.g.:
// .collect<Vec<String>>()
// or
// .collect<LinkedList<String>>
fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
    .iter()
    .map(|el| el.to_uppercase()) //map doesn't do anything until collect is called
    .collect::<Vec<String>>()
}

fn move_elements(from_vect: Vec<String>, to_vect: &mut Vec<String>) {
    // into_iter will take ownership of the elements of from_vect
    // and modify both vectors
    from_vect
    .into_iter()
    .for_each(|element| to_vect.push(element));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
    .iter()
    .map(|element| {
        element.chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
    })
    .collect::<Vec<Vec<String>>>()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
    .iter()
    .find(|element| element.contains(search))
    .map_or(String::from(fallback), |element| element.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];

    println!("Printing elements with while:");
    iterate_with_while(&colors);
    println!("Printing elements with while:");
    iterate_with_for(&colors);
    println!("Printing elements with while:");
    iterate_with_for_each(&colors);
    println!("Printing also works with vector slices");
    iterate_over_vector_slices(&colors[1..3]);

    let exploded_colors = explode(&colors);
    println!("Exploded colors: {:#?}", exploded_colors);

    let color = find_color_or(&colors, "ora", "Orange");
    println!("Found: {}", color);

    let uppercase_colors = to_uppercase(&colors);
    println!("{:#?}", uppercase_colors);

    shorten_strings(&mut colors);
    println!("{:#?}", &colors);

    let mut  colors_dest: Vec<String>  = vec![];
    move_elements(colors, &mut colors_dest);
    println!("Colors Destination: {:#?}", colors_dest);
}
