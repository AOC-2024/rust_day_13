use day_13::count_tokens_to_will_all;

fn main() {
    let token_to_win_all_prices = count_tokens_to_will_all("src/resources/puzzle.txt");
    println!("Tokens needed to win all prices: {:#?}", token_to_win_all_prices);
}
