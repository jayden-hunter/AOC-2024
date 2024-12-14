use anyhow::{anyhow, Context, Result};
use humantime::format_duration;
use std::{collections::HashMap, fs::read_to_string, time::Instant};

type Page = i32;
type ProcessedInput = (HashMap<Page, Vec<Page>>, Vec<Vec<Page>>);
type Output = Page;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    println!("Lines in Input: {}", input.lines().count());

    let processing_start = Instant::now();
    let processed = process_input(input)?;
    let time_processing = processing_start.elapsed();

    let part_one = solve_part_one(processed.clone());
    let part_one_start = Instant::now();
    let time_one = part_one_start.elapsed();

    let part_two_start = Instant::now();
    let part_two = solve_part_two(processed);
    let time_two = part_two_start.elapsed();

    println!(
        "Processing Complete. (Took {:?})",
        format_duration(time_processing).to_string()
    );
    println!(
        "Part One: {:?} (Took {:?})",
        part_one,
        format_duration(time_one).to_string()
    );
    println!(
        "Part Two: {:?} (Took {:?})",
        part_two,
        format_duration(time_two).to_string()
    );
    Ok(())
}

fn process_input(input: String) -> Result<ProcessedInput> {
    let mut iter = input.lines();
    let ordering_rules = generate_ordering_rules(&mut iter)?;
    let books = generate_books(&mut iter)?;
    Ok((ordering_rules, books))
}

fn generate_books(iter: &mut std::str::Lines<'_>) -> Result<Vec<Vec<Page>>> {
    let mut books = vec![];
    for book in iter {
        if book.is_empty() {
            return Ok(books);
        }
        let mut book_vec = vec![];
        let pages = book.split(",");
        for page in pages {
            book_vec.push(page.parse()?);
        }
        books.push(book_vec);
    }
    Ok(books)
}

fn generate_ordering_rules(iter: &mut std::str::Lines<'_>) -> Result<HashMap<Page, Vec<Page>>> {
    let mut rule_map = HashMap::new();
    loop {
        let rule_str = iter
            .next()
            .with_context(|| "Input should have a newline seperating rules from books")?;
        if rule_str.trim().is_empty() {
            return Ok(rule_map);
        }
        let (left, right) = rule_str
            .split_once("|")
            .with_context(|| "Rules should be split by | delimiter")?;
        let left = left.parse()?;
        let right = right.parse()?;
        rule_map.entry(left).or_insert(Vec::new()).push(right);
    }
}

fn solve_part_one(data: ProcessedInput) -> Result<Output> {
    let (rule_map, books) = data;
    let sum = books
        .iter()
        .filter(|book| is_valid_book(book.to_vec(), &rule_map).is_ok_and(|v| v))
        .map(|valid_book| get_middle_page(valid_book))
        .sum();
    Ok(sum)
}

fn get_middle_page(valid_book: &[Page]) -> Page {
    let middle = valid_book.len() / 2;
    valid_book[middle]
}

fn is_valid_book(book: Vec<Page>, rule_map: &HashMap<Page, Vec<Page>>) -> Result<bool> {
    for i in 0..book.len() {
        let before = &book[0..i];
        if !are_valid_predecessors(book[i], before, rule_map)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn are_valid_predecessors(
    i: Page,
    before: &[Page],
    rule_map: &HashMap<Page, Vec<Page>>,
) -> Result<bool> {
    let blacklist = get_afterpages(i, rule_map);
    for num in blacklist {
        if before.contains(&num) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn solve_part_two(data: ProcessedInput) -> Result<Output> {
    let (rule_map, books) = data.clone();
    let invalid_book_sum = books
        .iter()
        .filter(|book| !is_valid_book(book.to_vec(), &rule_map).is_ok_and(|v| v))
        .map(|invalid_book| find_valid_book_ordering(invalid_book, &rule_map))
        .collect::<Result<Vec<Vec<Page>>>>()?
        .iter()
        .map(|valid_book| get_middle_page(valid_book))
        .sum();
    Ok(invalid_book_sum)
}

fn find_valid_book_ordering(
    invalid_book: &[Page],
    rule_map: &HashMap<Page, Vec<Page>>,
) -> Result<Vec<Page>> {
    let mut remaining_pages = invalid_book.to_vec();
    let mut valid_book = Vec::new();
    while !remaining_pages.is_empty() {
        for i in 0..remaining_pages.len() {
            let attempt = remaining_pages[i];
            let mut others = remaining_pages[0..i].to_vec();
            others.append(&mut remaining_pages[i + 1..].to_vec());
            if predecessor_count(attempt, others, rule_map)? == 0 {
                remaining_pages.remove(i);
                valid_book.push(attempt);
                break;
            }
        }
    }
    valid_book.reverse();
    if !is_valid_book(valid_book.clone(), rule_map)? {
        panic!(
            "Book was made valid, but isn't: {:#?} -> {:#?}",
            invalid_book, valid_book
        );
    }
    Ok(valid_book)
}

fn predecessor_count(
    attempt: Page,
    others: Vec<Page>,
    rule_map: &HashMap<Page, Vec<Page>>,
) -> Result<usize> {
    let blacklist = get_afterpages(attempt, rule_map);
    let count = others.iter().filter(|p| blacklist.contains(p)).count();
    Ok(count)
}

fn get_afterpages(page: Page, rule_map: &HashMap<Page, Vec<Page>>) -> Vec<Page> {
    rule_map.get(&page).map(|v| v.to_vec()).unwrap_or_default()
}
