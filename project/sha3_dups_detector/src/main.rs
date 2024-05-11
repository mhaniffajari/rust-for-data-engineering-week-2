use sha3_dups_detector::generate_random_phrases;
fn main() {
    let pharses = generate_random_phrases();
    sha3_dups_detector::analyze_duplicates(&pharses);
}
