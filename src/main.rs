mod az;
mod za_rev;

fn main() {
    println!("---------print a~Z---------");
    za_rev::za_rev();
    println!("---------print A~z---------");
    az::az();
}
