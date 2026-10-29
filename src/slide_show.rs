//! Module slide_show creates the contents of the xml files describing the slide show and the
//! wallpaper.

/// SlideShow groups the necessary information to create the slide show description file.
pub struct SlideShow<'a> {
    images: Vec<&'a str>,
    image_duration: usize,
    transition: bool,
    transition_duration: usize,
}

/// make_wallpaper_xml creates the content for a slide show description xml file.
pub fn make_description_xml(slide_show: SlideShow) -> String {
    let mut description = String::from(
        "\
<background>
    <starttime>
        <year>2000</year>
        <month>1</month>
        <day>1</day>
        <hour>00</hour>
        <minute>00</minute>
        <second>00</second>
    </starttime>
",
    );

    for i in 0..slide_show.images.len() {
        let img = slide_show.images[i];
        description.push_str(&format!(
            "
    <static>
        <duration>{}.0</duration>
        <file>{}</file>
    </static>
",
            slide_show.image_duration, img
        ));

        if slide_show.transition {
            let next_img = if i < slide_show.images.len() - 1 {
                slide_show.images[i + 1]
            } else {
                slide_show.images[0]
            };
            description.push_str(&format!(
                r#"
    <transition type="overlay">
        <duration>{}.0</duration>
        <from>{}</from>
        <to>{}</to>
    </transition>
"#,
                slide_show.transition_duration, img, next_img
            ));
        }
    }

    description.push_str("</background>\n");

    description
}

/// make_wallpaper_xml creates the content of the wallpaper xml file that links to the slide show.
pub fn make_wallpaper_xml(path: &str, name: &str) -> String {
    format!(
        "<?xml version=\"1.0\"?>
<!DOCTYPE wallpapers SYSTEM \"gnome-wp-list.dtd\">
<wallpapers>
    <wallpaper deleted=\"false\">
        <name>{name} timed</name>
        <filename>{path}</filename>
        <options>zoom</options>
        <pcolor>#ffffff</pcolor>
        <scolor>#000000</scolor>
    </wallpaper>
</wallpapers>
"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_description_without_transition() {
        let images = vec![
            "/some-dir/img1.png",
            "/some-dir/img2.png",
            "/some-dir/img3.png",
        ];

        let slide_show = SlideShow {
            images: images,
            image_duration: 777,
            transition: false,
            transition_duration: 0,
        };

        let expected = "\
<background>
    <starttime>
        <year>2000</year>
        <month>1</month>
        <day>1</day>
        <hour>00</hour>
        <minute>00</minute>
        <second>00</second>
    </starttime>

    <static>
        <duration>777.0</duration>
        <file>/some-dir/img1.png</file>
    </static>

    <static>
        <duration>777.0</duration>
        <file>/some-dir/img2.png</file>
    </static>

    <static>
        <duration>777.0</duration>
        <file>/some-dir/img3.png</file>
    </static>
</background>
";
        let actual = make_description_xml(slide_show);
        assert!(actual == expected);
    }

    #[test]
    fn test_make_description_with_transition() {
        let images = vec![
            "/some-dir/img1.png",
            "/some-dir/img2.png",
            "/some-dir/img3.png",
        ];

        let slide_show = SlideShow {
            images: images,
            image_duration: 777,
            transition: true,
            transition_duration: 10,
        };

        let expected = "\
<background>
    <starttime>
        <year>2000</year>
        <month>1</month>
        <day>1</day>
        <hour>00</hour>
        <minute>00</minute>
        <second>00</second>
    </starttime>

    <static>
        <duration>777.0</duration>
        <file>/some-dir/img1.png</file>
    </static>

    <transition type=\"overlay\">
        <duration>10.0</duration>
        <from>/some-dir/img1.png</from>
        <to>/some-dir/img2.png</to>
    </transition>

    <static>
        <duration>777.0</duration>
        <file>/some-dir/img2.png</file>
    </static>

    <transition type=\"overlay\">
        <duration>10.0</duration>
        <from>/some-dir/img2.png</from>
        <to>/some-dir/img3.png</to>
    </transition>

    <static>
        <duration>777.0</duration>
        <file>/some-dir/img3.png</file>
    </static>

    <transition type=\"overlay\">
        <duration>10.0</duration>
        <from>/some-dir/img3.png</from>
        <to>/some-dir/img1.png</to>
    </transition>
</background>
";
        let actual = make_description_xml(slide_show);
        assert!(actual == expected);
    }

    #[test]
    fn test_make_wallpaper() {
        let expected = "\
<?xml version=\"1.0\"?>
<!DOCTYPE wallpapers SYSTEM \"gnome-wp-list.dtd\">
<wallpapers>
    <wallpaper deleted=\"false\">
        <name>my awesome wallpaper timed</name>
        <filename>/path/to/wallpaper.xml</filename>
        <options>zoom</options>
        <pcolor>#ffffff</pcolor>
        <scolor>#000000</scolor>
    </wallpaper>
</wallpapers>
";
        let actual = make_wallpaper_xml("/path/to/wallpaper.xml", "my awesome wallpaper");

        assert!(actual == expected)
    }
}
