use super::*;

#[test]
pub fn sulfuras_sell_in_does_not_decrease_in_quality() {
    // given
    let expected_quality = 6;
    let name = String::from("Sulfuras, Hand of Ragnaros");
    let sell_in = 4;
    let quality = expected_quality;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}

#[test]
pub fn sulfuras_sell_in_does_not_age_before_sell_in_date() {
    // given
    let expected_sell_in = 6;
    let name = String::from("Sulfuras, Hand of Ragnaros");
    let sell_in = expected_sell_in;
    let quality = 49;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].sell_in, expected_sell_in);
}


#[test]
pub fn sulfuras_sell_in_does_not_age_after_sell_in_date() {
    // given
    let expected_sell_in = -3;
    let name = String::from("Sulfuras, Hand of Ragnaros");
    let sell_in = expected_sell_in;
    let quality = 49;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].sell_in, expected_sell_in);
}

