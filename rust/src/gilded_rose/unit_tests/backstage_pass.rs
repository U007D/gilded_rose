use super::*;

#[test]
pub fn backstage_pass_quality_increases_by_1_as_it_ages_long_before_sell_in_date() {
    // given
    let expected_quality = 1;
    let name = String::from("Backstage passes to a TAFKAL80ETC concert");
    let sell_in = 11;
    let quality = 0;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}

#[test]
pub fn backstage_pass_quality_increases_by_three_near_sell_in_date() {
    // given
    let expected_quality = 3;
    let name = String::from("Backstage passes to a TAFKAL80ETC concert");
    let sell_in = 5;
    let quality = 0;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}

#[test]
pub fn backstage_pass_quality_increases_by_two_somewhat_near_sell_in_date() {
    // given
    let expected_quality = 2;
    let name = String::from("Backstage passes to a TAFKAL80ETC concert");
    let sell_in = 10;
    let quality = 0;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}

#[test]
pub fn backstage_pass_quality_does_not_exceed_50_before_sell_in_date() {
    // given
    let expected_quality = 50;
    let name = String::from("Backstage passes to a TAFKAL80ETC concert");
    let sell_in = 5;
    let quality = 50;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}

#[test]
pub fn backstage_pass_quality_drops_to_0_after_sell_in_date() {
    // given
    let expected_quality = 0;
    let name = String::from("Backstage passes to a TAFKAL80ETC concert");
    let sell_in = -1;
    let quality = 49;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}
