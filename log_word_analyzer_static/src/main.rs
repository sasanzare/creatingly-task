use std::collections::HashMap;

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

/// Main function with static log data for demonstration
///
/// # Example Output
///
/// [("error", 3), ("disk", 2)]
fn main() {
    // Static log data for demonstration
    let logs = vec![
        "Error: Disk full".to_string(),
        "Warning: Memory low".to_string(),
        "error: network down".to_string(),
        "Error: Disk full".to_string(),
    ];
    
    // Process the logs and get top 2 words
    let result = top_k_words(&logs, 2);
    
    // Print the result
    println!("{:?}", result);
}

/// Unit tests for the top_k_words function
#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic functionality with multiple log entries
    #[test]
    fn test_basic_functionality() {
        let logs = vec![
            "Error: Disk full".to_string(),
            "Warning: Memory low".to_string(),
            "error: network down".to_string(),
            "Error: Disk full".to_string(),
        ];
        
        let result = top_k_words(&logs, 2);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("error".to_string(), 3));
        assert_eq!(result[1], ("disk".to_string(), 2));
    }

    /// Test case insensitivity of word counting
    #[test]
    fn test_case_insensitivity() {
        let logs = vec![
            "ERROR: test".to_string(),
            "Error: TEST".to_string(),
            "error: Test".to_string(),
        ];
        
        let result = top_k_words(&logs, 2);
        
        assert_eq!(result[0], ("error".to_string(), 3));
        assert_eq!(result[1], ("test".to_string(), 3));
    }

    /// Test sorting by frequency descending
    #[test]
    fn test_sorting_order() {
        let logs = vec![
            "apple banana".to_string(),
            "banana cherry".to_string(),
            "apple apple".to_string(),
        ];
        
        let result = top_k_words(&logs, 3);
        
        assert_eq!(result[0], ("apple".to_string(), 3));
        assert_eq!(result[1], ("banana".to_string(), 2));
        assert_eq!(result[2], ("cherry".to_string(), 1));
    }

    /// Test alphabetical sorting for words with same frequency
    #[test]
    fn test_alphabetical_sorting_for_ties() {
        let logs = vec![
            "apple zebra".to_string(),  
            "zebra apple".to_string(),  
            "banana cherry".to_string(), 
        ];
        
        let result = top_k_words(&logs, 4);
    
        assert_eq!(result[0], ("apple".to_string(), 2));
        assert_eq!(result[1], ("zebra".to_string(), 2));
        assert_eq!(result[2], ("banana".to_string(), 1));
        assert_eq!(result[3], ("cherry".to_string(), 1));
    }

    /// Test when k is larger than number of unique words
    #[test]
    fn test_k_larger_than_unique_words() {
        let logs = vec![
            "hello world".to_string(),
            "hello rust".to_string(),
        ];
        
        let result = top_k_words(&logs, 5);
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], ("hello".to_string(), 2));
    }

    /// Test k = 0 (should return empty vector)
    #[test]
    fn test_k_zero() {
        let logs = vec!["test".to_string()];
        let result = top_k_words(&logs, 0);
        assert_eq!(result.len(), 0);
    }

    /// Test empty input (should return empty vector)
    #[test]
    fn test_empty_input() {
        let logs: Vec<String> = vec![];
        let result = top_k_words(&logs, 5);
        assert_eq!(result.len(), 0);
    }

    /// Test handling of punctuation and special characters
    #[test]
    fn test_punctuation_handling() {
        let logs = vec![
            "Error disk full".to_string(),
            "error network down".to_string(),
        ];
        
        let result = top_k_words(&logs, 5);
        
        assert_eq!(result[0], ("error".to_string(), 2));
        assert!(result.iter().any(|(word, _)| word == "disk"));
        assert!(result.iter().any(|(word, _)| word == "full"));
        assert!(result.iter().any(|(word, _)| word == "network"));
        assert!(result.iter().any(|(word, _)| word == "down"));
    }

    /// Test exact output format and content
    #[test]
    fn test_exact_output_format() {
        let logs = vec![
            "test test test".to_string(),
            "hello hello".to_string(),
        ];
        
        let result = top_k_words(&logs, 2);
        let expected = vec![("test".to_string(), 3), ("hello".to_string(), 2)];
        assert_eq!(result, expected);
    }
}