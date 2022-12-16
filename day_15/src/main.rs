use day_15::day_fifteen_part_two;

fn main() {
    let result = day_fifteen_part_two("example.txt", 20);
    println!("{result:#?}");
    let result = day_fifteen_part_two("data.txt", 4_000_000);
    println!("{result:#?}");
}
