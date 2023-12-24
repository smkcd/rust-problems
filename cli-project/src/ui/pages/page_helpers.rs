use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    // use the truncate_ellipse function from the ellipse crate
    // let truncated = text.truncate_ellipse(width).as_ref().to_owned();
    // let truncated_slice = &truncated[..];
    // truncated_slice.truncate_ellipse_with(width, "").as_ref().to_owned()
    let len = text.chars().count();

    match len.cmp(&width) {
        std::cmp::Ordering::Less => {
            let leftover = width - len;
            let mut result = text.to_owned();
            for _i in 1..=leftover {
                result.push(' ');
            }
            result
        },
        std::cmp::Ordering::Equal => text.to_owned(),
        std::cmp::Ordering::Greater => {
            match width {
                0 => "".to_owned(),
                1 => ".".to_owned(),
                2 => "..".to_owned(),
                3 => "...".to_owned(),
                _ => text.truncate_ellipse(width-3).as_ref().to_owned(),
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    } 
}