mod joltage;
mod repeating_sequence;
mod repeating_sequence_multiple;
mod rotate_count_zeros;
mod forklift;

fn main() {
    rotate_count_zeros::times_zero_dial("rotate_zero_count_puzzle_input.txt");
    // repeating_sequence::calculate_invalid_in_range();
    //repeating_sequence_multiple::calculate_invalid_in_range();
    // joltage::find_joltage();
    // joltage::find_12_digits_joltage();
    forklift::find_roll_count();
    forklift::find_total_roll_count();
}
