use super::*;

proptest! {
    #[test]
    fn backstage_pass_quality_increases_by_1_as_it_ages_long_before_sell_in_date(
            sell_in in 11..=i32::max_value(),
            quality in 0..=49) {
        // given
        let expected_quality = quality + 1;
        let name = String::from("Backstage passes to a TAFKAL80ETC concert");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn backstage_pass_quality_increases_by_three_near_sell_in_date(
            sell_in in 1..=5,
            quality in 0..=47) {
        // given
        let expected_quality = quality + 3;
        let name = String::from("Backstage passes to a TAFKAL80ETC concert");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn backstage_pass_quality_increases_by_two_somewhat_near_sell_in_date(
            sell_in in 6..=10,
            quality in 0..=48) {
        // given
        let expected_quality = quality + 2;
        let name = String::from("Backstage passes to a TAFKAL80ETC concert");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn backstage_pass_quality_does_not_exceed_50_before_sell_in_date(
            sell_in in 1..=i32::max_value(),
            quality in 49..=50) {
        // given
        let expected_quality = 50;
        let name = String::from("Backstage passes to a TAFKAL80ETC concert");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn backstage_pass_quality_drops_to_0_at_and_after_sell_in_date(
            sell_in in i32::min_value()..=0,
            quality in 0..=50) {
        // given
        let expected_quality = 0;
        let name = String::from("Backstage passes to a TAFKAL80ETC concert");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }
}
