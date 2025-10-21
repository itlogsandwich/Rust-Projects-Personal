
#[derive(Debug)]
struct Rectangle
{
    width: i32,
    height: i32,
}

impl Rectangle
{
    fn rect_area(&self) -> i32
    {
        self.width * self.height
    }
}


fn main() 
{   
    let rectangle = Rectangle 
    {
        width: 50,
        height: 100,
    };
    
    dbg!("Area: {}", rectangle.rect_area());
}
