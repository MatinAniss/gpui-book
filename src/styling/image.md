## Image

The `Img` element implements the `StyledImage` trait which allows styling of a image through the `.image_style()` function or with the utility CSS-like styling functions.

### Grayscale

The `grayscale` style controls if the image should be rendered in grayscale.

### Object Fit

The `object_fit` style controls how the image should fit in the parent element. The `ObjectFit` enum contains `Fill`, `Contain`, `Cover`, `ScaleDown`, `None`.

### Loading

The `loading` style controls the optional loading function that allows you to render a loading element while the element is loading.

### Fallback

The `fallback` style controls the optional fallback function that allows you to render a fallback element if the element fails to load.
