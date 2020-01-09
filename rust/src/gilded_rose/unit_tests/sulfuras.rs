use super::*;

proptest! {
    #[test]
    fn sulfuras_sell_in_does_not_decrease_in_quality(
            sell_in in i32::min_value()..=i32::max_value(),
            quality in 0..=50) {
        // given
        let expected_quality = quality;
        let name = String::from("Sulfuras, Hand of Ragnaros");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn sulfuras_sell_in_does_not_age(
            sell_in in i32::min_value()..=i32::max_value(),
            quality in 0..=50) {
        // given
        let expected_sell_in = sell_in;
        let name = String::from("Sulfuras, Hand of Ragnaros");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].sell_in, expected_sell_in);
    }
}
