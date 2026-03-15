// 动态数组 (Dynamic Array) generic vector implementation

// Define a struct for the dynamic array
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    // Create a new vector
    pub fn new() -> Self {
        Vector { data: Vec::new() }
    }

    // Add an element to the vector
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    // Get an element from the vector
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    // Return the number of elements in the vector
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// Example usage
/* 
fn main() {
    let mut vec = Vector::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Element at index 0: {:?}", vec.get(0));
    println!("Length of vector: {}", vec.len());
} 
*/
