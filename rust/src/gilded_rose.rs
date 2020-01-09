#[cfg(test)]
mod unit_tests;

use std::string;
use std::vec;

pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub const fn new(name: String, sell_in: i32, quality: i32) -> Self {
        Self {
            name,
            sell_in,
            quality,
        }
    }
}

pub struct GildedRose {
    pub items: vec::Vec<Item>,
}

impl GildedRose {
    pub const fn new(items: vec::Vec<Item>) -> Self {
        Self { items }
    }

    #[allow(clippy::integer_arithmetic)]
    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if item.quality > 0 && item.name != "Sulfuras, Hand of Ragnaros" {
                    item.quality -= 1;
                }
            } else if item.quality < 50 {
                item.quality += 1;

                if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                    if item.sell_in < 11 && item.quality < 50 {
                        item.quality += 1;
                    }

                    if item.sell_in < 6 && item.quality < 50 {
                        item.quality += 1;
                    }
                }
            }

            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.sell_in = item.sell_in.saturating_sub(1);
            }

            if item.sell_in < 0 {
                if item.name != "Aged Brie" {
                    if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                        item.quality = 0;
                    } else if item.quality > 0 && item.name != "Sulfuras, Hand of Ragnaros" {
                        item.quality -= 1;
                    }
                } else if item.quality < 50 {
                    item.quality += 1;
                }
            }
        }
    }
}
