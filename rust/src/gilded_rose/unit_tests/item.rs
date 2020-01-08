use super::*;

#[test]
pub fn item_quality_does_not_go_below_0() {
    // given
    let expected_quality = 0;
    let name = String::from("foo");
    let sell_in = 0;
    let quality = expected_quality;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}

#[test]
pub fn non_zero_item_quality_decreases_by_1_before_sell_in_date() {
    // given
    let expected_quality = 0;
    let name = String::from("foo");
    let sell_in = 0;
    let quality = 1;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}

#[test]
pub fn non_zero_item_quality_decreases_by_2_at_sell_in_date() {
    // given
    let expected_quality = 1;
    let name = String::from("foo");
    let sell_in = 0;
    let quality = 3;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}

#[test]
pub fn non_zero_item_quality_decreases_by_2_after_sell_in_date() {
    // given
    let expected_quality = 1;
    let name = String::from("foo");
    let sell_in = -2;
    let quality = 3;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].quality, expected_quality);
}


#[test]
pub fn item_name_is_retained_through_update_quality() {
    // given
    let expected_name = String::from("foo");
    let name = expected_name.clone();
    let sell_in = 0;
    let quality = 0;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].name, expected_name);
}

#[test]
pub fn item_sell_in_goes_below_0() {
    // given
    let expected_sell_in = -1;
    let name = String::from("foo");
    let sell_in = 0;
    let quality = 0;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].sell_in, expected_sell_in);
}
