use super::*;

proptest! {
    #[test]
    fn arbitrary_names_dont_crash(name in "\\PC*") {
        // given
        let sell_in = 0;
        let quality = 0;
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        // no panic occurred
    }

    #[test]
    fn item_quality_does_not_go_below_0(
            sell_in in i32::min_value()..=i32::max_value(),
            quality in 0_i32..=1) {
        // given
        let expected_quality = 0;
        let name = String::from("foo");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn item_sell_in_goes_below_0(
            sell_in in i32::min_value()..=0) {
        // given
        let expected_sell_in = sell_in.saturating_sub(1);
        let name = String::from("foo");
        let quality = 0;
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].sell_in, expected_sell_in);
    }

    #[test]
    fn non_zero_item_quality_decreases_by_1_before_sell_in_date(
            sell_in in 1..=i32::max_value(),
            quality in 1..=i32::max_value()) {
        // given
        let expected_quality = quality - 1;
        let name = String::from("foo");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn non_zero_item_quality_decreases_by_2_at_and_after_sell_in_date(
            sell_in in i32::min_value()..=0,
            quality in 1..=i32::max_value()) {
        // given
        let expected_quality = i32::max(quality - 2, 0);
        let name = String::from("foo");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }
}

#[test]
pub fn item_sell_in_saturates_at_i32_min_value() {
    // given
    let expected_sell_in = i32::min_value();
    let name = String::from("foo");
    let sell_in = i32::min_value();
    let quality = 0;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].sell_in, expected_sell_in);
}
