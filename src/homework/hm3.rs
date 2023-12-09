/*
  三、利用歌曲中的重複內容印製聖誕頌歌「The Twelve Days of Christmas」的歌詞
*/

const ANSWER: &str = "On the first day of Christmas,
My true love gave to me,
A partridge in a pear tree.

On the second day of Christmas,
My true love gave to me,
Two turtle doves,
And a partridge in a pear tree.

On the third day of Christmas,
My true love gave to me,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the fourth day of Christmas,
My true love gave to me,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the fifth day of Christmas,
My true love gave to me,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the sixth day of Christmas,
My true love gave to me,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the seventh day of Christmas,
My true love gave to me,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the eighth day of Christmas,
My true love gave to me,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the ninth day of Christmas,
My true love gave to me,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the tenth day of Christmas,
My true love gave to me,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the eleventh day of Christmas,
My true love gave to me,
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the twelfth day of Christmas,
My true love gave to me,
Twelve drummers drumming,
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.";

pub fn christmas_gen() -> String {
    let num_english: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let paragraph_list: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut lyrics = String::new();

    for paragraph in 0..12 {
        lyrics.push_str(&format!(
            "On the {} day of Christmas,\n",
            num_english[paragraph]
        ));
        lyrics.push_str("My true love gave to me,\n");

        for sentence in (0..=paragraph).rev() {
            if paragraph != 0 && sentence == 0 {
                lyrics.push_str("And ");
            }

            let sen = if paragraph != 0 && sentence == 0 {
                paragraph_list[sentence].to_lowercase()
            } else {
                paragraph_list[sentence].to_string()
            };

            lyrics.push_str(&sen);

            let end_of_sentence = if sentence == 0 { "." } else { "," };

            lyrics.push_str(end_of_sentence);

            if !(paragraph == 11 && sentence == 0) {
                lyrics.push_str("\n");
            }
        }

        if paragraph != 11 {
            lyrics.push_str("\n");
        }
    }

    return lyrics;
}

fn test() {
    let lyrics = christmas_gen();
    assert_eq!(ANSWER, lyrics, "運算結果與歌詞相符");
}

pub fn print_christmas() {
    println!("\n作業三、利用歌曲中的重複內容印製聖誕頌歌「The Twelve Days of Christmas」的歌詞\n");
    test();

    let lyrics = christmas_gen();
    println!("The Twelve Days of Christmas: \n\n{}\n", lyrics);
    println!("歌詞與印出結果相符: {}", lyrics == ANSWER);
}
