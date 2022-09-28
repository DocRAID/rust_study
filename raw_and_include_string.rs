fn main() {
    let a = r#"
    <div class="advice">
        Raw string are useful for some some
    </div>
    "#;
    let readme = include_str!("README.md");
    println!("{} {}",a,readme);
}