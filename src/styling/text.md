## Text

Any element that implements the `Styled` trait allows its text style to be modified through the `.text_style()` function or with the utility CSS-like styling functions.

### Color

The `color` style controls the color of the text.

### Background Color

The `background_color` style controls the background color of the text.

### Font Family

The `font_family` style controls the font family that is used when rendering the text.

### Font Features

The `font_features` style controls the OpenType features that can be configured for a given font.

### Font Size

The `font_size` style controls the size of the font.

### Font Style

The `font_style` style controls the style of the font. The `FontStyle` enum contains `Normal`, `Italic`, and `Oblique`.

### Font Weight

The `font_weight` style controls the weight of the font.

### Line Clamp

The `line_clamp` style controls the number of lines before truncating the text.

### Line Height

The `line_height` style controls the height of the text line.

### Strikethrough

The `strikethrough` style controls strikethrough style of the text. The `StrikethroughStyle` struct allows you to configure the thickness and color of the strikethrough.

### Text Align

The `text_align` style controls the alignment of the text. The `TextAlign` enum contains `Left`, `Center`, and `Right`.

### Text Overflow

The `text_overflow` style controls if the text should be truncated if it overflows the width of the element.

### Underline

The `underline` style controls the underline style of the text. The `UnderlineStyle` struct allows you to configure the thickness, color, and if it should be wavy.

### White Space

The `white_space` style controls how white space is handled when text wrapping. The `WhiteSpace` enum contains `Normal` and `Nowrap`.
