fn main() {
    let input = include_str!("../input/day1.txt");
    let mut sorted_from_greatest = input
        .split("\n\n")
        .map(|lines| {
            lines
                .split("\n")
                .flat_map(|str_num| str_num.parse::<u32>())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    sorted_from_greatest.sort_by(|a, b| b.cmp(a));

    let most_calories = sorted_from_greatest.iter().take(3).sum::<u32>();

    println!("{:?}", most_calories);
}
