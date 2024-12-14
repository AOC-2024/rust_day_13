use day_13::count_tokens_to_will_all;

fn main() {
    let token_to_win_all_prices = count_tokens_to_will_all("src/resources/puzzle.txt", None);
    //31552
    println!("Tokens needed to win all prices: {:#?}", token_to_win_all_prices);

    let token_to_win_all_prices = count_tokens_to_will_all("src/resources/puzzle.txt", Some(10000000000000));
    //95273925552482
    println!("Tokens needed to win all prices: {:#?}", token_to_win_all_prices);
}
