use crate::types::genetic_sequence::GeneticSequence;

#[test]
fn test_byte_parsing() {
    for length in 0..100 {
        let sequence = GeneticSequence::random(length);
        let bytes = sequence.get_bytes();
        let initial_sequence = GeneticSequence::from_bytes(&bytes);
        assert_eq!(sequence, initial_sequence);
    }
}
