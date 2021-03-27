// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.


fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");

    let a: usize = 1;
    let b = &a;
    let mut c: Vec<&usize> = Vec::new();
    c.push(b);
}
