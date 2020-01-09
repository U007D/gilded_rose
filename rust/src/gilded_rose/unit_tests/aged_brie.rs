#![allow(clippy::indexing_slicing)]
use super::*;

proptest! {
    #[test]
    fn aged_brie_quality_increases_by_1_as_it_ages_before_sell_in_date(
            sell_in in 1..=i32::max_value(),
            quality in 0..=49) {
        // given
        let expected_quality = quality + 1;
        let name = String::from("Aged Brie");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn aged_brie_quality_increases_by_two_at_and_after_sell_in_date(
        sell_in in i32::min_value()..=0,
        quality in 0..=48) {
        // given
        let expected_quality = quality + 2;
        let name = String::from("Aged Brie");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn aged_brie_quality_does_not_exceed_50(
            sell_in in i32::min_value()..=i32::max_value(),
            quality in 49..=50) {
        // given
        let expected_quality = 50;
        let name = String::from("Aged Brie");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }
}
