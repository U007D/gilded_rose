use super::*;

#[test]
pub fn aged_brie_quality_increases_as_it_ages_before_sell_in_date() {
    // given
    let expected_quality = 1;
    let name = String::from("Aged Brie");
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
pub fn aged_brie_quality_increases_by_two_at_sell_in_date() {
    // given
    let expected_quality = 2;
    let name = String::from("Aged Brie");
    let sell_in = 0;
    let quality = 0;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}

#[test]
pub fn aged_brie_quality_increases_by_two_past_sell_in_date() {
    // given
    let expected_quality = 2;
    let name = String::from("Aged Brie");
    let sell_in = -1;
    let quality = 0;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}

#[test]
pub fn aged_brie_quality_does_not_exceed_50_before_sell_in_date() {
    // given
    let expected_quality = 50;
    let name = String::from("Aged Brie");
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
pub fn aged_brie_quality_does_not_exceed_50_after_sell_in_date() {
    // given
    let expected_quality = 50;
    let name = String::from("Aged Brie");
    let sell_in = -2;
    let quality = 49;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}
