## Element

Any element that implements the `Styled` trait allows its styles to be modified through the `.style()` function or with the utility CSS-like styling functions.

### Align Content

The `align_content` style controls how content contained in this element should be aligned. The `AlignContent` enum contains `Start`, `End`, `FlexStart`, `FlexEnd`, `Center`, `Stretch`, `SpaceBetween`, `SpaceEvenly`, and `SpaceAround`.

### Align Items

The `align_items` style controls how the children of the element should be aligned. The `AlignItems` enum contains `Start`, `End`, `FlexStart`, `FlexEnd`, `Center`, `Baseline`, `Stretch`.

### Align Self

The `align_self` style controls how the element should be aligned. The `AlignSelf` enum contains `Start`, `End`, `FlexStart`, `FlexEnd`, `Center`, `Baseline`, `Stretch`.

### Aspect Ratio

The `aspect_ratio` style controls the proportional relationship between the width and height of the element.

### Background

The `background` style controls the background fill option of the element.

### Border Color

The `border_color` style controls the border color of the element.

### Box Shadow

The `box_shadow` style controls the box shadow applied to the element.

### Corner Radii

The `corner_radii` style controls the radius of the corners of the element.

### Display

The `display` style controls how the children of the element are laid out. The `Display` enum contains `Block`, `Flex`, `Grid`, and `None`.

### Flex Basis

The `flex_basis` style controls the initial size of the flex item element.

### Flex Direction

The `flex_direction` style controls which direction the children a flex element will be laid out. The `FlexDirection` enum contains `Row`, `Column`, `RowReverse`, and `ColumnReverse`.

### Flex Grow

The `flex_grow` style controls the relative rate at which the flex item element grows to fill space.

### Flex Shrink

The `flex_shrink` style controls the relative rate at which the flex item element shrinks to fit into space.

### Flex Wrap

The `flex_wrap` style controls if the element should wrap its children. The `FlexWrap` enum contains `NoWrap`, `Wrap`, and `WrapReverse`.

### Gap

The `gap` style controls how much gap is between children of the element.

### Inset

The `inset` style controls the offset of the element relative to its containing block.

### Justify Content

The `justify_content` style controls the distribution of space between and around elements. The `JustifyContent` enum contains `Start`, `End`, `FlexStart`, `FlexEnd`, `Center`, `Stretch`, `SpaceBetween`, `SpaceEvenly`, and `SpaceAround`.

### Margin

The `margin` style controls how much margin should be added on each side of the element.

### Max Size

The `max_size` style controls the maximum size of the element.

### Min Size

The `max_size` style controls the minimum size of the element.

### Mouse Cursor

The `mouse_cursor` style controls the mouse cursor style that is shown when the mouse pointer is hovered over the element.

### Opacity

The `opacity` style controls the transparency level of an element, where the value range from `0` which is completely transparent to `1` which is completely opaque.

### Overflow

The `overflow` style controls how children of the element should overflow the container. The value is wrapped with the `Point<T>` type, this allows `Overflow` to have both a `x` and `y` option. The `Overflow` enum contains `Visible`, `Clip`, `Hidden`, and `Scroll`.

### Padding

The `padding` style controls how much padding should be added on each side of the element.

### Position

The `position` style controls the positioning of the element is determined. The `Position` enum contains `Relative` and `Absolute`.

### Size

The `size` style controls the size of the element.

### Text

The `text` style controls how child text elements are drawn. A mutable reference to the text style of a element can be acquired from the `.text_style()` function.

To learn more about configuring the text style, read the [Text](text.md) chapter.

### Visibility

The `visibility` style controls if the element should be painted. The `Visibility` enum contains `Visible` and `Hidden`.
