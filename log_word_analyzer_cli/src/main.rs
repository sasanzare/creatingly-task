use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Finds the top K most frequently occurring words in a list of log lines.
///
/// # Arguments
///
/// * `logs` - A slice of strings containing log lines to process
/// * `k` - The number of top frequent words to return
///
/// # Returns
///
/// A vector of tuples containing the word and its frequency count,
/// sorted by frequency (descending) and alphabetically for ties.
///
/// # Example
///
/// ```
/// let logs = vec!["Error: disk full".to_string(), "error: network down".to_string()];
/// let result = top_k_words(&logs, 2);
/// // Returns [("error", 2), ("disk", 1)] or similar
/// ```
fn top_k_words(logs: &[String], k: usize) -> Vec<(String, usize)> {
    // HashMap to store word frequency counts
    let mut frequency_map: HashMap<String, usize> = HashMap::new();
    
    // Process each log line
    for line in logs {
        // Convert to lowercase for case-insensitive comparison
        let lower_line = line.to_lowercase();
        
        // Split line into words using non-alphanumeric characters as delimiters
        for word in lower_line.split(|c: char| !c.is_ascii_alphanumeric()) {
            // Skip empty strings that may result from splitting
            if word.is_empty() {
                continue;
            }
            // Increment count for existing word or insert new word with count 1
            *frequency_map.entry(word.to_string()).or_insert(0) += 1;
        }
    }
    
    // Convert HashMap to vector of tuples for sorting
    let mut word_counts: Vec<(String, usize)> = frequency_map.into_iter().collect();
    
    // Sort by frequency descending, then alphabetically ascending for ties
    word_counts.sort_by(|a, b| {
        // Primary sort: frequency descending
        b.1.cmp(&a.1)
            // Secondary sort: alphabetical order for words with same frequency
            .then_with(|| a.0.cmp(&b.0))
    });
    
    // Keep only the top K words
    word_counts.truncate(k);
    
    word_counts
}

/// Main function that handles command-line arguments and file processing
///
/// # Usage
///
/// ```bash
/// cargo run -- <filename> <k>
/// cargo run -- logs.txt 5
/// ```
///
/// # Arguments
///
/// * `filename` - Path to the log file to process
/// * `k` - Number of top words to display (positive integer)
fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Validate argument count
    if args.len() < 3 {
        eprintln!("Usage: {} <filename> <k>", args[0]);
        eprintln!("Example: {} logs.txt 5", args[0]);
        std::process::exit(1);
    }
    
    // Extract filename and k from arguments
    let filename = &args[1];
    let k: usize = args[2].parse().expect("k must be a positive number");
    
    // Open and read the log file
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    
    // Read all lines from the file into a vector
    let logs: Vec<String> = reader.lines()
        .map(|line| line.expect("Unable to read line"))
        .collect();
    
    // Process the logs and get top K words
    let result = top_k_words(&logs, k);
    
    // Print the result
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic functionality with case insensitivity
    #[test]
    fn test_basic_functionality() {
        let logs = vec![
            "Error: Disk full".to_string(),
            "error: network down".to_string(),
            "ERROR: disk error".to_string(), 
        ];
        
        let result = top_k_words(&logs, 2);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("error".to_string(), 4)); 
        assert_eq!(result[1], ("disk".to_string(), 2));
    }

    /// Test sorting order: frequency descending, then alphabetical
    #[test]
    fn test_sorting_order() {
        let logs = vec![
            "apple banana apple".to_string(),
            "banana cherry".to_string(),
            "apple cherry date".to_string(),
            "date egg".to_string(),
        ];
        
        let result = top_k_words(&logs, 4);
        
        // Expected order: apple(3), banana(2), cherry(2), date(2)
        assert_eq!(result[0], ("apple".to_string(), 3));
        assert_eq!(result[1], ("banana".to_string(), 2));
        assert_eq!(result[2], ("cherry".to_string(), 2));
        assert_eq!(result[3], ("date".to_string(), 2));
    }

    /// Test with alphanumeric words and special characters
    #[test]
    fn test_alphanumeric_words() {
        let logs = vec![
            "Error123 test 123".to_string(),  
            "error123 test test".to_string(), 
            "test123 456".to_string(),        
        ];
        
        let result = top_k_words(&logs, 3);
        
        
        assert_eq!(result[0], ("test".to_string(), 3));
        assert_eq!(result[1], ("error123".to_string(), 2));
        assert_eq!(result[2], ("123".to_string(), 1)); 
    }

    /// Test empty input
    #[test]
    fn test_empty_input() {
        let logs: Vec<String> = vec![];
        let result = top_k_words(&logs, 5);
        assert_eq!(result.len(), 0);
    }

    /// Test k larger than number of unique words
    #[test]
    fn test_k_larger_than_unique_words() {
        let logs = vec![
            "word1 word2".to_string(),
            "word1 word3".to_string(),
        ];
        
        let result = top_k_words(&logs, 10);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], ("word1".to_string(), 2));
    }

    /// Test k = 0
    #[test]
    fn test_k_zero() {
        let logs = vec!["test".to_string()];
        let result = top_k_words(&logs, 0);
        assert_eq!(result.len(), 0);
    }

    /// Test with punctuation and special characters
    #[test]
    fn test_punctuation_handling() {
        let logs = vec![
            "Error, disk; full!".to_string(),
            "error: network-down".to_string(),
            "error (disk) full?".to_string(),
        ];
        
        let result = top_k_words(&logs, 3);
        
        assert_eq!(result[0], ("error".to_string(), 3));
        assert_eq!(result[1], ("disk".to_string(), 2));
        assert_eq!(result[2], ("full".to_string(), 2));
    }

    /// Test exact matching instead of contains
    #[test]
    fn test_exact_matching() {
        let logs = vec![
            "test test test".to_string(),
            "hello world".to_string(),
        ];
        
        let result = top_k_words(&logs, 2);
        let expected = vec![("test".to_string(), 3), ("hello".to_string(), 1)];
        assert_eq!(result, expected);
    }
}