use crate::css::computed::ComputedStyle;

pub struct Inheritance;

impl Inheritance {
    pub fn inherit(parent: &ComputedStyle, child: &mut ComputedStyle) {
        let inheritable = vec![
            "color",
            "font-family",
            "font-size",
            "font-weight",
            "line-height",
            "text-align",
            "text-transform",
            "visibility",
            "white-space",
            "word-spacing",
            "letter-spacing",
        ];
        for prop in inheritable {
            if let Some(val) = parent.get(prop) {
                if !child.properties.contains_key(prop) {
                    child.set(prop, val.clone());
                }
            }
        }
    }
}
